use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// Representa un estado completo del mundo en un momento dado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldState {
    pub timestamp: u64,
    pub agent_action: AgentAction,
    pub visual_state: VisualState,
    pub physics_state: PhysicsState,
    pub performance_metrics: PerformanceMetrics,
}

/// Acción ejecutada por el agente
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentAction {
    pub action_type: ActionType,
    pub parameters: ActionParameters,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ActionType {
    SpawnPrimitive,
    SpawnAsset,       // Nueva acción para Furniture/Buildings/Cars
    ModifyTransform,
    ApplyMaterial,
    DeleteObject,
    PCGGeneration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActionParameters {
    pub primitive_type: Option<String>,      // "Cube", "Cylinder", etc.
    pub asset_id: Option<String>,           // "sofa_modern", "wheel_sport"
    pub asset_path: Option<String>,         // Ruta completa de Unreal
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
    pub pcg_seed: Option<u32>,
    pub pcg_density: Option<f32>,
}

/// Estado visual comprimido (feature vector)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualState {
    pub feature_vector: Vec<f32>,           // Embeddings visuales (128-512 dims)
    pub object_count: u32,
    pub scene_complexity: f32,
    pub dominant_colors: Vec<[f32; 3]>,     // RGB
    pub spatial_distribution: Vec<f32>,     // Histograma 3D
}

/// Estado de física
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsState {
    pub static_objects: u32,
    pub dynamic_objects: u32,
    pub collision_active: bool,
    pub gravity_enabled: bool,
    pub total_mass: f32,
}

/// Métricas de rendimiento
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub fps: f32,
    pub draw_calls: u32,
    pub triangles: u32,
    pub memory_mb: f32,
}

/// Transición estado-a-estado
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StateTransition {
    pub state_before: WorldState,
    pub state_after: WorldState,
    pub success: bool,
    pub reward: f32,
}

/// Buffer circular de estados históricos
pub struct StateBuffer {
    capacity: usize,
    buffer: VecDeque<StateTransition>,
    successful_only: bool,
}

impl StateBuffer {
    pub fn new(capacity: usize, successful_only: bool) -> Self {
        Self {
            capacity,
            buffer: VecDeque::with_capacity(capacity),
            successful_only,
        }
    }

    /// Añade una transición al buffer
    pub fn push(&mut self, transition: StateTransition) {
        // Si solo guardamos exitosas, filtrar
        if self.successful_only && !transition.success {
            return;
        }

        if self.buffer.len() >= self.capacity {
            self.buffer.pop_front();
        }

        self.buffer.push_back(transition);
        
        println!(
            "📦 Buffer: {} transiciones almacenadas (capacidad: {})",
            self.buffer.len(),
            self.capacity
        );
    }

    /// Obtiene las últimas n transiciones
    pub fn get_recent(&self, n: usize) -> Vec<&StateTransition> {
        self.buffer.iter().rev().take(n).collect()
    }

    /// Obtiene todas las transiciones
    pub fn get_all(&self) -> &VecDeque<StateTransition> {
        &self.buffer
    }

    /// Calcula estadísticas del buffer
    pub fn statistics(&self) -> BufferStatistics {
        if self.buffer.is_empty() {
            return BufferStatistics::default();
        }

        let total = self.buffer.len();
        let successful = self.buffer.iter().filter(|t| t.success).count();
        let avg_reward: f32 = self.buffer.iter().map(|t| t.reward).sum::<f32>() / total as f32;
        
        let avg_fps: f32 = self.buffer
            .iter()
            .map(|t| t.state_after.performance_metrics.fps)
            .sum::<f32>() / total as f32;

        BufferStatistics {
            total_transitions: total,
            successful_transitions: successful,
            success_rate: successful as f32 / total as f32,
            average_reward: avg_reward,
            average_fps: avg_fps,
        }
    }

    /// Exporta el buffer para entrenamiento
    pub fn export_for_training(&self) -> Vec<TrainingExample> {
        self.buffer
            .iter()
            .map(|transition| TrainingExample {
                input_state: transition.state_before.visual_state.feature_vector.clone(),
                action: transition.state_before.agent_action.clone(),
                predicted_state: transition.state_after.visual_state.feature_vector.clone(),
                actual_metrics: transition.state_after.performance_metrics.clone(),
            })
            .collect()
    }

    /// Limpia el buffer
    pub fn clear(&mut self) {
        self.buffer.clear();
        println!("🗑️ Buffer limpiado");
    }

    /// Guarda buffer a disco
    pub fn save(&self, path: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(&self.buffer)?;
        std::fs::write(path, json)?;
        println!("💾 Buffer guardado en: {}", path);
        Ok(())
    }

    /// Carga buffer desde disco
    pub fn load(&mut self, path: &str) -> std::io::Result<()> {
        let json = std::fs::read_to_string(path)?;
        self.buffer = serde_json::from_str(&json)?;
        println!("📂 Buffer cargado desde: {} ({} transiciones)", path, self.buffer.len());
        Ok(())
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct BufferStatistics {
    pub total_transitions: usize,
    pub successful_transitions: usize,
    pub success_rate: f32,
    pub average_reward: f32,
    pub average_fps: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingExample {
    pub input_state: Vec<f32>,
    pub action: AgentAction,
    pub predicted_state: Vec<f32>,
    pub actual_metrics: PerformanceMetrics,
}
