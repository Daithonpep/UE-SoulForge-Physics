use crate::trinity::judge::naturalness_evaluator::*;

/// Agente conversacional que aprende a hablar
pub struct ConversationalAgent {
    pub id: String,
    pub personality_seed: PersonalitySeed,
    pub learned_phrases: Vec<LearnedPhrase>,
    pub conversation_memory: Vec<String>,
    pub performance_history: Vec<f64>,
    pub emotional_state: EmotionalState, // NUEVO: Vector de estado emocional
}

#[derive(Debug, Clone)]
pub struct EmotionalState {
    pub frustration: f64,
    pub curiosity: f64,
    pub joy: f64,
}

#[derive(Debug, Clone)]
pub struct PersonalitySeed {
    pub formality: f64,
    pub verbosity: f64,
    pub humor_tendency: f64,
    pub technical_language: f64,
}

#[derive(Debug, Clone)]
pub struct LearnedPhrase {
    pub phrase: String,
    pub context_tags: Vec<String>,
    pub success_rate: f64,
    pub usage_count: usize,
}

impl ConversationalAgent {
    pub fn new(id: &str, personality: PersonalitySeed) -> Self {
        Self {
            id: id.to_string(),
            personality_seed: personality,
            learned_phrases: Vec::new(),
            conversation_memory: Vec::new(),
            performance_history: Vec::new(),
            emotional_state: EmotionalState {
                frustration: 0.1,
                curiosity: 0.8,
                joy: 0.5,
            },
        }
    }

    /// Actualiza el estado emocional basado en inercia, feedback y ruido
    pub fn update_emotional_state(&mut self, feedback_score: f64) {
        let alpha: f64 = 0.8; // Inercia
        let beta: f64 = 0.3;  // Reacción al entorno
        let gamma: f64 = 0.1; // Ruido
        
        let random_noise: f64 = 0.05; // Ficticio

        // Actualizar frustración basada en feedback negativo
        let delta_frustration = if feedback_score < 0.6 { 1.0 - feedback_score } else { -0.1 };
        self.emotional_state.frustration = (alpha * self.emotional_state.frustration) + (beta * delta_frustration) + (gamma * random_noise);
        self.emotional_state.frustration = f64::clamp(self.emotional_state.frustration, 0.0, 1.0);
        
        // Actualizar alegría basada en feedback positivo
        let delta_joy = if feedback_score > 0.8 { feedback_score } else { -0.1 };
        self.emotional_state.joy = (alpha * self.emotional_state.joy) + (beta * delta_joy) + (gamma * random_noise);
        self.emotional_state.joy = f64::clamp(self.emotional_state.joy, 0.0, 1.0);
    }

    /// Generar una respuesta en una conversación
    pub fn generate_response(
        &mut self,
        prompt: &str,
        context: &ConversationContext,
    ) -> String {
        // En una implementación real, aquí se llamaría al modelo de lenguaje
        // o a un sistema de generación más complejo. Por ahora simularemos.
        
        let should_use_learned = self.emotional_state.frustration < 0.8; // Si está muy frustrado, no usa lo aprendido
        let response = if should_use_learned {
             let learned_response = self.try_learned_phrase(context);
             if let Some(r) = learned_response {
                r
             } else {
                self.construct_novel_response(prompt, context)
             }
        } else {
            self.construct_frustrated_response(prompt)
        };

        // Guardar en memoria
        self.conversation_memory.push(response.clone());
        if self.conversation_memory.len() > 20 {
            self.conversation_memory.remove(0);
        }

        response
    }

    fn construct_frustrated_response(&self, prompt: &str) -> String {
         let responses = [
            "Y eso qué importa.",
            "No preguntes eso ahora.",
            "Podríamos hablar de algo que tenga sentido.",
            "Es obvio, ¿no?",
         ];
         // Lógica simplificada de selección
         responses[prompt.len() % responses.len()].to_string()
    }


    fn try_learned_phrase(&self, context: &ConversationContext) -> Option<String> {
        // Buscar frase aprendida que coincida con el contexto
        let suitable: Vec<&LearnedPhrase> = self.learned_phrases.iter()
            .filter(|p| {
                p.success_rate > 0.7 && 
                p.context_tags.iter().any(|tag| context.topic.contains(tag))
            })
            .collect();

        if suitable.is_empty() {
            return None;
        }

        // Seleccionar la más exitosa  (Simulado aquí, normalmente requeriría ordernar)
        Some(suitable[0].phrase.clone())
    }

    fn construct_novel_response(
        &self,
        prompt: &str,
        _context: &ConversationContext,
    ) -> String {
        let mut response = String::new();

        let use_marker = self.emotional_state.joy > 0.4;
        if use_marker {
            let markers = ["Bueno", "Mira", "Pues", "Entonces"];
            // Selección simplificada sin rand
            let idx = prompt.len() % markers.len();
            response.push_str(markers[idx]);
            response.push_str(", ");
        }

        // Contenido principal
        response.push_str("sobre lo de '");
        response.push_str(prompt);
        response.push_str("', creo que es interesante");

        // Decidir si añadir pregunta
        let ask_followup = self.emotional_state.curiosity > 0.6;
        if ask_followup {
            response.push_str(". ¿Qué te parece?");
        } else {
            response.push_str(".");
        }

        response
    }

    /// Registrar feedback del juez
    pub fn register_feedback(&mut self, phrase: String, score: f64, context: Vec<String>) {
        self.performance_history.push(score);
        self.update_emotional_state(score);

        // Si la frase fue exitosa, aprenderla
        if score > 0.7 {
            if let Some(existing) = self.learned_phrases.iter_mut()
                .find(|p| p.phrase == phrase)
            {
                existing.usage_count += 1;
                existing.success_rate = (existing.success_rate * 0.9) + (score * 0.1);
            } else {
                self.learned_phrases.push(LearnedPhrase {
                    phrase,
                    context_tags: context,
                    success_rate: score,
                    usage_count: 1,
                });
            }
        }

        // Limitar tamaño de frases aprendidas
        if self.learned_phrases.len() > 1000 {
            // Eliminar las menos exitosas
            self.learned_phrases.sort_by(|a, b| 
                a.success_rate.partial_cmp(&b.success_rate).unwrap()
            );
            self.learned_phrases.truncate(800);
        }
    }

    /// Obtener score promedio reciente
    pub fn recent_performance(&self) -> f64 {
        if self.performance_history.is_empty() {
            return 0.5;
        }

        let recent = self.performance_history.iter()
            .rev()
            .take(100)
            .sum::<f64>();

        recent / self.performance_history.len().min(100) as f64
    }
}
