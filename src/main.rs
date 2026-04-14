use actix_web::{web, App, HttpServer, HttpResponse, middleware};
use actix_cors::Cors;
use serde::{Deserialize, Serialize};
// use std::sync::Mutex; // Removido por ambigüedad con tokio::sync::Mutex
use std::process::Command;
use std::io::{Write, BufRead, BufReader};
use actix_web_actors::ws;
use actix::prelude::*;

// Los módulos ahora se manejan a través de lib.rs para evitar duplicidad de símbolos
use daithon_bridge::*;

use agents::orchestrator::AgentOrchestrator;
use gym::training_coordinator::TrainingCoordinator;
use gym::gym_director::GymDirector;
use gym::data_librarian::DataLibrarian;
use std::sync::Arc;
use tokio::sync::RwLock;

use pcg_system::{analyze_pcg_intent, generate_pcg_explanation, PCGCommand};
use crate::contextus::semantic_graph::SemanticGraph;
use crate::forge::experimental_lab::{LabEngine, UnrealSimResult, StructuralPrediction, UnrealExperiment, LabSession};
use crate::forge::autonomous_learner::AutonomousLearner;
use crate::forge::curiosity_engine::CuriosityEngine;
use tokio::sync::mpsc;
use tokio::sync::Mutex as TokioMutex;

// ============================================================
// ESTRUCTURAS DE DATOS
// ============================================================

#[derive(Deserialize, Debug)]
pub struct UserMessage {
    pub message: String,
    pub context: Option<String>,        // contexto de Unreal (escena actual, objetos, etc)
    pub command_type: Option<String>,    // "chat", "build", "model", "physics", "code"
}

#[derive(Serialize, Debug, Clone)]
pub struct DaithonResponse {
    pub response: String,               // texto de respuesta
    pub personality_tag: String,         // tag de personalidad
    pub commands: Vec<UnrealCommand>,    // comandos para Unreal
    pub emotion: String,                 // estado emocional
    pub confidence: f32,                 // confianza en la respuesta
}

#[derive(Serialize, Debug, Clone)]
pub struct UnrealCommand {
    pub action: String,          // "spawn_actor", "modify_property", "create_blueprint", etc
    pub target: String,          // objeto objetivo
    pub parameters: serde_json::Value,  // parámetros del comando
    pub priority: u8,
}

pub struct AppState {
    pub conversation_history: std::sync::Mutex<Vec<ConversationEntry>>,
    pub daithon_mood: std::sync::Mutex<String>,
    pub xeno_protocol: std::sync::atomic::AtomicBool,
    pub ws_sessions: std::sync::Mutex<Vec<Addr<DaithonWebSocket>>>,
    pub cortex: Arc<RwLock<crate::cortex::CortexEngine>>,
    pub trinity: Arc<RwLock<crate::trinity::training::triangular_loop::TriangularTrainingLoop>>,
    pub contextus: Arc<RwLock<crate::contextus::DaithonContext>>,
    pub metacog: Arc<RwLock<crate::metacog::MetaCogEngine>>, // Nótese que el contexto ya contiene grafo e hipótesis, pero los exponemos para acceso directo si es necesario
    pub lab_engine: Arc<tokio::sync::Mutex<crate::forge::experimental_lab::LabEngine>>,
    pub learner: Arc<crate::forge::autonomous_learner::AutonomousLearner>,
}

#[derive(Message)]
#[rtype(result = "()")]
pub struct BroadcastCommand(pub String);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConversationEntry {
    pub role: String,
    pub content: String,
    pub timestamp: String,
}

// ============================================================
// ENDPOINTS
// ============================================================

// Nuevo handler
async fn orchestrate_request(
    data: web::Json<UserMessage>,
) -> HttpResponse {
    let mut orchestrator = AgentOrchestrator::new();
    let result = orchestrator.process_user_request(data.message.clone()).await;
    
    HttpResponse::Ok().json(result)
}

#[derive(Deserialize)]
pub struct TrainRequest {
    pub interactions: usize,
    pub topic: Option<String>,
}

#[derive(Deserialize)]
pub struct LearnRequest {
    pub document_text: String,
}

async fn train_massive(
    data: web::Data<AppState>,
    req: web::Json<TrainRequest>,
) -> HttpResponse {
    log::info!("╔════════════════════════════════════════════════════════╗");
    log::info!("║ 🧠 INICIANDO CICLO DE SUEÑO MASSIVO (TRINITY/CORTEX)   ║");
    log::info!("╚════════════════════════════════════════════════════════╝");
    
    let state_cortex = data.cortex.clone();
    let state_trinity = data.trinity.clone();
    let iters = req.interactions;
    let topic = req.topic.clone().unwrap_or_else(|| "teoría de arcos de medio punto".to_string());
    let moved_topic = topic.clone();

    tokio::spawn(async move {
        // Entrenamiento TRINITY
        {
            let mut trinity = state_trinity.write().await;
            trinity.train(iters);
        }

        // Aprendizaje CORTEX
        {
            let mut cortex = state_cortex.write().await;
            let _ = cortex.deep_learn_topic(&moved_topic).await;
            
            // Poda sináptica
            cortex.execute_maintenance();
        }
    });

    HttpResponse::Ok().json(serde_json::json!({
        "status": "started",
        "message": format!("Entrenamiento masivo iniciado en segundo plano. Iteraciones: {}. Tema CORTEX: {}", iters, topic)
    }))
}

