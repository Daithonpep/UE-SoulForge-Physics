// ═══════════════════════════════════════════════════════════════
//  AUTONOMOUS — Sistema de Aprendizaje Autónomo de Daithon
//  
//  Ciclo: Curiosidad → Investigación (Wikipedia/Wiktionary) →
//         Discusión TRINITY → Persistencia → Siguiente Tema
// ═══════════════════════════════════════════════════════════════

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet, VecDeque};

/// Helper para normalización de conceptos (quitar plurales básicos y minúsculas)
pub fn normalize_concept(word: &str) -> String {
    let lower = word.to_lowercase().trim().to_string();
    if lower.len() > 3 && lower.ends_with('s') {
        lower[..lower.len()-1].to_string()
    } else {
        lower
    }
}

/// Flag global para detener el loop autónomo desde el dashboard
pub static AUTONOMOUS_RUNNING: AtomicBool = AtomicBool::new(false);

// ────────────────────────────────────────────────────────────────
//  ESTRUCTURAS DE DATOS
// ────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExplorationRecord {
    pub topic: String,
    pub words_learned: Vec<String>,
    pub definitions_found: usize,
    pub trinity_discussions: usize,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutonomousStats {
    pub cycles_completed: usize,
    pub total_words_learned: usize,
    pub total_topics_explored: usize,
    pub topics_explored: Vec<String>,
    pub running: bool,
    pub current_topic: String,
}

/// Cola de temas pendientes por explorar
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TopicQueue {
    pub pending: VecDeque<String>,
    pub explored: HashMap<String, u32>,
}

impl TopicQueue {
    pub fn new() -> Self {
        Self {
            pending: VecDeque::new(),
            explored: HashMap::new(),
        }
    }

    /// Añadir tema solo si no ha superado el límite de refuerzos (3) ni está en cola
    pub fn enqueue(&mut self, topic: &str) {
        let normalized = topic.to_lowercase().trim().to_string();
        if !normalized.is_empty() 
            && self.explored.get(&normalized).copied().unwrap_or(0) < 3 
            && !self.pending.contains(&normalized) 
        {
            self.pending.push_back(normalized);
        }
    }

    /// Obtener próximo tema a explorar
    pub fn dequeue(&mut self) -> Option<String> {
        while let Some(topic) = self.pending.pop_front() {
            if self.explored.get(&topic).copied().unwrap_or(0) < 3 {
                return Some(topic);
            }
        }
        None
    }

    /// Marcar tema como explorado (incrementa contador para refuerzo)
    pub fn mark_explored(&mut self, topic: &str) {
        *self.explored.entry(topic.to_lowercase().trim().to_string()).or_insert(0) += 1;
    }

    /// Persistir la cola a disco para no repetir temas al reiniciar
    pub fn save(&self) -> Result<(), String> {
        let _ = std::fs::create_dir_all("autonomous_data");
        let json = serde_json::to_string_pretty(self)
            .map_err(|e| format!("Serialize error: {}", e))?;
        std::fs::write("autonomous_data/topic_queue.json", json)
            .map_err(|e| format!("Write error: {}", e))?;
        Ok(())
    }

    /// Cargar cola desde disco
    pub fn load() -> Self {
        if let Ok(content) = std::fs::read_to_string("autonomous_data/topic_queue.json") {
            if let Ok(queue) = serde_json::from_str::<TopicQueue>(&content) {
                log::info!("[AUTONOMOUS] Cola restaurada: {} pendientes, {} explorados", 
                    queue.pending.len(), queue.explored.len());
                return queue;
            }
        }
        Self::new()
    }
}

// ────────────────────────────────────────────────────────────────
//  SEMILLAS TEMÁTICAS INICIALES
// ────────────────────────────────────────────────────────────────

fn seed_topics() -> Vec<&'static str> {
    vec![
        // Ciencias fundamentales
        "aritmética", "álgebra", "geometría", "física", "química", "biología",
        // Arquitectura y diseño (dominio core de Daithon)
        "arquitectura", "diseño estructural", "columna", "arco", "bóveda", "cúpula",
        "puente", "viga", "cimentación",
        // Materiales
        "madera", "acero", "concreto", "piedra", "vidrio", "cerámica",
        // Lenguaje y cultura
        "gramática española", "literatura", "poesía", "modismos guatemaltecos",
        // Tecnología
        "programación", "inteligencia artificial", "robótica",
        // Filosofía y lógica
        "lógica", "filosofía", "ética", "epistemología",
        // Naturaleza
        "ecología", "geología", "astronomía", "oceanografía",
        // Arte
        "escultura", "pintura", "música", "teatro",
    ]
}

