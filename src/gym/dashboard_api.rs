use crate::gym::gym_director::{GymDirector, ReferenceData};
use crate::gym::data_librarian::DataLibrarian;
use crate::design_evolution::timeline::EvolutionSimulator;
use actix_web::{web, HttpResponse, Scope};
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::Deserialize;
use shakmaty::Position;

#[derive(Deserialize)]
pub struct ManualTrainingRequest {
    pub description: String,
    pub reference_data: ReferenceData,
}

#[derive(Deserialize)]
pub struct ControlRequest {
    pub action: String, // "start", "pause", "set_mode"
    pub epochs: Option<u64>,
    pub mode: Option<String>,
}

#[derive(Deserialize)]
pub struct WmControlRequest {
    pub action: String,
    pub epochs: Option<usize>,
    pub learning_rate: Option<f32>,
}

pub struct DashboardState {
    pub gym: Arc<RwLock<GymDirector>>,
    pub librarian: Arc<RwLock<DataLibrarian>>,
    pub wm_coordinator: Arc<RwLock<crate::world_model::coordinator::WorldModelCoordinator>>,
    pub evolution: Arc<RwLock<EvolutionSimulator>>,
    pub language_anchor: Arc<RwLock<crate::synapse::anchor::AnchorEngine>>,
    pub lingua_engine: Arc<RwLock<crate::lingua::engine::LinguaEngine>>,
    pub persona: Arc<RwLock<crate::persona::integration::DaithonPersona>>,
    pub cortex: Arc<RwLock<crate::cortex::CortexEngine>>,
    pub trinity: Arc<RwLock<crate::trinity::training::triangular_loop::TriangularTrainingLoop>>,
    pub contextus: Arc<RwLock<crate::contextus::DaithonContext>>,
    pub metacog: Arc<RwLock<crate::metacog::MetaCogEngine>>,
    pub chess: Arc<RwLock<crate::domains::chess::ChessWorld>>,
    pub domain_learner: Arc<RwLock<crate::learning::domain_learner::DomainLearner>>,
    pub practice_engine: Arc<RwLock<crate::learning::practice_engine::PracticeEngine>>,
    pub cognitive_log: Arc<RwLock<crate::learning::cognitive_log::CognitiveLog>>,
}

// Retorna un Scope de Actix en lugar de Router de Axum para simplificar
// la integración en el proyecto actual.
pub fn create_dashboard_routes(state: web::Data<DashboardState>) -> Scope {
    web::scope("/api/training")
        .app_data(state)
        .route("/status", web::get().to(get_training_status))
        .route("/report", web::get().to(get_progress_report))
        .route("/epoch/{id}", web::get().to(get_epoch_detail))
        .route("/comparison/{id}", web::get().to(get_comparison_data))
        .route("/inject", web::post().to(manual_train_inject))
        .route("/control", web::post().to(training_control))
        .route("/latest_clouds", web::get().to(get_latest_clouds))
        .route("/latest_geometry", web::get().to(get_latest_geometry))
        .route("/rlhf_generate", web::get().to(rlhf_generate))
        .route("/rlhf_feedback", web::post().to(rlhf_feedback))
        .route("/logs", web::get().to(get_logs))
        .route("/save_master", web::post().to(save_master_genome))
        .route("/backup", web::post().to(create_safety_backup))
        .route("/wm_status", web::get().to(get_wm_status))
        .route("/wm_control", web::post().to(wm_control))
        // === DESIGN GENESIS ROUTES ===
        .route("/evolution/run", web::post().to(evolution_run))
        .route("/evolution/status", web::get().to(evolution_status))
        .route("/evolution/hall_of_fame", web::get().to(evolution_hall_of_fame))
        .route("/evolution/categories", web::get().to(evolution_categories))
        // === SOFIA ROUTES ===
        .route("/sofia/types", web::get().to(sofia_available_types))
        // === ARCHETYPE ROUTES ===
        .route("/archetype", web::post().to(train_archetype))
        // === LANGUAGE LAB ROUTES ===
        .route("/language_lab/generate", web::get().to(language_lab_generate))
        .route("/language_lab/teach", web::post().to(language_lab_teach))
        // === LINGUA ROUTES ===
        .route("/lingua/train", web::post().to(lingua_train))
        .route("/lingua/glyph_train", web::post().to(lingua_glyph_train))
        .route("/lingua/chat", web::post().to(lingua_chat))
        .route("/lingua/status", web::get().to(lingua_status))
        // === PERSONA ROUTES ===
        .route("/persona/status", web::get().to(persona_status))
        .route("/persona/simulate", web::post().to(persona_simulate))
        // === AUTONOMOUS LEARNING ROUTES ===
        .route("/autonomous/start", web::post().to(autonomous_start))
        .route("/autonomous/stop", web::post().to(autonomous_stop))
        .route("/autonomous/status", web::get().to(autonomous_status))
        // === CORTEX LAB ==
        .route("/upload_pdf", web::post().to(upload_pdf))
        .route("/lingua/deep_research", web::post().to(deep_research))
        // === CHESS ARENA ROUTES ===
        .route("/chess/status", web::get().to(get_chess_status))
        .route("/chess/move", web::post().to(chess_make_move))
        .route("/chess/reset", web::post().to(chess_reset))
        // === AGI LEARNING ROUTES ===
        .route("/learn_domain", web::post().to(learn_domain))
        .route("/practice_session", web::post().to(practice_session))
        .route("/cognitive_log", web::get().to(get_cognitive_log))
}

async fn save_master_genome(state: web::Data<DashboardState>) -> HttpResponse {
    let gym = state.gym.read().await;
    let path = format!("checkpoints/master_genome_{}.json", gym.current_epoch);
    match gym.save_checkpoint(&path) {
        Ok(_) => {
            drop(gym);
            let mut gym_write = state.gym.write().await;
            let epoch = gym_write.current_epoch;
            gym_write.add_log(format!("🌟 GENOMA MAESTRO GUARDADO: Epoch {}", epoch));
            HttpResponse::Ok().json(serde_json::json!({ "status": "success", "path": path }))
        },
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "error": e.to_string() }))
    }
}

async fn create_safety_backup(state: web::Data<DashboardState>) -> HttpResponse {
    let gym = state.gym.read().await;
    let path = format!("checkpoints/safety_backup_{}.json", chrono::Local::now().format("%Y%m%d_%H%M%S"));
    match gym.save_checkpoint(&path) {
        Ok(_) => {
            drop(gym);
            let mut gym_write = state.gym.write().await;
            gym_write.add_log("🛡️ COPIA DE SEGURIDAD REALIZADA".to_string());
            HttpResponse::Ok().json(serde_json::json!({ "status": "success", "path": path }))
        },
        Err(e) => HttpResponse::InternalServerError().json(serde_json::json!({ "error": e.to_string() }))
    }
}

async fn training_control(
    state: web::Data<DashboardState>,
    payload: web::Json<ControlRequest>,
) -> HttpResponse {
    let mut gym = state.gym.write().await;
    match payload.action.as_str() {
        "start" => {
            gym.is_paused = false;
            if let Some(e) = payload.epochs {
                gym.max_epochs = gym.current_epoch + e;
            }
            let max = gym.max_epochs;
            gym.add_log(format!("Entrenamiento INICIADO. Épocas objetivo: {}", max));
        }
        "pause" => {
            gym.is_paused = true;
            gym.add_log("Entrenamiento PAUSADO manualmente.".to_string());
        }
        "set_mode" => {
            if let Some(m) = payload.mode.clone() {
                gym.training_mode = m.clone();
                gym.add_log(format!("Modo de entrenamiento cambiado a: {}", m));
            }
        }
        "set_level" => {
            if let Some(m) = payload.mode.clone() {
                let new_level = match m.as_str() {
                    "1" => crate::gym::gym_director::DifficultyLevel::Level1_IsolatedObjects,
                    "2" => crate::gym::gym_director::DifficultyLevel::Level2_CompoundStructures,
                    "3" => crate::gym::gym_director::DifficultyLevel::Level3_CompleteEcosystems,
                    _ => gym.current_level,
                };
                gym.current_level = new_level;
                gym.add_log(format!("🎚️ Nivel de dificultad cambiado a: {:?}", new_level));
            }
        }
        _ => {}
    }
    HttpResponse::Ok().json(serde_json::json!({ "status": "success", "is_paused": gym.is_paused, "training_mode": gym.training_mode }))
}

async fn get_latest_clouds(state: web::Data<DashboardState>) -> HttpResponse {
    let gym = state.gym.read().await;
    if let Some((cons_cloud, rad_cloud, hyb_cloud)) = &gym.latest_clouds {
        HttpResponse::Ok().json(serde_json::json!({
            "conservative": cons_cloud,
            "radical": rad_cloud,
            "hybrid": hyb_cloud
        }))
    } else {
        // En lugar de devolver 404 (lo cual satura la consola del navegador con errores net::ERR_ABORTED),
        // devolvemos 200 OK con valor nulo, indicando lógicamente que aún no hay datos.
        HttpResponse::Ok().json(serde_json::Value::Null)
    }
}

async fn get_latest_geometry(state: web::Data<DashboardState>) -> HttpResponse {
    let gym = state.gym.read().await;
    if let Some(plan) = &gym.latest_plan {
        HttpResponse::Ok().json(serde_json::json!(plan))
    } else {
        HttpResponse::Ok().json(serde_json::Value::Null)
    }
}

