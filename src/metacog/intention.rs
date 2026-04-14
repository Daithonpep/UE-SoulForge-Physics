use serde::{Deserialize, Serialize};

/// Detector de intención real (Sarcasmo, frustración, etc)
#[derive(Debug, Serialize, Deserialize)]
pub struct IntentionDetector {
    pub emotional_history: Vec<(u64, EmotionalState)>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum EmotionalState {
    Neutral,
    Curious,
    Frustrated,
    Sarcastic,
    Testing,
    Friendly,
}

#[derive(Debug, Clone)]
pub struct IntentionAnalysis {
    pub literal_meaning: String,
    pub real_intention: RealIntention,
    pub emotional_state: EmotionalState,
    pub confidence: f64,
    pub evidence: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum RealIntention {
    GenuineQuestion,
    SincereCompliment,
    Sarcasm { actual_sentiment: String },
    FrustratedRequest,
    ImplicitCorrection { what_was_wrong: String },
    Humor,
    TestingKnowledge,
}

impl IntentionDetector {
    pub fn new() -> Self {
        Self {
            emotional_history: Vec::new(),
        }
    }

    pub fn analyze_intention(
        &mut self,
        user_input: &str,
        context: &IntentionContext,
    ) -> IntentionAnalysis {
        let mut evidence = Vec::new();
        let input_lower = user_input.to_lowercase();

        // 1. Detección de Sarcasmo
        let sarcasm_score = self.calculate_sarcasm_score(&input_lower, context, &mut evidence);
        if sarcasm_score > 0.6 {
            let state = EmotionalState::Sarcastic;
            self.emotional_history.push((Self::now(), state.clone()));
            return IntentionAnalysis {
                literal_meaning: user_input.into(),
                real_intention: RealIntention::Sarcasm { actual_sentiment: "Frustración o burla por error previo".into() },
                emotional_state: state,
                confidence: sarcasm_score,
                evidence,
            };
        }

        // 2. Detección de Frustración
        let frustration_score = self.calculate_frustration_score(&input_lower, context, &mut evidence);
        if frustration_score > 0.6 {
            let state = EmotionalState::Frustrated;
            self.emotional_history.push((Self::now(), state.clone()));
            return IntentionAnalysis {
                literal_meaning: user_input.into(),
                real_intention: RealIntention::FrustratedRequest,
                emotional_state: state,
                confidence: frustration_score,
                evidence,
            };
        }

        // 3. Intención por defecto
        let state = if user_input.contains("?") { EmotionalState::Curious } else { EmotionalState::Neutral };
        IntentionAnalysis {
            literal_meaning: user_input.into(),
            real_intention: RealIntention::GenuineQuestion,
            emotional_state: state,
            confidence: 0.8,
            evidence: vec!["Análisis estándar".into()],
        }
    }

    fn calculate_sarcasm_score(&self, input: &str, context: &IntentionContext, evidence: &mut Vec<String>) -> f64 {
        let mut score: f64 = 0.0;
        let positive_words = ["inteligente", "genial", "brillante", "increíble", "asombroso"];
        let has_positive = positive_words.iter().any(|w| input.contains(w));

        if has_positive && context.daithon_just_made_error {
            score += 0.7;
            evidence.push("Halago detectado justo después de un error de Daithon.".into());
        }

        if input.starts_with("vaya") || input.starts_with("wow") || input.starts_with("claro") {
            if has_positive {
                score += 0.2;
                evidence.push("Uso de prefijo exclamativo con halago.".into());
            }
        }

        score.min(1.0)
    }

    fn calculate_frustration_score(&self, input: &str, context: &IntentionContext, evidence: &mut Vec<String>) -> f64 {
        let mut score: f64 = 0.0;
        if input.contains("no entendiste") || input.contains("no es eso") {
            score += 0.6;
            evidence.push("El usuario señala falta de comprensión.".into());
        }
        if context.user_repeated_question {
            score += 0.4;
            evidence.push("Pregunta repetida detectada.".into());
        }
        score.min(1.0)
    }

    fn now() -> u64 {
        std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
    }
}

pub struct IntentionContext {
    pub daithon_just_made_error: bool,
    pub last_daithon_error: Option<String>,
    pub user_repeated_question: bool,
}