// ────────────────────────────────────────────────────────────────
//  INVESTIGADOR AUTÓNOMO (Wikipedia API)
// ────────────────────────────────────────────────────────────────

/// Buscar resumen de Wikipedia para extraer conceptos relacionados
pub async fn fetch_wikipedia_summary(topic: &str) -> Result<(String, Vec<String>), String> {
    // Wikipedia REST API usa underscores, no %20
    let wiki_title = topic.replace(' ', "_");
    let url = format!(
        "https://es.wikipedia.org/api/rest_v1/page/summary/{}",
        wiki_title
    );

    let client = reqwest::Client::new();
    let response = client.get(&url)
        .header("User-Agent", "DaithonResearchBot/1.0 (daithon@example.com)")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    
    // Si no encontramos en es.wikipedia, intentar búsqueda
    if !response.status().is_success() {
        return fetch_wikipedia_search(topic).await;
    }

    let json: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;

    let extract = json.get("extract")
        .and_then(|e| e.as_str())
        .unwrap_or("")
        .to_string();

    if extract.is_empty() || extract.len() < 20 {
        // Intentar búsqueda como fallback
        return fetch_wikipedia_search(topic).await;
    }

    // Extraer palabras clave del resumen como temas derivados
    let derived_topics: Vec<String> = extract
        .split(|c: char| !c.is_alphabetic() && c != 'á' && c != 'é' && c != 'í' && c != 'ó' && c != 'ú' && c != 'ñ')
        .filter(|w| w.len() > 5)
        .map(|w| w.to_lowercase())
        .collect::<HashSet<_>>()
        .into_iter()
        .take(5)
        .collect();

    Ok((extract, derived_topics))
}

/// Fallback: buscar tema en Wikipedia con la API de búsqueda
async fn fetch_wikipedia_search(topic: &str) -> Result<(String, Vec<String>), String> {
    let encoded = urlencoding::encode(topic);
    let url = format!(
        "https://es.wikipedia.org/w/api.php?action=query&list=search&srsearch={}&format=json&utf8=1&srlimit=1",
        encoded
    );

    let client = reqwest::Client::new();
    let response = client.get(&url)
        .header("User-Agent", "DaithonResearchBot/1.0 (daithon@example.com)")
        .send()
        .await
        .map_err(|e| e.to_string())?;
    let json: serde_json::Value = response.json().await.map_err(|e| e.to_string())?;

    // Extraer título del primer resultado de búsqueda
    let title = json.pointer("/query/search/0/title")
        .and_then(|t| t.as_str())
        .ok_or_else(|| format!("Sin resultados de búsqueda para '{}'", topic))?;

    let snippet = json.pointer("/query/search/0/snippet")
        .and_then(|s| s.as_str())
        .unwrap_or("");

    // Limpiar HTML del snippet
    let clean_text = snippet
        .replace("<span class=\"searchmatch\">", "")
        .replace("</span>", "")
        .replace("&quot;", "\"")
        .replace("&amp;", "&");

    if clean_text.len() < 20 {
        return Err(format!("Snippet muy corto para '{}'", topic));
    }

    let derived = vec![title.to_lowercase()];
    Ok((clean_text, derived))
}

/// Extraer palabras del resumen para inyectarlas al vocabulario
pub fn extract_learnable_words(text: &str) -> Vec<String> {
    let stopwords: HashSet<&str> = [
        "de", "la", "el", "en", "los", "las", "del", "una", "un", "que", "por",
        "con", "para", "como", "más", "pero", "sus", "les", "fue", "ser", "son",
        "este", "esta", "estos", "estas", "hay", "también", "entre", "desde",
        "sobre", "tiene", "puede", "cual", "cuando", "donde", "quien", "todo",
        "cada", "otro", "otra", "otros", "otras", "muy", "bien", "así", "sin",
        "embargo", "parte", "forma", "manera", "tipo", "gran", "mayor", "menor",
    ].iter().cloned().collect();

    text.split(|c: char| !c.is_alphabetic() && c != 'á' && c != 'é' && c != 'í' && c != 'ó' && c != 'ú' && c != 'ñ')
        .filter(|w| w.len() > 3 && !stopwords.contains(w.to_lowercase().as_str()))
        .map(|w| w.to_lowercase())
        .collect::<HashSet<_>>()
        .into_iter()
        .collect()
}

// ────────────────────────────────────────────────────────────────
//  LOOP AUTÓNOMO PRINCIPAL
// ────────────────────────────────────────────────────────────────