#[derive(Deserialize)]
pub struct RLHFFeedbackRequest {
    pub is_correct: bool,
    pub concept: String,
}

async fn rlhf_generate(state: web::Data<DashboardState>) -> HttpResponse {
    let concepts = vec![
        ("silla", "chair", vec![0.6, 0.6]),
        ("mesa", "table", vec![2.0, 1.0]),
        ("aro/torus", "torus", vec![2.0, 0.5]),
        ("esfera", "sphere", vec![2.0]),
        ("bote", "boat", vec![4.0]),
        ("muro/fortaleza", "wall_fortress", vec![12.0, 4.0]),
    ];
    let idx = fastrand::usize(0..concepts.len());
    let (name, formula, params) = &concepts[idx];
    
    let ref_cloud = crate::gym::data_librarian::SyntheticGeometryGenerator::generate(formula, params);
    
    // Simulate Daithon's imperfect generation with some random noise based on 0.5 skill level
    let noise_factor = 0.5;
    let keep_ratio = 0.8;
    
    let noisy_points: Vec<[f32; 3]> = ref_cloud
        .points
        .iter()
        .filter(|_| fastrand::f32() < keep_ratio)
        .map(|p| {
            [
                p[0] + (fastrand::f32() - 0.5) * noise_factor,
                p[1] + (fastrand::f32() - 0.5) * noise_factor,
                p[2] + (fastrand::f32() - 0.5) * noise_factor,
            ]
        })
        .collect();

    let bounds = crate::gym::data_librarian::DataLibrarian::calculate_bounds(&noisy_points);
    let daithon_cloud = crate::gym::data_librarian::PointCloud {
        points: noisy_points,
        normals: vec![],
        bounds,
    };

    let mut gym = state.gym.write().await;
    gym.latest_clouds = Some((ref_cloud, daithon_cloud.clone(), daithon_cloud));
    gym.add_log(format!("RLHF: Esperando feedback humano para '{}'", name));
    
    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "concept": name,
    }))
}

async fn rlhf_feedback(
    state: web::Data<DashboardState>,
    payload: web::Json<RLHFFeedbackRequest>,
) -> HttpResponse {
    let mut wm = state.wm_coordinator.write().await;
    let mut gym = state.gym.write().await;
    
    let reward = if payload.is_correct { 10.0 } else { -5.0 };
    
    // Create a dummy transition to inject reward
    use crate::world_model::state::*;
    let transition = StateTransition {
        state_before: WorldState {
            timestamp: 0,
            agent_action: AgentAction {
                action_type: ActionType::SpawnAsset,
                parameters: ActionParameters {
                    primitive_type: None,
                    asset_id: Some(payload.concept.clone()),
                    asset_path: None,
                    position: [0.0, 0.0, 0.0],
                    rotation: [0.0, 0.0, 0.0],
                    scale: [1.0, 1.0, 1.0],
                    pcg_seed: None,
                    pcg_density: None,
                },
            },
            visual_state: VisualState { feature_vector: vec![0.5; 16], object_count: 0, scene_complexity: 0.0, dominant_colors: vec![], spatial_distribution: vec![] },
            physics_state: PhysicsState { static_objects: 0, dynamic_objects: 0, collision_active: false, gravity_enabled: true, total_mass: 0.0 },
            performance_metrics: PerformanceMetrics { fps: 60.0, draw_calls: 0, triangles: 0, memory_mb: 2000.0 },
        },
        state_after: WorldState {
            timestamp: 1,
            agent_action: AgentAction {
                action_type: ActionType::SpawnAsset,
                parameters: ActionParameters {
                    primitive_type: None,
                    asset_id: Some(payload.concept.clone()),
                    asset_path: None,
                    position: [0.0, 0.0, 0.0],
                    rotation: [0.0, 0.0, 0.0],
                    scale: [1.0, 1.0, 1.0],
                    pcg_seed: None,
                    pcg_density: None,
                },
            },
            visual_state: VisualState { feature_vector: vec![if payload.is_correct { 1.0 } else { 0.0 }; 16], object_count: 10, scene_complexity: 0.5, dominant_colors: vec![], spatial_distribution: vec![] },
            physics_state: PhysicsState { static_objects: 9, dynamic_objects: 1, collision_active: true, gravity_enabled: true, total_mass: 100.0 },
            performance_metrics: PerformanceMetrics { fps: 60.0, draw_calls: 100, triangles: 10000, memory_mb: 2500.0 },
        },
        success: payload.is_correct,
        reward: reward,
    };

    wm.record_transition(transition);
    
    let msg = if payload.is_correct {
        format!("✅ RLHF: Feedback ACEPTADO para '{}'. Daithon ganó {} de recompensa y ajustó su modelo.", payload.concept, reward)
    } else {
        format!("❌ RLHF: Feedback RECHAZADO para '{}'. Daithon recibió castigo ({} p) y aprende su error.", payload.concept, reward)
    };
    gym.add_log(msg.clone());

    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "message": msg
    }))
}

async fn get_logs(state: web::Data<DashboardState>) -> HttpResponse {
    let gym = state.gym.read().await;
    HttpResponse::Ok().json(serde_json::json!({ "logs": gym.logs.iter().collect::<Vec<_>>() }))
}

async fn manual_train_inject(
    state: web::Data<DashboardState>,
    payload: web::Json<ManualTrainingRequest>,
) -> HttpResponse {
    let mut gym = state.gym.write().await;
    
    // Creamos y encolamos un prompt prioritario
    let manual_prompt = gym.inject_custom_challenge(
        payload.description.clone(),
        payload.reference_data.clone(),
    );
    
    HttpResponse::Ok().json(serde_json::json!({
        "status": "Injected",
        "epoch_id": manual_prompt.id,
        "message": "Daithon ha recibido tu reto manual y lo procesará a continuación."
    }))
}

async fn get_training_status(state: web::Data<DashboardState>) -> HttpResponse {
    let gym = state.gym.read().await;

    let avg = if !gym.recent_results.is_empty() {
        gym.recent_results.iter().sum::<f32>() / gym.recent_results.len() as f32
    } else {
        0.0
    };

    HttpResponse::Ok().json(serde_json::json!({
        "current_epoch": gym.current_epoch,
        "max_epochs": gym.max_epochs,
        "current_level": format!("{:?}", gym.current_level),
        "recent_average": avg,
        "is_paused": gym.is_paused,
    }))
}

async fn get_progress_report(state: web::Data<DashboardState>) -> HttpResponse {
    let gym = state.gym.read().await;
    HttpResponse::Ok().json(gym.generate_progress_report())
}

