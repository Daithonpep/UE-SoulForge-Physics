use super::predictor::WorldModelPredictor;
use super::state::{StateBuffer, WorldState, StateTransition, AgentAction};
use super::correction::{CorrectionTrigger, DiscrepancyReport};
use serde::{Deserialize, Serialize};

pub struct WorldModelCoordinator {
    predictor: WorldModelPredictor,
    state_buffer: StateBuffer,
    correction_trigger: CorrectionTrigger,
    training_config: TrainingConfig,
    is_trained: bool,
    last_prediction: Option<super::predictor::PredictionResult>,
    training_status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingConfig {
    pub buffer_size: usize,
    pub successful_only: bool,
    pub retrain_interval: u64,
    pub learning_rate: f32,
    pub epochs_per_training: usize,
    pub min_examples: usize,
}

impl Default for TrainingConfig {
    fn default() -> Self {
        Self {
            buffer_size: 25000,
            successful_only: false, 
            retrain_interval: 100,
            learning_rate: 0.01,
            epochs_per_training: 50,
            min_examples: 30,
        }
    }
}

impl WorldModelCoordinator {
    pub fn new(config: TrainingConfig) -> Self {
        Self {
            predictor: WorldModelPredictor::default(),
            state_buffer: StateBuffer::new(config.buffer_size, false),
            correction_trigger: CorrectionTrigger::default(),
            training_config: config,
            is_trained: false,
            last_prediction: None,
            training_status: "Esperando datos...".to_string(),
        }
    }

    pub async fn should_query_unreal(
        &mut self,
        current_state: &WorldState,
        planned_action: &AgentAction,
    ) -> QueryDecision {
        if !self.is_trained {
            return QueryDecision {
                query_unreal: true,
                reason: "Modelo no entrenado aún".to_string(),
                prediction: None,
            };
        }

        let prediction = self.predictor.predict(
            &current_state.visual_state.feature_vector,
            planned_action,
        );

        println!(
            "🔮 Predicción: FPS={:.1}, DrawCalls={}, Confianza={:.1}%",
            prediction.predicted_fps,
            prediction.predicted_draw_calls,
            prediction.confidence * 100.0
        );

        if prediction.confidence < 0.5 {
            self.last_prediction = None;
            return QueryDecision {
                query_unreal: true,
                reason: format!("Confianza baja ({:.1}%)", prediction.confidence * 100.0),
                prediction: Some(prediction),
            };
        }

        self.last_prediction = Some(prediction.clone());

        QueryDecision {
            query_unreal: false,
            reason: "Predicción confiable".to_string(),
            prediction: Some(prediction),
        }
    }

    pub fn record_transition(&mut self, mut transition: StateTransition) -> Option<DiscrepancyReport> {
        let mut curiosity_bonus = 0.0;

        let discrepancy = if let Some(prediction) = self.get_last_prediction() {
            let report = self.correction_trigger.evaluate(
                &prediction,
                &transition.state_after,
            );

            // CURIOSIDAD: Si el modelo se sorprendió (error alto), premiamos la exploración
            // Bonus = Discrepancia * Factor de Curiosidad (eta = 0.2)
            curiosity_bonus = (report.overall_discrepancy * 0.2).min(0.5); 
            
            if curiosity_bonus > 0.05 {
                println!("✨ Bonus de Curiosidad aplicado: +{:.4} (Exploración detectada)", curiosity_bonus);
            }

            if report.requires_correction {
                println!("🔄 Corrección activada - Datos inyectados al buffer");
            }

            Some(report)
        } else {
            None
        };

        // R_total = R_extrinseca + R_intrinseca
        transition.reward += curiosity_bonus;
        
        self.state_buffer.push(transition);

        if self.state_buffer.get_all().len() % self.training_config.retrain_interval as usize == 0 {
            self.retrain();
        }

        discrepancy
    }

    pub fn add_warmup_data(&mut self, transitions: Vec<StateTransition>) {
        for t in transitions {
            self.state_buffer.push(t);
        }
        self.training_status = format!("Buffer: {} items. Listo para entrenar.", self.state_buffer.get_all().len());
        println!("✅ Buffer contiene {} items.", self.state_buffer.get_all().len());
    }