pub async fn start_autonomous_loop(
    lingua_engine: Arc<RwLock<crate::lingua::engine::LinguaEngine>>,
    trinity: Arc<RwLock<crate::trinity::training::triangular_loop::TriangularTrainingLoop>>,
) {
    AUTONOMOUS_RUNNING.store(true, Ordering::SeqCst);

    log::info!("╔════════════════════════════════════════════════════════════╗");
    log::info!("║     DAITHON — MODO AUTÓNOMO ACTIVADO                      ║");
    log::info!("║     Exploración libre del conocimiento...                 ║");
    log::info!("╚════════════════════════════════════════════════════════════╝");

    // Restaurar o crear cola de temas
    let mut topic_queue = TopicQueue::load();

    // Si la cola está vacía, sembrar temas iniciales
    if topic_queue.pending.is_empty() {
        for topic in seed_topics() {
            topic_queue.enqueue(topic);
        }
        log::info!("[AUTONOMOUS] {} temas semilla cargados", topic_queue.pending.len());
    }

    let mut cycle = 0usize;
    let mut total_words = 0usize;
    let mut current_topic = String::new();

    while AUTONOMOUS_RUNNING.load(Ordering::SeqCst) {
        cycle += 1;

        // Obtener próximo tema
        let topic = match topic_queue.dequeue() {
            Some(t) => t,
            None => {
                // Ola 2: temas más específicos y profundos
                let wave2 = vec![
                    "trigonometría", "termodinámica", "electromagnetismo", "óptica",
                    "genética", "botánica", "zoología", "paleontología",
                    "cartografía", "topografía", "sismología", "vulcanología",
                    "criptografía", "algoritmo", "compilador", "microprocesador",
                    "renacimiento", "barroco", "expresionismo", "surrealismo",
                    "gravitación", "relatividad", "mecánica cuántica",
                    "fotosíntesis", "mitocondria", "sistema nervioso",
                    "contrafuerte", "arbotante", "dovela", "clave",
                    "mortero", "hormigón", "ladrillo", "cantería",
                ];
                for t in wave2 {
                    topic_queue.enqueue(t);
                }
                continue;
            }
        };

        current_topic = topic.clone();

        log::info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        log::info!("  CICLO AUTÓNOMO #{} — Tema: '{}'", cycle, topic);
        log::info!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");

        // ═══════════════════════════════════════
        //  FASE 1: INVESTIGACIÓN WIKIPEDIA
        // ═══════════════════════════════════════
        log::info!("[FASE 1] Investigando '{}' en Wikipedia...", topic);

        let (summary, derived_topics) = match fetch_wikipedia_summary(&topic).await {
            Ok(result) => result,
            Err(e) => {
                log::warn!("[AUTONOMOUS] Sin resultados para '{}': {}", topic, e);
                topic_queue.mark_explored(&topic);
                tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
                continue;
            }
        };

        // Encolar temas derivados
        for dt in &derived_topics {
            topic_queue.enqueue(dt);
        }

        log::info!("  ✓ Resumen obtenido ({} chars)", summary.len());
        log::info!("  ✓ {} temas derivados encolados", derived_topics.len());

        // ═══════════════════════════════════════
        //  FASE 1.5: INVESTIGACIÓN ACADÉMICA (OPENALEX)
        // ═══════════════════════════════════════
        log::info!("[FASE 1.5] Buscando papers académicos sobre '{}' en OpenAlex...", topic);
        let openalex = crate::omni_inject::openalex::OpenAlexClient::new("cortex@daithon.ai");
        let mut openalex_words = Vec::new();

        match openalex.search_works(&topic, 5).await {
            Ok(works) => {
                log::info!("  ✓ {} papers encontrados.", works.len());
                for work in works {
                    if let Some(title) = &work.title {
                        log::info!("    * Paper: {}", title);
                        openalex_words.extend(extract_learnable_words(title));
                        
                        // Extraer conceptos ontológicos masivos y encolarlos
                        if let Some(concepts) = &work.concepts {
                            // Algoritmo de "Peso de Relevancia"
                            // Nivel >= 2 priorizado (Especializaciones puras)
                            let mut filtered: Vec<_> = concepts.iter().filter(|c| c.level.unwrap_or(0) >= 2 || c.score.unwrap_or(0.0) > 0.6).collect();
                            filtered.sort_by(|a, b| b.score.unwrap_or(0.0).partial_cmp(&a.score.unwrap_or(0.0)).unwrap_or(std::cmp::Ordering::Equal));
                            
                            for concept in filtered.into_iter().take(3) {
                                if let Some(name) = &concept.display_name {
                                    if name.to_lowercase() != topic.to_lowercase() {
                                        topic_queue.enqueue(name); // ¡Nutriendo el loop con conceptos de alto nivel!
                                    }
                                }
                            }
                        }

                        // Extracción de Constantes y Fórmulas
                        let constants = openalex.extract_action_variables(&work);
                        if !constants.is_empty() {
                            log::info!("    [!] Constante Asimilada: {} ({})", constants[0].name, constants[0].description.as_deref().unwrap_or(""));
                            // Push this into Trinity's direct discussion queue:
                            let mut trinity_lock = trinity.write().await;
                            trinity_lock.agent_b.learned_phrases.push(
                                crate::trinity::agents::conversational_agent::LearnedPhrase {
                                    phrase: format!("Asimilé {}", constants[0].name),
                                    context_tags: vec!["constante_dinámica".to_string(), constants[0].name.clone()],
                                    success_rate: 1.0,
                                    usage_count: 5,
                                }
                            );
                        }
                    }
                }
            }
            Err(e) => {
                log::warn!("  x Error en OpenAlex: {}", e);
            }
        }

        // ═══════════════════════════════════════
        //  FASE 1.6: ONTOLOGÍA WIKIDATA (RELACIONES)
        // ═══════════════════════════════════════
        log::info!("[FASE 1.6] Buscando relaciones en Wikidata para '{}'...", topic);
        let wikidata = crate::omni_inject::wikidata::WikidataClient::new();
        let mut api_words = Vec::new();
        
        match wikidata.search_concept(&topic).await {
            Ok(entities) => {
                for entity in entities {
                    if let Some(desc) = entity.description {
                        log::info!("    * WD Desc: {}", desc);
                        api_words.extend(extract_learnable_words(&desc));
                    }
                }
            }
            Err(e) => log::warn!("  x Error en Wikidata: {}", e),
        }

        // ═══════════════════════════════════════
        //  FASE 1.7: FILOSOFÍA Y CLÁSICOS (GUTENDEX)
        // ═══════════════════════════════════════
        log::info!("[FASE 1.7] Buscando en literatura clásica (Gutendex)...");
        let gutendex = crate::omni_inject::gutendex::GutendexClient::new();
        match gutendex.search_books(&topic).await {
            Ok(books) => {
                for book in books {
                    log::info!("    * Libro: {}", book.title);
                    api_words.extend(extract_learnable_words(&book.title));
                }
            }
            Err(e) => log::warn!("  x Error en Gutendex: {}", e),
        }

        // ═══════════════════════════════════════
        //  FASE 1.8: ASTROFÍSICA (NASA API)
        // ═══════════════════════════════════════
        log::info!("[FASE 1.8] Consultando repositorio físico de NASA...");
        let nasa = crate::omni_inject::nasa::NasaClient::new();
        match nasa.search_physics_data(&topic).await {
            Ok(items) => {
                for item in items {
                    log::info!("    * NASA Data: {}", item.title);
                    api_words.extend(extract_learnable_words(&item.title));
                    if let Some(desc) = item.description {
                        api_words.extend(extract_learnable_words(&desc));
                    }
                }
            }
            Err(e) => log::warn!("  x Error en NASA API: {}", e),
        }

        // ═══════════════════════════════════════
        //  FASE 2: INYECCIÓN DE VOCABULARIO Y CORTEX
        // ═══════════════════════════════════════
        log::info!("[FASE 2] Extrayendo e inyectando vocabulario acumulado...");

        let mut learnable = extract_learnable_words(&summary);
        learnable.extend(openalex_words);
        learnable.extend(api_words);
        let mut words_this_cycle = 0;

        {
            let mut engine = lingua_engine.write().await;
            let vocab = engine.acquisition.vocabulary().clone();

            for word in &learnable {
                if !vocab.contains_key(word) {
                    // Intentar buscar en Wiktionary para definición rica
                    match engine.acquisition.fetch_from_wiktionary(word).await {
                        Ok(_entry) => {
                            words_this_cycle += 1;
                            total_words += 1;
                        }
                        Err(_) => {
                            // Fallback: registrar palabra simple extraída del contexto
                            let simple_entry = crate::lingua::acquisition::AcquiredWord {
                                word: word.clone(),
                                language: "es".to_string(),
                                part_of_speech: vec!["noun".to_string()],
                                meanings: vec![format!("Concepto relacionado con {}", topic)],
                                examples: vec![],
                                synonyms: vec![],
                                antonyms: vec![],
                                related_terms: derived_topics.clone(),
                                gender: crate::lingua::acquisition::Gender::Neutral,
                                number: crate::lingua::acquisition::WordNumber::Singular,
                                structural_features: vec![],
                                design_category: Some(crate::lingua::acquisition::DesignCategory::Uncategorized),
                                is_visually_grounded: false,
                                grounded_concept_id: None,
                            };
                            engine.acquisition.update_entry(simple_entry);
                            words_this_cycle += 1;
                            total_words += 1;
                        }
                    }

                    // Rate limiting: no saturar APIs
                    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;

                    // Chequear si nos pidieron parar
                    if !AUTONOMOUS_RUNNING.load(Ordering::SeqCst) { break; }

                    // Limitar palabras por ciclo para no bloquear mucho tiempo
                    if words_this_cycle >= 15 { break; }
                }
            }

            // Persistir vocabulario
            let _ = engine.acquisition.save_cache();
            log::info!("  ✓ {} palabras nuevas inyectadas (Total: {})", 
                words_this_cycle, engine.acquisition.vocabulary_size());
        }

        if !AUTONOMOUS_RUNNING.load(Ordering::SeqCst) { break; }

        // ═══════════════════════════════════════
        //  FASE 3: DISCUSIÓN TRINITY
        // ═══════════════════════════════════════
        log::info!("[FASE 3] Agentes TRINITY debatiendo sobre '{}'...", topic);

        {
            let mut trinity_lock = trinity.write().await;

            // Inyectar frases temáticas para que los agentes las procesen
            let short_summary: String = summary.chars().take(200).collect();
            let discussion_phrases = vec![
                format!("Investigué sobre {} y descubrí cosas fascinantes.", topic),
                format!("En mis estudios sobre {} aprendí que: {}", topic, short_summary),
            ];

            for phrase in &discussion_phrases {
                trinity_lock.agent_b.learned_phrases.push(
                    crate::trinity::agents::conversational_agent::LearnedPhrase {
                        phrase: phrase.clone(),
                        context_tags: vec![topic.clone()],
                        success_rate: 0.9,
                        usage_count: 1,
                    }
                );
            }

            // Entrenamiento focalizado corto
            trinity_lock.train(10);
            log::info!("  ✓ TRINITY completó discusión sobre '{}'", topic);
        }

        // ═══════════════════════════════════════
        //  FASE 4: REGISTRO Y PERSISTENCIA
        // ═══════════════════════════════════════
        topic_queue.mark_explored(&topic);
        let _ = topic_queue.save();

        log::info!("📊 ESTADÍSTICAS AUTÓNOMAS:");
        log::info!("  ├─ Ciclo: #{}", cycle);
        log::info!("  ├─ Palabras este ciclo: {}", words_this_cycle);
        log::info!("  ├─ Total palabras aprendidas: {}", total_words);
        log::info!("  ├─ Temas explorados: {}", topic_queue.explored.len());
        log::info!("  ├─ Temas pendientes: {}", topic_queue.pending.len());
        log::info!("  └─ Último tema: {}", topic);

        // Pausa entre ciclos (3 segundos)
        tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
    }

    // Guardar estado final
    let _ = topic_queue.save();
    {
        let engine = lingua_engine.read().await;
        let _ = engine.acquisition.save_cache();
    }

    AUTONOMOUS_RUNNING.store(false, Ordering::SeqCst);
    log::info!("╔════════════════════════════════════════════════════════════╗");
    log::info!("║     DAITHON — MODO AUTÓNOMO DETENIDO                      ║");
    log::info!("║     Conocimiento persistido. {} ciclos completados.       ║", cycle);
    log::info!("╚════════════════════════════════════════════════════════════╝");
}

/// Detener el loop autónomo
pub fn stop_autonomous() {
    log::info!("[AUTONOMOUS] Señal de parada recibida. Finalizando ciclo actual...");
    AUTONOMOUS_RUNNING.store(false, Ordering::SeqCst);
}

/// Obtener estadísticas del sistema autónomo
pub fn get_autonomous_status() -> AutonomousStats {
    let queue = TopicQueue::load();
    AutonomousStats {
        cycles_completed: queue.explored.len(),
        total_words_learned: 0, // Se actualiza desde el caller
        total_topics_explored: queue.explored.len(),
        topics_explored: queue.explored.keys().take(20).cloned().collect(),
        running: AUTONOMOUS_RUNNING.load(Ordering::SeqCst),
        current_topic: String::new(),
    }
}
