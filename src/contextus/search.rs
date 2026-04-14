use crate::contextus::memory::*;
use crate::contextus::anchors::*;

/// Orquestador de búsqueda con pirámide de prioridad
pub struct SearchOrchestrator {
    /// Memoria de trabajo
    pub working_memory: WorkingMemory,
}

impl SearchOrchestrator {
    pub fn new(working_memory: WorkingMemory) -> Self {
        Self { working_memory }
    }

    /// Búsqueda principal: sigue la pirámide de prioridad
    pub async fn search(&mut self, query: &str) -> SearchResult {
        // Extraer término principal de la query
        let main_term = self.extract_main_term(query);
        println!("[SEARCH] Buscando: '{}' (término principal: '{}')", query, main_term);

        // ─── PRIORIDAD ERUDITA: Términos Compuestos ───
        if main_term.contains(' ') {
             println!("  [Prioridad] Término compuesto detectado. Consultando APIs externas primero...");
             let raw_results = self.fetch_from_apis(&main_term).await;
             if !raw_results.is_empty() {
                 println!("  ✓ Éxito en búsqueda de precisión para '{}'", main_term);
                 return SearchResult {
                     query: query.to_string(),
                     answer: raw_results[0].content.clone(),
                     source: SearchSource::Wikipedia,
                     confidence: 0.9,
                     disambiguation: None,
                 };
             }
        }

        // ─── CAPA 1: Contexto del Hilo ───
        println!("  [Capa 1] Revisando contexto del hilo...");
        
        if self.working_memory.was_mentioned_recently(&main_term, 10) {
            if let Some(context) = self.working_memory.get_mention_context(&main_term) {
                println!("  ✓ Encontrado en contexto del hilo");
                return SearchResult {
                    query: query.to_string(),
                    answer: self.synthesize_from_context(&main_term, &context),
                    source: SearchSource::ThreadContext,
                    confidence: 0.9,
                    disambiguation: Some(AnchorExtractor::disambiguate(
                        &main_term,
                        &self.working_memory.semantic_anchors,
                        &self.working_memory,
                    )),
                };
            }
        }

        // ─── CAPA 2: Documentos Activos ───
        println!("  [Capa 2] Revisando documentos activos...");
        
        for doc in &self.working_memory.active_documents {
            if doc.extracted_entities.iter().any(|e| e.to_lowercase() == main_term.to_lowercase()) {
                println!("  ✓ Encontrado en documento: {}", doc.filename);
                
                if let Some(anchor) = self.working_memory.semantic_anchors.get(&main_term.to_lowercase()) {
                    return SearchResult {
                        query: query.to_string(),
                        answer: self.synthesize_from_anchor(anchor, &doc.content_summary),
                        source: SearchSource::ActiveDocument {
                            filename: doc.filename.clone(),
                        },
                        confidence: anchor.confidence,
                        disambiguation: Some(AnchorExtractor::disambiguate(
                            &main_term,
                            &self.working_memory.semantic_anchors,
                            &self.working_memory,
                        )),
                    };
                }
            }
        }

        // ─── CAPA 3: Memoria CORTEX ───
        println!("  [Capa 3] Revisando memoria de CORTEX...");
        
        if let Some(anchor) = self.working_memory.check_anchor(&main_term) {
            if anchor.confidence > 0.5 {
                println!("  ✓ Encontrado en anclas semánticas");
                return SearchResult {
                    query: query.to_string(),
                    answer: self.synthesize_from_anchor(anchor, ""),
                    source: SearchSource::CortexMemory,
                    confidence: anchor.confidence,
                    disambiguation: Some(AnchorExtractor::disambiguate(
                        &main_term,
                        &self.working_memory.semantic_anchors,
                        &self.working_memory,
                    )),
                };
            }
        }

        // ─── CAPA 4: APIs Externas ───
        println!("  [Capa 4] Consultando APIs externas...");
        
        let raw_results = self.fetch_from_apis(&main_term).await;
        
        if raw_results.is_empty() {
            return SearchResult {
                query: query.to_string(),
                answer: format!("No encontré información sobre '{}'.", main_term),
                source: SearchSource::None,
                confidence: 0.0,
                disambiguation: None,
            };
        }

        // Re-Ranking: filtrar por relevancia con contexto
        let ranked = self.rerank(raw_results, &main_term);

        println!("  ✓ Mejor resultado (relevancia: {:.0}%)", ranked[0].relevance * 100.0);

        SearchResult {
            query: query.to_string(),
            answer: ranked[0].content.clone(),
            source: SearchSource::ExternalAPI {
                url: ranked[0].source_url.clone(),
            },
            confidence: ranked[0].relevance,
            disambiguation: Some(AnchorExtractor::disambiguate(
                &main_term,
                &self.working_memory.semantic_anchors,
                &self.working_memory,
            )),
        }
    }

