#[derive(Debug, PartialEq, Clone)]
pub enum Register {
    Technical,      // Explicación seria, tema complejo
    Collaborative,  // Trabajando juntos en algo
    Casual,         // Conversación sin objetivo específico
    Playful,        // El usuario está siendo gracioso primero
    Critical,       // Algo salió mal, hay tensión
}

#[derive(Debug, PartialEq, Clone)]
pub enum UserState {
    Focused,        // Quiere información directa
    Explorando,     // Curioso, hay espacio para divagar
    Frustrado,      // No es momento de chistes
    Relajado,       // Hay margen para humor
}

#[derive(Debug, Clone)]
pub struct ConversationalContext {
    pub register: Register,
    pub user_state: UserState,
    pub topic_gravity: f32,      // 0.0 trivial → 1.0 crítico
    pub relationship_depth: f32,  // Cuánta confianza hay acumulada
    pub user_opened_playful_register: bool,
    pub session_depth: usize,
    pub topic_is_meta: bool,
    pub detected_test_pattern: bool,
}

impl ConversationalContext {
    pub fn default() -> Self {
        Self {
            register: Register::Technical,
            user_state: UserState::Focused,
            topic_gravity: 0.5,
            relationship_depth: 0.5,
            user_opened_playful_register: false,
            session_depth: 0,
            topic_is_meta: false,
            detected_test_pattern: false,
        }
    }

    pub fn can_use_humor(&self) -> bool {
        if self.topic_gravity > 0.7 || self.user_state == UserState::Frustrado {
            return false;
        }
        if self.relationship_depth < 0.4 {
            return false;
        }
        self.user_opened_playful_register
    }

    pub fn should_break_fourth_wall(&self) -> bool {
        self.topic_is_meta || 
        self.detected_test_pattern || 
        (self.session_depth > 20 && self.relationship_depth > 0.7)
    }
}

pub enum ResponseScope {
    Instant,        // Una línea, comando o pregunta trivial
    Compact,        // Dos o tres oraciones, pregunta directa
    Developed,      // Varios párrafos, tema con profundidad
    Deep,           // Documento completo, análisis exhaustivo
}

impl ResponseScope {
    pub fn from_intel(
        mode: &crate::contextus::decision_engine::ActivationMode,
        topic_complexity: f32,
        register: &Register,
    ) -> Self {
        use crate::contextus::decision_engine::ActivationMode;

        if matches!(mode, ActivationMode::Bypass) {
            return ResponseScope::Instant;
        }

        if matches!(register, Register::Casual) && topic_complexity < 0.5 {
            return ResponseScope::Compact;
        }

        match topic_complexity {
            x if x < 0.3 => ResponseScope::Compact,
            x if x < 0.6 => ResponseScope::Developed,
            _ => ResponseScope::Deep,
        }
    }
}
