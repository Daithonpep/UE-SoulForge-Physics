use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use candle_core::{Device, Tensor, DType, Module};
use candle_nn::{VarBuilder, VarMap, Optimizer};

/// Red neuronal usando Candle — arquitectura compacta para datos reales del Gym
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldModelPredictor {
    /// Arquitectura de la red [input, hidden..., output]
    architecture: Vec<usize>,
    /// Pesos y biases persistidos
    weights_data: HashMap<String, Vec<f32>>,
    /// Estadísticas de entrenamiento
    training_stats: TrainingStatistics,
    /// Log de entrenamiento para el dashboard
    #[serde(skip)]
    pub training_log: Vec<TrainingLogEntry>,
    /// Cache de predicciones recientes
    #[serde(skip)]
    prediction_cache: HashMap<String, PredictionResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PredictionResult {
    pub predicted_visual_state: Vec<f32>,
    pub predicted_fps: f32,
    pub predicted_draw_calls: u32,
    pub confidence: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TrainingStatistics {
    pub total_predictions: u64,
    pub total_training_steps: u64,
    pub average_loss: f32,
    pub best_loss: f32,
    pub last_epoch_losses: Vec<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingLogEntry {
    pub epoch: usize,
    pub loss: f32,
    pub timestamp: String,
}

impl WorldModelPredictor {
    /// Arquitectura: 16 inputs → 64 → 32 → 4 outputs
    /// Inputs: [obj_count, complexity, reward, fps_norm, draw_calls_norm, 
    ///          action_onehot(5), position(3), similarity, mass]
    /// Outputs: [predicted_reward, predicted_fps, predicted_similarity, predicted_draw_calls]
    pub fn new(architecture: &[usize]) -> Self {
        let mut weights_data = HashMap::new();
        let mut rng = fastrand::Rng::new();

        for i in 0..architecture.len() - 1 {
            let in_dim = architecture[i];
            let out_dim = architecture[i+1];
            // Kaiming He initialization for ReLU networks
            let scale = (2.0 / in_dim as f32).sqrt();
            
            let w_key = format!("layer{}.weight", i);
            let b_key = format!("layer{}.bias", i);
            
            let weights: Vec<f32> = (0..in_dim * out_dim)
                .map(|_| (rng.f32() - 0.5) * 2.0 * scale)
                .collect();
            let biases: Vec<f32> = (0..out_dim)
                .map(|_| rng.f32() * 0.01)
                .collect();
            
            weights_data.insert(w_key, weights);
            weights_data.insert(b_key, biases);
        }

        Self {
            architecture: architecture.to_vec(),
            weights_data,
            training_stats: TrainingStatistics::default(),
            training_log: Vec::new(),
            prediction_cache: HashMap::new(),
        }
    }

    fn get_optimal_device() -> Device {
        if let Ok(dev) = Device::new_cuda(0) {
            println!("🚀 Aceleración CUDA detectada");
            dev
        } else if let Ok(dev) = Device::new_metal(0) {
            println!("🍎 Aceleración Metal detectada");
            dev
        } else {
            Device::Cpu
        }
    }

    pub fn predict(&mut self, current_state: &[f32], action: &super::state::AgentAction) -> PredictionResult {
        let input_vec = self.encode_input(current_state, action);
        let cache_key = self.hash_input(&input_vec);

        if let Some(cached) = self.prediction_cache.get(&cache_key) {
            return cached.clone();
        }

        let device = Self::get_optimal_device();
        let result = match self.forward(&input_vec, &device) {
            Ok(output) => self.decode_output(&output),
            Err(e) => {
                eprintln!("❌ Error en predicción: {}", e);
                PredictionResult {
                    predicted_visual_state: vec![0.0; current_state.len()],
                    predicted_fps: 60.0,
                    predicted_draw_calls: 300,
                    confidence: 0.0,
                }
            }
        };

        self.prediction_cache.insert(cache_key, result.clone());
        self.training_stats.total_predictions += 1;
        result
    }

    fn forward(&self, input: &[f32], device: &Device) -> candle_core::Result<Vec<f32>> {
        let padded = self.pad_input(input);
        let mut current = Tensor::from_vec(padded, (1, self.architecture[0]), device)?;

        for i in 0..self.architecture.len() - 1 {
            let w_key = format!("layer{}.weight", i);
            let b_key = format!("layer{}.bias", i);
            
            let weights = self.weights_data.get(&w_key).unwrap();
            let biases = self.weights_data.get(&b_key).unwrap();
            
            let in_dim = self.architecture[i];
            let out_dim = self.architecture[i+1];
            
            let weight_t = Tensor::from_vec(weights.clone(), (out_dim, in_dim), device)?;
            let bias_t = Tensor::from_vec(biases.clone(), (out_dim,), device)?;
            
            current = current.matmul(&weight_t.t()?)?;
            current = current.broadcast_add(&bias_t)?;

            if i < self.architecture.len() - 2 {
                current = current.relu()?;
            }
        }

        current.flatten_all()?.to_vec1::<f32>()
    }

    fn pad_input(&self, input: &[f32]) -> Vec<f32> {
        let needed = self.architecture[0];
        let mut v = input.to_vec();
        v.resize(needed, 0.0);
        v.truncate(needed);
        v
    }

    /// Entrenamiento completo con parámetros configurables
    pub fn train(
        &mut self, 
        examples: &[super::state::TrainingExample], 
        learning_rate: f32, 
        epochs: usize
    ) {
        let device = Self::get_optimal_device();
        let batch_size = 64;
        let output_dim = *self.architecture.last().unwrap();
        let input_dim = self.architecture[0];
        
        println!("🧠 World Model Training");
        println!("   Arquitectura: {:?}", self.architecture);
        println!("   Ejemplos: {}", examples.len());
        println!("   Epochs: {}", epochs);
        println!("   Learning Rate: {}", learning_rate);
        println!("   Batch Size: {}", batch_size);

        // Construir datos de entrenamiento
        let mut all_inputs: Vec<f32> = Vec::new();
        let mut all_targets: Vec<f32> = Vec::new();
        let mut valid_count = 0usize;

        for ex in examples {
            let input = self.build_training_input(ex);
            let target = self.build_training_target(ex);
            
            if input.len() >= input_dim && target.len() >= output_dim {
                all_inputs.extend_from_slice(&input[..input_dim]);
                all_targets.extend_from_slice(&target[..output_dim]);
                valid_count += 1;
            }
        }

        if valid_count < 2 {
            println!("⚠️ Insuficientes ejemplos válidos ({}/2 mínimo)", valid_count);
            return;
        }

        println!("   Ejemplos válidos: {}", valid_count);

        // Diagnosticar datos
        let input_mean: f32 = all_inputs.iter().sum::<f32>() / all_inputs.len() as f32;
        let target_mean: f32 = all_targets.iter().sum::<f32>() / all_targets.len() as f32;
        let input_var: f32 = all_inputs.iter().map(|x| (x - input_mean).powi(2)).sum::<f32>() / all_inputs.len() as f32;
        let target_var: f32 = all_targets.iter().map(|x| (x - target_mean).powi(2)).sum::<f32>() / all_targets.len() as f32;
        
        println!("   📊 Input  — mean: {:.4}, var: {:.6}", input_mean, input_var);
        println!("   📊 Target — mean: {:.4}, var: {:.6}", target_mean, target_var);

        // Crear VarMap e inicializar capas
        let mut varmap = VarMap::new();
        let vb = VarBuilder::from_varmap(&varmap, DType::F32, &device);

        let mut layers = Vec::new();
        for i in 0..self.architecture.len() - 1 {
            let in_d = self.architecture[i];
            let out_d = self.architecture[i+1];
            let w_key = format!("layer{}.weight", i);
            let b_key = format!("layer{}.bias", i);
            
            if let (Some(w), Some(b)) = (self.weights_data.get(&w_key), self.weights_data.get(&b_key)) {
                let mut data = varmap.data().lock().unwrap();
                let w_t = Tensor::from_vec(w.clone(), (out_d, in_d), &device).unwrap();
                let b_t = Tensor::from_vec(b.clone(), (out_d,), &device).unwrap();
                data.insert(w_key.clone(), candle_core::Var::from_tensor(&w_t).unwrap());
                data.insert(b_key.clone(), candle_core::Var::from_tensor(&b_t).unwrap());
            }
            layers.push(candle_nn::linear(in_d, out_d, vb.pp(&format!("layer{}", i))).unwrap());
        }

        // Usar SGD que es estable en candle 0.10
        let mut opt = candle_nn::SGD::new(
            varmap.all_vars(), 
            learning_rate as f64
        ).expect("Error creando optimizador SGD");

        self.training_log.clear();
        self.training_stats.last_epoch_losses.clear();

        for epoch in 1..=epochs {
            let mut epoch_loss = 0.0f32;
            let mut batch_count = 0u32;

            // Iterar por batches
            let total_batches = (valid_count + batch_size - 1) / batch_size;
            for batch_idx in 0..total_batches {
                let start = batch_idx * batch_size;
                let end = ((batch_idx + 1) * batch_size).min(valid_count);
                let bs = end - start;
                if bs == 0 { continue; }

                let inp_start = start * input_dim;
                let inp_end = end * input_dim;
                let tar_start = start * output_dim;
                let tar_end = end * output_dim;

                let inp_slice = &all_inputs[inp_start..inp_end];
                let tar_slice = &all_targets[tar_start..tar_end];

                let input_t = Tensor::from_vec(inp_slice.to_vec(), (bs, input_dim), &device).unwrap();
                let target_t = Tensor::from_vec(tar_slice.to_vec(), (bs, output_dim), &device).unwrap();

                // Forward pass
                let mut out = input_t;
                for (li, layer) in layers.iter().enumerate() {
                    out = layer.forward(&out).unwrap();
                    if li < layers.len() - 1 {
                        out = out.relu().unwrap();
                    }
                }

                // MSE Loss
                let diff = out.sub(&target_t).unwrap();
                let loss = diff.sqr().unwrap().mean_all().unwrap();
                
                // Backward + step
                opt.backward_step(&loss).unwrap();

                let loss_val = loss.to_vec0::<f32>().unwrap();
                if !loss_val.is_nan() && !loss_val.is_infinite() {
                    epoch_loss += loss_val;
                    batch_count += 1;
                }
            }

            let avg_loss = if batch_count > 0 { epoch_loss / batch_count as f32 } else { f32::NAN };
            
            // Actualizar stats
            self.training_stats.average_loss = avg_loss;
            if avg_loss < self.training_stats.best_loss || self.training_stats.best_loss == 0.0 {
                self.training_stats.best_loss = avg_loss;
            }
            self.training_stats.last_epoch_losses.push(avg_loss);

            // Log entry
            self.training_log.push(TrainingLogEntry {
                epoch,
                loss: avg_loss,
                timestamp: chrono::Utc::now().format("%H:%M:%S").to_string(),
            });

            // Print progress
            if epoch == 1 || epoch % 5 == 0 || epoch == epochs {
                println!("   [Epoch {:>4}/{}] Loss: {:.6} | Batches: {}", epoch, epochs, avg_loss, batch_count);
            }

            let _ = std::fs::write("training_loss.tmp", format!("{:.8}", avg_loss));
        }

        // Guardar pesos actualizados
        let data = varmap.data().lock().unwrap();
        for i in 0..self.architecture.len() - 1 {
            let w_key = format!("layer{}.weight", i);
            let b_key = format!("layer{}.bias", i);
            if let Some(w_var) = data.get(&w_key) {
                let t = w_var.as_tensor().to_device(&Device::Cpu).unwrap();
                self.weights_data.insert(w_key, t.flatten_all().unwrap().to_vec1::<f32>().unwrap());
            }
            if let Some(b_var) = data.get(&b_key) {
                let t = b_var.as_tensor().to_device(&Device::Cpu).unwrap();
                self.weights_data.insert(b_key, t.flatten_all().unwrap().to_vec1::<f32>().unwrap());
            }
        }
        self.training_stats.total_training_steps += epochs as u64;
        self.prediction_cache.clear();
        
        println!("   ✅ Entrenamiento completado. Loss final: {:.6}", self.training_stats.average_loss);
    }

    /// Construye un vector de entrada de exactamente `architecture[0]` dimensiones
    /// usando datos reales y variados del ejemplo de entrenamiento
    fn build_training_input(&self, ex: &super::state::TrainingExample) -> Vec<f32> {
        let mut input = Vec::with_capacity(self.architecture[0]);
        
        // Feature vector (truncado o paddeado a los primeros slots)
        for i in 0..8 {
            input.push(*ex.input_state.get(i).unwrap_or(&0.0));
        }
        
        // Action encoding (one-hot, 5 dims)
        let action_enc = match ex.action.action_type {
            super::state::ActionType::SpawnPrimitive => [1.0, 0.0, 0.0, 0.0, 0.0],
            super::state::ActionType::SpawnAsset => [0.0, 1.0, 1.0, 0.0, 0.0],
            super::state::ActionType::ModifyTransform => [0.0, 1.0, 0.0, 0.0, 0.0],
            super::state::ActionType::ApplyMaterial => [0.0, 0.0, 1.0, 0.0, 0.0],
            super::state::ActionType::DeleteObject => [0.0, 0.0, 0.0, 1.0, 0.0],
            super::state::ActionType::PCGGeneration => [0.0, 0.0, 0.0, 0.0, 1.0],
        };
        input.extend_from_slice(&action_enc);
        
        // Posición (3 dims, normalizada)
        input.push(ex.action.parameters.position[0] / 100.0);
        input.push(ex.action.parameters.position[1] / 100.0);
        input.push(ex.action.parameters.position[2] / 100.0);
        
        // Pad/truncate a exactamente architecture[0]
        input.resize(self.architecture[0], 0.0);
        input.truncate(self.architecture[0]);
        input
    }

    /// Construye vector target: [reward_norm, fps_norm, similarity, draw_calls_norm]
    fn build_training_target(&self, ex: &super::state::TrainingExample) -> Vec<f32> {
        let output_dim = *self.architecture.last().unwrap();
        let mut target = Vec::with_capacity(output_dim);

        // Target features extraídos del estado resultante
        for i in 0..output_dim.saturating_sub(2) {
            target.push(*ex.predicted_state.get(i).unwrap_or(&0.0));
        }
        
        // FPS normalizado (0-1 range para 0-120 fps)
        target.push(ex.actual_metrics.fps / 120.0);
        // Draw calls normalizados
        target.push(ex.actual_metrics.draw_calls as f32 / 10000.0);
        
        target.resize(output_dim, 0.0);
        target.truncate(output_dim);
        target
    }

    fn decode_output(&self, output: &[f32]) -> PredictionResult {
        let out_dim = *self.architecture.last().unwrap();
        let visual_size = out_dim.saturating_sub(2);
        let predicted_visual_state = output[..visual_size.min(output.len())].to_vec();
        let predicted_fps = (output.get(visual_size).copied().unwrap_or(0.5) * 120.0).max(0.0);
        let predicted_draw_calls = (output.get(visual_size + 1).copied().unwrap_or(0.1) * 10000.0).max(0.0) as u32;
        PredictionResult { predicted_visual_state, predicted_fps, predicted_draw_calls, confidence: 0.9 }
    }

    fn encode_input(&self, state: &[f32], action: &super::state::AgentAction) -> Vec<f32> {
        let mut input = Vec::new();
        for i in 0..8 { input.push(*state.get(i).unwrap_or(&0.0)); }
        let action_enc = match action.action_type {
            super::state::ActionType::SpawnPrimitive => vec![1.0, 0.0, 0.0, 0.0, 0.0],
            super::state::ActionType::SpawnAsset => vec![0.0, 1.0, 1.0, 0.0, 0.0],
            super::state::ActionType::ModifyTransform => vec![0.0, 1.0, 0.0, 0.0, 0.0],
            super::state::ActionType::ApplyMaterial => vec![0.0, 0.0, 1.0, 0.0, 0.0],
            super::state::ActionType::DeleteObject => vec![0.0, 0.0, 0.0, 1.0, 0.0],
            super::state::ActionType::PCGGeneration => vec![0.0, 0.0, 0.0, 0.0, 1.0],
        };
        input.extend(action_enc);
        input.extend(&action.parameters.position);
        input
    }

    fn hash_input(&self, input: &[f32]) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        for &val in input { val.to_bits().hash(&mut hasher); }
        format!("{:x}", hasher.finish())
    }

    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub fn load(path: &str) -> std::io::Result<Self> {
        let json = std::fs::read_to_string(path)?;
        Ok(serde_json::from_str(&json)?)
    }

    pub fn get_stats(&self) -> &TrainingStatistics {
        &self.training_stats
    }
}

impl Default for WorldModelPredictor {
    fn default() -> Self {
        // Arquitectura compacta: 16 → 64 → 32 → 4
        Self::new(&[16, 64, 32, 4])
    }
}