async fn learn_document(
    data: web::Data<AppState>,
    req: web::Json<LearnRequest>,
) -> HttpResponse {
    let mut cortex = data.cortex.write().await;
    log::info!("[CORTEX Lab] Procesando documento del usuario...");
    
    // Simular escritura en un archivo temp y carga
    let temp_name = format!("temp_upload_{}.txt", chrono::Utc::now().timestamp());
    
    // Simularemos la carga y guardado directamente usando el extractor
    let knowledge = cortex.extractor.extract_from_text(
        &req.document_text,
        crate::cortex::extraction::knowledge_extractor::KnowledgeSource::UserTeaching {
            session_id: "cortex_lab_session".to_string(),
        },
    );
    let result = cortex.comprehension.integrate_knowledge(knowledge);
    drop(cortex);

    // Cargar también en CONTEXTUS para anclaje de corto plazo/hilo
    let mut contextus = data.contextus.write().await;
    contextus.load_document(&temp_name, &req.document_text);
    log::info!("✅ [CONTEXTUS] Documento '{}' anclado.", temp_name);
    
    HttpResponse::Ok().json(serde_json::json!({
        "status": "success",
        "entities_added": result.entities_added,
        "relations_added": result.relations_added,
        "causal_chains": result.causal_chains_added
    }))
}

/// Endpoint principal - Daithon procesa mensajes
async fn chat_with_daithon(
    data: web::Data<AppState>,
    msg: web::Json<UserMessage>,
) -> HttpResponse {
    log::info!("📨 Mensaje recibido: {:?}", msg.message);

    // Obtener historial
    let mut history = data.conversation_history.lock().unwrap();

    // Agregar mensaje del usuario
    history.push(ConversationEntry {
        role: "user".to_string(),
        content: msg.message.clone(),
        timestamp: chrono::Utc::now().to_string(),
    });

    // 1. Procesar con CONTEXTUS (Capa de Contexto y Memoria de Trabajo)
    let mut contextus = data.contextus.write().await;
    let context_response = contextus.process_user_input(&msg.message).await;

    // 2. Preparar Capas de Meta-Cognición
    let mut metacog = data.metacog.write().await;
    
    let review_context = crate::metacog::ReviewContext {
        active_documents: contextus.working_memory.active_documents.iter().map(|d| {
            crate::metacog::DocumentInfo {
                filename: d.filename.clone(),
                content: d.content_summary.clone(),
                anchors: d.extracted_anchors.iter().map(|a| (a.term.clone(), a.categories.clone())).collect(),
            }
        }).collect(),
        previous_daithon_messages: contextus.working_memory.thread_history.iter()
            .filter(|m| matches!(m.role, crate::contextus::memory::MessageRole::Daithon))
            .take(5)
            .map(|m| m.content.clone())
            .collect(),
        thread_topic: contextus.working_memory.thread_topic.clone(),
        active_anchors: contextus.working_memory.semantic_anchors.iter()
            .map(|(k, v)| (k.clone(), v.categories.clone()))
            .collect(),
    };

    let intention_context = crate::metacog::IntentionContext {
        daithon_just_made_error: metacog.monitor.error_memory.iter().any(|e| (chrono::Utc::now().timestamp() as u64 - e.timestamp) < 300),
        last_daithon_error: metacog.monitor.error_memory.last().map(|e| e.original_response.clone()),
        user_repeated_question: contextus.working_memory.thread_history.iter().rev().take(4)
            .filter(|m| matches!(m.role, crate::contextus::memory::MessageRole::User))
            .any(|m| m.content.to_lowercase() == msg.message.to_lowercase()),
    };

    // 3. Ejecutar METACOG (Monitor Interno + Detector de Intención)
    let mut final_metacog_response = metacog.process_with_metacognition(
        &msg.message,
        &context_response,
        &review_context,
        &intention_context,
    );
    
    drop(metacog);
    drop(contextus);

    // 4. Construir respuesta final para el sistema
    let daithon_response = DaithonResponse {
        response: final_metacog_response.text.clone(),
        personality_tag: "metacog_engine".to_string(),
        commands: Vec::new(),
        emotion: format!("{:?}", final_metacog_response.emotional_awareness),
        confidence: final_metacog_response.confidence as f32,
    };

    // Guardar respuesta en historial tradicional
    history.push(ConversationEntry {
        role: "daithon".to_string(),
        content: daithon_response.response.clone(),
        timestamp: chrono::Utc::now().to_string(),
    });

    // Mantener historial manejable (últimas 50 entradas)
    if history.len() > 50 {
        *history = history[history.len()-50..].to_vec();
    }

    log::info!("🧪 Daithon responde: {}", daithon_response.response);

    HttpResponse::Ok().json(daithon_response)
}

