use crate::contextus::DaithonContext;
use crate::cortex::CortexEngine;
use crate::forge::curiosity_engine::CuriosityEngine;
use crate::forge::experiment_generator::{ExperimentGenerator, ExperimentIntent};
use crate::forge::experimental_lab::{LabEngine, UnrealExperiment, UnrealSimResult, LabSession, StressTest};
use crate::forge::debate_session::LabDebateSession;
use crate::contextus::semantic_graph::{SemanticGraph, AnchorSource};
use crate::knowledge::chronicles::ChronicleEngine;
use std::sync::Arc;
use std::collections::VecDeque;
use tokio::sync::{RwLock, Mutex as TokioMutex, mpsc};

pub struct AutonomousLearner {
    pub chronicles: ChronicleEngine,
    pub cortex: Arc<RwLock<CortexEngine>>,
    pub priority_queue: Arc<TokioMutex<VecDeque<UnrealExperiment>>>,
}

impl AutonomousLearner {
    pub fn new(cortex: Arc<RwLock<CortexEngine>>) -> Self {
        Self { 
            chronicles: ChronicleEngine::new(),
            cortex,
            priority_queue: Arc::new(TokioMutex::new(VecDeque::new())),
        }
    }

    pub async fn run_forever(
        &self,
        lab_engine: Arc<TokioMutex<LabEngine>>,
        context: Arc<RwLock<DaithonContext>>,
        _curiosity: Arc<TokioMutex<CuriosityEngine>>, // Mantenido por interfaz, pero usaremos Generator
        unreal_tx: mpsc::Sender<serde_json::Value>,
    ) {
        println!("\n[DAITHON] 🧠 MODO INVESTIGADOR ESTRATÉGICO DINÁMICO ACTIVADO.");
        let mut generator = ExperimentGenerator::new();
        
        loop {
            // 1. Verificar cola de prioridad primero
            let (experiment, reason_str) = {
                let mut pq = self.priority_queue.lock().await;
                if let Some(exp) = pq.pop_front() {
                    (exp, "INVESTIGACIÓN DE FALLO PREVIO".to_string())
                } else {
                    let ctx = context.read().await;
                    let (exp, intent) = generator.generate(&ctx.semantic_graph);
                    (exp, format!("{:?}", intent))
                }
            };

            println!("\n[DAITHON] 🔍 Objetivo científico: {:?} | Razón: {}", experiment.structure_type, reason_str);

            // 2. Crear sesión de laboratorio
            let session_id = format!("AUTO_{}", uuid::Uuid::new_v4().to_string()[..8].to_string());
            let mut lab = lab_engine.lock().await;
            let mut ctx = context.write().await;
            
            let prediction = lab.generate_structured_prediction(&experiment, &ctx.semantic_graph);
            
            let session = LabSession {
                hypothesis_id: session_id.clone(),
                hypothesis: None,
                experiment: experiment.clone(),
                prediction,
                result: None,
                accuracy_delta: None,
            };

            lab.active_sessions.push(session.clone());
            println!("[LAB] Sesión: {}", session_id);
            println!("   🔭 Predicción: {} (Conf: {:.2})", 
                if session.prediction.predicts_survival { "Sobrevive" } else { "Colapso" },
                session.prediction.confidence
            );

            // 3. Debate Trinity
            let debate_session = LabDebateSession::new();
            let _debate = debate_session.analyze_experiment(&experiment, &ctx.semantic_graph);

            // 4. Enviar a Unreal/Sim
            let command = lab.to_unreal_command(&session);
            let _ = unreal_tx.send(command).await;

            drop(lab);
            drop(ctx);

            tokio::time::sleep(tokio::time::Duration::from_secs(10)).await;
        }
    }

    pub async fn handle_experiment_result(
        &self,
        experiment: &UnrealExperiment,
        _result: &UnrealSimResult,
        delta: f32,
    ) {
        if delta < 0.2 {
            println!("[DAITHON] ⚠ Delta {:.2} muy bajo. Mi predicción fue incorrecta.", delta);
            println!("[DAITHON] Investigando anomalía...");

            let verification = self.create_verification_experiment(experiment);
            let mut pq = self.priority_queue.lock().await;
            pq.push_front(verification);
        }
    }

    fn create_verification_experiment(&self, failed: &UnrealExperiment) -> UnrealExperiment {
        let mut verification = failed.clone();
        match &mut verification.stress_test {
            StressTest::Seismic(s) => { s.magnitude *= 0.95; },
            StressTest::Wind(w) => { w.speed *= 0.95; },
        }
        verification
    }

    pub async fn run_experiment_cycle(
        &mut self,
        experiment: UnrealExperiment,
        graph: Arc<RwLock<SemanticGraph>>,
    ) {
        println!("\n╔══════════════════════════════════════════╗");
        println!("║  NUEVO EXPERIMENTO AUTÓNOMO: {} ║", experiment.duration_seconds);
        println!("╚══════════════════════════════════════════╝");

        let debate_session = LabDebateSession::new();
        let debate = {
            let g = graph.read().await;
            debate_session.analyze_experiment(&experiment, &g)
        };

        println!("\n[RAPIER] Ejecutando simulación física...");
        let rapier_result = crate::design_evolution::physics_sim::PhysicsSimulator::simulate_autonomous_experiment(&experiment);

        println!("\n[RESULTADO] Superviviencia: {}, Deformación: {:.4}", rapier_result.survived, rapier_result.max_deformation);

        self.chronicles.check_for_milestone("current_exp", &debate.senku, &rapier_result, &experiment);

        {
            let mut g = graph.write().await;
            g.strengthen_anchor(
                "exp_latest".into(),
                "Resultado de simulación física real",
                0.8,
                rapier_result.survived,
                rapier_result.max_deformation,
                vec![],
                format!("Simulación Rapier3D con debate previo"),
                AnchorSource::UnrealPhysics
            );
        }

        println!("\n[DAITHON] Ciclo completado. Memoria actualizada.");
    }
}