    /// Re-Ranking: comparar resultados con contexto actual
    fn rerank(&self, results: Vec<RawSearchResult>, term: &str) -> Vec<RankedResult> {
        let mut ranked: Vec<RankedResult> = results.into_iter().map(|r| {
            let relevance = self.calculate_relevance(&r, term);
            RankedResult {
                content: r.content,
                source_url: r.source_url,
                title: r.title,
                relevance,
            }
        }).collect();

        ranked.sort_by(|a, b| b.relevance.partial_cmp(&a.relevance).unwrap());
        ranked
    }

    /// Calcular relevancia de un resultado respecto al contexto
    fn calculate_relevance(&self, result: &RawSearchResult, term: &str) -> f64 {
        let mut score: f64 = 0.5; // Base

        let content_lower = result.content.to_lowercase();
        let title_lower = result.title.to_lowercase();

        // Bonus por ancla semántica
        if let Some(anchor) = self.working_memory.check_anchor(term) {
            for category in &anchor.categories {
                let cat_lower = category.to_lowercase();
                if content_lower.contains(&cat_lower) || title_lower.contains(&cat_lower) {
                    score += 0.3;
                }
            }
        }

        // Bonus por categorías de entidades activas
        for entity in &self.working_memory.active_entities {
            for category in &entity.categories {
                let cat_lower = category.to_lowercase();
                if content_lower.contains(&cat_lower) {
                    score += 0.1;
                }
            }
        }

        // Penalización por categorías contradictorias
        let contradiction_patterns = [
            ("música", "localidad"),
            ("audio", "pueblo"),
            ("IA", "italia"),
            ("arma", "arquitectura"),
            ("estructura", "flecha"),
        ];

        for (context_cat, contradicts) in &contradiction_patterns {
            // Si el contexto dice "música" y el resultado dice "pueblo"
            let has_context = self.working_memory.active_entities.iter()
                .any(|e| e.categories.iter().any(|c| c.to_lowercase().contains(context_cat)));

            let has_contradiction = content_lower.contains(contradicts);

            if has_context && has_contradiction {
                score -= 0.4;
            }
        }

        // Bonus por coincidencia exacta del término
        if title_lower.contains(&term.to_lowercase()) {
            score += 0.2;
        }

        score.clamp(0.0, 1.0)
    }

    async fn fetch_from_apis(&self, term: &str) -> Vec<RawSearchResult> {
        let mut results = Vec::new();

        // Wikipedia
        match self.fetch_wikipedia(term).await {
            Ok(wiki_results) => results.extend(wiki_results),
            Err(e) => println!("  [ERROR API] Fallo al consultar Wikipedia: {}", e),
        }

        // Si hay contexto, refinar búsqueda
        let hints = self.working_memory.get_search_hints();
        if !hints.is_empty() {
            let refined_query = format!("{} {}", term, hints.split('|').next().unwrap_or(""));
            if let Ok(more_results) = self.fetch_wikipedia(&refined_query).await {
                results.extend(more_results);
            }
        }

        results
    }

    pub async fn fetch_wikipedia_full(&self, title: &str) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        // Usamos la API de consulta por títulos con redirecciones automáticas
        let url = format!(
            "https://es.wikipedia.org/w/api.php?action=query&prop=extracts&explaintext=1&redirects=1&titles={}&format=json",
            urlencoding::encode(title)
        );

        let client = reqwest::Client::new();
        let response_text = client.get(&url)
            .header("User-Agent", "DaithonEngine/1.0 (josep; daithon_project)")
            .send()
            .await?
            .text()
            .await?;

        let json: serde_json::Value = serde_json::from_str(&response_text)?;

        if let Some(pages) = json["query"]["pages"].as_object() {
            for (page_id, page) in pages {
                if page_id == "-1" { continue; } // Página no encontrada
                
                if let Some(extract) = page["extract"].as_str() {
                    let content = extract.to_string();
                    if content.len() > 10 {
                        return Ok(content);
                    }
                }
            }
        }
        