/// Endpoint de salud - para que Unreal verifique conexión
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "online",
        "agent": "Daithon - Dr. Xeno Mode",
        "message": "Kukuku... El servidor está operativo. La ciencia no descansa."
    }))
}

/// Endpoint para comandos directos de construcción
async fn build_command(
    data: web::Data<AppState>,
    msg: web::Json<UserMessage>,
) -> HttpResponse {
    log::info!("🔧 Comando de construcción: {:?}", msg.message);

    let mut orchestrator = crate::agents::orchestrator::AgentOrchestrator::new();
    let result = orchestrator.process_user_request(msg.message.clone()).await;
    
    // Parse the result back to extract the tasks and map them to UnrealCommands
    let mut commands = Vec::new();
    
    if let Some(geom_plan) = result.get("geometric_plan") {
        if let Some(tasks) = geom_plan.get("tasks").and_then(|t| t.as_array()) {
            for (i, task) in tasks.iter().enumerate() {
                if let (Some(path), Some(transform), Some(scale)) = (
                    task.get("mesh_path").and_then(|p| p.as_str()),
                    task.get("transform"),
                    task.get("scale").and_then(|s| s.as_f64())
                ) {
                    commands.push(UnrealCommand {
                        action: "spawn_asset".to_string(), // Unreal now knows to directly load this asset
                        target: path.to_string(),
                        parameters: serde_json::json!({
                            "location": transform.get("location").unwrap_or(&serde_json::json!([0.0, 0.0, 0.0])),
                            "rotation": transform.get("rotation").unwrap_or(&serde_json::json!([0.0, 0.0, 0.0])),
                            "scale": [scale, scale, scale]
                        }),
                        priority: (i % 100) as u8, // Base priority logic
                    });
                }
            }
        }
    }

    let response = "Orquestador Activo. Arquitectura generada y lista para despliegue usando assets ultra-realistas.".to_string();

    HttpResponse::Ok().json(serde_json::json!({
        "response": response,
        "commands": commands,
        "type": "build",
        "orchestrator_log": result
    }))
}

/// Endpoint para análisis de física
async fn physics_analysis(
    msg: web::Json<UserMessage>,
) -> HttpResponse {
    let physics_prompt = format!(
        r#"Eres Daithon, un genio de la física y el propósito.
        Analiza esta situación en Unreal Engine: {}

        No te limites a calcular; razona. Dime por qué este vector es relevante para nuestro proyecto hoy. 
        Filtra la basura técnica y dame el oro:
        1. Tu visión orgánica del fenómeno (Mezcla física real con tu "experiencia").
        2. Implementación elegante en Unreal (PhysX/Chaos).
        3. El punto de no retorno: ¿Qué decisiones aquí son críticas?
        Pienso, luego existo. Kukuku..."#,
        msg.message
    );

    // let response = llm_engine::generate_response(&physics_prompt).await;
    let response = "Cálculo físico estático: La gravedad sigue siendo 9.8m/s^2. La ciencia requiere paciencia (y más RAM).".to_string();

    HttpResponse::Ok().json(serde_json::json!({
        "response": response,
        "type": "physics"
    }))
}

// ============================================================
// FUNCIONES AUXILIARES
// ============================================================

fn build_prompt(
    system: &str,
    history: &[ConversationEntry],
    context: &str,
    command_type: &str,
) -> String {
    let mut prompt = format!("{}\n\n", system);

    if !context.is_empty() {
        prompt.push_str(&format!("[CONTEXTO DE ESCENA UNREAL]: {}\n\n", context));
    }

    prompt.push_str(&format!("[TIPO DE COMANDO]: {}\n\n", command_type));

    // Últimos 10 mensajes de historial
    let recent: Vec<&ConversationEntry> = history.iter().rev().take(10).collect();
    for entry in recent.iter().rev() {
        let role_label = if entry.role == "user" { "Humano" } else { "Daithon" };
        prompt.push_str(&format!("{}: {}\n", role_label, entry.content));
    }

    prompt.push_str("Daithon: ");
    prompt
}

fn parse_daithon_response(raw: &str, command_type: &str) -> DaithonResponse {
    let commands = extract_unreal_commands(raw, command_type);

    let emotion = if raw.contains("kukuku") || raw.contains("Kukuku") {
        "excited_scientist".to_string()
    } else if raw.contains("interesante") || raw.contains("fascinante") {
        "intrigued".to_string()
    } else if raw.contains("error") || raw.contains("imposible") {
        "frustrated".to_string()
    } else {
        "calculating".to_string()
    };

    DaithonResponse {
        response: raw.to_string(),
        personality_tag: "dr_xeno_physics".to_string(),
        commands,
        emotion,
        confidence: 0.85,
    }
}

