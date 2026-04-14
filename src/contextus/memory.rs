use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};

/// Memoria de trabajo: lo que Daithon tiene "en mente" ahora
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkingMemory {
    /// Entidades activamente en conversación
    pub active_entities: Vec<ActiveEntity>,
    
    /// Anclas semánticas (aprendidas de documentos o conversación)
    pub semantic_anchors: HashMap<String, SemanticAnchor>,
    
    /// Historial del hilo actual
    pub thread_history: VecDeque<ThreadMessage>,
    
    /// Documentos activos cargados por el usuario
    pub active_documents: Vec<ActiveDocument>,
    
    /// Tema predominante del hilo
    pub thread_topic: Option<String>,
    
    /// Máximo mensajes a recordar
    pub max_history: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveEntity {
    pub name: String,
    pub categories: Vec<String>,
    pub first_mentioned_at: usize, // índice en thread_history
    pub mention_count: usize,
    pub last_mentioned_at: usize,
    pub attributes: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticAnchor {
    pub term: String,
    pub categories: Vec<String>,
    pub context_source: AnchorSource,
    pub confidence: f64,
    pub created_at: u64,
    pub times_validated: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AnchorSource {
    Document { filename: String },
    Conversation { message_index: usize },
    WebLearning { url: String },
    UserExplicit { user_statement: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreadMessage {
    pub index: usize,
    pub role: MessageRole,
    pub content: String,
    pub entities_mentioned: Vec<String>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessageRole {
    User,
    Daithon,
    System,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveDocument {
    pub filename: String,
    pub content_summary: String,
    pub extracted_entities: Vec<String>,
    pub extracted_anchors: Vec<SemanticAnchor>,
    pub loaded_at: u64,
}

impl WorkingMemory {
    pub fn new() -> Self {
        Self {
            active_entities: Vec::new(),
            semantic_anchors: HashMap::new(),
            thread_history: VecDeque::with_capacity(100),
            active_documents: Vec::new(),
            thread_topic: None,
            max_history: 100,
        }
    }

    /// Añadir mensaje del usuario
    pub fn add_user_message(&mut self, content: &str) {
        let entities = self.extract_entities(content);
        let msg = ThreadMessage {
            index: self.thread_history.len(),
            role: MessageRole::User,
            content: content.to_string(),
            entities_mentioned: entities.clone(),
            timestamp: Self::current_timestamp(),
        };

        // Actualizar entidades activas
        for entity_name in &entities {
            self.update_active_entity(entity_name, msg.index);
        }

        // Actualizar tema del hilo
        self.update_thread_topic(content);

        self.thread_history.push_back(msg);

        // Limitar historial
        while self.thread_history.len() > self.max_history {
            self.thread_history.pop_front();
        }
    }

    /// Añadir respuesta de Daithon
    pub fn add_daithon_message(&mut self, content: &str) {
        let entities = self.extract_entities(content);
        let msg = ThreadMessage {
            index: self.thread_history.len(),
            role: MessageRole::Daithon,
            content: content.to_string(),
            entities_mentioned: entities,
            timestamp: Self::current_timestamp(),
        };

        self.thread_history.push_back(msg);

        while self.thread_history.len() > self.max_history {
            self.thread_history.pop_front();
        }
    }

    /// Registrar un documento cargado
    pub fn register_document(
        &mut self,
        filename: &str,
        content: &str,
        extracted_anchors: Vec<SemanticAnchor>,
    ) {
        let entities: Vec<String> = extracted_anchors.iter()
            .map(|a| a.term.clone())
            .collect();

        let summary = if content.len() > 200 {
            format!("{}...", &content[..200])
        } else {
            content.to_string()
        };

        // Registrar anclas del documento
        for anchor in &extracted_anchors {
            self.semantic_anchors.insert(
                anchor.term.clone(),
                anchor.clone(),
            );
        }

        // Registrar entidades
        for entity_name in &entities {
            self.update_active_entity(entity_name, self.thread_history.len());
        }

        self.active_documents.push(ActiveDocument {
            filename: filename.to_string(),
            content_summary: summary,
            extracted_entities: entities,
            extracted_anchors,
            loaded_at: Self::current_timestamp(),
        });

        println!("[WORKING MEMORY] Documento registrado: {}", filename);
        println!("  Anclas creadas: {}", self.semantic_anchors.len());
        println!("  Entidades: {}", self.active_entities.len());
    }

    /// Crear ancla semántica manualmente
    pub fn create_anchor(
        &mut self,
        term: &str,
        categories: Vec<String>,
        source: AnchorSource,
        confidence: f64,
    ) {
        let anchor = SemanticAnchor {
            term: term.to_string(),
            categories,
            context_source: source,
            confidence,
            created_at: Self::current_timestamp(),
            times_validated: 0,
        };

        self.semantic_anchors.insert(term.to_lowercase(), anchor);
    }

    /// Pre-check: ¿Tenemos ancla para este término?
    pub fn check_anchor(&self, term: &str) -> Option<&SemanticAnchor> {
        self.semantic_anchors.get(&term.to_lowercase())
    }

    /// Obtener pistas de búsqueda basadas en contexto
    pub fn get_search_hints(&self) -> String {
        let mut hints = Vec::new();

        if let Some(topic) = &self.thread_topic {
            hints.push(format!("Tema del hilo: {}", topic));
        }

        if !self.active_entities.is_empty() {
            let entity_names: Vec<String> = self.active_entities.iter()
                .take(5)
                .map(|e| e.name.clone())
                .collect();
            hints.push(format!("Entidades activas: {}", entity_names.join(", ")));
        }

        let active_categories: Vec<String> = self.active_entities.iter()
            .flat_map(|e| e.categories.iter())
            .cloned()
            .collect();

        if !active_categories.is_empty() {
            hints.push(format!("Categorías relevantes: {}", active_categories.join(", ")));
        }

        if !self.active_documents.is_empty() {
            let doc_names: Vec<String> = self.active_documents.iter()
                .map(|d| d.filename.clone())
                .collect();
            hints.push(format!("Documentos activos: {}", doc_names.join(", ")));
        }

        hints.join(" | ")
    }

    /// Obtener contexto de los últimos N mensajes
    pub fn get_recent_context(&self, n: usize) -> Vec<&ThreadMessage> {
        self.thread_history.iter().rev().take(n).collect()
    }

    /// Verificar si un término fue mencionado recientemente
    pub fn was_mentioned_recently(&self, term: &str, last_n: usize) -> bool {
        self.thread_history.iter().rev()
            .take(last_n)
            .any(|msg| msg.content.to_lowercase().contains(&term.to_lowercase()))
    }

    /// Obtener el contexto en el que se mencionó un término
    pub fn get_mention_context(&self, term: &str) -> Option<String> {
        let term_lower = term.to_lowercase();

        for msg in self.thread_history.iter().rev() {
            if msg.content.to_lowercase().contains(&term_lower) {
                // Buscar mensajes adyacentes
                let idx = msg.index;
                let context_msgs: Vec<String> = self.thread_history.iter()
                    .filter(|m| m.index >= idx.saturating_sub(2) && m.index <= idx + 2)
                    .map(|m| format!("{:?}: {}", m.role, m.content))
                    .collect();

                return Some(context_msgs.join("\n"));
            }
        }

        None
    }

    // ─── Métodos privados ───

    pub fn extract_entities(&self, text: &str) -> Vec<String> {
        // Extraer palabras que coincidan con entidades conocidas
        let mut entities = Vec::new();

        // Buscar entidades activas en el texto
        for entity in &self.active_entities {
            if text.to_lowercase().contains(&entity.name.to_lowercase()) {
                entities.push(entity.name.clone());
            }
        }

        // Buscar términos con anclas
        for (term, _) in &self.semantic_anchors {
            if text.to_lowercase().contains(term) {
                if !entities.contains(term) {
                    entities.push(term.clone());
                }
            }
        }

        // Buscar palabras capitalizadas (posibles nombres propios)
        let words: Vec<&str> = text.split_whitespace().collect();
        for word in words {
            let clean_word = word.trim_matches(|c: char| !c.is_alphabetic());
            if clean_word.chars().next().map(|c| c.is_uppercase()).unwrap_or(false) 
                && clean_word.len() > 2
                && !entities.contains(&clean_word.to_string())
            {
                entities.push(clean_word.to_string());
            }
        }

        entities
    }

    fn update_active_entity(&mut self, name: &str, message_index: usize) {
        if let Some(entity) = self.active_entities.iter_mut()
            .find(|e| e.name.to_lowercase() == name.to_lowercase())
        {
            entity.mention_count += 1;
            entity.last_mentioned_at = message_index;
        } else {
            // Nueva entidad: inferir categoría de anclas si existe
            let categories = self.semantic_anchors
                .get(&name.to_lowercase())
                .map(|a| a.categories.clone())
                .unwrap_or_default();

            self.active_entities.push(ActiveEntity {
                name: name.to_string(),
                categories,
                first_mentioned_at: message_index,
                mention_count: 1,
                last_mentioned_at: message_index,
                attributes: HashMap::new(),
            });
        }
    }

    fn update_thread_topic(&mut self, content: &str) {
        // Simple: el tema es la entidad más mencionada en los últimos mensajes
        let mut entity_counts: HashMap<String, usize> = HashMap::new();

        for msg in self.thread_history.iter().rev().take(10) {
            for entity in &msg.entities_mentioned {
                *entity_counts.entry(entity.clone()).or_insert(0) += 1;
            }
        }

        // Añadir las del mensaje actual
        let current_entities = self.extract_entities(content);
        for entity in &current_entities {
            *entity_counts.entry(entity.clone()).or_insert(0) += 1;
        }

        self.thread_topic = entity_counts.into_iter()
            .max_by_key(|(_, count)| *count)
            .map(|(entity, _)| entity);
    }

    pub fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }

    /// Obtener resumen del estado actual
    pub fn get_state_summary(&self) -> String {
        let mut summary = String::new();

        summary.push_str("╔════════════════════════════════════════╗\n");
        summary.push_str("║       WORKING MEMORY STATE             ║\n");
        summary.push_str("╠════════════════════════════════════════╣\n");

        if let Some(topic) = &self.thread_topic {
            summary.push_str(&format!("║ Tema: {}\n", topic));
        }

        summary.push_str(&format!("║ Mensajes: {}\n", self.thread_history.len()));
        summary.push_str(&format!("║ Entidades: {}\n", self.active_entities.len()));
        summary.push_str(&format!("║ Anclas: {}\n", self.semantic_anchors.len()));
        summary.push_str(&format!("║ Documentos: {}\n", self.active_documents.len()));

        if !self.active_entities.is_empty() {
            summary.push_str("╠════════════════════════════════════════╣\n");
            summary.push_str("║ Entidades activas:\n");
            for entity in self.active_entities.iter().take(5) {
                let cats = if entity.categories.is_empty() {
                    "sin categoría".to_string()
                } else {
                    entity.categories.join(",")
                };
                summary.push_str(&format!(
                    "║  • {} [{}] (×{})\n",
                    entity.name, cats, entity.mention_count
                ));
            }
        }

        if !self.semantic_anchors.is_empty() {
            summary.push_str("╠════════════════════════════════════════╣\n");
            summary.push_str("║ Anclas semánticas:\n");
            for (term, anchor) in self.semantic_anchors.iter().take(5) {
                summary.push_str(&format!(
                    "║  • {} → {} ({:.0}%)\n",
                    term,
                    anchor.categories.join(","),
                    anchor.confidence * 100.0
                ));
            }
        }

        summary.push_str("╚════════════════════════════════════════╝\n");

        summary
    }
}
