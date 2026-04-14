use std::sync::Arc;
use tokio::sync::{RwLock, mpsc};
use crate::contextus::semantic_graph::{SemanticGraph, EvidenceType, AnchorSource};
use crate::contextus::hypothesis::HypothesisEngine;
use crate::forge::experimental_lab::{LabEngine, UnrealExperiment, UnrealSimResult, LabSession, StructureType, StressTest};
use crate::cortex::comprehension::deep_understanding::KnowledgeBase;
use rayon::prelude::*;

// --- EVENTOS DEL LABORATORIO ---

pub enum LabEvent {
    ConceptFound { 
        concept: String, 
        relevance: f32,
        for_hypothesis: String,
    },
    ExperimentComplete {
        session_id: String,
        result: UnrealSimResult,
    },
    MetricsProcessed {
        session_id: String,
        delta: f32,
        insights: Vec<String>,
    },
}

// --- AGENTE 1: EL BUSCADOR (RAYON) ---

pub struct SearchAgent {
    pub knowledge_base: Arc<RwLock<KnowledgeBase>>,
}

impl SearchAgent {
    pub async fn search_for_experiment(
        &self,
        experiment: &UnrealExperiment,
        hypothesis_id: String,
        tx: mpsc::Sender<LabEvent>,
    ) {
        let keywords = self.extract_keywords(experiment);
        let kb = self.knowledge_base.read().await;

        // RAYON: Buscamos en paralelo sobre los términos del índice invertido
        let found_concepts: Vec<String> = keywords.par_iter()
            .flat_map(|kw| {
                kb.inverted_index.get(kw).cloned().unwrap_or_default()
            })
            .collect();

        for concept in found_concepts {
            let _ = tx.send(LabEvent::ConceptFound {
                concept,
                relevance: 0.8,
                for_hypothesis: hypothesis_id.clone(),
            }).await;
        }
    }

    fn extract_keywords(&self, experiment: &UnrealExperiment) -> Vec<String> {
        let mut kw = vec!["ingeniería".to_string()];
        match &experiment.structure_type {
            StructureType::Arch(_) => kw.push("arco".to_string()),
            StructureType::Pyramid(_) => kw.push("pirámide".to_string()),
            _ => kw.push("estructura".to_string()),
        }
        kw
    }
}

// --- AGENTE 2: EL ANALIZADOR DE MÉTRICAS ---

pub struct MetricsAgent {
    pub graph: Arc<RwLock<SemanticGraph>>,
}

impl MetricsAgent {
    pub async fn analyze(
        &self,
        session: &LabSession,
        result: &UnrealSimResult,
        tx: mpsc::Sender<LabEvent>,
        lab_engine: &LabEngine,
    ) {
        let delta = lab_engine.calculate_real_delta(&session.prediction, result);
        let mut insights = Vec::new();

        if result.max_deformation > 0.05 {
            insights.push(format!("Alerta: Deformación de {:.3} excede márgenes teóricos.", result.max_deformation));
        }

        // Actualización asíncrona del grafo
        {
            let mut g = self.graph.write().await;
            let key = lab_engine.experiment_to_anchor_key(&session.experiment);
            g.strengthen_anchor(
                key,
                &session.hypothesis_id,
                delta,
                result.survived,
                result.max_deformation,
                result.failure_points.iter().map(|f| f.name.clone()).collect(),
                format!("{:?}", session.experiment.stress_test),
                crate::contextus::semantic_graph::AnchorSource::LabExperiment,
            );
        }

        let _ = tx.send(LabEvent::MetricsProcessed {
            session_id: session.hypothesis_id.clone(),
            delta,
            insights,
        }).await;
    }
}

// --- AGENTE 3: EL EXPERIMENTADOR ---

pub struct ExperimentAgent {
    pub unreal_tx: mpsc::Sender<serde_json::Value>,
}

impl ExperimentAgent {
    pub async fn run_experiment(&self, session: LabSession, lab_engine: &LabEngine) {
        let command = lab_engine.to_unreal_command(&session);
        let _ = self.unreal_tx.send(command).await;
    }
}

// --- EL ORQUESTADOR PARALELO ---

pub struct LabOrchestrator {
    pub searcher: SearchAgent,
    pub metrics: MetricsAgent,
    pub experimenter: ExperimentAgent,
    pub event_rx: mpsc::Receiver<LabEvent>,
    pub tx_template: mpsc::Sender<LabEvent>,
}

impl LabOrchestrator {
    pub async fn process_events(
        &mut self,
        graph: Arc<RwLock<SemanticGraph>>,
        hypothesis_engine: Arc<RwLock<HypothesisEngine>>,
    ) {
        while let Ok(event) = self.event_rx.try_recv() {
            match event {
                LabEvent::ConceptFound { concept, relevance, for_hypothesis } => {
                    let mut g = graph.write().await;
                    g.add_theoretical_support(&for_hypothesis, &concept, relevance);
                },
                LabEvent::MetricsProcessed { session_id, delta, insights } => {
                    println!("[ORQUESTADOR] Sesión {} cerrada. Precisión: {:.2}", session_id, delta);
                    for ins in insights { println!("  → Insight: {}", ins); }
                    
                    let mut hyp = hypothesis_engine.write().await;
                    if delta > 0.8 { hyp.resolve(&session_id, true); }
                    else if delta < 0.3 { hyp.resolve(&session_id, false); }
                },
                _ => {}
            }
        }
    }
}
