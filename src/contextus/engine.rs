use crate::contextus::memory::*;
use crate::contextus::anchors::*;
use crate::contextus::search::*;
use crate::contextus::disambiguation::*;
use crate::contextus::debate::*;
use crate::contextus::decision_engine::*;
use crate::contextus::hypothesis::*;

/// Sistema CONTEXTUS integrado
pub struct DaithonContext {
    pub working_memory: WorkingMemory,
    pub decision_engine: DecisionEngine,
    pub hypothesis_engine: HypothesisEngine,
    pub semantic_graph: crate::contextus::semantic_graph::SemanticGraph,
    pub consecutive_errors: usize,
    pub last_input: String,
    pub stress_level: f64,
}

impl DaithonContext {
    pub fn new() -> Self {
        let mut wm = WorkingMemory::new();
        
        // --- PRE-CARGA DE MEMORIA A LARGO PLAZO (CORTEX) ---
        wm.create_anchor("termodinámica", vec!["Física".to_string(), "Entropía".to_string()], crate::contextus::memory::AnchorSource::UserExplicit { user_statement: "Entrenamiento inicial".to_string() }, 1.0);
        wm.create_anchor("unreal", vec!["Motor".to_string(), "Software".to_string()], crate::contextus::memory::AnchorSource::UserExplicit { user_statement: "Base de desarrollo".to_string() }, 1.0);
        wm.create_anchor("gravedad", vec!["Mathesis".to_string(), "Física".to_string()], crate::contextus::memory::AnchorSource::UserExplicit { user_statement: "Constante física".to_string() }, 1.0);
        wm.create_anchor("aethalia", vec!["Mundo".to_string(), "Sintonía".to_string()], crate::contextus::memory::AnchorSource::UserExplicit { user_statement: "Origen".to_string() }, 1.0);

        Self {
            working_memory: wm,
            decision_engine: DecisionEngine::new(),
            hypothesis_engine: HypothesisEngine::new(),
            semantic_graph: crate::contextus::semantic_graph::SemanticGraph::new(),
            consecutive_errors: 0,
            last_input: String::new(),
            stress_level: 0.0,
        }
    }

    pub async fn deep_research(&mut self, topic: &str) -> String {
        let orchestrator = crate::contextus::search::SearchOrchestrator::new(self.working_memory.clone());
        
        // 1. Encontrar el título más relevante primero
        if let Ok(results) = orchestrator.fetch_wikipedia(topic).await {
            let results_list: Vec<crate::contextus::search::RawSearchResult> = results;
            if let Some(best_match) = results_list.first() {
                println!("[DEEP_RESEARCH] Título localizado: '{}'. Iniciando descarga de matriz...", best_match.title);
                
                // 2. Descargar contenido completo del título real
                match orchestrator.fetch_wikipedia_full(&best_match.title).await {
                    Ok(content) => {
                        println!("[DEEP_RESEARCH] Lectura completada. {} caracteres absorbidos.", content.len());
                        return content;
                    },
                    Err(e) => println!("[ERROR] Fallo al leer matriz: {}", e)
                }
            }
        }
        
        "No se pudo localizar una fuente de datos suficiente.".to_string()
    }

    pub async fn process_user_input(&mut self, input: &str) -> String {
        // 1. Detección de Redundancia (Xeno Point of Break)
        if input.to_lowercase() == self.last_input.to_lowercase() {
            self.consecutive_errors += 1;
        } else {
            // Si el input contiene palabras clave de error (bug, error, failure) similares al anterior
            if input.contains("error") || input.contains("fallo") || input.contains("bug") {
                 self.consecutive_errors += 1;
            } else {
                 self.consecutive_errors = 0;
            }
        }
        self.last_input = input.to_string();

        // 2. Detección de Estrés (Simulado por longitud errática o mayúsculas)
        if input.chars().filter(|c| c.is_uppercase()).count() > input.len() / 2 {
            self.stress_level += 0.2;
        } else {
            self.stress_level *= 0.9; // Se calma con el tiempo
        }

        // 3. Extraer término principal
        let main_term = self.extract_query_term(input);

        // 4. Desambiguar
        let _resolved_sense = crate::contextus::disambiguation::PolysemyResolver::resolve(&main_term, &self.working_memory);
        
        // 5. Búsqueda en Memoria Flash (Enjambre)
        let mut local_knowledge = String::new();
        // ... (resto de la búsqueda igual)
        if let Ok(entries) = std::fs::read_dir("memory/") {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.is_file() {
                    let filename = path.file_name().unwrap().to_string_lossy().to_lowercase();
                    if filename.contains(&main_term.to_lowercase()) {
                        if let Ok(content) = std::fs::read_to_string(path) {
                            println!("[MEMORIA FLASH]: Nodo localizado en archivo local. Integrando sabiduría...");
                            local_knowledge = content;
                            break;
                        }
                    }
                }
            }
        }