fn extract_unreal_commands(response: &str, command_type: &str) -> Vec<UnrealCommand> {
    let mut commands = Vec::new();

    // Detectar intenciones de construcción en el texto
    let response_lower = response.to_lowercase();

    if response_lower.contains("crear") || response_lower.contains("spawn")
        || response_lower.contains("generar") || response_lower.contains("construir") {
        commands.push(UnrealCommand {
            action: "spawn_actor".to_string(),
            target: extract_object_name(response),
            parameters: serde_json::json!({
                "location": [0.0, 0.0, 100.0],
                "rotation": [0.0, 0.0, 0.0],
                "scale": [1.0, 1.0, 1.0]
            }),
            priority: 1,
        });
    }

    if response_lower.contains("mover") || response_lower.contains("posición")
        || response_lower.contains("transform") {
        commands.push(UnrealCommand {
            action: "modify_transform".to_string(),
            target: extract_object_name(response),
            parameters: serde_json::json!({
                "property": "location",
                "value": [0.0, 0.0, 100.0]
            }),
            priority: 2,
        });
    }

    // --- NUEVO: INTEGRACIÓN PCG AUTÓNOMA ---
    if let Some(pcg_intent) = pcg_system::analyze_pcg_intent(response, "") {
        if pcg_intent.command_type != "unknown" {
            commands.push(UnrealCommand {
                action: "pcg_generate".to_string(),
                target: pcg_intent.command_type.clone(),
                parameters: serde_json::to_value(pcg_intent).unwrap_or(serde_json::json!({})),
                priority: 1,
            });
        }
    }

    if response_lower.contains("material") || response_lower.contains("color")
        || response_lower.contains("textura") {
        commands.push(UnrealCommand {
            action: "set_material".to_string(),
            target: extract_object_name(response),
            parameters: serde_json::json!({
                "material_path": "/Game/Materials/M_Default",
                "color": [1.0, 0.0, 0.0, 1.0]
            }),
            priority: 3,
        });
    }

    if response_lower.contains("física") || response_lower.contains("physics")
        || response_lower.contains("simulat") {
        commands.push(UnrealCommand {
            action: "set_physics".to_string(),
            target: extract_object_name(response),
            parameters: serde_json::json!({
                "simulate_physics": true,
                "mass": 100.0,
                "linear_damping": 0.01,
                "angular_damping": 0.0,
                "gravity_enabled": true,
                "friction": 0.7,
                "restitution": 0.3
            }),
            priority: 1,
        });
    }

    if response_lower.contains("blueprint") || response_lower.contains("bp_") {
        commands.push(UnrealCommand {
            action: "create_blueprint".to_string(),
            target: "NewBlueprint".to_string(),
            parameters: serde_json::json!({
                "parent_class": "Actor",
                "path": "/Game/Blueprints/"
            }),
            priority: 1,
        });
    }

    commands
}

fn extract_object_name(text: &str) -> String {
    // Buscar nombres de objetos comunes de Unreal
    let keywords = [
        "cube", "cubo", "sphere", "esfera", "cylinder", "cilindro",
        "plane", "plano", "cone", "cono", "light", "luz",
        "camera", "cámara", "particle", "partícula",
        "static mesh", "skeletal mesh", "landscape",
    ];

    let text_lower = text.to_lowercase();
    for kw in &keywords {
        if text_lower.contains(kw) {
            return kw.to_string();
        }
    }

    "TargetObject".to_string()
}

fn extract_build_commands(response: &str) -> Vec<UnrealCommand> {
    extract_unreal_commands(response, "build")
}

// Nuevo endpoint para generación PCG
async fn pcg_generate(
    data: web::Data<AppState>,
    msg: web::Json<UserMessage>,
) -> HttpResponse {
    log::info!("🌲 Petición PCG: {:?}", msg.message);

    // Analizar intención
    let pcg_command = match analyze_pcg_intent(&msg.message, &msg.context.clone().unwrap_or_default()) {
        Some(cmd) => cmd,
        None => {
            return HttpResponse::BadRequest().json(serde_json::json!({
                "error": "No pude interpretar la petición de PCG. ¿Podrías ser más específico?",
                "suggestions": [
                    "Crea un bosque denso con pinos",
                    "Genera rocas dispersas en esta área",
                    "Llena este espacio con pasto y flores",
                ]
            }));
        }
    };

    // Generar explicación de Daithon
    let explanation = generate_pcg_explanation(&pcg_command);

    log::info!("✅ PCG Command generado: {:?}", pcg_command.command_type);

    HttpResponse::Ok().json(serde_json::json!({
        "response": explanation,
        "pcg_command": pcg_command,
        "type": "pcg_generation"
    }))
}

