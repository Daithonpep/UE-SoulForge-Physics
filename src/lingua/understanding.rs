//! LINGUA — Fase 3: Comprensión Contextual Profunda
//!
//! Parser que entiende INTENCIÓN, no solo sintaxis.
//! Mantiene memoria conversacional y resuelve referencias.

use crate::lingua::training::{TrainedKnowledge, SentenceGenome};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ────────────────────────────────────────────────────────────────
//  TIPOS DE INTENCIÓN
// ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntentType {
    CreateNewDesign,
    ModifyExisting,
    AskQuestion,
    RequestExplanation,
    SetPreference,
    Compare,
    Greeting,
    Gratitude,
    Complaint,
    Rejection,
    Approval,
    Other,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IntentParameter {
    Dimension(f64),
    Material(String),
    Style(String),
    Quantity(usize),
    Color(String),
    Constraint(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intent {
    pub intent_type: IntentType,
    pub target_concept: Option<String>,
    pub parameters: HashMap<String, IntentParameter>,
    pub confidence: f64,
    pub raw_input: String,
}

// ────────────────────────────────────────────────────────────────
//  MEMORIA CONVERSACIONAL
// ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct ConversationTurn {
    pub turn_number: usize,
    pub user_input: String,
    pub parsed_intent: Intent,
    pub system_response: String,
    pub active_object: Option<String>,
}

// ────────────────────────────────────────────────────────────────
//  PARSER CONTEXTUAL
// ────────────────────────────────────────────────────────────────

pub struct DeepContextParser {
    trained_knowledge: TrainedKnowledge,
    conversation_memory: Vec<ConversationTurn>,
    /// Objeto activo actual (para resolver "hazla más alta" → "la" = último objeto)
    active_object: Option<String>,
    turn_counter: usize,
}

impl DeepContextParser {
    pub fn new(trained_knowledge: TrainedKnowledge) -> Self {
        Self {
            trained_knowledge,
            conversation_memory: Vec::new(),
            active_object: None,
            turn_counter: 0,
        }
    }

    /// Parsear entrada del usuario con contexto conversacional completo
    pub fn parse_with_context(&mut self, user_input: &str) -> Intent {
        let lower = user_input.to_lowercase();

        // 1. Clasificar intención primaria
        let intent_type = self.classify_intent(&lower);

        // 2. Extraer parámetros
        let mut parameters = self.extract_parameters(&lower);

        // 3. Resolver referencias contextuales
        self.resolve_context(&lower, &mut parameters);

        // 4. Encontrar concepto objetivo
        let target_concept = self.find_target_concept(&lower);

        // 5. Actualizar contexto
        if let Some(ref concept) = target_concept {
            self.active_object = Some(concept.clone());
        }

        // 6. Calcular confianza
        let confidence = self.calculate_confidence(&lower, &intent_type, &parameters);

        let intent = Intent {
            intent_type,
            target_concept,
            parameters,
            confidence,
            raw_input: user_input.to_string(),
        };

        // Guardar en memoria
        self.turn_counter += 1;
        self.conversation_memory.push(ConversationTurn {
            turn_number: self.turn_counter,
            user_input: user_input.to_string(),
            parsed_intent: intent.clone(),
            system_response: String::new(), // Se llena después por el generador
            active_object: self.active_object.clone(),
        });

        intent
    }

    /// Clasificar intención usando patrones lingüísticos
    fn classify_intent(&self, input: &str) -> IntentType {
        // Comandos de creación
        if Self::starts_with_any(input, &["diseña", "crea", "genera", "construye", "haz", "hazme", "necesito"]) {
            return IntentType::CreateNewDesign;
        }

        // Modificaciones
        if Self::starts_with_any(input, &["modifica", "cambia", "ajusta", "hazlo", "hazla", "ponle"]) ||
           input.contains("más ") || input.contains("menos ") {
            return IntentType::ModifyExisting;
        }

        // Preguntas
        if input.contains('?') || 
           Self::starts_with_any(input, &["por qué", "cómo", "qué", "cuánto", "dónde", "cuál"]) ||
           Self::contains_any(input, &["que significa", "qué significa", "como lo haces", "qué haces", "que haces", "qué pasa", "por que "]) 
        {
            return IntentType::AskQuestion;
        }

        // Explicaciones
        if Self::starts_with_any(input, &["explica", "describe", "cuéntame", "dime"]) {
            return IntentType::RequestExplanation;
        }

        // Preferencias
        if Self::starts_with_any(input, &["prefiero", "quiero", "me gusta"]) {
            return IntentType::SetPreference;
        }

        // Comparación
        if input.contains("compara") || input.contains("diferencia") || input.contains("mejor") {
            return IntentType::Compare;
        }

        // Saludos
        if Self::contains_any(input, &["hola", "buenos días", "buenas tardes", "saludos", "hey"]) {
            return IntentType::Greeting;
        }

        // Agradecimiento
        if Self::contains_any(input, &["gracias", "genial", "excelente", "perfecto", "me encanta"]) {
            return IntentType::Gratitude;
        }

        // Rechazo
        if Self::contains_any(input, &["no me gusta", "está mal", "horrible", "terrible", "no", "feo"]) {
            return IntentType::Rejection;
        }

        // Aprobación
        if Self::contains_any(input, &["sí", "dale", "adelante", "ok", "bien", "apruebo", "acepto"]) {
            return IntentType::Approval;
        }

        // Quejas
        if Self::contains_any(input, &["otra vez", "no funciona", "siempre", "nunca"]) {
            return IntentType::Complaint;
        }

        IntentType::Other
    }

    /// Extraer parámetros concretos del texto
    fn extract_parameters(&self, input: &str) -> HashMap<String, IntentParameter> {
        let mut params = HashMap::new();

        // Dimensiones numéricas: "80cm", "1.5 metros", "2m"
        let words: Vec<&str> = input.split_whitespace().collect();
        for i in 0..words.len() {
            let w = words[i];
            // Intentar parsear como número + unidad
            if let Some(num) = Self::try_parse_dimension(w) {
                params.insert("dimension".to_string(), IntentParameter::Dimension(num));
            }
            // "N patas/cajones/estantes"
            if let Ok(n) = w.parse::<usize>() {
                if i + 1 < words.len() {
                    let next = words[i + 1];
                    if ["patas", "cajones", "estantes", "asientos", "niveles"].iter().any(|s| next.starts_with(s)) {
                        params.insert("quantity".to_string(), IntentParameter::Quantity(n));
                    }
                }
            }
        }

        // Materiales
        let materials = ["madera", "acero", "vidrio", "metal", "piedra", "mármol", "concreto", "aluminio", "cuero", "plástico"];
        for mat in materials {
            if input.contains(mat) {
                params.insert("material".to_string(), IntentParameter::Material(mat.to_string()));
                break;
            }
        }

        // Estilos
        let styles = ["minimalista", "moderno", "clásico", "industrial", "rústico", "contemporáneo", "escandinavo", "barroco", "orgánico"];
        for style in styles {
            if input.contains(style) {
                params.insert("style".to_string(), IntentParameter::Style(style.to_string()));
                break;
            }
        }

        // Colores
        let colors = ["blanco", "negro", "gris", "marrón", "rojo", "azul", "verde", "beige"];
        for color in colors {
            if input.contains(color) {
                params.insert("color".to_string(), IntentParameter::Color(color.to_string()));
                break;
            }
        }

        params
    }

    /// Resolver referencias contextuales (pronombres, elipsis)
    fn resolve_context(&self, input: &str, params: &mut HashMap<String, IntentParameter>) {
        // "hazla más alta" → "la" se refiere al active_object
        // "ponle madera" → "le" se refiere al active_object
        // No necesitamos hacer nada con los params; el target_concept se resuelve abajo.
        // Pero si el usuario dice "más alto" sin especificar qué, heredamos el objeto activo.
        let _ = (input, params);
    }

    /// Encontrar el concepto 3D objetivo
    fn find_target_concept(&self, input: &str) -> Option<String> {
        // Buscar directamente en conceptos conocidos
        let furniture_map: Vec<(&str, &str)> = vec![
            ("mesa", "dining_table"),
            ("silla", "chair"),
            ("sofá", "sofa"),
            ("escritorio", "desk"),
            ("estantería", "storage"),
            ("cama", "beds"),
            ("taburete", "stool"),
            ("banco", "bench"),
            ("armario", "storage"),
            ("aparador", "console_table"),
        ];

        for (word, concept) in &furniture_map {
            if input.contains(word) {
                return Some(concept.to_string());
            }
        }

        // Fallback: buscar en conceptos entrenados
        for concept in &self.trained_knowledge.grounded_concept_ids {
            if input.contains(concept) {
                return Some(concept.clone());
            }
        }

        // Último recurso: heredar del contexto activo
        self.active_object.clone()
    }

    /// Calcular confianza de la interpretación
    fn calculate_confidence(&self, input: &str, intent: &IntentType, params: &HashMap<String, IntentParameter>) -> f64 {
        let mut conf: f64 = 0.3; // Base

        match intent {
            IntentType::Greeting | IntentType::Gratitude | IntentType::Approval | IntentType::Rejection | IntentType::AskQuestion | IntentType::RequestExplanation => {
                conf += 0.5;
            }
            _ => {}
        }

        // Parámetros detectados → más confianza
        conf += params.len() as f64 * 0.15;

        // Palabras reconocidas en vocabulario entrenado
        let words: Vec<&str> = input.split_whitespace().collect();
        let recognized = words.iter()
            .filter(|w| self.trained_knowledge.grounded_concept_ids.iter().any(|c| c.contains(*w)))
            .count();
        
        if !words.is_empty() {
            conf += (recognized as f64 / words.len() as f64) * 0.25;
        }

        // Similitud con frases élite entrenadas
        if self.find_similar_elite(input).is_some() {
            conf += 0.2;
        }

        conf.min(1.0)
    }

    /// Buscar frase similar entre las entrenadas
    fn find_similar_elite(&self, input: &str) -> Option<&SentenceGenome> {
        let input_words: std::collections::HashSet<&str> = input.split_whitespace().collect();

        self.trained_knowledge.elite_sentences.iter()
            .filter(|s| {
                let sw: std::collections::HashSet<&str> = s.sentence.split_whitespace().collect();
                let overlap = input_words.intersection(&sw).count();
                overlap >= 2 // Al menos 2 palabras en común
            })
            .max_by(|a, b| a.fitness.partial_cmp(&b.fitness).unwrap_or(std::cmp::Ordering::Equal))
    }

    // ─── Utilidades ───

    fn starts_with_any(input: &str, prefixes: &[&str]) -> bool {
        prefixes.iter().any(|p| input.starts_with(p))
    }

    fn contains_any(input: &str, keywords: &[&str]) -> bool {
        keywords.iter().any(|k| input.contains(k))
    }

    fn try_parse_dimension(token: &str) -> Option<f64> {
        // "80cm" → 0.80, "1.5m" → 1.5, "120cm" → 1.2
        let token = token.trim();
        if token.ends_with("cm") {
            token.trim_end_matches("cm").parse::<f64>().ok().map(|v| v / 100.0)
        } else if token.ends_with("m") {
            token.trim_end_matches("m").parse::<f64>().ok()
        } else if token.ends_with("metros") {
            token.trim_end_matches("metros").trim().parse::<f64>().ok()
        } else {
            None
        }
    }

    /// Obtener memoria conversacional
    pub fn memory(&self) -> &[ConversationTurn] {
        &self.conversation_memory
    }

    /// Obtener objeto activo
    pub fn active_object(&self) -> Option<&String> {
        self.active_object.as_ref()
    }
}
