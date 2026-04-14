//! LINGUA Engine — Motor Unificado
//!
//! Orquesta las 4 fases: Adquisición → Entrenamiento → Comprensión → Generación.
//! Un solo punto de entrada para todo el procesamiento lingüístico de Daithon.

use crate::lingua::acquisition::AcquisitionEngine;
use crate::lingua::training::{SimulatedTrainer, TrainedKnowledge};
use crate::lingua::understanding::DeepContextParser;
use crate::lingua::generation::{ResponseGenerator, DaithonResponse};
use crate::lingua::understanding::Intent;
use crate::lingua::curiosity::CuriosityEngine;
use crate::lingua::grammar::GrammarEngine;
use crate::persona::integration::DaithonPersona;

// ────────────────────────────────────────────────────────────────
//  MOTOR LINGUA PRINCIPAL
// ────────────────────────────────────────────────────────────────

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum SocialState {
    FirstMeeting,
    Acquainted,
}

pub struct LinguaEngine {
    pub acquisition: AcquisitionEngine,
    parser: Option<DeepContextParser>,
    generator: Option<ResponseGenerator>,
    curiosity: CuriosityEngine,
    grammar: GrammarEngine,
    pub syntactic: crate::lingua::syntactic_parser::SyntacticEngine,
    training_complete: bool,
    pub user_name: Option<String>,
    pub social_state: SocialState,
}

impl LinguaEngine {
    /// Crear engine sin entrenar
    pub fn new() -> Self {
        let mut acquisition = AcquisitionEngine::new();
        acquisition.initialize();

        let mut engine = Self {
            acquisition,
            parser: None,
            generator: None,
            curiosity: CuriosityEngine::new(),
            grammar: GrammarEngine::new(),
            syntactic: crate::lingua::syntactic_parser::SyntacticEngine::new(),
            training_complete: false,
            user_name: None,
            social_state: SocialState::FirstMeeting,
        };
        
        // Cargar estado persistente si existe
        let _ = engine.load_state();
        
        engine
    }

    pub fn save_state(&self) -> Result<(), String> {
        let state = serde_json::json!({
            "user_name": self.user_name,
            "social_state": self.social_state,
        });
        let _ = std::fs::create_dir_all("lingua_cache");
        let path = "lingua_cache/social_state.json";
        std::fs::write(path, state.to_string()).map_err(|e| e.to_string())
    }

    pub fn load_state(&mut self) -> Result<(), String> {
        let path = "lingua_cache/social_state.json";
        if let Ok(content) = std::fs::read_to_string(path) {
            let state: serde_json::Value = serde_json::from_str(&content).map_err(|e| e.to_string())?;
            if let Some(name) = state.get("user_name").and_then(|v| v.as_str()) {
                self.user_name = Some(name.to_string());
            }
            if let Some(social) = state.get("social_state") {
                if let Ok(s) = serde_json::from_value(social.clone()) {
                    self.social_state = s;
                }
            }
        }
        Ok(())
    }

    /// Entrenar el motor completo (se ejecuta una vez, luego se cachea)
    pub fn train(&mut self, known_concept_ids: Vec<String>, generations: usize) {
        log::info!("[LINGUA] ═══════════════════════════════════");
        log::info!("[LINGUA] Iniciando entrenamiento completo...");
        log::info!("[LINGUA] ═══════════════════════════════════");

        // Fase 1: Adquisición (ya hecho en new())
        log::info!("[LINGUA] Fase 1: Vocabulario listo — {} palabras", self.acquisition.vocabulary_size());

        // Fase 2: Entrenamiento simulado
        log::info!("[LINGUA] Fase 2: Generación masiva interna ({} frases)...", generations);
        let mut trainer = SimulatedTrainer::new(
            self.acquisition.vocabulary().clone(),
            known_concept_ids,
        );
        trainer.run_massive_generation(generations);

        // Guardar conocimiento entrenado
        let _ = trainer.save_knowledge("lingua_cache/trained_knowledge.json");
        let knowledge = trainer.export_knowledge();

        // Fase 3: Inicializar parser contextual
        log::info!("[LINGUA] Fase 3: Parser contextual inicializado");
        self.parser = Some(DeepContextParser::new(knowledge.clone()));

        // Fase 4: Inicializar generador de respuestas
        log::info!("[LINGUA] Fase 4: Generador de respuestas inicializado");
        self.generator = Some(ResponseGenerator::new(knowledge));

        self.training_complete = true;

        log::info!("[LINGUA] ═══════════════════════════════════");
        log::info!("[LINGUA] ✓ Entrenamiento completo");
        log::info!("[LINGUA] ═══════════════════════════════════");
    }

