// gym/gym_director.rs
// Director del Gimnasio - Sistema Híbrido de Entrenamiento para SoulForge
//
// Genera prompts de entrenamiento con 3 niveles de dificultad progresiva,
// calcula recompensas multi-factor y gestiona la progresión automática.

use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use crate::gym::data_librarian::PointCloud;

// ============================================================
// TIPOS PRINCIPALES
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingPrompt {
    pub id: u64,
    pub prompt_type: PromptType,
    pub description: String,
    pub reference_data: ReferenceData,
    pub difficulty_level: DifficultyLevel,
    pub epoch: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PromptType {
    RealReference,      // Basado en mesh real
    SyntheticChallenge, // Forma geométrica procedural
    Hybrid,             // Combinación
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReferenceData {
    MeshFile {
        path: String,
        point_cloud: Vec<[f32; 3]>,
        complexity_score: f32,
    },
    SyntheticGeometry {
        formula: String,
        parameters: Vec<f32>,
        expected_vertices: u32,
    },
    HybridTask {
        base_mesh: String,
        procedural_modifications: Vec<String>,
    },
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum DifficultyLevel {
    Level1_IsolatedObjects,     // Lápidas, Árboles
    Level2_CompoundStructures,  // Criptas, Puentes
    Level3_CompleteEcosystems,  // Biomas, Pueblos
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingResult {
    pub prompt_id: u64,
    pub visual_similarity: f32,
    pub fps_stability: f32,
    pub draw_calls: u32,
    pub collision_errors: u32,
    pub memory_leaks: bool,
    pub aesthetic_score: f32,
    pub execution_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardCalculation {
    pub total_reward: f32,
    pub breakdown: RewardBreakdown,
    pub feedback_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardBreakdown {
    pub similarity_reward: f32,
    pub performance_reward: f32,
    pub stability_penalty: f32,
    pub aesthetic_bonus: f32,
}

// ============================================================
// HISTORIAL Y REPORTES
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EpochReport {
    pub epoch: u64,
    pub level: DifficultyLevel,
    pub prompt_description: String,
    pub result: TrainingResult,
    pub reward: RewardCalculation,
    pub timestamp: String,
}

// ============================================================
// GYM DIRECTOR
// ============================================================

pub struct GymDirector {
    pub current_epoch: u64,
    pub max_epochs: u64,
    pub current_level: DifficultyLevel,
    pub recent_results: VecDeque<f32>,
    pub training_history: Vec<EpochReport>,
    /// Curiosity-Driven: rastrea debilidades por tipo de geometría
    weakness_tracker: std::collections::HashMap<String, Vec<f32>>,
    /// Cola de retos inyectados manualmente
    pub manual_queue: VecDeque<TrainingPrompt>,
    /// Nubes de puntos de la última iteración (Conservative, Radical, Hybrid)
    pub latest_clouds: Option<(PointCloud, PointCloud, PointCloud)>,
    /// Plan geométrico (cajas) generado por los Agentes
    pub latest_plan: Option<crate::agents::geometer::GeometricInstruction>,
    /// Estado del entrenamiento
    pub is_paused: bool,
    /// Bitácora para la consola del dashboard
    pub logs: VecDeque<String>,
    /// Modo de entrenamiento (auto, synthetic, mesh, hybrid)
    pub training_mode: String,
}

impl GymDirector {
    pub fn new(max_epochs: u64) -> Self {
        Self {
            current_epoch: 0,
            max_epochs,
            current_level: DifficultyLevel::Level1_IsolatedObjects,
            recent_results: VecDeque::with_capacity(5),
            training_history: Vec::new(),
            weakness_tracker: std::collections::HashMap::new(),
            manual_queue: VecDeque::new(),
            latest_clouds: None,
            latest_plan: None,
            is_paused: true,
            logs: VecDeque::new(),
            training_mode: "auto".to_string(),
        }
    }

    pub fn add_log(&mut self, msg: String) {
        let timestamp = chrono::Local::now().format("%H:%M:%S").to_string();
        self.logs.push_back(format!("[{}] {}", timestamp, msg));
        if self.logs.len() > 50 {
            self.logs.pop_front();
        }
    }

    /// Genera el siguiente prompt de entrenamiento
    pub fn generate_next_prompt(&mut self) -> TrainingPrompt {
        self.current_epoch += 1;

        // Curiosity-Driven: si hay debilidades detectadas, forzar práctica
        if let Some((weak_type, _)) = self.find_worst_weakness() {
            if self.current_epoch % 4 == 0 {
                log::info!("🧠 CURIOSITY: Forzando práctica en debilidad: {}", weak_type);
                return self.generate_weakness_drill(&weak_type);
            }
        }

        // Sistema híbrido: alterna entre real, sintético e híbrido
        let manual_type = match self.training_mode.as_str() {
            "synthetic" => Some(PromptType::SyntheticChallenge),
            "mesh" => Some(PromptType::RealReference),
            "hybrid" => Some(PromptType::Hybrid),
            _ => None,
        };

        let prompt_type = manual_type.unwrap_or_else(|| match self.current_epoch % 3 {
            0 => PromptType::RealReference,
            1 => PromptType::SyntheticChallenge,
            _ => PromptType::Hybrid,
        });

        let (description, reference_data) = self.generate_task_for_level(
            self.current_level,
            &prompt_type,
        );

        TrainingPrompt {
            id: self.current_epoch,
            prompt_type,
            description,
            reference_data,
            difficulty_level: self.current_level,
            epoch: self.current_epoch,
        }
    }

    /// Inyecta un reto manual prioritario en la cola
    pub fn inject_custom_challenge(&mut self, description: String, reference_data: ReferenceData) -> TrainingPrompt {
        self.current_epoch += 1;
        
        let prompt_type = match &reference_data {
            ReferenceData::SyntheticGeometry { .. } => PromptType::SyntheticChallenge,
            ReferenceData::MeshFile { .. } => PromptType::RealReference,
            ReferenceData::HybridTask { .. } => PromptType::Hybrid,
        };

        let prompt = TrainingPrompt {
            id: self.current_epoch,
            prompt_type,
            description,
            reference_data,
            difficulty_level: self.current_level,
            epoch: self.current_epoch,
        };

        self.manual_queue.push_back(prompt.clone());
        prompt
    }

    /// Genera tarea según nivel y tipo
    fn generate_task_for_level(
        &self,
        level: DifficultyLevel,
        prompt_type: &PromptType,
    ) -> (String, ReferenceData) {
        match (level, prompt_type) {
            // ===================== NIVEL 1 - Objetos Aislados =====================
            (DifficultyLevel::Level1_IsolatedObjects, PromptType::RealReference) => {
                let tasks = vec![
                    ("Replica esta lápida gótica usando cubos y cilindros", "references/tombstone_gothic_01.obj"),
                    ("Crea este árbol muerto con ramas retorcidas", "references/dead_tree_01.obj"),
                    ("Replica esta fuente de agua circular", "references/fountain_circular.obj"),
                ];
                let (desc, path) = tasks[self.current_epoch as usize % tasks.len()];
                (desc.to_string(), ReferenceData::MeshFile { path: path.to_string(), point_cloud: vec![], complexity_score: 0.3 })
            }

            (DifficultyLevel::Level1_IsolatedObjects, PromptType::SyntheticChallenge) => {
                let formulas = vec![
                    ("Crea un cilindro hueco r=2.0", "hollow_cylinder(outer_radius=2.0, inner_radius=1.5, height=4.0)", vec![2.0, 1.5, 4.0], 640),
                    ("Estrella de 5 puntas extruida", "star_extrusion(points=5, inner_radius=1.0, outer_radius=2.0, depth=0.5)", vec![5.0, 1.0, 2.0, 0.5], 20),
                    ("Genera un caballo", "horse(scale=1.5)", vec![1.5], 400),
                    ("Genera un pájaro volando", "bird(scale=0.8)", vec![0.8], 300),
                    ("Genera un pez", "fish(scale=1.2)", vec![1.2], 300),
                    ("Mesa de madera", "table(width=2.5, height=1.2)", vec![2.5, 1.2], 150),
                    ("Silla simple", "chair(width=0.6, height=0.6)", vec![0.6, 0.6], 180),
                    ("Árbol de roble frondoso", "tree_oak(height=6.0)", vec![6.0], 500),
                    ("Árbol de pino", "tree_pine(height=7.0)", vec![7.0], 400),
                ];
                let idx = self.current_epoch as usize % formulas.len();
                let (desc, formula, params, verts) = &formulas[idx];
                (desc.to_string(), ReferenceData::SyntheticGeometry { formula: formula.to_string(), parameters: params.clone(), expected_vertices: *verts })
            }

            (DifficultyLevel::Level1_IsolatedObjects, PromptType::Hybrid) => {
                ("Toma roca base y añade cristales".to_string(), ReferenceData::HybridTask { base_mesh: "references/rock_base.obj".to_string(), procedural_modifications: vec!["add_crystals".to_string()] })
            }

            // ===================== NIVEL 2 - Estructuras Compuestas =====================
            (DifficultyLevel::Level2_CompoundStructures, PromptType::RealReference) => {
                ("Replica esta cripta medieval con puerta arqueada y columnas laterales".to_string(), ReferenceData::MeshFile { path: "references/crypt_medieval_01.obj".to_string(), point_cloud: vec![], complexity_score: 0.6 })
            }

            (DifficultyLevel::Level2_CompoundStructures, PromptType::SyntheticChallenge) => {
                let formulas = vec![
                    ("Casa simple de dos aguas", "house(width=5.0, height=3.0, depth=6.0, roof_height=2.5)", vec![5.0, 3.0, 6.0, 2.5], 800),
                    ("Cabaña del bosque con porche", "cabin(width=3.5, height=2.8, depth=4.5, roof_height=1.8)", vec![3.5, 2.8, 4.5, 1.8], 900),
                    ("Auto compacto", "car(length=4.2, width=1.8, height=1.5)", vec![4.2, 1.8, 1.5], 500),
                    ("Avión de pasajeros", "airplane(fuselage=12.0, wingspan=14.0)", vec![12.0, 14.0], 700),
                    ("Torre de vigilancia medieval", "tower(height=12.0, radius=2.0)", vec![12.0, 2.0], 600),
                    ("Barco de madera con mástil", "boat(length=7.0)", vec![7.0], 450),
                    ("Muro de fortaleza con almenas", "wall_fortress(length=15.0, height=5.0)", vec![15.0, 5.0], 400),
                    ("Puente de 3 arcos", "bridge_arched(arcs=3, span=10.0, arc_height=3.0)", vec![3.0, 10.0, 3.0], 400),
                ];
                let idx = self.current_epoch as usize % formulas.len();
                let (desc, formula, params, verts) = &formulas[idx];
                (desc.to_string(), ReferenceData::SyntheticGeometry { formula: formula.to_string(), parameters: params.clone(), expected_vertices: *verts })
            }

            (DifficultyLevel::Level2_CompoundStructures, PromptType::Hybrid) => {
                ("Usa esta base de templo griego y genera columnas".to_string(), ReferenceData::HybridTask { base_mesh: "references/temple.obj".to_string(), procedural_modifications: vec!["add_doric_columns".to_string()] })
            }

            // ===================== NIVEL 3 - Ecosistemas Completos =====================
            (DifficultyLevel::Level3_CompleteEcosystems, PromptType::RealReference) => {
                ("Replica este pueblo completo".to_string(), ReferenceData::MeshFile { path: "references/village_complete_01.obj".to_string(), point_cloud: vec![], complexity_score: 0.9 })
            }

            (DifficultyLevel::Level3_CompleteEcosystems, PromptType::SyntheticChallenge) => {
                let formulas = vec![
                    ("Aldea completa: 5 casas, caminos, y pinos dispersos", "village(houses=5)", vec![5.0], 3500),
                    ("Terreno montañoso con picos procedurales", "terrain_hills(size=30.0, hills=4)", vec![30.0, 4.0], 2500),
                    ("Montaña masiva con una cascada lateral cayendo a un pozo", "mountain_cascade(radius=12.0, height=15.0)", vec![12.0, 15.0], 2000),
                    ("Bioma necrótico con 20 árboles secos y hongos luminescentes", "biome_necrotic(trees=20, fog_density=0.7, mushroom_clusters=8)", vec![20.0, 0.7, 8.0], 5000),
                ];
                let idx = self.current_epoch as usize % formulas.len();
                let (desc, formula, params, verts) = &formulas[idx];
                (desc.to_string(), ReferenceData::SyntheticGeometry { formula: formula.to_string(), parameters: params.clone(), expected_vertices: *verts })
            }

            (DifficultyLevel::Level3_CompleteEcosystems, PromptType::Hybrid) => {
                ("Toma cementerio base y puebla".to_string(), ReferenceData::HybridTask { base_mesh: "references/cemetery.obj".to_string(), procedural_modifications: vec!["populate_graves".to_string()] })
            }
        }
    }

    // ============================================================
    // CURIOSITY-DRIVEN TRAINING
    // ============================================================

    /// Registra el rendimiento en un tipo específico de geometría
    pub fn track_weakness(&mut self, geometry_type: &str, score: f32) {
        let entry = self.weakness_tracker
            .entry(geometry_type.to_string())
            .or_insert_with(Vec::new);
        entry.push(score);
        // Mantener solo las últimas 10 puntuaciones
        if entry.len() > 10 {
            entry.remove(0);
        }
    }

    /// Encuentra la peor debilidad (promedio más bajo con al menos 3 muestras)
    fn find_worst_weakness(&self) -> Option<(String, f32)> {
        self.weakness_tracker
            .iter()
            .filter(|(_, scores)| scores.len() >= 3)
            .map(|(geo_type, scores)| {
                let avg = scores.iter().sum::<f32>() / scores.len() as f32;
                (geo_type.clone(), avg)
            })
            .filter(|(_, avg)| *avg < 0.7) // Solo debilidades reales
            .min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
    }

    /// Genera un prompt de práctica enfocado en la debilidad detectada
    fn generate_weakness_drill(&self, weakness_type: &str) -> TrainingPrompt {
        let (description, reference_data) = match weakness_type {
            t if t.contains("star") => (
                format!("DRILL: Practica estrella extruida - variante {} puntas",
                    3 + (self.current_epoch % 5) as u32),
                ReferenceData::SyntheticGeometry {
                    formula: format!("star_extrusion(points={}, inner_radius=1.0, outer_radius=2.0, depth=0.5)",
                        3 + (self.current_epoch % 5)),
                    parameters: vec![(3 + self.current_epoch % 5) as f32, 1.0, 2.0, 0.5],
                    expected_vertices: 20,
                },
            ),
            t if t.contains("pyramid") => (
                "DRILL: Practica pirámide truncada con proporciones variadas".to_string(),
                ReferenceData::SyntheticGeometry {
                    formula: "pyramid_truncated(base=5.0, top=1.5, height=6.0)".to_string(),
                    parameters: vec![5.0, 1.5, 6.0],
                    expected_vertices: 48,
                },
            ),
            t if t.contains("bridge") => (
                "DRILL: Practica puente arqueado con 4 arcos".to_string(),
                ReferenceData::SyntheticGeometry {
                    formula: "bridge_arched(arcs=4, span=12.0, arc_height=4.0)".to_string(),
                    parameters: vec![4.0, 12.0, 4.0],
                    expected_vertices: 160,
                },
            ),
            _ => (
                format!("DRILL: Práctica genérica enfocada en '{}'", weakness_type),
                ReferenceData::SyntheticGeometry {
                    formula: "default_cube()".to_string(),
                    parameters: vec![1.0],
                    expected_vertices: 8,
                },
            ),
        };

        TrainingPrompt {
            id: self.current_epoch,
            prompt_type: PromptType::SyntheticChallenge,
            description,
            reference_data,
            difficulty_level: self.current_level,
            epoch: self.current_epoch,
        }
    }

    // ============================================================
    // RECOMPENSAS
    // ============================================================

    /// Calcula la recompensa basada en el resultado
    pub fn calculate_reward(&self, result: &TrainingResult) -> RewardCalculation {
        let mut breakdown = RewardBreakdown {
            similarity_reward: 0.0,
            performance_reward: 0.0,
            stability_penalty: 0.0,
            aesthetic_bonus: 0.0,
        };

        // SIMILARIDAD VISUAL (peso: 50%)
        breakdown.similarity_reward = if result.visual_similarity > 0.9 {
            0.5
        } else if result.visual_similarity > 0.7 {
            0.3
        } else {
            result.visual_similarity * 0.3
        };

        // RENDIMIENTO (peso: 30%)
        let fps_reward = if result.fps_stability > 0.95 { 0.2 } else { 0.0 };
        let drawcall_penalty = if result.draw_calls > 500 { -0.1 } else { 0.1 };
        breakdown.performance_reward = fps_reward + drawcall_penalty;

        // ESTABILIDAD (peso: 15%)
        breakdown.stability_penalty = if result.collision_errors > 0 {
            -0.5
        } else if result.memory_leaks {
            -1.0
        } else {
            0.15
        };

        // ESTÉTICA (peso: 5% bonus)
        breakdown.aesthetic_bonus = if result.aesthetic_score > 0.8 {
            0.05
        } else {
            0.0
        };

        let total = breakdown.similarity_reward
            + breakdown.performance_reward
            + breakdown.stability_penalty
            + breakdown.aesthetic_bonus;

        let feedback = self.generate_feedback(total, result);

        RewardCalculation {
            total_reward: total.clamp(-1.0, 1.0),
            breakdown,
            feedback_message: feedback,
        }
    }

    /// Genera feedback textual para los agentes
    fn generate_feedback(&self, total: f32, result: &TrainingResult) -> String {
        let mut feedback = Vec::new();

        if total > 0.9 {
            feedback.push("🏆 EXCELENTE: Construcción casi perfecta.".to_string());
        } else if total > 0.5 {
            feedback.push("✅ BUENO: Resultado aceptable.".to_string());
        } else if total > 0.0 {
            feedback.push("⚠️ MEJORABLE: Revisa los detalles.".to_string());
        } else {
            feedback.push("❌ FALLIDO: Requerido replantear estrategia.".to_string());
        }

        if result.visual_similarity < 0.7 {
            feedback.push(format!(
                "Similaridad visual baja ({:.1}%). Analiza la referencia más cuidadosamente.",
                result.visual_similarity * 100.0
            ));
        }

        if result.draw_calls > 500 {
            feedback.push(format!(
                "Draw Calls excesivas ({}). Optimiza el uso de instancias.",
                result.draw_calls
            ));
        }

        if result.collision_errors > 0 {
            feedback.push(format!(
                "⚠️ {} errores de colisión detectados. Verifica volúmenes.",
                result.collision_errors
            ));
        }

        if result.memory_leaks {
            feedback.push("🚨 CRÍTICO: Fuga de memoria detectada. Limpia referencias.".to_string());
        }

        feedback.join(" | ")
    }

    // ============================================================
    // PROGRESIÓN Y REGISTRO
    // ============================================================

    /// Registra el resultado y evalúa si subir de nivel
    pub fn record_result(
        &mut self,
        prompt_description: &str,
        result: TrainingResult,
        reward: RewardCalculation,
    ) {
        // Agregar a historial reciente
        self.recent_results.push_back(reward.total_reward);
        if self.recent_results.len() > 5 {
            self.recent_results.pop_front();
        }

        // Guardar epoch completo
        let report = EpochReport {
            epoch: self.current_epoch,
            level: self.current_level,
            prompt_description: prompt_description.to_string(),
            result,
            reward,
            timestamp: chrono::Utc::now().to_rfc3339(),
        };
        self.training_history.push(report);

        // Evaluar progresión
        self.evaluate_progression();
    }

    /// Evalúa si debe incrementar el nivel de dificultad
    fn evaluate_progression(&mut self) {
        if self.recent_results.len() < 5 {
            return;
        }

        let average: f32 = self.recent_results.iter().sum::<f32>() / 5.0;

        if average > 0.9 {
            self.current_level = match self.current_level {
                DifficultyLevel::Level1_IsolatedObjects => {
                    log::info!("🎓 NIVEL UP: Avanzando a Estructuras Compuestas");
                    DifficultyLevel::Level2_CompoundStructures
                }
                DifficultyLevel::Level2_CompoundStructures => {
                    log::info!("🎓 NIVEL UP: Avanzando a Ecosistemas Completos");
                    DifficultyLevel::Level3_CompleteEcosystems
                }
                DifficultyLevel::Level3_CompleteEcosystems => {
                    log::info!("👑 MAESTRÍA ALCANZADA: Manteniendo nivel máximo");
                    DifficultyLevel::Level3_CompleteEcosystems
                }
            };

            self.recent_results.clear();
        }
    }

    // ============================================================
    // REPORTES
    // ============================================================

    /// Genera reporte completo en JSON
    pub fn generate_progress_report(&self) -> serde_json::Value {
        serde_json::json!({
            "training_session": {
                "current_epoch": self.current_epoch,
                "max_epochs": self.max_epochs,
                "current_level": format!("{:?}", self.current_level),
                "progress_percentage": (self.current_epoch as f32 / self.max_epochs as f32) * 100.0
            },
            "recent_performance": {
                "last_5_rewards": self.recent_results.iter().collect::<Vec<_>>(),
                "average_reward": if !self.recent_results.is_empty() {
                    self.recent_results.iter().sum::<f32>() / self.recent_results.len() as f32
                } else {
                    0.0
                },
            },
            "curiosity_driven": {
                "tracked_weaknesses": self.weakness_tracker.keys().collect::<Vec<_>>(),
                "current_worst": self.find_worst_weakness()
                    .map(|(t, s)| serde_json::json!({"type": t, "avg_score": s}))
                    .unwrap_or(serde_json::json!(null)),
            },
            "history": self.training_history.iter().rev().take(10).collect::<Vec<_>>(),
            "statistics": self.calculate_statistics(),
        })
    }

    /// Calcula estadísticas globales
    fn calculate_statistics(&self) -> serde_json::Value {
        if self.training_history.is_empty() {
            return serde_json::json!({});
        }

        let rewards: Vec<f32> = self
            .training_history
            .iter()
            .map(|r| r.reward.total_reward)
            .collect();

        let avg_reward = rewards.iter().sum::<f32>() / rewards.len() as f32;
        let max_reward = rewards.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
        let min_reward = rewards.iter().cloned().fold(f32::INFINITY, f32::min);

        let successful = rewards.iter().filter(|&&r| r > 0.5).count();
        let failed = rewards.iter().filter(|&&r| r < 0.0).count();

        serde_json::json!({
            "average_reward": avg_reward,
            "max_reward": max_reward,
            "min_reward": min_reward,
            "success_rate": (successful as f32 / rewards.len() as f32) * 100.0,
            "failure_rate": (failed as f32 / rewards.len() as f32) * 100.0,
            "total_epochs_completed": self.training_history.len(),
        })
    }

    /// Checkpoint del estado actual
    pub fn save_checkpoint(&self, path: &str) -> std::io::Result<()> {
        let data = serde_json::json!({
            "current_epoch": self.current_epoch,
            "max_epochs": self.max_epochs,
            "current_level": self.current_level,
            "recent_results": self.recent_results,
            "training_history": self.training_history,
        });
        let json = serde_json::to_string_pretty(&data)?;
        // Crear directorio si no existe
        if let Some(parent) = std::path::Path::new(path).parent() {
            std::fs::create_dir_all(parent)?;
        }
        std::fs::write(path, json)?;
        log::info!("💾 Checkpoint guardado en: {}", path);
        Ok(())
    }

    /// Restaurar desde checkpoint
    pub fn load_checkpoint(path: &str) -> std::io::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let data: serde_json::Value = serde_json::from_str(&content)
            .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;

        let current_level: DifficultyLevel = serde_json::from_value(
            data["current_level"].clone()
        ).unwrap_or(DifficultyLevel::Level1_IsolatedObjects);

        let recent: VecDeque<f32> = serde_json::from_value(
            data["recent_results"].clone()
        ).unwrap_or_default();

        let history: Vec<EpochReport> = serde_json::from_value(
            data["training_history"].clone()
        ).unwrap_or_default();

        Ok(GymDirector {
            current_epoch: data["current_epoch"].as_u64().unwrap_or(0),
            max_epochs: data["max_epochs"].as_u64().unwrap_or(50),
            current_level: current_level,
            recent_results: recent,
            training_history: history,
            weakness_tracker: std::collections::HashMap::new(),
            manual_queue: std::collections::VecDeque::new(),
            latest_clouds: None,
            latest_plan: None,
            is_paused: true,
            logs: std::collections::VecDeque::new(),
            training_mode: "auto".to_string(),
        })
    }
}