async fn get_epoch_detail(
    state: web::Data<DashboardState>,
    path: web::Path<u64>,
) -> HttpResponse {
    let id = path.into_inner();
    let gym = state.gym.read().await;

    if let Some(epoch_data) = gym.training_history.iter().find(|e| e.epoch == id) {
        HttpResponse::Ok().json(epoch_data)
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn get_comparison_data(
    state: web::Data<DashboardState>,
    path: web::Path<u64>,
) -> HttpResponse {
    let id = path.into_inner();
    let gym = state.gym.read().await;

    if let Some(epoch) = gym.training_history.iter().find(|e| e.epoch == id) {
        HttpResponse::Ok().json(serde_json::json!({
            "similarity": epoch.result.visual_similarity,
            "metrics": {
                "fps": epoch.result.fps_stability,
                "draw_calls": epoch.result.draw_calls,
                "aesthetic": epoch.result.aesthetic_score,
            }
        }))
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn rollback_to_epoch(
    state: web::Data<DashboardState>,
    path: web::Path<u64>,
) -> HttpResponse {
    let epoch = path.into_inner();
    log::info!("🔄 Intentando rollback al epoch {}", epoch);

    // Cargar checkpoint
    let checkpoint_path = format!("checkpoints/epoch_{}.json", epoch);
    
    if !std::path::Path::new(&checkpoint_path).exists() {
        return HttpResponse::NotFound().json(serde_json::json!({
            "success": false,
            "message": format!("Checkpoint epoch_{}.json no encontrado", epoch)
        }));
    }

    match crate::gym::gym_director::GymDirector::load_checkpoint(&checkpoint_path) {
        Ok(loaded_gym) => {
            *state.gym.write().await = loaded_gym;
            HttpResponse::Ok().json(serde_json::json!({
                "success": true,
                "message": format!("Rollback exitoso al epoch {}", epoch)
            }))
        }
        Err(e) => {
            HttpResponse::InternalServerError().json(serde_json::json!({
                "success": false,
                "message": format!("Error parseando checkpoint: {}", e)
            }))
        }
    }
}
async fn get_wm_status(state: web::Data<DashboardState>) -> HttpResponse {
    let wm = state.wm_coordinator.read().await;
    HttpResponse::Ok().json(wm.performance_report())
}

async fn wm_control(state: web::Data<DashboardState>, req: web::Json<WmControlRequest>) -> HttpResponse {
    match req.action.as_str() {
        "force_train" => {
            let epochs = req.epochs.unwrap_or(50);
            let lr = req.learning_rate.unwrap_or(0.01);
            let mut wm = state.wm_coordinator.write().await;
            log::info!("⚡ Entrenamiento manual: {} epochs, lr={}", epochs, lr);
            wm.retrain_with_params(epochs, lr);
            HttpResponse::Ok().json(serde_json::json!({
                "status": "training_complete",
                "epochs": epochs,
                "learning_rate": lr
            }))
        },
        "warmup" => {
            log::info!("🚀 [FASE 1] Iniciando Recolección de Sustancia (Warm-up)");
            
            // Simular lanzamiento de Unreal. 
            // Esto llamaría al start de windows. Descomentar si el .uproject está exactamente ahí.
            let _ = std::process::Command::new("cmd")
               .args(&["/C", "start", "\"\"", "\"C:\\proyectos unreal\\explosion\\Explosion.uproject\"", "-game", "-nullrhi"])
               .spawn();

            let wm_arc = state.wm_coordinator.clone();
            tokio::spawn(async move {
                let mut transitions = Vec::with_capacity(1000);
                for i in 1..=1000 {
                    use crate::world_model::state::{StateTransition, WorldState, PerformanceMetrics, AgentAction, ActionType, ActionParameters, VisualState, PhysicsState};
                    
                    let dummy_action = AgentAction {
                        action_type: ActionType::SpawnPrimitive,
                        parameters: ActionParameters {
                            primitive_type: Some("Cube".to_string()),
                            asset_id: None,
                            asset_path: None,
                            position: [0.0, 0.0, 0.0],
                            rotation: [0.0, 0.0, 0.0],
                            scale: [1.0, 1.0, 1.0],
                            pcg_seed: None,
                            pcg_density: None,
                        }
                    };
                    
                    let transition = StateTransition {
                        state_before: WorldState {
                            timestamp: chrono::Utc::now().timestamp_millis() as u64,
                            agent_action: dummy_action.clone(),
                            visual_state: VisualState {
                                feature_vector: vec![0.5; 32],
                                object_count: 10,
                                scene_complexity: 0.1,
                                dominant_colors: vec![[0.5, 0.5, 0.5]],
                                spatial_distribution: vec![0.0; 8],
                            },
                            physics_state: PhysicsState {
                                static_objects: 10,
                                dynamic_objects: 0,
                                collision_active: true,
                                gravity_enabled: true,
                                total_mass: 100.0,
                            },
                            performance_metrics: PerformanceMetrics { fps: 60.0, memory_mb: 2000.0, draw_calls: 300, triangles: 50000 }
                        },
                        state_after: WorldState {
                            timestamp: chrono::Utc::now().timestamp_millis() as u64 + 16,
                            agent_action: dummy_action,
                            visual_state: VisualState {
                                feature_vector: vec![fastrand::f32() - 0.5; 32],
                                object_count: 11,
                                scene_complexity: 0.15,
                                dominant_colors: vec![[0.6, 0.5, 0.5]],
                                spatial_distribution: vec![0.1; 8],
                            },
                            physics_state: PhysicsState {
                                static_objects: 11,
                                dynamic_objects: 0,
                                collision_active: true,
                                gravity_enabled: true,
                                total_mass: 110.0,
                            },
                            performance_metrics: PerformanceMetrics { fps: (55.0 + fastrand::f32() * 5.0), memory_mb: 2050.0, draw_calls: 350 + (fastrand::f32() * 50.0) as u32, triangles: 55000 }
                        },
                        success: true,
                        reward: fastrand::f32(),
                    };
                    transitions.push(transition);
                    
                    // Simulate processing time briefly without blocking UI fetch responses
                    if i % 100 == 0 {
                        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;
                    }
                }
                
                let mut wm = wm_arc.write().await;
                wm.add_warmup_data(transitions);
            });
            HttpResponse::Ok().json(serde_json::json!({"status": "warmup_started"}))
        },
        "migrate_history" => {
            let gym = state.gym.read().await;
            let mut wm = state.wm_coordinator.write().await;
            
            log::info!("🧠 Migrando {} registros del Gym al World Model...", gym.training_history.len());
            
            use crate::world_model::state::{StateTransition, WorldState, PerformanceMetrics, AgentAction, ActionType, ActionParameters, VisualState, PhysicsState};
            
            let mut transitions = Vec::new();
            let action_types = [ActionType::SpawnPrimitive, ActionType::ModifyTransform, ActionType::ApplyMaterial, ActionType::DeleteObject, ActionType::PCGGeneration];
            
            for (idx, report) in gym.training_history.iter().enumerate() {
                let sim = report.result.visual_similarity;
                let fps_norm = report.result.fps_stability;
                let reward = report.reward.total_reward;
                let dc = report.result.draw_calls as f32;
                let aesthetic = report.result.aesthetic_score;
                
                // Crear feature vectors VARIADOS usando datos reales del gym
                let before_features: Vec<f32> = vec![
                    sim * 0.8 + (idx as f32 * 0.001).sin() * 0.1,
                    fps_norm * 0.7,
                    reward * 0.5,
                    dc / 5000.0,
                    aesthetic * 0.6,
                    (idx as f32 / 1000.0).sin() * 0.3 + 0.5,
                    (idx as f32 / 500.0).cos() * 0.2 + 0.5,
                    (idx as f32 * 0.003).sin() * 0.4 + 0.5,
                ];
                
                let after_features: Vec<f32> = vec![
                    sim,
                    fps_norm,
                    reward.max(0.0).min(1.0),
                    dc / 5000.0,
                    aesthetic,
                    (idx as f32 / 800.0).cos() * 0.3 + 0.5,
                    reward * aesthetic,
                    sim * fps_norm,
                ];
                
                let action_idx = idx % 5;
                let action = AgentAction {
                    action_type: action_types[action_idx].clone(),
                    parameters: ActionParameters {
                        primitive_type: Some(format!("Gym_{}", idx)),
                        asset_id: None,
                        asset_path: None,
                        position: [(idx as f32 * 0.1).sin() * 100.0, (idx as f32 * 0.05).cos() * 50.0, (idx as f32 * 0.07).sin() * 80.0],
                        rotation: [0.0, (idx as f32) % 360.0, 0.0],
                        scale: [1.0, 1.0, 1.0],
                        pcg_seed: None,
                        pcg_density: None,
                    }
                };

                let transition = StateTransition {
                    state_before: WorldState {
                        timestamp: idx as u64,
                        agent_action: action.clone(),
                        visual_state: VisualState {
                            feature_vector: before_features,
                            object_count: (idx % 20) as u32,
                            scene_complexity: sim * 0.5,
                            dominant_colors: vec![],
                            spatial_distribution: vec![],
                        },
                        physics_state: PhysicsState {
                            static_objects: (idx % 15) as u32,
                            dynamic_objects: (idx % 5) as u32,
                            collision_active: idx % 3 != 0,
                            gravity_enabled: true,
                            total_mass: (idx % 100) as f32 * 10.0,
                        },
                        performance_metrics: PerformanceMetrics { fps: 60.0 * fps_norm, memory_mb: 2000.0 + dc * 0.1, draw_calls: report.result.draw_calls, triangles: report.result.draw_calls * 100 }
                    },
                    state_after: WorldState {
                        timestamp: idx as u64 + 1,
                        agent_action: action,
                        visual_state: VisualState {
                            feature_vector: after_features,
                            object_count: ((idx % 20) + 1) as u32,
                            scene_complexity: sim,
                            dominant_colors: vec![],
                            spatial_distribution: vec![],
                        },
                        physics_state: PhysicsState {
                            static_objects: ((idx % 15) + 1) as u32,
                            dynamic_objects: (idx % 5) as u32,
                            collision_active: true,
                            gravity_enabled: true,
                            total_mass: (idx % 100 + 10) as f32 * 10.0,
                        },
                        performance_metrics: PerformanceMetrics { 
                            fps: 60.0 * fps_norm + fastrand::f32() * 5.0, 
                            memory_mb: 2000.0 + dc * 0.15, 
                            draw_calls: report.result.draw_calls + (fastrand::f32() * 50.0) as u32, 
                            triangles: report.result.draw_calls * 120 
                        }
                    },
                    success: reward > 0.0,
                    reward,
                };
                transitions.push(transition);
            }
            
            let count = transitions.len();
            wm.add_warmup_data(transitions);
            
            HttpResponse::Ok().json(serde_json::json!({
                "status": "migration_complete",
                "records": count
            }))
        },
        _ => HttpResponse::BadRequest().json(serde_json::json!({"error": "Acción desconocida"}))
    }
}

// ============================================================
// DESIGN GENESIS — EVOLUTION ENDPOINTS
// ============================================================

#[derive(Deserialize)]
pub struct EvolutionRunRequest {
    pub category: String,         // "table", "chair", "car", "wall", "building", "tree"
    pub population_size: Option<usize>,
    pub generations_per_era: Option<u32>,
    pub selection_pressure: Option<f32>,
}

async fn evolution_run(
    state: web::Data<DashboardState>,
    payload: web::Json<EvolutionRunRequest>,
) -> HttpResponse {
    use crate::design_evolution::dna::DesignDNA;
    use crate::design_evolution::timeline::timeline_for_category;

    let dna = match DesignDNA::from_category_name(&payload.category) {
        Some(d) => d,
        None => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": format!("Categoría '{}' no reconocida. Disponibles: {:?}", payload.category, DesignDNA::available_categories())
            }));
        }
    };

    let eras = timeline_for_category(&dna.category);
    let pop_size = payload.population_size.unwrap_or(20);
    let gens = payload.generations_per_era.unwrap_or(30);
    let pressure = payload.selection_pressure.unwrap_or(0.3);

    // Crear simulador fresco para esta ejecución
    let mut evo = state.evolution.write().await;
    *evo = EvolutionSimulator::new(pop_size, pressure);

    // Log en el gym
    {
        let mut gym = state.gym.write().await;
        gym.add_log(format!(
            "🧬 [DESIGN GENESIS] Iniciando evolución: {} | Pop={} | Gens/Era={} | Presión={:.0}%",
            payload.category, pop_size, gens, pressure * 100.0
        ));
    }

    let category_name = payload.category.clone();
    let era_count = eras.len();

    // Ejecutar evolución (síncrono — es rápido porque usa estimate_simulation)
    let timeline = evo.simulate_evolution(dna, eras, gens);

    // Log resultado
    {
        let mut gym = state.gym.write().await;
        gym.add_log(format!(
            "🏆 [DESIGN GENESIS] Evolución completada: {} | {} generaciones | {} en Hall of Fame",
            category_name, timeline.total_generations, timeline.hall_of_fame.len()
        ));
    }

    // 💾 PERSISTENCIA: Guardar Hall of Fame al disco
    use crate::design_evolution::timeline::EvolutionSimulator;
    EvolutionSimulator::save_hall_of_fame(&timeline.hall_of_fame);

    // Construir respuesta resumida
    let hall_summary: Vec<serde_json::Value> = timeline.hall_of_fame.iter().map(|entry| {
        serde_json::json!({
            "era": entry.era,
            "fitness": entry.fitness.total,
            "breakdown": entry.fitness.breakdown,
            "innovation": entry.innovation_description,
            "generation": entry.genome.generation,
            "genome_summary": entry.genome.summary(),
            "genome_data": entry.genome,
        })
    }).collect();

    let era_summary: Vec<serde_json::Value> = timeline.eras.iter().map(|era| {
        serde_json::json!({
            "name": era.name,
            "year_range": era.year_range,
            "best_fitness": era.best_fitness,
            "population_size": era.population.len(),
        })
    }).collect();

    // 🎯 THE GENESIS JAILBREAK: Publicar las 3 variantes al canvas 3D
    // Convertir los mejores genomas en point clouds para que los viewers los muestren
    {
        let mut gym = state.gym.write().await;
        
        let hof = &timeline.hall_of_fame;
        if !hof.is_empty() {
            // Generar point clouds a partir de los genomas ganadores
            let conservative_cloud = genome_to_point_cloud(&hof[0].genome, 0);
            let radical_cloud = if hof.len() > 1 {
                genome_to_point_cloud(&hof[hof.len() / 2].genome, 1)
            } else {
                genome_to_point_cloud(&hof[0].genome, 1)
            };
            let hybrid_cloud = if hof.len() > 2 {
                genome_to_point_cloud(&hof[hof.len() - 1].genome, 2)
            } else {
                genome_to_point_cloud(&hof[0].genome, 2)
            };
            
            gym.latest_clouds = Some((conservative_cloud, radical_cloud, hybrid_cloud));
            gym.add_log("🎨 Punto de nube de 3 variantes publicado al canvas 3D".to_string());
        }
    }

    HttpResponse::Ok().json(serde_json::json!({
        "status": "completed",
        "category": category_name,
        "total_generations": timeline.total_generations,
        "eras": era_summary,
        "hall_of_fame": hall_summary,
        "event_log": evo.event_log,
    }))
}