    /// Entrenar con parámetros configurables desde el dashboard
    pub fn retrain_with_params(&mut self, epochs: usize, learning_rate: f32) {
        let examples = self.state_buffer.export_for_training();
        let min_ex = 10; // Mínimo reducido para pruebas

        if examples.len() < min_ex {
            self.training_status = format!("⚠️ Solo {} ejemplos ({} mínimo)", examples.len(), min_ex);
            println!("{}", self.training_status);
            return;
        }

        println!("\n🔄 WORLD MODEL — ENTRENAMIENTO MANUAL");
        println!("═══════════════════════════════");
        
        self.training_status = format!("🏋️ Entrenando: {} epochs, lr={}, {} ejemplos", epochs, learning_rate, examples.len());

        self.predictor.train(&examples, learning_rate, epochs);

        self.is_trained = true;
        let final_loss = self.predictor.get_stats().average_loss;
        self.training_status = format!("✅ Completado. Loss: {:.6} ({} epochs)", final_loss, epochs);
        self.correction_trigger.adapt_threshold();

        println!("═══════════════════════════════\n");
    }

    pub fn retrain(&mut self) {
        let epochs = self.training_config.epochs_per_training;
        let lr = self.training_config.learning_rate;
        self.retrain_with_params(epochs, lr);
    }

    fn get_last_prediction(&self) -> Option<super::predictor::PredictionResult> {
        self.last_prediction.clone()
    }

    pub fn performance_report(&self) -> serde_json::Value {
        let buffer_stats = self.state_buffer.statistics();
        let correction_stats = self.correction_trigger.get_stats();
        let predictor_stats = self.predictor.get_stats();

        // Incluir log de entrenamiento para el dashboard
        let training_log: Vec<serde_json::Value> = self.predictor.training_log
            .iter()
            .map(|entry| serde_json::json!({
                "epoch": entry.epoch,
                "loss": entry.loss,
                "time": entry.timestamp,
            }))
            .collect();

        serde_json::json!({
            "world_model": {
                "is_trained": self.is_trained,
                "status_text": self.training_status,
                "config": {
                    "architecture": "16 → 64 → 32 → 4",
                    "epochs": self.training_config.epochs_per_training,
                    "learning_rate": self.training_config.learning_rate,
                    "min_examples": self.training_config.min_examples,
                },
                "predictor": {
                    "total_predictions": predictor_stats.total_predictions,
                    "training_steps": predictor_stats.total_training_steps,
                    "average_loss": predictor_stats.average_loss,
                    "best_loss": predictor_stats.best_loss,
                    "loss_history": predictor_stats.last_epoch_losses,
                },
                "training_log": training_log,
                "correction_system": {
                    "threshold": self.correction_trigger.discrepancy_threshold,
                    "total_corrections": correction_stats.total_corrections,
                    "correction_rate": correction_stats.correction_rate,
                    "average_discrepancy": correction_stats.average_discrepancy,
                    "max_discrepancy": correction_stats.max_discrepancy,
                },
                "state_buffer": {
                    "total_transitions": buffer_stats.total_transitions,
                    "success_rate": buffer_stats.success_rate,
                    "average_reward": buffer_stats.average_reward,
                    "average_fps": buffer_stats.average_fps,
                },
            }
        })
    }

    pub fn save(&self, base_path: &str) -> std::io::Result<()> {
        self.predictor.save(&format!("{}/predictor.json", base_path))?;
        self.state_buffer.save(&format!("{}/buffer.json", base_path))?;
        
        let config_json = serde_json::to_string_pretty(&self.training_config)?;
        std::fs::write(format!("{}/config.json", base_path), config_json)?;

        println!("💾 World Model guardado en: {}", base_path);
        Ok(())
    }

    pub fn load(base_path: &str) -> std::io::Result<Self> {
        let predictor = WorldModelPredictor::load(&format!("{}/predictor.json", base_path))?;
        
        let mut state_buffer = StateBuffer::new(1000, false);
        if let Ok(()) = state_buffer.load(&format!("{}/buffer.json", base_path)) {
            println!("📂 Buffer cargado");
        }

        let config_json = std::fs::read_to_string(format!("{}/config.json", base_path))?;
        let training_config: TrainingConfig = serde_json::from_str(&config_json)?;

        Ok(Self {
            predictor,
            state_buffer,
            correction_trigger: CorrectionTrigger::default(),
            training_config,
            is_trained: true,
            last_prediction: None,
            training_status: "Cargado desde disco.".to_string(),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryDecision {
    pub query_unreal: bool,
    pub reason: String,
    pub prediction: Option<super::predictor::PredictionResult>,
}