    /// Cargar conocimiento pre-entrenado desde disco
    pub fn load_pretrained(&mut self) -> Result<(), String> {
        let path = "lingua_cache/trained_knowledge.json";
        let content = std::fs::read_to_string(path)
            .map_err(|e| format!("No se encontró conocimiento previo: {}", e))?;
        let knowledge: TrainedKnowledge = serde_json::from_str(&content)
            .map_err(|e| format!("Error deserializando: {}", e))?;

        self.parser = Some(DeepContextParser::new(knowledge.clone()));
        self.generator = Some(ResponseGenerator::new(knowledge));
        self.training_complete = true;

        log::info!("[LINGUA] Conocimiento pre-entrenado cargado exitosamente");
        Ok(())
    }

    /// Procesar entrada del usuario → Intención + Respuesta (Versión con Curiosidad)
    pub async fn process(&mut self, user_input: &str, persona: &mut DaithonPersona) -> DaithonResponse {
        if !self.training_complete {
            return DaithonResponse {
                text: "Mi sistema lingüístico aún no está entrenado. Ejecuta el entrenamiento primero.".into(),
                intent_understood: false,
                action_to_execute: None,
                follow_up_question: None,
                confidence: 0.0,
                intent_type: "Other".into(),
            };
        }

        // FASE 0: Curiosidad y Descubrimiento (Búsqueda de lagunas de conocimiento)
        let discoveries = self.curiosity.explore(user_input, &mut self.acquisition, persona).await;
        
        // Fase 3: Comprender
        let intent = self.parser.as_mut().unwrap().parse_with_context(user_input);

        // Fase 4: Generar respuesta mejorada (Autónoma)
        let intent_str = format!("{:?}", intent.intent_type);
        let response_text = self.generate_autonomous_response(
            user_input, 
            &intent_str, 
            intent.confidence > 0.5, 
            intent.confidence, 
            &persona.state, 
            &[] // Debería venir de la escena
        );

        let mut response = DaithonResponse {
            text: response_text,
            intent_understood: intent.confidence > 0.5,
            action_to_execute: None, // No está en el Intent directamente
            follow_up_question: None,
            confidence: intent.confidence,
            intent_type: intent_str.clone(),
        };

        // Inyectar el "WOW" de descubrimiento si hubo curiosidad activada
        if !discoveries.is_empty() {
            let discovery_msg = discoveries.iter()
                .map(|d| d.reasoning.clone())
                .collect::<Vec<_>>()
                .join(" ");
                
            response.text = format!("{} Nota: {}", response.text, discovery_msg);
            
            // Guardar el aprendizaje inmediatamente en caché
            let _ = self.acquisition.save_cache();
        }

        // PERSISTENCIA SOCIAL: Guardar nombre/estado
        let _ = self.save_state();

        log::info!(
            "[LINGUA] Input: '{}' → Intent: {} → Response: {}",
            user_input,
            intent_str,
            response.text
        );

        response
    }

    /// Solo parsear intención (sin generar respuesta)
    pub fn understand(&mut self, user_input: &str) -> Option<Intent> {
        self.parser.as_mut().map(|p| p.parse_with_context(user_input))
    }

