use std::sync::Arc;
use tokio::sync::{Mutex, RwLock, mpsc};
use daithon_bridge::forge::experimental_lab::{LabEngine, UnrealSimResult, FailurePoint};
use daithon_bridge::forge::autonomous_learner::AutonomousLearner;
use daithon_bridge::forge::curiosity_engine::CuriosityEngine;
use daithon_bridge::contextus::semantic_graph::SemanticGraph;
use daithon_bridge::cortex::CortexEngine;
use daithon_bridge::trinity::judge::naturalness_evaluator::NaturalnessJudge;

#[tokio::main]
async fn main() {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║       🧠 DAITHON: SOBERANÍA COGNITIVA ACTIVADA            ║");
    println!("║       Modo: APRENDIZAJE ESTRATÉGICO AUTÓNOMO              ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    // 1. Inicialización de Motores
    let judge = NaturalnessJudge::new();
    let cortex = Arc::new(RwLock::new(CortexEngine::new(judge)));
    let lab = Arc::new(Mutex::new(LabEngine::new()));
    let context = Arc::new(RwLock::new(daithon_bridge::contextus::DaithonContext::new()));
    let curiosity = Arc::new(Mutex::new(CuriosityEngine::new()));

    // 2. Canal de salida para Unreal (Mocked for this script)
    let (tx, mut rx) = mpsc::channel(100);

    // 3. El Estudiante Autónomo
    let learner = Arc::new(AutonomousLearner::new(cortex.clone()));

    let lab_for_learner = lab.clone();
    let context_for_learner = context.clone();
    let curiosity_for_learner = curiosity.clone();
    let tx_for_learner = tx.clone();
    let learner_for_thinker = learner.clone();

    // Iniciar el loop de pensamiento en segundo plano
    let learner_handle = tokio::spawn(async move {
        learner_for_thinker.run_forever(lab_for_learner, context_for_learner, curiosity_for_learner, tx_for_learner).await;
    });

    // 4. Simulador de Respuesta de Unreal (Sovereign Lab Loop)
    let lab_for_mock = lab.clone();
    let context_for_mock = context.clone();
    let curiosity_for_mock = curiosity.clone();
    let learner_for_mock = learner.clone();

    let simulation_handle = tokio::spawn(async move {
        let mut count = 0;
        while let Some(command) = rx.recv().await {
            count += 1;
            let session_id = command["session_id"].as_str().unwrap_or("unknown").to_string();
            
            // 1. Reconstruir el objeto UnrealExperiment desde el JSON
            let experiment: daithon_bridge::forge::experimental_lab::UnrealExperiment = 
                serde_json::from_value(command["experiment"].clone()).unwrap();
            
            println!("\n[Sovereign-Lab] [{} / 50] 🏗️ Simulando: {:?}", count, experiment.structure_type);
            println!("               ⚡ Estrés: {:?}", experiment.stress_test);
            println!("               📐 Rotación: {:?}", experiment.placement.rotation);
            println!("               🧱 Material: {}", experiment.material);
            
            // 2. Ejecutar Simulación Real
            let mut result = daithon_bridge::design_evolution::physics_sim::PhysicsSimulator::simulate_autonomous_experiment(&experiment);
            result.session_id = session_id.clone();

            // Procesar resultado en el motor
            let mut l = lab_for_mock.lock().await;
            let mut ctx = context_for_mock.write().await;
            let mut cur = curiosity_for_mock.lock().await;
            let g = &mut ctx.semantic_graph;
            
            if let Some(pos) = l.active_sessions.iter().position(|s| s.hypothesis_id == session_id) {
                let experiment_obj = l.active_sessions[pos].experiment.clone();
                let survived = result.survived;
                let prediction = l.active_sessions[pos].prediction.clone();

                l.process_result(pos, result.clone(), g, daithon_bridge::contextus::semantic_graph::AnchorSource::MockSimulator);
                cur.observe_result(&experiment_obj, survived);
                
                // NOTA: Calculamos el delta nosotros para disparar la investigación
                let delta = l.calculate_real_delta(&prediction, &result);
                learner_for_mock.handle_experiment_result(&experiment_obj, &result, delta).await;

                println!("[Sovereign-Lab] ✅ Sesión {} completada. Daithon analiza el feedback físico real...", session_id);
            }

            if count >= 50 {
                println!("\n[SYSTEM] Se han completado los 50 experimentos de la prueba de soberanía masiva.");
                std::process::exit(0);
            }
        }
    });


    println!("🚀 Sistema listo. Daithon está analizando áreas de poca confianza...");
    
    // Mantener vivo el programa
    let _ = tokio::join!(learner_handle, simulation_handle);
}
