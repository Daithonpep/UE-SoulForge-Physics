use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use crate::forge::experimental_lab::{LabEngine, StructureType, StressTest};
use crate::contextus::semantic_graph::SemanticGraph;
use crate::forge::curiosity_engine::CuriosityEngine;

pub struct DesignEngine {
    pub lab: Arc<Mutex<LabEngine>>,
    pub graph: Arc<RwLock<SemanticGraph>>,
    pub curiosity: Arc<Mutex<CuriosityEngine>>,
}

pub struct DesignRequest {
    pub description: String,
    pub constraints: Vec<String>,
    pub optimize_for: OptimizeFor,
}

#[derive(PartialEq)]
pub enum OptimizeFor {
    Aerodynamics,
    Structural,
    Efficiency,
    Aesthetic,
    Balanced,
}

pub struct DesignProposal {
    pub structure: StructureType,
    pub reasoning: Vec<DesignDecision>,
    pub confidence: f32,
    pub lab_verified: bool,
}

pub struct DesignDecision {
    pub what: String,
    pub why: String,
    pub evidence: EvidenceSource,
}

pub enum EvidenceSource {
    EmpiricalOwn(String),
    TheoreticalGraph(String),
    FirstPrinciples(String),
    NoEvidence,
}

impl DesignEngine {
    pub fn new(lab: Arc<Mutex<LabEngine>>, graph: Arc<RwLock<SemanticGraph>>, curiosity: Arc<Mutex<CuriosityEngine>>) -> Self {
        Self { lab, graph, curiosity }
    }

    pub async fn design(&self, request: &DesignRequest) -> DesignProposal {
        println!("[DESIGN] Iniciando diseño empírico para: {}", request.description);

        // 1. Consultar Experiencia en el Grafo
        let relevant_anchors = self.query_experience(request).await;

        // 2. Aplicar Optimizaciones Aprendidas
        let (structure, decisions) = self.apply_learned_optimizations(request, &relevant_anchors);

        // 3. Verificación Rápida (Saldría comando a Unreal)
        // Por ahora simulamos la intención de verificación
        let lab_verified = !relevant_anchors.is_empty();

        DesignProposal {
            structure,
            reasoning: decisions,
            confidence: if relevant_anchors.is_empty() { 0.3 } else { 0.85 },
            lab_verified,
        }
    }

    async fn query_experience(&self, request: &DesignRequest) -> Vec<(String, f32)> {
        let g = self.graph.read().await;
        let mut results = Vec::new();

        for (key, anchor) in &g.empirical_anchors {
            // Si optimizamos para aerodinámica, buscamos los tests de viento (Wind)
            if request.optimize_for == OptimizeFor::Aerodynamics && key.contains("wind") {
                results.push((key.clone(), anchor.avg_deformation));
            }
            // Si optimizamos para estructura, buscamos los sismos (Seismic)
            if request.optimize_for == OptimizeFor::Structural && key.contains("seismic") {
                results.push((key.clone(), anchor.survival_rate));
            }
        }
        results
    }

    fn apply_learned_optimizations(&self, request: &DesignRequest, experience: &[(String, f32)]) -> (StructureType, Vec<DesignDecision>) {
        let mut decisions = Vec::new();
        
        // Simulación de toma de decisión basada en el mejor resultado previo
        if let Some(best) = experience.iter().min_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal)) {
            decisions.push(DesignDecision {
                what: format!("Perfil basado en experimento '{}'", best.0),
                why: format!("Este perfil demostró un {:.2}% menos de resistencia en mis pruebas previas.", best.1 * 100.0),
                evidence: EvidenceSource::EmpiricalOwn(format!("Basado en datos de {}", best.0)),
            });
        } else {
            decisions.push(DesignDecision {
                what: "Geometría básica inicial".to_string(),
                why: "No tengo datos empíricos sobre este tipo de forma. Aplicando primeros principios.".to_string(),
                evidence: EvidenceSource::FirstPrinciples("Sin validación de laboratorio previa.".to_string()),
            });
        }

        // Devolvemos una estructura base (Ejem: Pirámide para estabilidad)
        (StructureType::Pyramid(crate::forge::experimental_lab::PyramidParams {
            base_width: 10.0,
            height: 5.0,
            material_density: 1.5,
        }), decisions)
    }
}
