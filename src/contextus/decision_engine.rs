use crate::contextus::conversational_intel::{Register, UserState, ConversationalContext};

#[derive(Debug, PartialEq, Clone)]
pub enum ActivationMode {
    Bypass,      // Comando directo, sin debate (ej. Unreal)
    Shallow,     // Un núcleo, respuesta simple o conversacional
    DeepThink,   // Trinidad completa (El protocolo actual)
    Investigate, // Requiere búsqueda externa antes de opinar
    Challenge,   // La premisa del usuario está mal y Xeno debe saltar
}

#[derive(Debug)]
pub struct DecisionContext {
    pub mode: ActivationMode,
    pub confidence: f32,
    pub reason: String,
    pub intel: ConversationalContext,
}

pub struct DecisionEngine {
    pub complexity_threshold: f32,
    pub command_triggers: Vec<String>,
}

impl DecisionEngine {
    pub fn new() -> Self {
        Self {
            complexity_threshold: 0.6,
            command_triggers: vec![
                "mueve".to_string(), "crea".to_string(), "elimina".to_string(), 
                "ejecuta".to_string(), "genera".to_string(), "spawn".to_string()
            ],
        }
    }

    pub fn evaluate(&self, input: &str) -> DecisionContext {
        let input_lower = input.to_lowercase();
        let mut intel = ConversationalContext::default();

        // 1. Detectar Registro y Estado (SOBRE TEXTO CRUDO)
        if input_lower.contains("jaja") || input_lower.contains("lol") || input_lower.contains("xd") {
            intel.register = Register::Playful;
            intel.user_opened_playful_register = true;
            intel.user_state = UserState::Relajado;
        } else if input_lower.contains("error") || input_lower.contains("fallo") || input_lower.contains("horrible") {
            intel.register = Register::Critical;
            intel.user_state = UserState::Frustrado;
        } else if input_lower.contains("daithon") || input_lower.contains("erest") {
            intel.topic_is_meta = true;
        }

        // 2. Gravedad del tema
        if input_lower.contains("agujero") || input_lower.contains("física") || input_lower.contains("unreal") {
            intel.topic_gravity = 0.8;
            intel.register = Register::Technical;
        }

        // 3. Modos de Activación
        let (mode, reason) = if self.command_triggers.iter().any(|t| input_lower.contains(t)) {
            (ActivationMode::Bypass, "Comando operativo detectado.".to_string())
        } else if input_lower.contains("siempre") || input_lower.contains("nunca") {
            (ActivationMode::Challenge, "Absolutismo detectado; requiere auditoría de Xeno.".to_string())
        } else if input_lower.contains("porque") || input_lower.contains("explícame") || input.len() > 80 {
            (ActivationMode::DeepThink, "Complejidad semántica detectada.".to_string())
        } else {
            (ActivationMode::Shallow, "Interacción conversacional simple.".to_string())
        };

        DecisionContext {
            mode,
            confidence: 0.9,
            reason,
            intel,
        }
    }
}