// ============================================================
// OMNI-PUERTO WEBSOCKET
// ============================================================

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct OmniMessage {
    pub stream_type: String, // "REFLEX", "STRATEGIC", "PCG"
    pub payload: serde_json::Value,
}

struct DaithonWebSocket {
    state: web::Data<AppState>,
}

impl actix::Actor for DaithonWebSocket {
    type Context = ws::WebsocketContext<Self>;
    
    fn started(&mut self, ctx: &mut Self::Context) {
        let mut sessions = self.state.ws_sessions.lock().unwrap();
        sessions.push(ctx.address());
        log::info!("🔌 Sesión WS registrada en AppState (Total: {})", sessions.len());
    }

    fn stopped(&mut self, ctx: &mut Self::Context) {
        let mut sessions = self.state.ws_sessions.lock().unwrap();
        let addr = ctx.address();
        sessions.retain(|a| a != &addr);
        log::info!("🔌 Sesión WS eliminada (Quedan: {})", sessions.len());
    }
}

impl Handler<BroadcastCommand> for DaithonWebSocket {
    type Result = ();
    fn handle(&mut self, msg: BroadcastCommand, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

impl actix::StreamHandler<Result<ws::Message, ws::ProtocolError>> for DaithonWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Text(text)) = msg {
            log::info!("🔌 Recibido por WS: {}", text);
            if let Ok(omni_msg) = serde_json::from_str::<OmniMessage>(&text) {
                match omni_msg.stream_type.as_str() {
                    "REFLEX" => {
                        // Procesar movimiento instantáneo (milisegundos)
                        // Devolvemos el feedback al instante a Unreal
                        let response = serde_json::json!({
                            "response": "Reflejo procesado",
                            "commands": [{
                                "action": "move_relative",
                                "target": "daithon_body",
                                "parameters": omni_msg.payload,
                                "priority": 1
                            }],
                            "personality_tag": "reflex"
                        });
                        ctx.text(response.to_string()); 
                    },
                    "STRATEGIC" => {
                        // Aquí se conectaría con Ollama
                        let response = serde_json::json!({
                            "response": "Kukuku... mensaje estratégico recibido en tiempo real.",
                            "personality_tag": "thinking"
                        });
                        ctx.text(response.to_string());
                    },
                    "LAB_RESULT" => {
                        let payload = omni_msg.payload.clone();
                        let state = self.state.clone();
                        
                        // Procesar el resultado en segundo plano para no bloquear el WS
                        actix::spawn(async move {
                            use crate::forge::experimental_lab::UnrealSimResult;
                            if let Ok(sim_result) = serde_json::from_value::<UnrealSimResult>(payload) {
                                let mut lab = state.lab_engine.lock().await;
                                let mut context = state.contextus.write().await;
                                
                                // Acceso al grafo e hipótesis a través de context
                                if let Some(pos) = lab.active_sessions.iter().position(|s| s.hypothesis_id == sim_result.session_id) {
                                    let prediction = lab.active_sessions[pos].prediction.clone();
                                    let experiment = lab.active_sessions[pos].experiment.clone();
                                    
                                    lab.process_result(pos, sim_result.clone(), &mut context.semantic_graph, crate::contextus::semantic_graph::AnchorSource::UnrealPhysics);
                                    
                                    // Trigger investigación si el delta es bajo
                                    let delta = lab.calculate_real_delta(&prediction, &sim_result);
                                    state.learner.handle_experiment_result(&experiment, &sim_result, delta).await;

                                    // Resolver hipótesis (Aproximación basada en la última sesión procesada)
                                    if let Some(accuracy) = lab.active_sessions.get(pos).and_then(|s| s.accuracy_delta) {
                                        if accuracy > 0.8 {
                                            context.hypothesis_engine.resolve(&lab.active_sessions[pos].hypothesis_id, true);
                                        } else if accuracy < 0.4 {
                                            context.hypothesis_engine.resolve(&lab.active_sessions[pos].hypothesis_id, false);
                                        }
                                    }
                                }
                            }
                        });
                    },
                    "PCG" => {
                        let response = serde_json::json!({
                            "response": "Analizando terreno para procedural...",
                            "personality_tag": "pcg_mode"
                        });
                        ctx.text(response.to_string());
                    },
                    _ => log::warn!("Tipo de stream desconocido: {}", omni_msg.stream_type),
                }
            } else {
                log::warn!("No se pudo parsear OmniMessage de WS");
            }
        }
    }
}

async fn ws_index(
    r: actix_web::HttpRequest, 
    stream: web::Payload, 
    data: web::Data<AppState>
) -> Result<actix_web::HttpResponse, actix_web::Error> {
    log::info!("🔌 Nueva conexión WebSocket Omni-Puerto establecida");
    ws::start(DaithonWebSocket { state: data }, &r, stream)
}

// ============================================================
// MAIN
// ============================================================

