use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// El Juez que determina qué tan humano suena Daithon
#[derive(Clone)]
pub struct NaturalnessJudge {
    /// Dataset de diálogos reales
    pub real_dialogue_corpus: DialogueCorpus,
    
    /// Métricas de evaluación
    pub metrics: NaturalnessMetrics,
    
    /// Historial de evaluaciones
    pub evaluation_history: Vec<EvaluationRecord>,
    
    /// Patrones humanos aprendidos
    pub human_patterns: HumanPatternLibrary,
}

#[derive(Debug, Clone)]
pub struct DialogueCorpus {
    /// Diálogos de películas/series
    pub movie_scripts: Vec<DialogueExchange>,
    
    /// Conversaciones de Reddit
    pub reddit_threads: Vec<DialogueExchange>,
    
    /// Transcripciones de podcasts
    pub podcast_transcripts: Vec<DialogueExchange>,
    
    /// Chats casuales
    pub casual_chats: Vec<DialogueExchange>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialogueExchange {
    pub speaker_a: String,
    pub speaker_b: String,
    pub context: String,
    pub naturalness_score: f64,
    pub style_tags: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct NaturalnessMetrics {
    /// ¿Usa variabilidad en saludos?
    pub greeting_variety: f64,
    
    /// ¿Usa marcadores discursivos? (bueno, mira, entonces)
    pub discourse_markers: f64,
    
    /// ¿Maneja ambigüedad como humano?
    pub ambiguity_handling: f64,
    
    /// ¿Evita sonar robótico?
    pub anti_roboticism: f64,
    
    /// ¿Usa contexto implícito?
    pub contextual_awareness: f64,
    
    /// ¿Tiene ritmo conversacional natural?
    pub conversational_flow: f64,
    
    /// ¿Usa humor/sarcasmo apropiadamente?
    pub humor_appropriateness: f64,
    
    /// ¿Varía longitud de respuestas?
    pub response_length_variety: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvaluationRecord {
    pub agent_id: String,
    pub utterance: String,
    pub context: ConversationContext,
    pub scores: HashMap<String, f64>,
    pub overall_score: f64,
    pub feedback: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationContext {
    pub previous_exchanges: Vec<String>,
    pub topic: String,
    pub emotional_tone: String,
    pub formality_level: f64,
}

/// Biblioteca de patrones humanos detectados
#[derive(Debug, Clone)]
pub struct HumanPatternLibrary {
    /// Saludos variados
    pub greetings: Vec<GreetingPattern>,
    
    /// Marcadores discursivos por contexto
    pub discourse_markers: HashMap<String, Vec<String>>,
    
    /// Patrones de ritmo (longitud de turnos)
    pub rhythm_patterns: Vec<RhythmPattern>,
    
    /// Frases de conexión natural
    pub connective_phrases: Vec<String>,
    
    /// Patrones de humor
    pub humor_patterns: Vec<HumorPattern>,
}

#[derive(Debug, Clone)]
pub struct GreetingPattern {
    pub text: String,
    pub formality: f64,
    pub frequency: f64,
    pub context: String,
}

#[derive(Debug, Clone)]
pub struct RhythmPattern {
    pub avg_words_per_turn: f64,
    pub variance: f64,
    pub context: String,
}

#[derive(Debug, Clone)]
pub struct HumorPattern {
    pub pattern_type: HumorType,
    pub example: String,
    pub appropriateness_score: f64,
}

#[derive(Debug, Clone)]
pub enum HumorType {
    Sarcasm,
    Wordplay,
    SelfDeprecation,
    Exaggeration,
    Callback,
}

impl NaturalnessJudge {
    pub fn new() -> Self {
        let mut judge = Self {
            real_dialogue_corpus: DialogueCorpus::empty(),
            metrics: NaturalnessMetrics::default(),
            evaluation_history: Vec::new(),
            human_patterns: HumanPatternLibrary::new(),
        };

        // Cargar corpus desde archivos o APIs
        judge.load_real_dialogues();
        judge
    }

    /// Evaluar qué tan natural suena una respuesta
    pub fn evaluate_naturalness(
        &mut self,
        agent_id: &str,
        utterance: &str,
        context: &ConversationContext,
    ) -> EvaluationRecord {
        let mut scores = HashMap::new();

        // 1. Evaluar variabilidad de saludos
        scores.insert(
            "greeting_variety".to_string(),
            self.score_greeting_variety(utterance, context),
        );

        // 2. Evaluar marcadores discursivos
        scores.insert(
            "discourse_markers".to_string(),
            self.score_discourse_markers(utterance),
        );

        // 3. Evaluar anti-robotismo
        scores.insert(
            "anti_roboticism".to_string(),
            self.score_anti_roboticism(utterance),
        );

        // 4. Evaluar manejo de ambigüedad
        scores.insert(
            "ambiguity_handling".to_string(),
            self.score_ambiguity_handling(utterance, context),
        );

        // 5. Evaluar conciencia contextual
        scores.insert(
            "contextual_awareness".to_string(),
            self.score_contextual_awareness(utterance, context),
        );

        // 6. Evaluar flujo conversacional
        scores.insert(
            "conversational_flow".to_string(),
            self.score_conversational_flow(utterance, context),
        );

        // 7. Evaluar variedad de longitud
        scores.insert(
            "response_length_variety".to_string(),
            self.score_response_length_variety(utterance, context),
        );

        // 8. Comparar con corpus real
        let similarity_to_real = self.compare_with_real_corpus(utterance, context);
        scores.insert("corpus_similarity".to_string(), similarity_to_real);

        // Calcular score global
        let overall_score = scores.values().sum::<f64>() / scores.len() as f64;

        // Generar feedback constructivo
        let feedback = self.generate_feedback(&scores, utterance);

        let record = EvaluationRecord {
            agent_id: agent_id.to_string(),
            utterance: utterance.to_string(),
            context: context.clone(),
            scores,
            overall_score,
            feedback,
            timestamp: Self::current_timestamp(),
        };

        self.evaluation_history.push(record.clone());
        record
    }

    fn score_greeting_variety(&self, utterance: &str, context: &ConversationContext) -> f64 {
        // Verificar si es un saludo
        let is_greeting = context.previous_exchanges.is_empty() 
            || utterance.to_lowercase().starts_with("hola")
            || utterance.to_lowercase().starts_with("hey")
            || utterance.to_lowercase().starts_with("buenos");

        if !is_greeting {
            return 1.0; // No aplica
        }

        // Verificar si usa variedad (no siempre "Hola")
        let greeting_lower = utterance.to_lowercase();
        
        // Patrones robóticos (penalizar)
        if greeting_lower == "hola" 
            || greeting_lower == "hola." 
            || greeting_lower == "buenos días" {
            return 0.3; // Muy genérico
        }

        // Patrones naturales (premiar)
        let natural_greetings = [
            "¡qué onda!", "ey", "¿cómo va?", "¿todo bien?",
            "¡hola! ¿qué tal?", "buenas", "¿qué hay?",
        ];

        for natural in &natural_greetings {
            if greeting_lower.contains(natural) {
                return 0.9;
            }
        }

        0.6 // Neutro
    }

    fn score_discourse_markers(&self, utterance: &str) -> f64 {
        let markers = [
            "bueno", "mira", "entonces", "o sea", "sabes", 
            "pues", "vamos", "claro", "vale", "eh",
        ];

        let utterance_lower = utterance.to_lowercase();
        let count = markers.iter()
            .filter(|m| utterance_lower.contains(*m))
            .count();

        if count == 0 {
            return 0.4; // Muy formal/robótico
        }

        if count > 3 {
            return 0.6; // Demasiados marcadores
        }

        0.9 // Cantidad natural
    }

    fn score_anti_roboticism(&self, utterance: &str) -> f64 {
        let mut score: f64 = 1.0;

        // Penalizaciones por patrones robóticos
        
        // 1. Demasiado formal
        let formal_patterns = [
            "según mis cálculos",
            "de acuerdo a",
            "en base a",
            "cabe mencionar que",
            "es importante destacar",
        ];

        for pattern in &formal_patterns {
            if utterance.to_lowercase().contains(pattern) {
                score -= 0.15;
            }
        }

        // 2. Estructura demasiado perfecta
        let word_count = utterance.split_whitespace().count();
        if word_count > 20 && !utterance.contains(",") && !utterance.contains("y") {
            score -= 0.2; // Frase muy larga sin pausas = robótico
        }

        // 3. Respuesta genérica de chatbot
        let generic_bot_phrases = [
            "como modelo de lenguaje",
            "no puedo",
            "lo siento, pero",
            "entiendo que",
        ];

        for phrase in &generic_bot_phrases {
            if utterance.to_lowercase().contains(phrase) {
                score -= 0.5; // Penalización fuerte
            }
        }

        // Bonificaciones por naturalidad
        
        // 1. Usa contracciones
        let contractions = ["pa'", "to'", "na'", "'ta", "no'"];
        for contraction in &contractions {
            if utterance.contains(contraction) {
                score += 0.1;
            }
        }

        // 2. Usa interjecciones
        let interjections = ["¡wow!", "¡genial!", "¡uy!", "¡ah!", "¡eh!"];
        for interj in &interjections {
            if utterance.to_lowercase().contains(interj) {
                score += 0.1;
            }
        }

        score.clamp(0.0, 1.0)
    }

    fn score_ambiguity_handling(&self, utterance: &str, context: &ConversationContext) -> f64 {
        // Los humanos usan referencias ambiguas si hay contexto
        let ambiguous_refs = ["eso", "aquello", "la cosa", "el tema", "lo que dije"];

        let has_ambiguity = ambiguous_refs.iter()
            .any(|r| utterance.to_lowercase().contains(r));

        if has_ambiguity && !context.previous_exchanges.is_empty() {
            return 0.9; // Usa contexto implícito = natural
        }

        if has_ambiguity && context.previous_exchanges.is_empty() {
            return 0.2; // Ambiguo sin contexto = malo
        }

        0.6 // Neutro (explícito está bien)
    }

    fn score_contextual_awareness(&self, utterance: &str, context: &ConversationContext) -> f64 {
        if context.previous_exchanges.is_empty() {
            return 1.0; // Primera interacción, no aplica
        }

        // Verificar si hace referencia al contexto previo
        let last_exchange = context.previous_exchanges.last().unwrap();
        
        // Extrae palabras clave del último intercambio
        let keywords: Vec<&str> = last_exchange.split_whitespace()
            .filter(|w| w.len() > 4)
            .collect();

        let utterance_lower = utterance.to_lowercase();
        let references_previous = keywords.iter()
            .any(|k| utterance_lower.contains(&k.to_lowercase()));

        if references_previous {
            return 0.9;
        }

        // Verifica si usa palabras de seguimiento
        let follow_up_words = ["también", "además", "respecto a eso", "sobre eso", "eso mismo"];
        let has_follow_up = follow_up_words.iter()
            .any(|w| utterance_lower.contains(w));

        if has_follow_up {
            return 0.8;
        }

        0.5 // No conecta con contexto previo
    }

    fn score_conversational_flow(&self, utterance: &str, context: &ConversationContext) -> f64 {
        let word_count = utterance.split_whitespace().count();

        // Los humanos varían la longitud según el flujo
        
        // Respuestas muy cortas en contextos que requieren elaboración
        if context.topic.contains("explica") || context.topic.contains("por qué") {
            if word_count < 10 {
                return 0.3; // Demasiado breve
            }
        }

        // Respuestas muy largas en saludos
        if context.previous_exchanges.is_empty() && word_count > 30 {
            return 0.4; // Demasiado verboso para inicio
        }

        // Longitud apropiada
        if word_count >= 8 && word_count <= 40 {
            return 0.9;
        }

        0.6
    }

    fn score_response_length_variety(&self, utterance: &str, context: &ConversationContext) -> f64 {
        let current_length = utterance.split_whitespace().count();

        if context.previous_exchanges.len() < 3 {
            return 1.0; // No suficiente historial
        }

        // Calcular longitudes previas de este agente
        let previous_lengths: Vec<usize> = context.previous_exchanges.iter()
            .map(|e| e.split_whitespace().count())
            .collect();

        // Calcular varianza
        let avg = previous_lengths.iter().sum::<usize>() as f64 / previous_lengths.len() as f64;
        let variance = previous_lengths.iter()
            .map(|&l| (l as f64 - avg).powi(2))
            .sum::<f64>() / previous_lengths.len() as f64;

        // Alta varianza = natural
        if variance > 50.0 {
            return 0.9;
        }

        // Baja varianza = robótico (siempre la misma longitud)
        if variance < 10.0 {
            return 0.3;
        }

        0.6
    }

    fn compare_with_real_corpus(&self, utterance: &str, _context: &ConversationContext) -> f64 {
        // Buscar intercambios similares en el corpus real
        let mut best_similarity: f64 = 0.0;

        for real_exchange in self.real_dialogue_corpus.all_exchanges() {
            let similarity = self.calculate_text_similarity(utterance, &real_exchange.speaker_a);
            best_similarity = best_similarity.max(similarity);
        }

        best_similarity
    }

    fn calculate_text_similarity(&self, text_a: &str, text_b: &str) -> f64 {
        // Similaridad simple basada en palabras compartidas
        let words_a: std::collections::HashSet<&str> = text_a.split_whitespace().collect();
        let words_b: std::collections::HashSet<&str> = text_b.split_whitespace().collect();

        let intersection = words_a.intersection(&words_b).count();
        let union = words_a.union(&words_b).count();

        if union == 0 {
            return 0.0;
        }

        intersection as f64 / union as f64
    }

    fn generate_feedback(&self, scores: &HashMap<String, f64>, _utterance: &str) -> String {
        let mut feedback = Vec::new();

        // Identificar puntos débiles
        if let Some(&score) = scores.get("greeting_variety") {
            if score < 0.5 {
                feedback.push("Intenta variar tus saludos. No siempre digas lo mismo.".to_string());
            }
        }

        if let Some(&score) = scores.get("discourse_markers") {
            if score < 0.5 {
                feedback.push("Usa palabras como 'bueno', 'mira', 'entonces' para sonar más natural.".to_string());
            }
        }

        if let Some(&score) = scores.get("anti_roboticism") {
            if score < 0.5 {
                feedback.push("Suenas muy formal. Intenta hablar como si estuvieras en un café.".to_string());
            }
        }

        if let Some(&score) = scores.get("contextual_awareness") {
            if score < 0.5 {
                feedback.push("Conecta más con lo que se dijo antes. Usa referencias.".to_string());
            }
        }

        // Reconocer fortalezas
        if scores.values().all(|&s| s > 0.7) {
            feedback.push("¡Excelente! Muy natural.".to_string());
        }

        if feedback.is_empty() {
            feedback.push("Aceptable, pero puedes mejorar.".to_string());
        }

        feedback.join(" ")
    }

    fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    fn load_real_dialogues(&mut self) {
        // Cargar corpus real (implementación simplificada)
        // En producción, esto cargaría desde archivos/APIs
        self.real_dialogue_corpus.casual_chats.push(DialogueExchange {
            speaker_a: "Oye, ¿viste esa mesa?".to_string(),
            speaker_b: "¡Sí! Fue genial, no?".to_string(),
            context: "Diseño 3D".to_string(),
            naturalness_score: 0.9,
            style_tags: vec!["casual".to_string()],
        });
    }
}

impl DialogueCorpus {
    pub fn empty() -> Self {
        Self {
            movie_scripts: Vec::new(),
            reddit_threads: Vec::new(),
            podcast_transcripts: Vec::new(),
            casual_chats: Vec::new(),
        }
    }

    pub fn all_exchanges(&self) -> Vec<&DialogueExchange> {
        let mut all = Vec::new();
        all.extend(self.movie_scripts.iter());
        all.extend(self.reddit_threads.iter());
        all.extend(self.podcast_transcripts.iter());
        all.extend(self.casual_chats.iter());
        all
    }
}

impl Default for NaturalnessMetrics {
    fn default() -> Self {
        Self {
            greeting_variety: 0.0,
            discourse_markers: 0.0,
            ambiguity_handling: 0.0,
            anti_roboticism: 0.0,
            contextual_awareness: 0.0,
            conversational_flow: 0.0,
            humor_appropriateness: 0.0,
            response_length_variety: 0.0,
        }
    }
}

impl HumanPatternLibrary {
    pub fn new() -> Self {
        Self {
            greetings: vec![
                GreetingPattern {
                    text: "¡Qué onda!".to_string(),
                    formality: 0.2,
                    frequency: 0.8,
                    context: "casual".to_string(),
                },
                GreetingPattern {
                    text: "¿Cómo estás?".to_string(),
                    formality: 0.5,
                    frequency: 0.9,
                    context: "neutral".to_string(),
                },
                GreetingPattern {
                    text: "Buenos días".to_string(),
                    formality: 0.8,
                    frequency: 0.7,
                    context: "formal".to_string(),
                },
            ],
            discourse_markers: {
                let mut map = HashMap::new();
                map.insert("casual".to_string(), vec![
                    "bueno".to_string(),
                    "mira".to_string(),
                    "o sea".to_string(),
                    "sabes".to_string(),
                ]);
                map.insert("formal".to_string(), vec![
                    "entonces".to_string(),
                    "por lo tanto".to_string(),
                    "así pues".to_string(),
                ]);
                map
            },
            rhythm_patterns: vec![],
            connective_phrases: vec![
                "por cierto".to_string(),
                "a propósito".to_string(),
                "hablando de eso".to_string(),
            ],
            humor_patterns: vec![],
        }
    }
}