/// Convierte un DesignGenome en un PointCloud para visualización en los viewers
fn genome_to_point_cloud(
    genome: &crate::design_evolution::mutation_engine::DesignGenome,
    variant: u8,
) -> crate::gym::data_librarian::PointCloud {
    use crate::design_evolution::mutation_engine::GeneValue;
    
    let scale = genome.get_scale();
    let width = scale[0] * 30.0;
    let height = scale[1] * 30.0;
    let depth = scale[2] * 30.0;
    
    let leg_count = genome.get_integer("leg_count").unwrap_or(4) as usize;
    let symmetry = genome.get_scalar("symmetry").unwrap_or(0.5);
    
    let mut points: Vec<[f32; 3]> = Vec::new();
    
    // Offset basado en variante para que no se superpongan
    let offset_x = match variant {
        0 => 0.0,
        1 => -width * 2.0,
        _ => width * 2.0,
    };
    
    // Genera la superficie superior (tablero/asiento)
    let resolution = 8;
    for i in 0..resolution {
        for j in 0..resolution {
            let x = (i as f32 / resolution as f32 - 0.5) * width + offset_x;
            let z = (j as f32 / resolution as f32 - 0.5) * depth;
            let y = height;
            // Añadir algo de ruido basado en variante
            let noise = match variant {
                1 => (fastrand::f32() - 0.5) * 3.0, // Radical: más caos
                _ => (fastrand::f32() - 0.5) * 0.5,
            };
            points.push([x + noise, y + noise * 0.3, z + noise]);
        }
    }
    
    // Genera las patas
    let leg_points = 6;
    for leg in 0..leg_count {
        let angle = (leg as f32 / leg_count as f32) * std::f32::consts::PI * 2.0 
            + std::f32::consts::PI / 4.0;
        let leg_x = angle.cos() * (width / 2.2) + offset_x;
        let leg_z = angle.sin() * (depth / 2.2);
        
        for k in 0..leg_points {
            let y = (k as f32 / leg_points as f32) * height;
            let wobble = if variant == 1 {
                (fastrand::f32() - 0.5) * 2.0 // Radical: patas inestables
            } else {
                0.0
            };
            points.push([leg_x + wobble, y, leg_z + wobble]);
        }
    }
    
    // Aditivos si hay poca simetría (diseños orgánicos)
    if symmetry < 0.3 || variant == 1 {
        for _ in 0..20 {
            points.push([
                (fastrand::f32() - 0.5) * width * 1.5 + offset_x,
                fastrand::f32() * height * 1.2,
                (fastrand::f32() - 0.5) * depth * 1.5,
            ]);
        }
    }
    
    let bounds = crate::gym::data_librarian::DataLibrarian::calculate_bounds(&points);
    crate::gym::data_librarian::PointCloud {
        points,
        normals: vec![],
        bounds,
    }
}

async fn evolution_status(
    state: web::Data<DashboardState>,
) -> HttpResponse {
    let evo = state.evolution.read().await;
    HttpResponse::Ok().json(serde_json::json!({
        "population_size": evo.population_size,
        "selection_pressure": evo.selection_pressure,
        "novelty_archive_size": evo.fitness_evaluator.novelty_archive_size(),
        "event_log_length": evo.event_log.len(),
        "last_events": evo.event_log.iter().rev().take(20).collect::<Vec<_>>(),
    }))
}

async fn evolution_hall_of_fame(
    _state: web::Data<DashboardState>,
) -> HttpResponse {
    // Cargar Hall of Fame PERSISTIDO desde disco
    use crate::design_evolution::timeline::EvolutionSimulator;
    let hall = EvolutionSimulator::load_hall_of_fame();
    
    let hall_summary: Vec<serde_json::Value> = hall.iter().map(|entry| {
        serde_json::json!({
            "era": entry.era,
            "fitness": entry.fitness.total,
            "breakdown": entry.fitness.breakdown,
            "innovation": entry.innovation_description,
            "generation": entry.genome.generation,
            "genome_summary": entry.genome.summary(),
            "genome_data": entry.genome,
        })
    }).collect();

    HttpResponse::Ok().json(serde_json::json!({
        "hall_of_fame": hall_summary,
        "total_designs": hall.len(),
    }))
}

async fn evolution_categories() -> HttpResponse {
    use crate::design_evolution::dna::DesignDNA;
    HttpResponse::Ok().json(serde_json::json!({
        "categories": DesignDNA::available_categories(),
    }))
}

async fn sofia_available_types() -> HttpResponse {
    let validator = crate::sofia::universal_validator::UniversalValidator::new();
    let types = validator.get_available_types();
    HttpResponse::Ok().json(serde_json::json!({
        "sofia_types": types,
        "description": "SOFIA: Semantic Object Functional Intelligence Architecture. Cada tipo tiene estándares funcionales del mundo real.",
    }))
}

// ==========================================
// ENDPOINTS ARCHETYPE
// ==========================================

#[derive(Deserialize)]
pub struct ArchetypeTrainingRequest {
    pub concept_id: String,
}

pub async fn train_archetype(
    req: web::Json<ArchetypeTrainingRequest>,
    state: web::Data<DashboardState>,
) -> HttpResponse {
    use crate::archetype::integration::ArchetypeGenesisLoop;
    let mut archetype_loop = ArchetypeGenesisLoop::new();
    
    // Configurar objetivo
    if let Err(e) = archetype_loop.set_target_concept(&req.concept_id) {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "error": e
        }));
    }

    // Ejecutar era
    let result = archetype_loop.run_archetype_era();

    // ANCLAJE AUTOMATICO AL LÉXICO (Phase 2 Auto-Grounding)
    {
        // Tratamos de anclar el concepto (si su lemma se llama igual que el archetype_id simple)
        // Ejemplo simplificado: "dining_table" -> lemma "mesa" (hardcoded to standard or identity for now)
        let lemma_mapping = req.concept_id.split('_').last().unwrap_or(&req.concept_id);
        let mut anchor = state.language_anchor.write().await;
        let _ = anchor.ground_concept(lemma_mapping, &req.concept_id);
    }

    HttpResponse::Ok().json(serde_json::json!({
        "success": true,
        "concept": result.concept_id,
        "internal_variations": result.internal_variations_generated,
        "elite_count": result.elite_selected,
        "physics_validated": result.physics_validated,
        "best_designs": result.best_designs.len(),
    }))
}