        Err(format!("Matriz vacía para '{}'. JSON: {}", title, response_text).into())
    }

    pub async fn fetch_wikipedia(&self, term: &str) -> Result<Vec<RawSearchResult>, Box<dyn std::error::Error + Send + Sync>> {
        let url = format!(
            "https://es.wikipedia.org/w/api.php?action=opensearch&search={}&limit=5&format=json",
            urlencoding::encode(term)
        );

        let client = reqwest::Client::new();
        let response_text = client.get(&url)
            .header("User-Agent", "DaithonEngine/1.0 (josep; daithon_project)")
            .send()
            .await?
            .text()
            .await?;
            
        let json: serde_json::Value = serde_json::from_str(&response_text)?;

        let mut results = Vec::new();

        if let Some(titles) = json.get(1).and_then(|v| v.as_array()) {
            if let Some(descriptions) = json.get(2).and_then(|v| v.as_array()) {
                if let Some(urls) = json.get(3).and_then(|v| v.as_array()) {
                    for (i, title_val) in titles.iter().enumerate() {
                        let title = title_val.as_str().unwrap_or("").to_string();
                        let mut description = descriptions.get(i)
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();
                            
                        let url = urls.get(i)
                            .and_then(|v| v.as_str())
                            .unwrap_or("")
                            .to_string();

                        // --- EXTRACCIÓN PROFUNDA (Sincronía Senku) ---
                        if description.trim().is_empty() || description.contains("Registro matriz") {
                            if let Ok(full_text) = self.fetch_wikipedia_full(&title).await {
                                description = full_text;
                            } else {
                                description = format!("Registro matriz localizado bajo la nomenclatura '{}'. Acceso limitado al nodo principal.", title);
                            }
                        }

                        results.push(RawSearchResult {
                            title,
                            content: description,
                            source_url: url,
                        });
                    }
                }
            }
        }

        Ok(results)
    }

    fn extract_main_term(&self, query: &str) -> String {
        let stop_words: std::collections::HashSet<&str> = [
            "qué", "que", "es", "un", "una", "el", "la", "los", "las", "de", "del", 
            "cuentame", "todo", "sabes", "quiero", "ver", "busca", "fondo", "acerca", "cuenta", "punto", "vista", "genera", "reporte", "escribe", "haz", "dime", "explica", "hola", "oye", "daithon", "joseph", "sobre", "entre", "también", "algún", "alguno", "está", "este", "esta",
            "necesito", "expliques", "háblame", "tema", "me", "con", "tu", "tus", "su", "sus", "mi", "mis", "son", "un", "una", "unos", "unas", "real", "voz", "unreal", "fondo", "fondo", "quiero", "quisiera", "podrías", "puedes", "explicar", "decir", "hablar", "sobre", "acerca"
        ].into_iter().collect();

        let raw_words: Vec<String> = query.to_lowercase()
            .split(|c: char| !c.is_alphabetic())
            .filter(|w| !stop_words.contains(w) && w.len() > 1)
            .map(|w| w.to_string())
            .collect();

        if raw_words.is_empty() { return query.to_string(); }

        // 1. Prioridad: Bigramas con anclas
        for i in 0..raw_words.len().saturating_sub(1) {
            let bigram = format!("{} {}", raw_words[i], raw_words[i+1]);
            if self.working_memory.semantic_anchors.contains_key(&bigram) {
                return bigram;
            }
        }

        // 2. Bigramas inteligentes por defecto
        if raw_words.len() >= 2 {
            if raw_words[0] == "teoría" || raw_words[0] == "ley" || raw_words[0] == "mecánica" {
                return format!("{} de {}", raw_words[0], raw_words[1]);
            }
            return format!("{} {}", raw_words[0], raw_words[1]);
        }

        // 3. Fallback: Palabra más relevante con penalización de genéricos
        let mut final_candidates = raw_words.clone();
        final_candidates.sort_by_key(|w| {
            let mut score = w.len() as i32;
            if w == "teoría" || w == "sistema" || w == "relación" {
                score -= 3;
            }
            std::cmp::Reverse(score)
        });

        final_candidates[0].clone()
    }

    fn synthesize_from_context(&self, term: &str, context: &str) -> String {
        format!("Basándome en nuestra conversación, {} se refiere a lo que estábamos hablando: {}", term, context)
    }

    fn synthesize_from_anchor(&self, anchor: &SemanticAnchor, extra: &str) -> String {
        let mut answer = format!("{} está relacionado con {}.", anchor.term, anchor.categories.join(", "));

        if !extra.is_empty() {
            answer.push_str(&format!(" Específicamente: {}", extra));
        }

        match &anchor.context_source {
            AnchorSource::Document { filename } => {
                answer.push_str(&format!(" (Aprendido del documento: {})", filename));
            }
            AnchorSource::Conversation { .. } => {
                answer.push_str(" (Aprendido de nuestra conversación)");
            }
            _ => {}
        }

        answer
    }
}

#[derive(Debug, Clone)]
pub struct RawSearchResult {
    pub title: String,
    pub content: String,
    pub source_url: String,
}

#[derive(Debug, Clone)]
pub struct RankedResult {
    pub title: String,
    pub content: String,
    pub source_url: String,
    pub relevance: f64,
}

#[derive(Debug, Clone)]
pub struct SearchResult {
    pub query: String,
    pub answer: String,
    pub source: SearchSource,
    pub confidence: f64,
    pub disambiguation: Option<DisambiguationResult>,
}

#[derive(Debug, Clone)]
pub enum SearchSource {
    None,
    ThreadContext,
    ActiveDocument { filename: String },
    CortexMemory,
    FlashMemory,
    Wikipedia,
    ExternalAPI { url: String },
}