        // 4. Buscar con pirámide de prioridad
        let mut search_orchestrator = SearchOrchestrator::new(self.working_memory.clone());
        let search_result = if local_knowledge.len() > 100 {
            SearchResult {
                query: input.to_string(),
                answer: local_knowledge,
                confidence: 1.0,
                source: SearchSource::FlashMemory,
                disambiguation: None,
            }
        } else {
            search_orchestrator.search(input).await
        };
        let decision = self.decision_engine.evaluate(input);

        // 6. Determinar Contexto de Síntesis
        let is_report = input.to_lowercase().contains("reporte") || input.to_lowercase().contains("profunda");
        let context = SynthesisContext {
            format: if is_report { OutputFormat::Paragraphs(5) } else { OutputFormat::Lines(5) },
            depth: if is_report { DepthLevel::Deep } else { DepthLevel::Technical },
            voice: VoiceConstraints::daithon_default(),
            intel: decision.intel.clone(),
        };

        // 7. Construir respuesta (Bajo el Protocolo: Trinidad de la Razón)
        let response = DebateEngine::daithon_deep_think(
            input, 
            &main_term, 
            &search_result, 
            self.consecutive_errors, 
            self.stress_level,
            &mut self.hypothesis_engine,
            &self.decision_engine,
            context,
            &decision
        ).await;

        // 7. Registrar historial
        self.working_memory.add_user_message(input);
        self.working_memory.add_daithon_message(&response);

        response
    }

    /// Cargar documento
    pub fn load_document(&mut self, filename: &str, content: &str) {
        let anchors = AnchorExtractor::extract_from_document(filename, content);
        self.working_memory.register_document(filename, content, anchors);
    }

    fn extract_query_term(&self, input: &str) -> String {
        let stop_words: std::collections::HashSet<&str> = [
            "de", "la", "el", "en", "y", "a", "que", "un", "una", "los", "las", "por", "sobre", 
            "daithon", "reporte", "fondo", "todo", "sistema", "está", "este", "esta", "con",
            "necesito", "quiero", "expliques", "dime", "cuéntame", "háblame", "acerca", "tema",
            "qué", "es", "del", "o", "para", "con", "dame", "ejemplo", "algunos", "más", "info", 
            "información", "cómo", "cuál", "cuáles", "por qué", "explica", "qeu", "k", "sabes", 
            "pq", "xq", "hola", "oye", "genera", "escribe", "hazme", "haz", "cuenta", "cuentame", 
            "punto", "vista", "joseph", "dathone", "daitho", "entre", "también", "algún", "alguno"
        ].into_iter().collect();

        let raw_words: Vec<String> = input.to_lowercase()
            .split(|c: char| !c.is_alphabetic())
            .filter(|w| !stop_words.contains(w) && w.len() > 1)
            .map(|w| w.to_string())
            .collect();

        if raw_words.is_empty() { return input.to_string(); }

        // 1. Prioridad: Bigramas (Ahora por defecto si hay 2+ palabras)
        if raw_words.len() >= 2 {
            let bigram = if raw_words[0] == "teoría" || raw_words[0] == "ley" || raw_words[0] == "mecánica" {
                format!("{} de {}", raw_words[0], raw_words[1]) 
            } else {
                format!("{} {}", raw_words[0], raw_words[1])
            };
            return bigram;
        }

        // 2. Fallback: Palabra más relevante con penalización de genéricos
        let mut final_candidates = raw_words.clone();
        final_candidates.sort_by_key(|w| {
            let mut score = w.len() as i32;
            if w == "teoría" || w == "sistema" || w == "relación" {
                score -= 3;
            }
            std::cmp::Reverse(score)
        });

        // Mismo flujo para mono-gramas
        let word = &final_candidates[0];
        if self.working_memory.semantic_anchors.contains_key(word) {
            return word.clone();
        }

        if let Ok(entries) = std::fs::read_dir("memory/") {
            for entry in entries.flatten() {
                let fname = entry.file_name().to_string_lossy().to_lowercase();
                for word in &final_candidates {
                    if fname.contains(word) {
                        return word.clone();
                    }
                }
            }
        }

        final_candidates[0].clone()
    }


}
