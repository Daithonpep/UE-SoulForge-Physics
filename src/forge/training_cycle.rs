use crate::forge::curiosity_engine::CuriosityEngine;
use crate::forge::experimental_lab::LabEngine;
use crate::contextus::semantic_graph::SemanticGraph;
use crate::forge::lab_agents::LabOrchestrator;
use std::sync::Arc;
use tokio::sync::RwLock;
use std::time::Duration;

pub struct TrainingManager {
    pub curiosity: CuriosityEngine,
    pub phase: TrainingPhase,
}

pub enum TrainingPhase {
    FoundationMapping,
    BoundaryExploration,
    EfficiencySearch,
    FreeExploration,
}

impl TrainingManager {
    pub fn new() -> Self {
        Self {
            curiosity: CuriosityEngine::new(),
            phase: TrainingPhase::FoundationMapping,
        }
    }

    pub async fn execute_cycle(
        &mut self,
        lab: &mut LabEngine,
        graph: Arc<RwLock<SemanticGraph>>,
        orchestrator: &mut LabOrchestrator,
    ) {
        println!("[TRAINING] Iniciando ciclo en fase: {:?}", self.phase);

        match self.phase {
            TrainingPhase::FoundationMapping => {
                // Durante el mapeo, forzamos combinaciones básicas
                if let Some((exp, reason)) = self.curiosity.generate_curious_experiment(&*graph.read().await) {
                    println!("[TRAINING] Mapeando base. Razón: {:?}", reason);
                    // Aquí iría el disparador del orquestador (simplificado)
                }

                if self.is_foundation_complete(&*graph.read().await) {
                    self.phase = TrainingPhase::BoundaryExploration;
                }
            },
            TrainingPhase::BoundaryExploration => {
                if let Some((exp, reason)) = self.curiosity.generate_curious_experiment(&*graph.read().await) {
                    println!("[TRAINING] Empujando límites físicos. Razón: {:?}", reason);
                }
            },
            _ => {
                tokio::time::sleep(Duration::from_secs(5)).await;
            }
        }
    }

    fn is_foundation_complete(&self, graph: &SemanticGraph) -> bool {
        // Completo si tenemos al menos 3 experimentos de cada tipo base
        graph.empirical_anchors.values().all(|a| a.reproduction_count >= 2) && !graph.empirical_anchors.is_empty()
    }
}

impl std::fmt::Debug for TrainingPhase {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FoundationMapping => write!(f, "Mapeo de Fundación"),
            Self::BoundaryExploration => write!(f, "Exploración de Límites"),
            Self::EfficiencySearch => write!(f, "Búsqueda de Eficiencia"),
            Self::FreeExploration => write!(f, "Exploración Libre"),
        }
    }
}