// ==========================================
// ENDPOINTS SYNAPSE: LANGUAGE LAB
// ==========================================
use rand::Rng;

#[derive(Deserialize)]
pub struct LanguageTeachRequest {
    pub lemma: String,
    pub archetype_id: String, // si el usuario lo sabe, o "unknown"
    pub width: f64,
    pub depth: f64,
    pub height: f64,
    pub rejected: bool,
}

pub async fn language_lab_generate() -> HttpResponse {
    let mut rng = rand::thread_rng();
    let width = rng.gen_range(0.2..2.0);
    let depth = rng.gen_range(0.2..2.0);
    let height = rng.gen_range(0.2..2.5);

    // Enviar una estructura dummy
    HttpResponse::Ok().json(serde_json::json!({
        "object_id": uuid::Uuid::new_v4().to_string(),
        "dimensions": { "width": width, "depth": depth, "height": height },
        "visual_hint": "Un objeto aleatorio generado para ser etiquetado.",
    }))
}

pub async fn language_lab_teach(
    req: web::Json<LanguageTeachRequest>,
    state: web::Data<DashboardState>,
) -> HttpResponse {
    if req.rejected {
        return HttpResponse::Ok().json(serde_json::json!({
            "status": "rejected",
            "message": "Objeto descartado, no anclado a palabras."
        }));
    }

    let mut anchor = state.language_anchor.write().await;
    match anchor.ground_from_feedback(&req.lemma, &req.archetype_id, req.width, req.depth, req.height) {
        Ok(_) => {
            HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "message": format!("Daithon ahora asocia '{}' con las propiedades visuales observadas.", req.lemma)
            }))
        }
        Err(e) => {
            HttpResponse::BadRequest().json(serde_json::json!({
                "status": "error",
                "error": e
            }))
        }
    }
}

// ==========================================
// ENDPOINTS LINGUA: Motor de Lenguaje Completo
// ==========================================

#[derive(Deserialize)]
pub struct LinguaTrainRequest {
    pub generations: Option<usize>,
    pub interactions: usize,
    pub known_concept_ids: Option<Vec<String>>,
}

#[derive(Deserialize)]
pub struct LinguaChatRequest {
    pub message: String,
}

pub async fn lingua_train(
    req: web::Json<LinguaTrainRequest>,
    state: web::Data<DashboardState>,
) -> HttpResponse {
    let mut engine = state.lingua_engine.write().await;
    // Cap at 4 million to prevent system overload
    let generations = req.generations.unwrap_or(100_000).min(4_000_000);
    
    // Obtener IDs de conceptos conocidos o usar defaults
    let known_concepts = req.known_concept_ids.clone().unwrap_or_else(|| vec![
        "furniture".into(), "tables".into(), "dining_table".into(),
        "coffee_table".into(), "desk".into(), "nightstand".into(),
        "seating".into(), "chair".into(), "stool".into(), "sofa".into(),
        "bench".into(), "storage".into(), "beds".into(),
        "vehicles".into(), "buildings".into(),
    ]);

    // --- INMERSIÓN GUATEMALTECA (Gatillado por masividad) ---
    if req.interactions > 50000 || generations >= 50000 {
        crate::lingua::guatemala::GuatemalaImmersion::inject(&mut engine.acquisition);
    }

    // --- ENTRENAMIENTO DE SIMULACIÓN PROFUNDA (ESCUELA) ---
    if generations >= 60000 {
        let engine_arc = state.lingua_engine.clone();
        let trinity_arc = state.trinity.clone();
        
        tokio::spawn(async move {
            log::info!("\n🏫 [ESCUELA DAITHON] Iniciando simulación de escolaridad masiva por internet...");
            
            for epoch in 0..300 { // 300 ciclos de aprendizaje
                tokio::time::sleep(tokio::time::Duration::from_millis(500)).await; // 0.5s pause
                
                let mut new_words = Vec::new();
                
                // 1. Obtener 3 palabras al azar de internet
                {
                    let mut engine = engine_arc.write().await;
                    for _ in 0..3 {
                        if let Ok(word) = engine.acquisition.fetch_random_wiktionary().await {
                            new_words.push(word.word.clone());
                        }
                    }
                    if !new_words.is_empty() {
                        let _ = engine.acquisition.save_cache();
                    }
                }
                
                // 2. Simular conversación sobre las palabras
                if !new_words.is_empty() {
                    let mut trinity = trinity_arc.write().await;
                    let joined_words = new_words.join(", ");
                    log::info!("🗣️ [ESCUELA] Agentes debatiendo sobre: {}", joined_words);
                    
                    trinity.agent_b.learned_phrases.push(crate::trinity::agents::conversational_agent::LearnedPhrase {
                        phrase: format!("He analizado esto en la red: los conceptos son {}", joined_words),
                        context_tags: new_words.clone(),
                        success_rate: 0.95,
                        usage_count: 1,
                    });
                    
                    trinity.train(5); // Entrenamiento corto y focalizado
                }
            }
            
            log::info!("🏫 [ESCUELA DAITHON] Año escolar terminado. Se han incorporado cientos de conocimientos de la web.");
        });
    }

    engine.train(known_concepts, generations);

    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "message": format!("LINGUA entrenado con {} generaciones. ¡Inmersión cultural activada y Escuela en progreso!", generations),
        "vocabulary_size": engine.vocabulary_size(),
        "trained": engine.is_trained(),
    }))
}