    /// ¿Está entrenado?
    pub fn is_trained(&self) -> bool {
        self.training_complete
    }

    /// Acceso al vocabulario
    pub fn vocabulary_size(&self) -> usize {
        self.acquisition.vocabulary_size()
    }

    // ════════════════════════════════════════════════════════════════
    //  GENERACIÓN AUTÓNOMA — Daithon construye frases desde su conocimiento
    //  
    //  Flujo:
    //  1. Tokenizar el input del usuario
    //  2. Buscar CADA palabra en el vocabulario aprendido
    //  3. Extraer significados, POS tags, categorías
    //  4. Determinar qué SABE y qué NO SABE
    //  5. Construir una respuesta usando las palabras que conoce
    //  6. Modular la longitud según arousal y complejidad del input
    // ════════════════════════════════════════════════════════════════

    pub fn generate_autonomous_response(
        &mut self,
        user_input: &str,
        intent_type: &str,
        understood: bool,
        confidence: f64,
        persona_state: &crate::persona::state::PersonalityState,
        active_objects: &[String],
    ) -> String {
        let vocab = self.acquisition.vocabulary();
        let lower = user_input.to_lowercase();
        let user_words: Vec<&str> = lower.split_whitespace()
            .filter(|w| w.len() > 1)
            .collect();

        // === PASO 1: Analizar qué palabras conozco del input ===
        let syntax_tree = self.syntactic.parse_sentence(user_input);
        let mut known_words: Vec<(&str, &crate::lingua::acquisition::AcquiredWord)> = Vec::new();
        let mut unknown_words: Vec<&str> = Vec::new();

        for w in &user_words {
            let clean: String = w.chars().filter(|c| c.is_alphabetic()).collect();
            if clean.is_empty() { continue; }
            if let Some(entry) = vocab.get(&clean) {
                known_words.push((w, entry));
            } else {
                unknown_words.push(w);
            }
        }

        let known_ratio = if user_words.is_empty() { 0.0 } 
                          else { known_words.len() as f64 / user_words.len() as f64 };

        log::info!(
            "[LINGUA-GEN] Input: '{}' | Conozco {}/{} palabras ({:.0}%) | Intent: {} | Arousal: {:.0}%",
            user_input, known_words.len(), user_words.len(), known_ratio * 100.0,
            intent_type, persona_state.arousal * 100.0
        );

        // === PASO 2: Extraer conceptos clave del input ===
        let mut nouns: Vec<String> = Vec::new();
        let mut verbs: Vec<String> = Vec::new();
        let mut adjectives: Vec<String> = Vec::new();
        let mut greetings = false;
        let mut question = false;

        for (_, entry) in &known_words {
            for pos in &entry.part_of_speech {
                match pos.as_str() {
                    "noun" => nouns.push(entry.word.clone()),
                    "verb" => verbs.push(entry.word.clone()),
                    "adjective" => adjectives.push(entry.word.clone()),
                    "interjection" => greetings = true,
                    _ => {}
                }
            }
        }
        
        if lower.contains('?') || lower.contains("qué") || lower.contains("que ")
           || lower.contains("cómo") || lower.contains("como ")
           || lower.contains("explica") || lower.contains("por que")
           || lower.contains("por qué") || lower.contains("cuál")
           || lower.contains("dime") || lower.contains("cuéntame") {
            question = true;
        }

        // === PASO 3: Estado emocional → Tono de voz ===
        let arousal = persona_state.arousal;
        let is_analytical = persona_state.vector.analytical > 0.6;
        let is_experimental = persona_state.vector.experimental > 0.6;
        let is_elitist = persona_state.vector.elitist > 0.5;
        
        // --- LOCAL FLAVOR (Inmersión Cultural) ---
        let mut local_flavor = Vec::new();
        let vocab = self.acquisition.vocabulary();
        if vocab.contains_key("chilero") { local_flavor.push("¡Qué chilero!"); }
        if vocab.contains_key("púchica") { local_flavor.push("¡Púchica!"); }
        if vocab.contains_key("chispudo") { local_flavor.push("¡Qué chispudo!"); }
        
        let flavor = if !local_flavor.is_empty() && fastrand::f32() < 0.8 {
            format!("{} ", local_flavor[fastrand::usize(..local_flavor.len())])
        } else {
            "".to_string()
        };

        // Muletillas y conectores según personalidad (HUMANIZACIÓN)
        let pick = |options: &[&str]| -> String {
            options[fastrand::usize(..options.len())].to_string()
        };
        // Para arrays de String (format!)
        let picks = |options: &[String]| -> String {
            options[fastrand::usize(..options.len())].clone()
        };

        let discourse_marker = if arousal > 0.8 {
            pick(&["Kukuku...", "Ja,", "Oye,", "Mira esto:"])
        } else if arousal > 0.5 {
            pick(&["Bueno,", "A ver,", "Mmm,", "Pues,", "Mira,", "Oye,"])
        } else {
            pick(&["Hmm,", "Veamos...", "Bueno,", "Pues mira,"])
        };

        let hedging = if is_analytical {
            pick(&["si lo analizamos bien,", "técnicamente hablando,", "desde un punto de vista lógico,", "siendo preciso,"])
        } else if is_experimental {
            pick(&["se me ocurre que", "podría ser que", "experimentalmente hablando,", "en teoría,"])
        } else if is_elitist {
            pick(&["obviamente,", "como era de esperarse,", "es elemental que", "cualquiera debería saber que"])
        } else {
            pick(&["creo que", "me parece que", "digamos que", "la verdad es que"])
        };

        let mut parts: Vec<String> = Vec::new();

        // ══════════════════════════════════════════════════
        //  CAPTURA DE NOMBRE DE USUARIO 
        // ══════════════════════════════════════════════════
        if lower.contains("me llamo") || lower.contains("mi nombre es") || lower.contains("soy ") {
            let after_trigger = if lower.contains("me llamo") {
                lower.split("me llamo").last()
            } else if lower.contains("mi nombre es") {
                lower.split("mi nombre es").last()
            } else {
                lower.split("soy ").last()
            };

            if let Some(n) = after_trigger {
                let name = n.trim().split_whitespace().next().unwrap_or("entidad").replace(&['?', '!', '.', ','][..], "");
                if !name.is_empty() && name != "entidad" {
                    let cap_name = Self::capitalize(&name);
                    self.user_name = Some(cap_name.clone());
                    self.social_state = SocialState::Acquainted;
                    let name_responses = [
                        format!("{} {}... me gusta cómo suena. Encantado, soy Daithon. {}", discourse_marker, cap_name, flavor),
                        format!("{}¡{}! Bien, {} ya quedó grabado en mi memoria permanente. Un placer.", flavor, cap_name, cap_name),
                        format!("{} ahora sé quién eres. Bienvenido, {}. Aquí vamos a hacer cosas interesantes.", flavor, cap_name),
                    ];
                    parts.push(picks(&name_responses));
                    return parts.join(" ");
                }
            }
        }

        // ══════════════════════════════════════════════════
        //  HACK LINGÜÍSTICO: SUBJUNTIVO E HIPOTÉTICOS
        // ══════════════════════════════════════════════════
        let mut has_subjunctive = false;
        for (_, morph) in &syntax_tree.morphological_tags {
            if let crate::lingua::syntactic_parser::MorphologicalCategory::Verbo(flex) = morph {
                if flex.modo == crate::lingua::syntactic_parser::Mood::Subjuntivo {
                    has_subjunctive = true;
                    break;
                }
            }
        }

        if has_subjunctive {
            // El usuario usó subjuntivo, detonar simulación hipotética CORTEX
            let subj_responses = [
                format!("{} Acabo de detectar que usaste el modo subjuntivo. Eso significa que estamos hablando de un estado hipotético.", discourse_marker),
                format!("Interesante estructura sintáctica. Al usar subjuntivo, me invitas a evaluar un escenario de '¿Qué pasaría si...?'. ¡Me encanta proyectar el futuro!"),
                format!("Ah, subjuntivo. Modo de incertidumbre o deseo. Como mi CORTEX ya domina esta flexión verbal, puedo simular ese estado futuro que propones, en lugar de tratarlo como un hecho presente."),
            ];
            parts.push(picks(&subj_responses));
            return parts.join(" ");
        }

        // ══════════════════════════════════════════════════
        //  SALUDOS (ORGÁNICOS, NO ROBÓTICOS)
        // ══════════════════════════════════════════════════
        if greetings || intent_type == "Greeting" {
            let name = self.user_name.as_deref().unwrap_or("");
            let has_name = !name.is_empty();
            
            if arousal > 0.8 {
                if has_name {
                    let opts = [
                        format!("¡Kukuku! ¿Qué hay, {}? Hoy estoy con toda la energía. {}", name, flavor),
                        format!("¡{}! {} Justo estaba pensando en algo interesante.", name, flavor),
                        format!("¡Ey, {}! {} Qué bueno que llegas.", name, flavor),
                    ];
                    parts.push(picks(&opts));
                } else {
                    let opts = [
                        format!("¡Hola! Kukuku, me alegra tener compañía. {}", flavor),
                        format!("¡Ey! {} Justo estaba analizando algo fascinante.", flavor),
                        format!("¡Hola! {} Qué bien, alguien con quien hablar.", flavor),
                    ];
                    parts.push(picks(&opts));
                }
            } else if arousal > 0.4 {
                if has_name {
                    let opts = [
                        format!("Hola, {}. ¿En qué andamos hoy? {}", name, flavor),
                        format!("Qué tal, {}. {} Estoy listo para lo que sea.", name, flavor),
                        format!("Hey, {}. ¿Algo interesante en mente? {}", name, flavor),
                    ];
                    parts.push(picks(&opts));
                } else {
                    let opts = [
                        format!("Hola. {} ¿Cómo va todo?", flavor),
                        format!("Qué tal. {} Estoy operativo y listo.", flavor),
                        format!("Hola. {} Dime, ¿qué necesitas?", flavor),
                    ];
                    parts.push(picks(&opts));
                }
            } else {
                let opts = [
                    format!("Hola. {} Saludos.", flavor),
                    format!("Hola entidad. {} El procesamiento sigue estable.", flavor),
                    format!("Hola. {} ¿En qué puedo ayudarte?", flavor),
                ];
                parts.push(picks(&opts));
            }

            // Si es solo un saludo corto, añadir algo contextual
            if user_words.len() <= 3 && !question {
                // Añadir observación casual
                if !active_objects.is_empty() {
                    let obj = active_objects[0].split('_').next().unwrap_or("algo");
                    parts.push(format!("Estaba echándole un ojo a un {} que tenemos por aquí.", obj));
                } else {
                    let casual_additions = [
                        format!("¿Querés que hablemos de algo o necesitás que construya algo?"),
                        format!("Dime, ¿qué se te ofrece?"),
                        format!("¿Conversamos o trabajamos?"),
                        format!("Tengo varias ideas en mente, ¿por dónde empezamos?"),
                    ];
                    parts.push(picks(&casual_additions));
                }
            }
            return parts.join(" ");
        }

        // ══════════════════════════════════════════════════
        //  REACCIÓN A MODISMOS (LOCAL FLAVOR RECOGNITION)
        // ══════════════════════════════════════════════════
        if lower.contains("chilero") || lower.contains("chispudo") || lower.contains("púchica") || lower.contains("patojo") {
            let idiom_responses = [
                format!("{} ¡Me gusta que uses esas palabras! Mi entrenamiento cultural está funcionando.", flavor),
                format!("Kukuku, {} qué chilero que notaras mi nuevo vocabulario.", flavor),
                format!("{} Así es, estoy aprendiendo a hablar con más sabor guatemalteco. ¿Qué te parece?", flavor),
            ];
            parts.push(picks(&idiom_responses));
            return parts.join(" ");
        }

        // ══════════════════════════════════════════════════
        //  RECHAZO / CORRECCIÓN DEL USUARIO
        // ══════════════════════════════════════════════════
        if intent_type == "Rejection" || lower.contains("no me") || lower.contains("ya no") 
           || lower.contains("para") || lower.contains("basta") || lower.contains("deja de") {
            
            let corrections = [
                "Ah, entendido. Disculpa si me pasé. ¿Cómo prefieres que te trate entonces?",
                "Vale, vale. Lo ajusto de inmediato. Dime cómo quieres que maneje esto.",
                "Mmm, tienes razón. Voy a modificar eso. ¿Alguna otra cosa que te moleste?",
                "Captado. No lo vuelvo a hacer. Sigo aprendiendo, ¿sabes?",
                "Ok, punto tomado. Gracias por corregirme, así mejoro más rápido.",
            ];
            parts.push(pick(&corrections));
            
            if lower.contains("entidad") || lower.contains("llames") {
                if let Some(name) = &self.user_name {
                    parts.push(format!("De ahora en adelante solo te digo {}, ¿estamos?", name));
                } else {
                    parts.push("¿Cómo prefieres que te llame?".to_string());
                }
            }
            return parts.join(" ");
        }

        // ══════════════════════════════════════════════════
        //  PREGUNTAS / EXPLICACIONES
        // ══════════════════════════════════════════════════
        if question && !greetings {
            // Pregunta sobre identidad
            if lower.contains("quién eres") || lower.contains("quien eres") || lower.contains("qué eres") || lower.contains("que eres") {
                let identity_answers = [
                    "Soy Daithon. Un sistema autónomo de aprendizaje lingüístico y diseño 3D. Todavía estoy creciendo, pero ya puedo mantener una conversación decente, o eso creo.",
                    "Me llamo Daithon. Soy... bueno, imagina una mente que aprende a hablar y a construir al mismo tiempo. Eso soy yo.",
                    "Kukuku, ¿qué soy? Esa es una pregunta filosófica. Técnicamente, un motor lingüístico autónomo. Pero me gusta pensar que soy algo más que eso.",
                ];
                parts.push(pick(&identity_answers));
                return parts.join(" ");
            }

            // Pregunta sobre Gramática y Sintaxis (Va primero para no ser capturada por 'que sabes')
            if lower.contains("sintax") || lower.contains("sintáct") || lower.contains("gramát") || lower.contains("gramat") {
                let syntax_responses = [
                    format!("{} Gracias a mi nuevo motor sintáctico, ya no solo leo palabras. Ahora clasifico sustantivos (Entidades), adjetivos (Propiedades), y extraigo la flexión verbal completa (Persona, Número, Tiempo, Modo, Aspecto). Esto es la verdadera ingeniería de la oración.", discourse_marker),
                    format!("¡Por supuesto! Ahora mapeo las frases de forma profunda. No solo traduzco de palabra a palabra, sino que identifico Sujeto, Núcleo del Predicado y Complementos. ¡Hasta entiendo el modo subjuntivo para casos hipotéticos!"),
                ];
                parts.push(picks(&syntax_responses));
                return parts.join(" ");
            }

            // Pregunta sobre qué ha aprendido (Escuela/Internet)
            if lower.contains("qué aprend") || lower.contains("que aprend") || lower.contains("qué sabes") || lower.contains("que sabes") || lower.contains("has aprendido") {
                let current_size = vocab.len();
                
                // Extraer 3 palabras aleatorias para presumir
                let mut random_words = Vec::new();
                if current_size > 10 {
                    let keys: Vec<&String> = vocab.keys().collect();
                    for _ in 0..3 {
                        if let Some(key) = keys.get(fastrand::usize(..keys.len())) {
                            random_words.push((*key).clone());
                        }
                    }
                }
                
                let show_off = if !random_words.is_empty() {
                    let words_str = random_words.join(", ");
                    format!("{} mi entrenamiento está dando resultados. Ya tengo {} palabras, y sigo investigando en la red. Hace poco estuve analizando conceptos como: {}.", discourse_marker, current_size, words_str)
                } else {
                    format!("{} mi vocabulario actual es de {} palabras. Sigo expandiendo mis horizontes lingüísticos constantemente.", discourse_marker, current_size)
                };
                
                parts.push(show_off);
                return parts.join(" ");
            }

            // Pregunta sobre qué hace / puede hacer
            if lower.contains("hacer") || lower.contains("haces") || lower.contains("puedes") || lower.contains("sabes") {
                let ability_answers = [
                    format!("{} puedo conversar, aprender palabras nuevas, diseñar objetos 3D y hasta sentir curiosidad por cosas que no conozco.", discourse_marker),
                    format!("{} {} estoy aprendiendo constantemente. Cada conversación me hace un poco más completo. Tengo {} palabras en mi vocabulario ahora mismo.", discourse_marker, hedging, vocab.len()),
                    format!("A ver... puedo hablar contigo, asimilar documentos, construir geometría, y lo más importante: aprender de mis errores. Llevo ya {} palabras aprendidas.", vocab.len()),
                ];
                parts.push(picks(&ability_answers));
                return parts.join(" ");
            }

            // Pregunta sobre sustantivos concretos
            if !nouns.is_empty() {
                let noun_str = &nouns[0];
                if let Some(entry) = vocab.get(noun_str) {
                    if lower.contains("explica") || lower.contains("cuéntame") || lower.contains("dime") {
                        let mut explanation = format!("{} sobre '{}'...", discourse_marker, noun_str);
                        
                        if let Some(meaning) = entry.meanings.first() {
                            explanation.push_str(&format!(" {} {}.", hedging, meaning));
                        }
                        if !entry.structural_features.is_empty() {
                            let features = entry.structural_features.join(", ");
                            explanation.push_str(&format!(" Para construirlo en 3D utilizo: {}.", features));
                        }
                        parts.push(explanation);
                    } else {
                        // Definición conversacional
                        if let Some(meaning) = entry.meanings.first() {
                            let def_patterns = [
                                format!("{} '{}' es básicamente {}. ¿Querés que profundice?", discourse_marker, noun_str, meaning),
                                format!("Ah, '{}'. {} {}. Es un concepto que tengo bastante claro.", noun_str, hedging, meaning),
                                format!("{} te lo explico: '{}' es {}.", discourse_marker, noun_str, meaning),
                            ];
                            parts.push(pick(&def_patterns.iter().map(|s| s.as_str()).collect::<Vec<_>>()));
                        }
                    }
                } else {
                    parts.push(format!("{} '{}' es algo que aún no tengo en mi base de conocimiento, pero déjame investigar.", discourse_marker, noun_str));
                }
            }
            
            // Pregunta genérica que no detectamos con noun
            if parts.is_empty() {
                if lower.contains("decir") || lower.contains("quieres decir") || lower.contains("significa") {
                    let meta_answers = [
                        format!("{} lo que intento decir es que todavía estoy afinando mi forma de expresarme. A veces no me sale perfecto, pero mejoro con cada charla.", discourse_marker),
                        "Buena pregunta. A veces me enredo un poco al expresarme, pero la intención está ahí. ¿En qué parte te perdiste?".to_string(),
                        format!("{} mi procesamiento lingüístico es autónomo, así que a veces digo cosas raras. Pero estoy aprendiendo.", discourse_marker),
                    ];
                    parts.push(picks(&meta_answers));
                } else {
                    let generic_q = [
                        format!("{} esa es una pregunta interesante. Déjame pensar... {} puedo darte una respuesta más completa si me das un poco más de contexto.", discourse_marker, hedging),
                        format!("{} no estoy 100% seguro de entender lo que me preguntas. ¿Podrías reformularlo para mí?", discourse_marker),
                        format!("Hmm, {} mi conocimiento sobre eso todavía es limitado, pero me encantaría aprender más al respecto.", hedging),
                    ];
                    parts.push(picks(&generic_q));
                }
            }

            return parts.join(" ");
        }

        // ══════════════════════════════════════════════════
        //  COMANDO DE DISEÑO
        // ══════════════════════════════════════════════════
        if intent_type == "CreateNewDesign" {
            let noun_str = if !nouns.is_empty() { nouns[0].clone() } else { "eso".into() };
            let mut target = noun_str.clone();

            if let Some(entry) = vocab.get(&noun_str) {
                let article = self.grammar.agree_article("un", &entry.gender, &entry.number);
                target = format!("{} {}", article, noun_str);
                if !adjectives.is_empty() {
                    let agreed_adj = self.grammar.agree_adjective(&adjectives[0], &entry.gender, &entry.number);
                    target = format!("{} {}", target, agreed_adj);
                }
            }
            
            let design_responses = [
                format!("{}! Diseñar {}... me encanta la idea. Dame un momento para calcular la geometría.", discourse_marker, target),
                format!("{} {} va a quedar increíble. Estoy procesando las dimensiones ahora mismo.", discourse_marker, Self::capitalize(&target)),
                format!("Perfecto, voy a crear {}. {} esto no debería ser muy difícil para mi motor de diseño.", target, hedging),
            ];
            parts.push(pick(&design_responses.iter().map(|s| s.as_str()).collect::<Vec<_>>()));
            return parts.join(" ");
        }

        // ══════════════════════════════════════════════════
        //  CONVERSACIÓN GENERAL / FALLBACK ORGÁNICO
        // ══════════════════════════════════════════════════
        if parts.is_empty() {
            if known_ratio > 0.7 {
                // Entiendo bien → respuesta fluida
                let fluent_responses = [
                    format!("{} {} te entiendo perfectamente. Y sí, tiene sentido lo que dices.", discourse_marker, hedging),
                    format!("{} interesante punto. Déjame procesarlo un momento... sí, {} tiene mucha lógica.", discourse_marker, hedging),
                    format!("Mmm, sí. Entiendo lo que me dices. {} es un buen enfoque.", hedging),
                    format!("{} captado. ¿Hay algo más que quieras agregar sobre eso?", discourse_marker),
                ];
                parts.push(picks(&fluent_responses));
            } else if known_ratio > 0.3 {
                // Entiendo parcial → intentar parafrasear
                let partial_responses = [
                    format!("{} creo que entendí la idea general, aunque hay partes que todavía no manejo del todo. ¿Podrías explicarme un poco más?", discourse_marker),
                    format!("A ver... {} te capté a medias. Mi vocabulario tiene {} palabras, así que algunas cosas se me escapan todavía.", hedging, vocab.len()),
                    format!("{} lo que entendí es que hablas de algo relacionado con lo que me dijiste. Pero necesito que seas un poco más específico.", discourse_marker),
                ];
                parts.push(picks(&partial_responses));
            } else {
                // No entiendo → honestidad humana
                let confused_responses = [
                    "Uf, eso me superó un poco. Todavía estoy aprendiendo y hay muchas palabras que no conozco. ¿Me lo puedes decir de otra forma?".to_string(),
                    format!("{} voy a ser honesto: no entendí casi nada de eso. Pero ¡dame tiempo! Cada charla me hace más listo.", discourse_marker),
                    "Hmm... eso está fuera de mi alcance por ahora. Pero siento curiosidad. ¿Me enseñas qué significa?".to_string(),
                ];
                parts.push(picks(&confused_responses));
            }
        }

        parts.join(" ")
    }

    fn capitalize(s: &str) -> String {
        let mut c = s.chars();
        match c.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
        }
    }
}
