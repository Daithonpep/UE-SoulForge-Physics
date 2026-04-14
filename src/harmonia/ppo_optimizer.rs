// harmonia/ppo_optimizer.rs
use serde::{Deserialize, Serialize};
use crate::sofia::universal_validator::*;
use crate::sofia::primitives::FunctionalPrimitive;

/// Configuración de PPO
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PPOConfig {
    pub learning_rate: f32,
    pub gamma: f32,                    // Discount factor
    pub epsilon_clip: f32,             // Clipping parameter
    pub value_loss_coef: f32,
    pub entropy_coef: f32,
    pub max_grad_norm: f32,
    pub num_epochs: u32,
    pub batch_size: usize,
}

impl Default for PPOConfig {
    fn default() -> Self {
        Self {
            learning_rate: 0.0003,
            gamma: 0.99,
            epsilon_clip: 0.2,
            value_loss_coef: 0.5,
            entropy_coef: 0.01,
            max_grad_norm: 0.5,
            num_epochs: 10,
            batch_size: 64,
        }
    }
}

/// Estado del diseño para RL
#[derive(Debug, Clone)]
pub struct DesignState {
    /// Vector de características del diseño actual
    pub features: Vec<f32>,
    
    /// Historial de acciones
    pub action_history: Vec<DesignAction>,
    
    /// Recompensas acumuladas
    pub cumulative_reward: f32,
}

/// Acciones posibles en el espacio de diseño
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DesignAction {
    /// Añadir primitiva
    AddPrimitive { 
        primitive_type: FunctionalPrimitive,
        position: [f32; 3],
        scale: [f32; 3],
    },
    
    /// Modificar primitiva existente
    ModifyPrimitive {
        index: usize,
        delta_position: [f32; 3],
        delta_scale: [f32; 3],
        delta_rotation: [f32; 3],
    },
    
    /// Eliminar primitiva
    RemovePrimitive { index: usize },
    
    /// Aplicar simetría
    ApplySymmetry { axis: String },
    
    /// Finalizar diseño
    Finalize,
}

/// Experiencia de entrenamiento
#[derive(Debug, Clone)]
pub struct Experience {
    pub state: DesignState,
    pub action: DesignAction,
    pub reward: f32,
    pub next_state: DesignState,
    pub done: bool,
    pub log_prob: f32,
    pub value: f32,
}

/// Optimizador PPO para diseño generativo
pub struct PPOOptimizer {
    config: PPOConfig,
    replay_buffer: Vec<Experience>,
}

impl PPOOptimizer {
    pub fn new(config: PPOConfig) -> Self {
        Self {
            config,
            replay_buffer: Vec::new(),
        }
    }

    /// Convierte diseño a vector de estado
    pub fn design_to_state(&self, design: &UniversalDesign) -> DesignState {
        let mut features = Vec::new();

        // Características globales
        features.push(design.primitives.len() as f32 / 50.0); // Normalizado
        features.push(design.bounding_box.width / 10.0);
        features.push(design.bounding_box.height / 10.0);
        features.push(design.bounding_box.depth / 10.0);

        // Características por primitiva (primeras 10)
        for prim in design.primitives.iter().take(10) {
            features.extend_from_slice(&prim.position);
            features.extend_from_slice(&prim.scale);
            features.extend_from_slice(&prim.rotation);
        }

        // Padding si hay menos de 10
        while features.len() < 4 + 10 * 9 {
            features.push(0.0);
        }

        DesignState {
            features,
            action_history: Vec::new(),
            cumulative_reward: 0.0,
        }
    }

    /// Almacena experiencia
    pub fn store_experience(&mut self, experience: Experience) {
        self.replay_buffer.push(experience);
    }

    /// Calcula ventaja usando GAE (Generalized Advantage Estimation)
    pub fn compute_advantages(&self, experiences: &[Experience]) -> Vec<f32> {
        let mut advantages = Vec::with_capacity(experiences.len());
        let mut gae = 0.0;

        for i in (0..experiences.len()).rev() {
            let exp = &experiences[i];
            
            let delta = if exp.done {
                exp.reward - exp.value
            } else if i + 1 < experiences.len() {
                exp.reward + self.config.gamma * experiences[i + 1].value - exp.value
            } else {
                0.0
            };

            gae = delta + self.config.gamma * 0.95 * gae; // 0.95 = lambda (GAE parameter)
            advantages.push(gae);
        }

        advantages.reverse();
        advantages
    }

    /// Calcula retornos
    pub fn compute_returns(&self, experiences: &[Experience]) -> Vec<f32> {
        let mut returns = Vec::with_capacity(experiences.len());
        let mut cumulative = 0.0;

        for exp in experiences.iter().rev() {
            cumulative = exp.reward + self.config.gamma * cumulative;
            returns.push(cumulative);
        }

        returns.reverse();
        returns
    }

    /// Limpia buffer de replay
    pub fn clear_buffer(&mut self) {
        self.replay_buffer.clear();
    }

    /// Obtiene batch de entrenamiento
    pub fn sample_batch(&self, batch_size: usize) -> Vec<Experience> {
        let mut rng = fastrand::Rng::new();
        let mut batch = Vec::with_capacity(batch_size);

        for _ in 0..batch_size.min(self.replay_buffer.len()) {
            let idx = rng.usize(0..self.replay_buffer.len());
            batch.push(self.replay_buffer[idx].clone());
        }

        batch
    }
}
