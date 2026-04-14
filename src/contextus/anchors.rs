use crate::contextus::memory::*;

/// Extrae anclas semánticas de documentos y texto
pub struct AnchorExtractor;

impl AnchorExtractor {
    /// Extraer anclas de un documento completo
    pub fn extract_from_document(
        filename: &str,
        content: &str,
    ) -> Vec<SemanticAnchor> {
        let mut anchors = Vec::new();

        // Patrones de definición
        let definition_patterns = [
            // "X es un/una Y"
            (r"(?i)(\w+(?:\s+\w+)*)\s+es\s+(?:un|una|el|la)\s+(\w+(?:\s+\w+)*)", "definition"),
            // "X sirve para Y"
            (r"(?i)(\w+(?:\s+\w+)*)\s+(?:sirve para|se usa para|permite)\s+(.+?)[\.\n]", "purpose"),
            // "X es un modelo de Y"
            (r"(?i)(\w+(?:\s+\w+)*)\s+es\s+(?:un|una)\s+(?:modelo|plataforma|herramienta|motor|sistema)\s+de\s+(.+?)[\.\n]", "tool"),
            // "X es una localidad/pueblo/ciudad"
            (r"(?i)(\w+(?:\s+\w+)*)\s+es\s+(?:una localidad|un pueblo|una ciudad|un municipio)\s+(.+?)[\.\n]", "location"),
        ];

        for (pattern, pattern_type) in &definition_patterns {
            let regex = regex::Regex::new(pattern).unwrap();
            for capture in regex.captures_iter(content) {
                let term = capture.get(1)
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_default();

                let definition = capture.get(2)
                    .map(|m| m.as_str().to_string())
                    .unwrap_or_default();

                if term.is_empty() || definition.is_empty() {
                    continue;
                }

                let categories = Self::infer_categories(&term, &definition, pattern_type);

                let anchor = SemanticAnchor {
                    term: term.clone(),
                    categories,
                    context_source: AnchorSource::Document {
                        filename: filename.to_string(),
                    },
                    confidence: 0.8,
                    created_at: std::time::SystemTime::now()
                        .duration_since(std::time::UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                    times_validated: 0,
                };

                // No duplicar
                if !anchors.iter().any(|a: &SemanticAnchor| a.term == term) {
                    anchors.push(anchor);
                }
            }
        }

        // Detectar menciones de herramientas/tecnologías conocidas
        let tech_patterns = [
            ("Suno", vec!["audio", "música", "IA", "generación"]),
            ("Arco", vec!["arquitectura", "forma", "estructura"]), // Se refinará
            ("Arco y flecha", vec!["arma", "proyectil", "tiro"]),
            ("Unreal", vec!["motor", "gráficos", "3D", "juegos"]),
            ("Rust", vec!["programación", "lenguaje", "sistemas"]),
            ("Blender", vec!["3D", "modelado", "animación"]),
        ];

        for (term, default_cats) in &tech_patterns {
            if content.to_lowercase().contains(&term.to_lowercase()) {
                // Inferir categoría del contexto circundante
                let context_cats = Self::infer_from_surrounding_context(term, content);
                let categories = if context_cats.is_empty() {
                    default_cats.iter().map(|&s| s.to_string()).collect()
                } else {
                    context_cats
                };

                if !anchors.iter().any(|a: &SemanticAnchor| a.term == *term) {
                    anchors.push(SemanticAnchor {
                        term: term.to_string(),
                        categories,
                        context_source: AnchorSource::Document {
                            filename: filename.to_string(),
                        },
                        confidence: 0.9,
                        created_at: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap()
                            .as_secs(),
                        times_validated: 0,
                    });
                }
            }
        }

        println!("[ANCHOR EXTRACTOR] Extraídas {} anclas de {}", anchors.len(), filename);
        for anchor in &anchors {
            println!("  • {} → [{}]", anchor.term, anchor.categories.join(", "));
        }

        anchors
    }

    /// Inferir categorías de una definición
    fn infer_categories(_term: &str, definition: &str, pattern_type: &str) -> Vec<String> {
        let mut categories = Vec::new();

        let def_lower = definition.to_lowercase();

        // Categorías por palabras clave en la definición
        let category_keywords = [
            ("música", "audio/música"),
            ("audio", "audio/música"),
            ("sonido", "audio/música"),
            ("canción", "audio/música"),
            ("generación", "IA/generación"),
            ("inteligencia artificial", "IA"),
            ("modelo", "IA/modelo"),
            ("pueblo", "geografía/localidad"),
            ("ciudad", "geografía/localidad"),
            ("localidad", "geografía/localidad"),
            ("italia", "geografía/localidad"),
            ("arquitectura", "arquitectura/estructura"),
            ("estructura", "arquitectura/estructura"),
            ("puente", "arquitectura/estructura"),
            ("forma", "geometría/forma"),
            ("curva", "geometría/forma"),
            ("arma", "armas/proyectil"),
            ("flecha", "armas/proyectil"),
            ("tiro", "armas/proyectil"),
            ("programación", "programación"),
            ("lenguaje", "programación/lenguaje"),
            ("física", "ciencia/física"),
            ("química", "ciencia/química"),
            ("matemáticas", "ciencia/matemáticas"),
        ];

        for (keyword, category) in &category_keywords {
            if def_lower.contains(keyword) {
                if !categories.contains(&category.to_string()) {
                    categories.push(category.to_string());
                }
            }
        }

        // Si no se encontró nada, usar el pattern_type
        if categories.is_empty() {
            categories.push(pattern_type.to_string());
        }

        categories
    }

    /// Inferir categoría del contexto circundante
    fn infer_from_surrounding_context(term: &str, content: &str) -> Vec<String> {
        let term_lower = term.to_lowercase();
        let content_lower = content.to_lowercase();

        // Buscar el término y analizar las ~50 palabras alrededor
        if let Some(pos) = content_lower.find(&term_lower) {
            let start = pos.saturating_sub(200);
            let end = (pos + term.len() + 200).min(content.len());
            let surrounding = &content_lower[start..end];

            return Self::infer_categories(term, surrounding, "context");
        }

        Vec::new()
    }

    /// Desambiguar un término basándose en anclas existentes
    pub fn disambiguate(
        term: &str,
        anchors: &std::collections::HashMap<String, SemanticAnchor>,
        context: &WorkingMemory,
    ) -> DisambiguationResult {
        let term_lower = term.to_lowercase();

        // 1. ¿Hay ancla directa?
        if let Some(anchor) = anchors.get(&term_lower) {
            return DisambiguationResult {
                term: term.to_string(),
                resolved_meaning: anchor.categories.join(", "),
                confidence: anchor.confidence,
                source: "anchor_direct".to_string(),
                alternatives: vec![],
            };
        }

        // 2. ¿Se mencionó en el contexto del hilo?
        if let Some(entity) = context.active_entities.iter()
            .find(|e| e.name.to_lowercase() == term_lower)
        {
            if !entity.categories.is_empty() {
                return DisambiguationResult {
                    term: term.to_string(),
                    resolved_meaning: entity.categories.join(", "),
                    confidence: 0.7,
                    source: "thread_context".to_string(),
                    alternatives: vec![],
                };
            }
        }

        // 3. ¿Está en documentos activos?
        for doc in &context.active_documents {
            if doc.extracted_entities.iter().any(|e| e.to_lowercase() == term_lower) {
                if let Some(anchor) = doc.extracted_anchors.iter()
                    .find(|a| a.term.to_lowercase() == term_lower)
                {
                    return DisambiguationResult {
                        term: term.to_string(),
                        resolved_meaning: anchor.categories.join(", "),
                        confidence: anchor.confidence,
                        source: "active_document".to_string(),
                        alternatives: vec![],
                    };
                }
            }
        }

        // 4. No se puede desambiguar con información local
        DisambiguationResult {
            term: term.to_string(),
            resolved_meaning: "desconocido".to_string(),
            confidence: 0.0,
            source: "none".to_string(),
            alternatives: vec![],
        }
    }
}

#[derive(Debug, Clone)]
pub struct DisambiguationResult {
    pub term: String,
    pub resolved_meaning: String,
    pub confidence: f64,
    pub source: String,
    pub alternatives: Vec<String>,
}