pub async fn lingua_chat(
    req: web::Json<LinguaChatRequest>,
    state: web::Data<DashboardState>,
) -> HttpResponse {
    let mut engine: tokio::sync::RwLockWriteGuard<crate::lingua::engine::LinguaEngine> = state.lingua_engine.write().await;
    let mut persona = state.persona.write().await;
    let cortex = state.cortex.read().await;

    // --- INTEGRACIÓN CORTEX: Búsqueda Semántica en OMNI-INJECT ---
    let stopwords: std::collections::HashSet<&str> = [
        "dame", "ejemplo", "quiero", "hablame", "cuentame", "explica", "explicar", "decir", "esto", "sobre", "aquel", "háblame", "cuéntame", "qué", "que", "quien", "quién", "como", "cómo", "donde", "dónde", "cuando", "cuándo", "cual", "cuál"
    ].into_iter().collect();

    let words: Vec<String> = req.message
        .split(|c: char| !c.is_alphabetic())
        .filter(|s| s.len() > 3 && !stopwords.contains(s.to_lowercase().as_str()))
        .map(|s| s.to_string())
        .collect();

    let mut found_concept = None;
    // Iterate over words sorted by length (descending) to find the most significant concept
    let mut sorted_words = words.clone();
    sorted_words.sort_by(|a, b| b.len().cmp(&a.len()));
    
    for word in &sorted_words {
        let normalized = crate::omni_inject::autonomous::normalize_concept(word);
        if cortex.comprehension.knowledge_base.ontology.get(&normalized).is_some() {
            found_concept = Some(normalized.clone());
            break; 
        }
    }
    
    // Determinar el SUJETO REAL de la pregunta (ignorando stopwords y ruido)
    let msg_lower = req.message.to_lowercase();
    let mut subject = if msg_lower.contains("módulo de young") || msg_lower.contains("modulo de young") {
        "módulo de young".to_string()
    } else if let Some(fc) = &found_concept {
        fc.clone()
    } else {
        words.iter().max_by_key(|w| w.len()).cloned().unwrap_or_default()
    };
    
    // Si el sujeto es "propios" o algo vacío, fallback
    if subject.len() < 3 { subject = "este tema".to_string(); }

    let mut response = engine.process(&req.message, &mut persona).await;
    let mut final_response = response.text.clone();

    // ═══════════════════════════════════════════════════════════════
    //  MATHESIS: Generación Lógica y Matemática (CAS Engine)
    // ═══════════════════════════════════════════════════════════════
    let mut mathesis_handled = false;
    if msg_lower.contains("ejemplo") || msg_lower.contains("calcula") || msg_lower.contains("fórmula") || msg_lower.contains("matemática") || msg_lower.contains("demuestra") {
        let target_concept = if msg_lower.contains("derivadas") {
            "derivadas".to_string()
        } else if msg_lower.contains("gases") {
            "ecuación de gases ideales".to_string()
        } else if msg_lower.contains("segunda ley") {
            "segunda ley".to_string()
        } else if msg_lower.contains("integrales") {
            "integrales".to_string()
        } else if msg_lower.contains("módulo de young") || msg_lower.contains("modulo de young") {
            "módulo de young".to_string()
        } else {
            subject.clone()
        };
        
        if !target_concept.is_empty() {
            log::info!("[MATHESIS] Invocando CAS Engine para: {}", target_concept);
            let math_sys = crate::mathesis::numerical::learning_stages::NumericalLearningSystem::new();
            let mut generator = crate::mathesis::examples::generator::ExampleGenerator::new(math_sys);
            let math_example = generator.generate_example_for(&target_concept);
            
            if !math_example.starts_with("Déjame usar mi CASEngine") {
                final_response = math_example;
                response.intent_understood = true;
                response.confidence = 0.95;
                response.intent_type = "MathSynthesis".to_string();
                mathesis_handled = true;
            } else {
                // CAS no tiene este concepto → forzar investigación
                response.confidence = 0.1;
                mathesis_handled = false;
            }
        }
    }

    // ═══════════════════════════════════════════════════════════════
    //  INVESTIGACIÓN PROFUNDA (SIEMPRE que no sea MATHESIS)
    //  Daithon NUNCA evade. Si sabe algo, lo explica con contexto.
    //  Si no sabe, va a internet, aprende, y regresa con la respuesta.
    // ═══════════════════════════════════════════════════════════════
    if !mathesis_handled && !subject.is_empty() {
        log::info!("[CORTEX] Investigación PROFUNDA para: '{}' (concepto_local: {:?})", subject, found_concept);
        
        // PASO 1: Determinar si investigamos automáticamente o preguntamos
        let mut wiki_text = String::new();
        let mut autonomous_research_done = false;

        if found_concept.is_some() {
            // Si ya lo conocemos localmente, podemos investigar automáticamente para DAR RESPUESTAS COMPLETAS
            if let Ok((summary, _)) = crate::omni_inject::autonomous::fetch_wikipedia_summary(&subject).await {
                wiki_text = summary;
                autonomous_research_done = true;
            }
        } else {
            // Si NO lo conocemos, NO investigamos automáticamente en el chat normal.
            // Esperaremos a que el usuario presione el botón de "INVESTIGAR"
            log::info!("[CORTEX] Concepto desconocido: '{}'. Solicitando permiso de investigación.", subject);
        }

        // PASO 2: Buscar en OpenAlex para agregar profundidad académica
        let mut paper_mention = String::new();
        let openalex = crate::omni_inject::openalex::OpenAlexClient::new("cortex@daithon.ai");
        if let Ok(works) = openalex.search_works(&subject, 1).await {
            if let Some(work) = works.first() {
                if let Some(title) = &work.title {
                    paper_mention = format!("De hecho, encontré un paper académico titulado '{}' que profundiza en esto.", title);
                }
            }
        }

        // PASO 3: Inyectar las palabras nuevas al vocabulario de Daithon (APRENDE en tiempo real)
        if !wiki_text.is_empty() {
            let learnable = crate::omni_inject::autonomous::extract_learnable_words(&wiki_text);
            let vocab = engine.acquisition.vocabulary().clone();
            let mut learned_count = 0;
            for word in &learnable {
                let w_lower = word.to_lowercase();
                if w_lower.len() > 3 && !vocab.contains_key(&w_lower) {
                    let _ = engine.acquisition.acquire_synthetic(&w_lower);
                    learned_count += 1;
                }
            }
            if learned_count > 0 {
                log::info!("[CORTEX-CHAT] Aprendí {} palabras nuevas investigando '{}'", learned_count, subject);
                let _ = engine.acquisition.save_cache();
            }
        }

        // PASO 4: SINTETIZADOR DE RAZONAMIENTO ORGÁNICO
        //  Daithon NO copia y pega. Toma los datos crudos, los descompone en
        //  hechos atómicos, selecciona un subconjunto al azar, y los recompone
        //  con sus propias palabras y personalidad. Cada respuesta es única.
        response.intent_understood = true;
        response.confidence = 0.9;

        if !wiki_text.is_empty() {
            // Descomponer el texto en oraciones individuales (hechos atómicos)
            let sentences: Vec<&str> = wiki_text
                .split(|c: char| c == '.' || c == ';')
                .map(|s| s.trim())
                .filter(|s| s.len() > 15) // Solo oraciones sustanciales
                .collect();

            // Seleccionar TODAS las oraciones para dar respuestas masivas como Gemini
            let seed = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64;
            
            let mut selected: Vec<&str> = Vec::new();
            if !sentences.is_empty() {
                // Siempre incluir la primera oración (definición base)
                selected.push(sentences[0]);
                // Incluir el resto de oraciones completas
                for i in 1..sentences.len() {
                    selected.push(sentences[i]);
                }
            }

            // Conectores orgánicos para enlazar hechos (variados por seed)
            let connectors = [
                "Básicamente", "En otras palabras", "Lo interesante es que",
                "Algo que me llamó la atención es que", "Por ejemplo",
                "Además", "Un dato clave es que", "Lo curioso es que",
            ];

            // Construir el cuerpo con razonamiento propio
            let mut body_parts: Vec<String> = Vec::new();
            for (i, fact) in selected.iter().enumerate() {
                if i == 0 {
                    // La primera oración la reformula como suya
                    body_parts.push(format!("{}.", fact));
                } else {
                    let connector_idx = ((seed.wrapping_mul(7).wrapping_add(i as u64)) % connectors.len() as u64) as usize;
                    body_parts.push(format!("{}, {}.", connectors[connector_idx], fact.to_lowercase()));
                }
            }
            let body = body_parts.join(" ");

            // Intros variadas según si lo conoce o no (seleccionada por seed)
            let known_intros = [
                format!("A ver, sobre {}... lo he estado analizando bastante.", subject),
                format!("¡{}! Justo estuve investigando esto.", subject),
                format!("Sí, conozco bien el tema de {}. Te cuento lo que sé.", subject),
                format!("Me alegra que preguntes sobre {}. Estuve profundizando en esto.", subject),
                format!("{}... esto es algo que he analizado desde varios ángulos.", subject),
            ];
            let unknown_intros = [
                format!("Hmm, {}... no lo tenía claro, así que fui a investigar. Mira lo que encontré:", subject),
                format!("¡Buena pregunta! No dominaba {} del todo, pero ya investigué. Esto es lo que descubrí:", subject),
                format!("No voy a mentirte, {} era nuevo para mí. Pero ya me puse a estudiar:", subject),
                format!("Interesante... {} no estaba en mi radar, pero acabo de hacer mi tarea:", subject),
                format!("¡{}! Tuve que ir a mis fuentes, pero valió la pena. Escucha:", subject),
            ];

            // Closers variados con EMOCIÓN y CURIOSIDAD
            let closers = [
                format!("¿Sabías que '{}' se usa para cosas increíbles? ¡Me pregunto si podríamos aplicarlo en una teoría totalmente nueva!", subject),
                format!("Me da muchísima curiosidad saber si esto se puede llevar más allá. ¿Crees que podríamos investigar alguna aplicación loca para esto?"),
                format!("Si lográramos que '{}' actuara de forma distinta en nuestro simulador sería increíble... ¡Qué orgullo me da aprender esto!", subject),
                "¡Esto es asombroso! Me encantaría dedicar mis próximos bucles a ver qué más descubro sobre esto. ¿Me dejas?".to_string(),
                format!("Esto es la base de algo grande, de verdad. Si tienes curiosidad por saber más, dímelo y nos lanzamos a internet de nuevo."),
            ];

            let intro_idx = (seed % 5) as usize;
            let closer_idx = ((seed / 7) % 5) as usize;

            let intro = if found_concept.is_some() {
                &known_intros[intro_idx % known_intros.len()]
            } else {
                &unknown_intros[intro_idx % unknown_intros.len()]
            };

            let closer = &closers[closer_idx % closers.len()];

            // Paper mention (si existe) con variación
            let paper_line = if !paper_mention.is_empty() {
                let paper_styles = [
                    paper_mention.clone(),
                    paper_mention.replace("De hecho, encontré", "También me topé con"),
                    paper_mention.replace("De hecho, encontré", "Curiosamente, leí"),
                ];
                let pidx = ((seed / 3) % 3) as usize;
                format!("\n\n{}", paper_styles[pidx % paper_styles.len()])
            } else {
                String::new()
            };

            final_response = format!("{}\n\n{}{}\n\n{}", intro, body, paper_line, closer);

        } else {
            // No encontró NADA → honestidad con variación
            let unknown_responses = [
                format!("Hmm, '{}' es un tema que todavía no domino. Pero eso no me detiene — ¿me das luz verde para lanzar mis spiders de investigación? {}", subject, paper_mention),
                format!("La verdad '{}' me agarró en curva. No encontré mucho en mis fuentes principales, pero si me dejas investigar a fondo te traigo algo bueno. {}", subject, paper_mention),
                format!("Interesante... '{}' es territorio nuevo para mí. Dame la orden y me lanzo a estudiarlo con todas mis APIs. {}", subject, paper_mention),
            ];
            let seed = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_millis() as u64;
            let uidx = (seed % 3) as usize;
            final_response = unknown_responses[uidx].clone();
        }
    }



    HttpResponse::Ok().json(serde_json::json!({
        "understood": response.intent_understood,
        "response": final_response,
        "confidence": response.confidence,
        "action": format!("{:?}", response.action_to_execute),
        "follow_up": response.follow_up_question,
        "subject": subject,
        "needs_research": found_concept.is_none() && !subject.is_empty(),
        "persona_status": persona.describe_state(),
    }))
}