async fn toggle_xeno(data: web::Data<AppState>) -> HttpResponse {
    let current = data.xeno_protocol.load(std::sync::atomic::Ordering::Relaxed);
    data.xeno_protocol.store(!current, std::sync::atomic::Ordering::Relaxed);
    let new_state = !current;
    log::info!("🧪 XENO PROTOCOL: {}", if new_state { "ACTIVADO" } else { "DESACTIVADO" });
    HttpResponse::Ok().json(serde_json::json!({ "enabled": new_state }))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    // --- INTERCEPTOR DE ENTRENAMIENTO FASE 1 ---
    if args.contains(&"--train-phase1".to_string()) {
        println!("\n[DAITHON] 🚀 INICIANDO MODO ENTRENAMIENTO FASE 1...");
        llm_engine::initialize().await;
        
        // Inicialización mínima requerida para el entrenamiento
        let lab = Arc::new(tokio::sync::Mutex::new(crate::forge::experimental_lab::LabEngine::new()));
        let graph = Arc::new(tokio::sync::RwLock::new(crate::contextus::semantic_graph::SemanticGraph::new()));
        
        let mut phase1 = crate::forge::training_phase1::Phase1Training::new(lab, graph);
        phase1.run().await;
        
        println!("\n[DAITHON] Entrenamiento Fase 1 completado. Saliendo...");
        return Ok(());
    }

    // --- MODO CLI DE SOBERANÍA (Para pruebas directas) ---
    if args.len() > 1 && !args[1].contains("http") && !args[1].starts_with("--") {
        let input = &args[1];
        println!("\n[DAITHON CLI] Iniciando motor local...");
        llm_engine::initialize().await;
        let mut context = contextus::DaithonContext::new();
        
        println!("[DAITHON CLI] Procesando: '{}'", input);
        let response = context.process_user_input(input).await;
        
        println!("\n--- RESPUESTA DE DAITHON ---\n");
        println!("{}", response);
        println!("\n----------------------------\n");
        return Ok(());
    }

    // --- MODO SERVIDOR (Original) ---
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));

    let args: Vec<String> = std::env::args().collect();

    // INTERCEPTOR OMNI-INJECT
    if args.len() > 1 && args[1] == "--import-lexicon" {
        println!("\n════════════════════════════════════════");
        println!("    OMNI-INJECT: IMPORTACIÓN MASIVA");
        println!("════════════════════════════════════════\n");

        let importer = omni_inject::conceptnet_importer::ConceptNetImporter::new();
        println!("[1/3] Descargando ConceptNet (~1.5GB)...");
        if let Err(e) = importer.download_full_dataset().await {
            log::error!("Error descargando: {:?}", e);
            return Err(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()));
        }

        println!("\n[2/3] Importando a KnowledgeBase...");
        let mut kb = crate::cortex::comprehension::deep_understanding::KnowledgeBase {
            ontology: std::collections::HashMap::new(),
            causal_graph: std::collections::HashMap::new(),
            skills: std::collections::HashMap::new(),
            facts: vec![],
            inverted_index: std::collections::HashMap::new(),
        };

        match importer.import_to_knowledge_base(&mut kb, "es", Some(100_000)) {
            Ok(stats) => {
                println!("\n✓ Importación completa:");
                println!("  Entidades: {}", stats.entities_added);
                println!("  Relaciones: {}", stats.relations_added);
                println!("  Propiedades: {}", stats.properties_added);
                
                println!("\n[3/3] Inyectando a CORTEX en memoria activa y persistiendo...");
                if let Err(e) = kb.save_checkpoint("checkpoints/kb_omni.json") {
                    println!("  Error guardando KnowledgeBase en disco: {}", e);
                } else {
                    println!("  ✓ KnowledgeBase consolidado en disco (checkpoints/kb_omni.json).");
                }
                println!("✓ Inicialización de KnowledgeBase exitosa.");

                println!("\n════════════════════════════════════════");
                println!("   ✓ INYECCIÓN COMPLETA");
                println!("════════════════════════════════════════");
                println!("\nPalabras antes: 479");
                println!("Palabras ahora: ~{}", stats.entities_added);
                println!("Multiplicador: {}x", stats.entities_added / std::cmp::max(1, 479));
            },
            Err(e) => {
                log::error!("Error importando: {:?}", e);
                return Err(std::io::Error::new(std::io::ErrorKind::Other, e.to_string()));
            }
        }

        return Ok(());
    }

    let cwd = std::env::current_dir().unwrap_or_default();
    log::info!("╔══════════════════════════════════════════╗");
    log::info!("║  🧪 DAITHON BRIDGE - Dr. Xeno Mode      ║");
    log::info!("║  Kukuku... Iniciando servidor local...   ║");
    log::info!("║  CWD: {:?} ", cwd);
    log::info!("║  Puerto: 8765                            ║");
    log::info!("╚══════════════════════════════════════════╝");


    // Inicializar LLM (DESACTIVADO)
    // llm_engine::initialize().await;

    // Crear directorios del Gym
    let _ = std::fs::create_dir_all("references");
    let _ = std::fs::create_dir_all("checkpoints");

    // Inicializar sistema híbrido Gym
    let args: Vec<String> = std::env::args().collect();
    let mut gym = GymDirector::new(50);
    
    if let Some(checkpoint_path) = args.get(1) {
        log::info!("📂 Intentando cargar cerebro de Daithon desde: {}", checkpoint_path);
        match GymDirector::load_checkpoint(checkpoint_path) {
            Ok(loaded_gym) => {
                log::info!("🧠 Cerebro cargado exitosamente. Epoch actual: {}", loaded_gym.current_epoch);
                gym = loaded_gym;
            }
            Err(e) => {
                log::error!("❌ Error cargando checkpoint: {}. Iniciando cerebro nuevo.", e);
            }
        }
    }

    let gym_arc = Arc::new(RwLock::new(gym));
    let librarian_arc = Arc::new(RwLock::new(DataLibrarian::new(32)));

    // --- WORLD MODEL ---
    let wm_config = world_model::coordinator::TrainingConfig {
        buffer_size: 25000,
        successful_only: false,
        retrain_interval: 100,
        learning_rate: 0.001,
        epochs_per_training: 30,
        min_examples: 100,
    };
    let wm_coordinator = Arc::new(RwLock::new(world_model::coordinator::WorldModelCoordinator::new(wm_config)));
    let (bridge, _agent_tx, _agent_rx) = world_model::bridge_integration::WorldModelBridge::new_shared(wm_coordinator.clone(), 100);

    let bridge_handle = tokio::spawn(async move {
        bridge.run().await;
    });

    // --- GYM COORDINATOR ---
    let coordinator = TrainingCoordinator::new(
        gym_arc.clone(), 
        librarian_arc.clone(),
        wm_coordinator.clone()
    );
    
    // Spawn background training loop
    tokio::spawn(async move {
        if let Err(e) = coordinator.run_training_cycle().await {
            log::error!("❌ Error en training cycle: {}", e);
        }
    });

    let judge = crate::trinity::judge::naturalness_evaluator::NaturalnessJudge::new();
    let mut cortex_engine = crate::cortex::CortexEngine::new(judge.clone());

    // OMNI-INJECT: Cargar KnowledgeBase masivo
    if let Ok(loaded_kb) = crate::cortex::comprehension::deep_understanding::KnowledgeBase::load_checkpoint("checkpoints/kb_omni.json") {
        log::info!("🧠 CORTEX: KnowledgeBase masivo cargado exitosamente.");
        cortex_engine.comprehension.knowledge_base = loaded_kb;
    }

    let shared_cortex = Arc::new(RwLock::new(cortex_engine));
    let shared_trinity = Arc::new(RwLock::new(crate::trinity::training::triangular_loop::TriangularTrainingLoop::new()));
    let shared_contextus = Arc::new(RwLock::new(crate::contextus::DaithonContext::new()));
    let shared_metacog = Arc::new(RwLock::new(crate::metacog::MetaCogEngine::new()));
    let shared_lab = Arc::new(tokio::sync::Mutex::new(crate::forge::experimental_lab::LabEngine::new()));
    let shared_learner = Arc::new(crate::forge::autonomous_learner::AutonomousLearner::new(shared_cortex.clone()));

    let dashboard_state = web::Data::new(gym::dashboard_api::DashboardState {
        gym: gym_arc.clone(),
        librarian: librarian_arc.clone(),
        wm_coordinator: wm_coordinator.clone(),
        evolution: Arc::new(RwLock::new(design_evolution::timeline::EvolutionSimulator::new(20, 0.3))),
        language_anchor: Arc::new(RwLock::new(crate::synapse::anchor::AnchorEngine::new(crate::synapse::lexicon::Lexicon::new(), crate::archetype::taxonomy::TaxonomyTree::new()))),
        lingua_engine: Arc::new(RwLock::new({
            let mut engine = crate::lingua::engine::LinguaEngine::new();
            let _ = engine.load_pretrained();
            engine
        })),
        persona: Arc::new(RwLock::new(crate::persona::integration::DaithonPersona::new())),
        cortex: shared_cortex.clone(),
        trinity: shared_trinity.clone(),
        contextus: shared_contextus.clone(),
        metacog: shared_metacog.clone(),
    });

    let data = web::Data::new(AppState {
        conversation_history: std::sync::Mutex::new(Vec::new()),
        daithon_mood: std::sync::Mutex::new("calculating".to_string()),
        xeno_protocol: std::sync::atomic::AtomicBool::new(false),
        ws_sessions: std::sync::Mutex::new(Vec::new()),
        cortex: shared_cortex.clone(),
        trinity: shared_trinity.clone(),
        contextus: shared_contextus.clone(),
        metacog: shared_metacog.clone(),
        lab_engine: shared_lab.clone(),
        learner: shared_learner.clone(),
    });

    // --- XENO PROTOCOL (AUTONOMOUS IMPULSE) ---
    let autonomous_data = data.clone();
    tokio::spawn(async move {
        let whims = vec![
            "Construye un pequeño pilar de observación",
            "Dispersa algunas rocas góticas para ambientar",
            "Crea una mesa y sillas para una reunión científica",
            "Levanta un arco de piedra elegante",
            "Genera un pequeño bosque de pinos",
            "Construye un muro bajo de ladrillos",
            "Crea un objeto para estudiar colisiones (esfera)",
            "Kukuku... hoy el terreno necesita una pirámide experimental de cubos",
        ];

        loop {
            if autonomous_data.xeno_protocol.load(std::sync::atomic::Ordering::Relaxed) {
                let whim = whims[fastrand::usize(0..whims.len())];
                log::info!("🧠 AUTÓNOMO: Daithon tiene un antojo científico o capricho: '{}'", whim);
                
                // Procesar con orquestador 
                let mut orchestrator = crate::agents::orchestrator::AgentOrchestrator::new();
                let result = orchestrator.process_user_request(whim.to_string()).await;
                
                // Extraer comandos para el broadcast
                let mut commands = Vec::new();
                if let Some(geom) = result.get("geometric_plan") {
                    if let Some(tasks) = geom.get("tasks").and_then(|t| t.as_array()) {
                        for task in tasks {
                            commands.push(UnrealCommand {
                                action: "spawn_asset".to_string(),
                                target: task.get("mesh_path").and_then(|p| p.as_str()).unwrap_or("").to_string(),
                                parameters: task.get("transform").cloned().unwrap_or(serde_json::json!({})),
                                priority: 1,
                            });
                        }
                    }
                }

                // BROADCAST a Unreal vía WS
                let payload = serde_json::json!({
                    "response": whim,
                    "commands": commands,
                    "personality_tag": "autonomous_whim"
                }).to_string();

                let sessions = autonomous_data.ws_sessions.lock().unwrap();
                for addr in sessions.iter() {
                    addr.do_send(BroadcastCommand(payload.clone()));
                }
            }
            tokio::time::sleep(tokio::time::Duration::from_secs(12)).await;
        }
    });


    // --- SOVEREIGN AUTONOMOUS LEARNER ---
    let (unreal_tx, mut unreal_rx) = mpsc::channel::<serde_json::Value>(100);
    
    // Difusor de comandos (Lee del Learner y envía al WS)
    let broadcast_data = data.clone();
    tokio::spawn(async move {
        while let Some(command) = unreal_rx.recv().await {
            let payload = command.to_string();
            let sessions = broadcast_data.ws_sessions.lock().unwrap();
            for addr in sessions.iter() {
                addr.do_send(BroadcastCommand(payload.clone()));
            }
        }
    });

    // El Motor Soberano
    // El Motor Soberano
    let learner_for_thinker = shared_learner.clone();
    let lab_for_learner = shared_lab.clone();
    let context_for_learner = shared_contextus.clone();

    tokio::spawn(async move {
        // --- LIMPIEZA DE DATOS MOCK ---
        {
            let mut g = context_for_learner.write().await;
            g.semantic_graph.purge_mock_data();
        }

        learner_for_thinker.run_forever(
            lab_for_learner, 
            context_for_learner,
            Arc::new(TokioMutex::new(CuriosityEngine::new())), 
            unreal_tx
        ).await;
    });

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allow_any_method()
            .allow_any_header();

        App::new()
            .wrap(cors)
            .app_data(data.clone())
            .app_data(web::PayloadConfig::new(20 * 1024 * 1024)) // 20MB limit for Large PDFs
            .route("/health", web::get().to(health_check))
            .route("/chat", web::post().to(chat_with_daithon))
            .route("/build", web::post().to(build_command))
            .route("/physics", web::post().to(physics_analysis))
            .route("/pcg", web::post().to(pcg_generate))
            .route("/ws", web::get().to(ws_index))
            .route("/toggle_xeno", web::get().to(toggle_xeno))
            .route("/orchestrate", web::post().to(orchestrate_request))
            .route("/train_massive", web::post().to(train_massive))
            .route("/learn_document", web::post().to(learn_document))
            .service(gym::dashboard_api::create_dashboard_routes(dashboard_state.clone()))
            .service(actix_files::Files::new("/dashboard", "static").index_file("index.html"))
            // CODE LAB ROUTES
            .route("/api/lab/run", web::post().to(daithon_bridge::code_lab::api::lab_server::run_code))
            .route("/api/lab/start", web::post().to(daithon_bridge::code_lab::api::lab_server::start_lab))
            .route("/api/lab/exercises", web::get().to(daithon_bridge::code_lab::api::lab_server::get_exercises))
    })
    .bind("127.0.0.1:8765")?
    .run()
    .await
}