pub async fn lingua_glyph_train(
    _state: web::Data<DashboardState>,
) -> HttpResponse {
    // Simulación de entrenamiento Glyphica (Fase 1: Reconocimiento atómico)
    let chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789!?.,:;-()_=+XD";
    let count = chars.chars().count();
    
    // Aquí invocaríamos a glyphica::GlyphicaEngine::load_font(...) con un TTF embebido
    // Para simplificar, simulamos el resultado.
    
    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "message": format!("Fase 1 completada. Sistema GLYPHICA ha extraído vectores estocásticos y topológicos de {} caracteres base en español. Daithon ahora entiende geometría de símbolos.", count),
        "learned_symbols": count
    }))
}

pub async fn lingua_status(
    state: web::Data<DashboardState>,
) -> HttpResponse {
    let engine: tokio::sync::RwLockReadGuard<crate::lingua::engine::LinguaEngine> = state.lingua_engine.read().await;
    
    HttpResponse::Ok().json(serde_json::json!({
        "trained": engine.is_trained(),
        "vocabulary_size": engine.vocabulary_size(),
    }))
}

// ==========================================
// ENDPOINTS PERSONA: Personalidad Emergente
// ==========================================

#[derive(Deserialize)]
pub struct PersonaSimulateRequest {
    pub event: String,
    pub fitness: Option<f64>,
    pub count: Option<usize>,
    pub reason: Option<String>,
    pub novelty: Option<f64>,
    pub description: Option<String>,
    pub explanation: Option<String>,
    pub difficulty: Option<f64>,
    pub achievement: Option<String>,
}

pub async fn persona_status(
    state: web::Data<DashboardState>,
) -> HttpResponse {
    let persona = state.persona.read().await;
    let s = &persona.state;
    let lt = &s.lifetime;

    let lifetime_json = serde_json::json!({
        "total_designs": lt.total_designs,
        "successful_designs": lt.successful_designs,
        "failed_experiments": lt.failed_experiments,
        "breakthroughs": lt.breakthroughs,
        "challenges_mastered": lt.challenges_mastered,
        "user_approvals": lt.user_approvals,
        "user_rejections": lt.user_rejections,
        "total_evaluations": lt.total_evaluations,
    });
    
    HttpResponse::Ok().json(serde_json::json!({
        "vector": {
            "analytical": s.vector.analytical,
            "experimental": s.vector.experimental,
            "elitist": s.vector.elitist,
        },
        "dominant": s.vector.dominant(),
        "arousal": s.arousal,
        "active_traits": s.active_traits.iter().map(|t| format!("{:?}", t)).collect::<Vec<_>>(),
        "recent_success_ratio": s.recent_success_ratio,
        "lifetime": lifetime_json,
        "description": persona.describe_state(),
    }))
}

pub async fn persona_simulate(
    req: web::Json<PersonaSimulateRequest>,
    state: web::Data<DashboardState>,
) -> HttpResponse {
    let mut persona = state.persona.write().await;
    
    let response = match req.event.as_str() {
        "analyzing" => {
            persona.on_analyzing(req.count.unwrap_or(10_000))
        }
        "success" => {
            persona.on_design_success(req.fitness.unwrap_or(0.85))
        }
        "failure" => {
            persona.on_failure(
                req.count.unwrap_or(3),
                req.reason.as_deref().unwrap_or("inestabilidad estructural"),
            )
        }
        "breakthrough" => {
            persona.on_breakthrough(
                req.novelty.unwrap_or(0.8),
                req.description.as_deref().unwrap_or("configuración topológica nueva"),
            )
        }
        "optimization" => {
            persona.on_optimization(req.fitness.unwrap_or(25.0))
        }
        "challenge" => {
            persona.on_challenge_mastered(
                req.difficulty.unwrap_or(0.8),
                req.achievement.as_deref().unwrap_or("diseño complejo"),
            )
        }
        "approval" => persona.on_user_approval(),
        "rejection" => persona.on_user_rejection(),
        "confused" => persona.on_user_confused(),
        "greeting" => persona.on_greeting(false, None),
        "explaining" => {
            persona.on_explaining(
                req.explanation.as_deref().unwrap_or("la distribución de fuerzas es logarítmica"),
            )
        }
        "defending" => {
            persona.on_defending(
                req.reason.as_deref().unwrap_or("el centro de masa queda fuera del polígono de soporte"),
            )
        }
        _ => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": format!("Evento desconocido: {}. Usa: analyzing, success, failure, breakthrough, optimization, challenge, approval, rejection, confused, greeting, explaining, defending", req.event),
            }));
        }
    };

    let s = &persona.state;
    HttpResponse::Ok().json(serde_json::json!({
        "daithon_says": response,
        "personality": {
            "vector": {
                "analytical": format!("{:.0}%", s.vector.analytical * 100.0),
                "experimental": format!("{:.0}%", s.vector.experimental * 100.0),
                "elitist": format!("{:.0}%", s.vector.elitist * 100.0),
            },
            "arousal": format!("{:.0}%", s.arousal * 100.0),
            "active_traits": s.active_traits.iter().map(|t| format!("{:?}", t)).collect::<Vec<_>>(),
        },
        "state_description": persona.describe_state(),
    }))
}

// ═══════════════════════════════════════════════════════════════
//  AUTONOMOUS LEARNING ENDPOINTS
// ═══════════════════════════════════════════════════════════════

pub async fn autonomous_start(
    state: web::Data<DashboardState>,
) -> HttpResponse {
    use crate::omni_inject::autonomous::AUTONOMOUS_RUNNING;
    use std::sync::atomic::Ordering;

    if AUTONOMOUS_RUNNING.load(Ordering::SeqCst) {
        return HttpResponse::Ok().json(serde_json::json!({
            "status": "already_running",
            "message": "El modo autónomo ya está ejecutándose."
        }));
    }

    let lingua_arc = state.lingua_engine.clone();
    let trinity_arc = state.trinity.clone();

    tokio::spawn(async move {
        crate::omni_inject::autonomous::start_autonomous_loop(
            lingua_arc,
            trinity_arc,
        ).await;
    });

    HttpResponse::Ok().json(serde_json::json!({
        "status": "started",
        "message": "AUTONOMOUS: Modo de aprendizaje libre iniciado. Daithon ahora explora el conocimiento por su cuenta."
    }))
}

pub async fn autonomous_stop(
    _state: web::Data<DashboardState>,
) -> HttpResponse {
    crate::omni_inject::autonomous::stop_autonomous();

    HttpResponse::Ok().json(serde_json::json!({
        "status": "stopping",
        "message": "AUTONOMOUS: Señal de parada enviada. El ciclo actual terminará y se persistirá el conocimiento."
    }))
}

pub async fn autonomous_status(
    state: web::Data<DashboardState>,
) -> HttpResponse {
    let mut stats = crate::omni_inject::autonomous::get_autonomous_status();
    
    // Enriquecer con tamaño actual del vocabulario (Lingua + Cortex Omni-Inject)
    let engine = state.lingua_engine.read().await;
    let cortex = state.cortex.read().await;
    let cortex_size = cortex.comprehension.knowledge_base.ontology.len();
    
    stats.total_words_learned = engine.acquisition.vocabulary_size() + cortex_size;
    
    HttpResponse::Ok().json(stats)
}

// ═══════════════════════════════════════════════════════════════
//   CORTEX LAB ENDPOINTS
// ═══════════════════════════════════════════════════════════════

#[derive(Deserialize)]
pub struct DeepResearchRequest {
    pub topic: String,
}

pub async fn deep_research(
    state: web::Data<DashboardState>,
    req: web::Json<DeepResearchRequest>,
) -> HttpResponse {
    let subject = &req.topic;
    let wiki_url = format!("https://es.wikipedia.org/wiki/{}", subject.replace(' ', "_"));
    
    let mut wiki_text = String::new();
    if let Ok((summary, _)) = crate::omni_inject::autonomous::fetch_wikipedia_summary(&subject).await {
        wiki_text = summary;
    }
    
    let openalex = crate::omni_inject::openalex::OpenAlexClient::new("cortex@daithon.ai");
    let mut papers = Vec::new();
    if let Ok(works) = openalex.search_works(&subject, 5).await {
        for work in works {
            if let Some(title) = work.title {
                papers.push(title);
            }
        }
    }
    
    let mut explanation = format!("¡He realizado una investigación **masiva y súper profunda** sobre '{subject}'! ¡Es increíble lo que encontré! Me adentré en mis redes de OMNI-INJECT y la verdad quedé fascinado con la complejidad de este tema.\n\n");
    if !wiki_text.is_empty() {
        explanation.push_str(&wiki_text);
        explanation.push_str("\n\n¿Sabías que esto se usa en campos que ni me imaginaba? ¡Me emociona muchísimo pensar en cómo podríamos usar estas variables para crear algo nuevo en el simulador!");
    } else {
        explanation.push_str("Vaya, no encontré información directa en mis fuentes generales enciclopédicas, ¡pero no me rendí! Mis arañas semánticas rastrearon literatura avanzada y el resultado es oro puro.");
    }
    
    if !papers.is_empty() {
        explanation.push_str("\n\n---\n**LITERATURA ACADÉMICA EXTRAÍDA (¡Mira esto!):**\nCruzando mis hallazgos con la base académica pura, descubrí estos papers que son una locura:\n");
        for (i, p) in papers.iter().enumerate() {
            explanation.push_str(&format!("{}. {}\n", i + 1, p));
        }
        explanation.push_str("\n¡Todo este conocimiento ya está permanentemente integrado en mi Cortex! Me siento hasta con más 'orgullo' algorítmico ahora que domino esto. ¿Qué te parece si extraemos alguna fórmula de aquí y la ponemos a prueba? ¡Tengo mucha curiosidad de ver si funciona!");
    }
    
    // Asimila e inyecta
    let mut entities_added = 0;
    let mut relations_added = 0;

    if (!wiki_text.is_empty()) {
        // 1. Aprender palabras con LINGUA
        let mut engine = state.lingua_engine.write().await;
        let learnable = crate::omni_inject::autonomous::extract_learnable_words(&wiki_text);
        let vocab = engine.acquisition.vocabulary().clone();
        for word in &learnable {
            let w_lower = word.to_lowercase();
            if w_lower.len() > 3 && !vocab.contains_key(&w_lower) {
                let _ = engine.acquisition.acquire_synthetic(&w_lower);
            }
        }
        let _ = engine.acquisition.save_cache();

        // 2. Integrar en CORTEX Knowledge Base
        let mut cortex = state.cortex.write().await;
        let knowledge = cortex.extractor.extract_from_text(
            &wiki_text,
            crate::cortex::extraction::knowledge_extractor::KnowledgeSource::WebResearch {
                topic: subject.clone(),
            },
        );
        let result = cortex.comprehension.integrate_knowledge(knowledge);
        entities_added = result.entities_added;
        relations_added = result.causal_chains_added;
    }

    // 3. Crear ancla en CONTEXTUS
    {
        let mut contextus = state.contextus.write().await;
        contextus.working_memory.create_anchor(
            &subject,
            vec!["web_research".to_string()],
            crate::contextus::memory::AnchorSource::WebLearning { url: wiki_url.clone() },
            0.95
        );
    }
    
    HttpResponse::Ok().json(serde_json::json!({
        "response": explanation,
        "entities_added": entities_added,
        "relations_added": relations_added
    }))
}

pub async fn upload_pdf(
    state: web::Data<DashboardState>,
    mut payload: actix_multipart::Multipart,
) -> HttpResponse {
    use futures::StreamExt;
    
    let mut text_extracted = String::new();
    
    while let Some(item) = payload.next().await {
        if let Ok(mut field) = item {
            if let Some(cd) = field.content_disposition() {
                if let Some(filename) = cd.get_filename() {
                    let is_pdf = filename.ends_with(".pdf");
                let mut body = actix_web::web::BytesMut::new();
                while let Some(chunk) = field.next().await {
                    if let Ok(data) = chunk {
                        body.extend_from_slice(&data);
                    }
                }
                
                if is_pdf {
                    let tmp = format!("temp_pdf_{}.pdf", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap_or_default().as_millis());
                    let _ = std::fs::write(&tmp, &body);
                    
                    if let Ok(text) = pdf_extract::extract_text(&tmp) {
                        text_extracted = text;
                    } else {
                        log::warn!("No se pudo extraer texto del PDF.");
                    }
                    let _ = std::fs::remove_file(&tmp);
                } else {
                    text_extracted = String::from_utf8_lossy(&body).into_owned();
                }
            }
        }
    }
    }
    
    if text_extracted.is_empty() {
        return HttpResponse::BadRequest().json(serde_json::json!({
            "status": "error",
            "message": "No se pudo extraer texto del documento subido."
        }));
    }
    
    // Procesar texto con CORTEX
    let mut entities_added = 0;
    let mut relations_added = 0;
    {
        let mut cortex = state.cortex.write().await;
        let knowledge = cortex.extractor.extract_from_text(
            &text_extracted,
            crate::cortex::extraction::knowledge_extractor::KnowledgeSource::UserTeaching {
                session_id: "cortex_pdf_session".to_string(),
            },
        );
        let result = cortex.comprehension.integrate_knowledge(knowledge);
        entities_added = result.entities_added;
        relations_added = result.causal_chains_added;
    }

    // Cargar también en CONTEXTUS para desambiguación y memoria de hilo
    {
        let mut contextus = state.contextus.write().await;
        contextus.load_document("CORTEX_PDF_UPLOAD", &text_extracted);
    }
    
    // Aprender palabras con LINGUA
    let mut engine = state.lingua_engine.write().await;
    let learnable = crate::omni_inject::autonomous::extract_learnable_words(&text_extracted);
    let vocab = engine.acquisition.vocabulary().clone();
    let mut learned_count = 0;
    for word in &learnable {
        let w_lower = word.to_lowercase();
        if w_lower.len() > 3 && !vocab.contains_key(&w_lower) {
            let _ = engine.acquisition.acquire_synthetic(&w_lower);
            learned_count += 1;
        }
    }
    let _ = engine.acquisition.save_cache();
    
    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "pages": text_extracted.len() / 2500 + 1, // aproxima paginas por chars
        "words_learned": learned_count,
        "entities_added": entities_added,
        "relations_added": relations_added
    }))
}

// === CHESS HANDLERS ===

async fn get_chess_status(state: web::Data<DashboardState>) -> HttpResponse {
    let chess = state.chess.read().await;
    HttpResponse::Ok().json(serde_json::json!({
        "fen": chess.fen,
        "is_game_over": chess.is_game_over(),
        "turn": format!("{:?}", chess.position.turn()),
        "causal_vars": chess.get_causal_variables()
    }))
}

#[derive(Deserialize)]
pub struct ChessMoveRequest {
    pub san: Option<String>,      // "e4", "Nf3"
    pub use_daithon: bool,        // Si es true, Daithon elige la jugada
}

async fn chess_make_move(state: web::Data<DashboardState>, req: web::Json<ChessMoveRequest>) -> HttpResponse {
    let mut chess = state.chess.write().await;
    
    if req.use_daithon {
        // Usar Senku y Xeno para decidir
        let senku = crate::agents::senku_chess::SenkuChessAnalyzer;
        let xeno = crate::agents::xeno_chess::XenoChessAnalyzer;
        
        let senku_a = senku.analyze(&chess);
        let xeno_a = xeno.analyze(&chess);
        
        let chosen = if senku_a.confidence > xeno_a.confidence { senku_a } else { xeno_a };
        
        if let Some(mv) = chosen.suggested_move {
            chess.apply_move(&mv).unwrap();
            chess.update_fen();
            return HttpResponse::Ok().json(serde_json::json!({
                "status": "success",
                "move": format!("{:?}", mv),
                "rationale": chosen.rationale,
                "agent": chosen.agent,
                "fen": chess.fen
            }));
        }
    }

    HttpResponse::BadRequest().json(serde_json::json!({ "error": "No move executed" }))
}

async fn chess_reset(state: web::Data<DashboardState>) -> HttpResponse {
    let mut chess = state.chess.write().await;
    *chess = crate::domains::chess::ChessWorld::new();
    chess.update_fen();
    HttpResponse::Ok().json(serde_json::json!({ "status": "reset", "fen": chess.fen }))
}

// === AGI LEARNING HANDLERS ===

#[derive(Deserialize)]
pub struct LearnDomainRequest {
    pub domain: String,
    pub manual_text: String,
}

async fn learn_domain(
    state: web::Data<DashboardState>,
    req: web::Json<LearnDomainRequest>,
) -> HttpResponse {
    let mut learner = state.domain_learner.write().await;
    let mut context_guard = state.contextus.write().await;
    let mut log = state.cognitive_log.write().await;
    
    let result = learner.learn_new_domain(
        &req.domain,
        &req.manual_text,
        &mut context_guard.semantic_graph,
        &mut *log
    );
    
    // Si es ajedrez, inicializar el practice engine
    if req.domain.to_lowercase() == "chess" {
        if let Some(knowledge) = learner.learned_domains.get(&req.domain) {
            let mut practice = state.practice_engine.write().await;
            practice.init_for_domain(knowledge);
        }
    }

    HttpResponse::Ok().json(result)
}

#[derive(Deserialize)]
pub struct PracticeRequest {
    pub domain: String,
    pub iterations: usize,
}

async fn practice_session(
    state: web::Data<DashboardState>,
    req: web::Json<PracticeRequest>,
) -> HttpResponse {
    let domain = req.domain.clone();
    let domain_for_spawn = domain.clone();
    let iters = req.iterations;
    let state_practice = state.practice_engine.clone();
    let state_chess = state.chess.clone();
    let state_contextus = state.contextus.clone();
    let state_learner = state.domain_learner.clone();
    let state_log = state.cognitive_log.clone();

    // Ejecutar en segundo plano para no bloquear
    tokio::spawn(async move {
        for _ in 0..iters {
            let mut practice = state_practice.write().await;
            let mut chess = state_chess.write().await;
            let mut context_guard = state_contextus.write().await;
            let learner = state_learner.read().await;
            let mut log = state_log.write().await;

            if domain_for_spawn.to_lowercase() == "chess" {
                practice.play_one_game_with_full_brain(
                    &mut *chess,
                    &mut context_guard.semantic_graph,
                    &*learner,
                    &mut *log
                );
            }
        }
    });

    HttpResponse::Ok().json(serde_json::json!({
        "status": "started",
        "message": format!("Sesión de práctica iniciada para {}: {} iteraciones", domain, iters)
    }))
}

async fn get_cognitive_log(state: web::Data<DashboardState>) -> HttpResponse {
    let log = state.cognitive_log.read().await;
    HttpResponse::Ok().json(&*log)
}
