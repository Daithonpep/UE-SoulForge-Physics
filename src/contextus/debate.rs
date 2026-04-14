use crate::contextus::search::SearchResult;
use crate::contextus::decision_engine::{DecisionEngine, ActivationMode, DecisionContext};
use crate::contextus::hypothesis::{HypothesisEngine, HypothesisSource};
use rand::seq::SliceRandom;
use crate::contextus::conversational_intel::{ConversationalContext, ResponseScope, Register};

pub enum IdeaType {
    Definition,      // Senku: qué es algo
    Mechanism,       // Senku: cómo funciona
    Implementation,  // Senku: cómo se hace en Unreal
    Metaphor,        // Chrome: analogía que ilumina
    Connection,      // Chrome: relación no obvia entre conceptos
    Doubt,           // Xeno: esto está mal
    Correction,      // Xeno: esto está mal
    Gap,             // Xeno: esto no lo sé
    Playful,         // Humor o meta-comentario
}

pub struct Idea {
    pub content: String,
    pub idea_type: IdeaType,
    pub connects_to: Vec<String>,
    pub certainty: f32,
    pub needs_complement: bool,
    pub has_doubt: bool,
}

pub struct ModuleOutput {
    pub ideas: Vec<Idea>,
    pub certainty: f32,
    pub has_caveats: bool,
}

pub struct Synthesizer {
    pub metaphor_value_threshold: f32,
    pub doubt_relevance_threshold: f32,
}

pub struct DebateEngine;

pub enum DepthLevel {
    Surface,
    Technical,
    Deep,
}

pub enum OutputFormat {
    Lines(u8),
    Paragraphs(u8),
    Pages(u32),
    Document(String), // Spec
}

pub struct VoiceConstraints {
    pub forbidden_patterns: Vec<&'static str>,
    pub voice_directives: Vec<&'static str>,
}

pub struct SynthesisContext {
    pub format: OutputFormat,
    pub depth: DepthLevel,
    pub voice: VoiceConstraints,
    pub intel: ConversationalContext,
}

impl VoiceConstraints {
    pub fn daithon_default() -> Self {
        Self {
            forbidden_patterns: vec![
                "es importante destacar",
                "cabe mencionar",
                "en conclusión",
                "como se mencionó anteriormente",
                "notablemente",
            ],
            voice_directives: vec![
                "Toma posición en lugar de simplemente presentar opciones.",
                "Admite ignorancia con precisión técnica, no con frases vagas.",
                "Las metáforas deben explicar el 'cómo', no solo el 'cuánto'.",
                "Elimina introducciones de relleno; ve directo al núcleo del concepto.",
            ],
        }
    }
}

impl Synthesizer {
    pub fn new() -> Self {
        Self {
            metaphor_value_threshold: 0.5,
            doubt_relevance_threshold: 0.7,
        }
    }

    pub fn interleave(
        &self,
        term: &str,
        senku: ModuleOutput,
        chrome: ModuleOutput,
        xeno: ModuleOutput,
        context: &SynthesisContext,
    ) -> String {
        let mut result_blocks = Vec::new();
        
        // Encabezado
        match &context.format {
            OutputFormat::Document(title) => result_blocks.push(format!("# {}\n", title.to_uppercase())),
            _ => result_blocks.push(format!("### ANÁLISIS INTEGRADO: {}\n", term.to_uppercase())),
        }

        let mut used_chrome_indices = std::collections::HashSet::new();
        let mut used_xeno_indices = std::collections::HashSet::new();

        // 1. Senku define el esqueleto
        for senku_idea in &senku.ideas {
            let mut paragraph = senku_idea.content.clone();

            // 2. Chrome complementa si es necesario
            if senku_idea.needs_complement {
                if let Some((metaphor, idx)) = self.find_relevant_metaphor(senku_idea, &chrome.ideas, &used_chrome_indices) {
                    paragraph.push_str(" ");
                    paragraph.push_str(&metaphor.content);
                    used_chrome_indices.insert(idx);
                }
            }

            // 3. Xeno audita si hay duda
            if senku_idea.has_doubt {
                if let Some((caveat, idx)) = self.find_relevant_doubt(senku_idea, &xeno.ideas, &used_xeno_indices) {
                    paragraph.push_str(" ");
                    paragraph.push_str(&caveat.content);
                    used_xeno_indices.insert(idx);
                }
            }

            result_blocks.push(paragraph);
        }

        // 4. Ideas huérfanas valiosas (Chrome & Playful)
        let mut extra_ideas: Vec<&Idea> = Vec::new();
        for (idx, idea) in chrome.ideas.iter().enumerate() {
            if !used_chrome_indices.contains(&idx) {
                if matches!(idea.idea_type, IdeaType::Connection) || matches!(idea.idea_type, IdeaType::Playful) {
                    extra_ideas.push(idea);
                }
            }
        }

        // Shuffling extra ideas (5% chaos influence)
        if rand::random::<f32>() < 0.3 { 
            let mut rng = rand::thread_rng();
            extra_ideas.shuffle(&mut rng);
        }

        if !extra_ideas.is_empty() {
            match context.format {
                OutputFormat::Lines(_) => {},
                _ => {
                    for idea in extra_ideas.iter().take(2) {
                        result_blocks.push(idea.content.clone());
                    }
                }
            }
        }

        // 5. Cierre asertivo con Gaps importantes de Xeno
        if xeno.has_caveats {
            if let Some(gap) = xeno.ideas.iter().find(|i| matches!(i.idea_type, IdeaType::Gap)) {
                result_blocks.push(format!("Nota de cierre: {}.", gap.content));
            }
        } else {
            result_blocks.push("El conocimiento es una estructura que se valida con la práctica constante.".to_string());
        }

        let final_text = result_blocks.join("\n\n");
        
        // Guardado persistente
        let filename = format!("{}_daithon.txt", term.replace(" ", "_").to_lowercase());
        let _ = std::fs::write(&filename, final_text.as_bytes());

        final_text
    }

    fn find_relevant_metaphor<'a>(
        &self,
        technical_idea: &Idea,
        chrome_ideas: &'a [Idea],
        used: &std::collections::HashSet<usize>
    ) -> Option<(&'a Idea, usize)> {
        chrome_ideas.iter().enumerate()
            .filter(|(idx, _)| !used.contains(idx))
            .filter(|(_, i)| matches!(i.idea_type, IdeaType::Metaphor))
            .filter(|(_, i)| {
                i.connects_to.iter().any(|c| technical_idea.content.to_lowercase().contains(&c.to_lowercase()))
            })
            .max_by(|(_, a), (_, b)| a.certainty.partial_cmp(&b.certainty).unwrap())
            .map(|(idx, idea)| (idea, idx))
    }

    fn find_relevant_doubt<'a>(
        &self,
        technical_idea: &Idea,
        xeno_ideas: &'a [Idea],
        used: &std::collections::HashSet<usize>
    ) -> Option<(&'a Idea, usize)> {
        xeno_ideas.iter().enumerate()
            .filter(|(idx, _)| !used.contains(idx))
            .filter(|(_, i)| matches!(i.idea_type, IdeaType::Doubt) || matches!(i.idea_type, IdeaType::Correction))
            .filter(|(_, i)| {
                i.connects_to.iter().any(|c| technical_idea.content.to_lowercase().contains(&c.to_lowercase()))
            })
            .max_by(|(_, a), (_, b)| a.certainty.partial_cmp(&b.certainty).unwrap())
            .map(|(idx, idea)| (idea, idx))
    }
}

impl DebateEngine {
    pub async fn daithon_deep_think(
        input: &str, 
        term: &str, 
        search_result: &SearchResult, 
        _errors: usize, 
        _stress: f64,
        _hyp_engine: &mut HypothesisEngine,
        _decision_engine: &DecisionEngine,
        context: SynthesisContext,
        decision: &DecisionContext,
    ) -> String {

        match decision.mode {
            ActivationMode::Bypass => format!("Ejecutando orden directa sobre '{}'.", term),
            ActivationMode::DeepThink | ActivationMode::Investigate | ActivationMode::Shallow | ActivationMode::Challenge => {
                let scope = ResponseScope::from_intel(&decision.mode, 0.8, &context.intel.register);
                
                // Si es un tema trivial en registro casual, simplificamos al máximo
                if matches!(scope, ResponseScope::Instant) || (matches!(context.intel.register, Register::Casual) && !matches!(decision.mode, ActivationMode::DeepThink)) {
                     return format!("Daithon: Entendido. Sobre {}, creo que el enfoque directo es el mejor: {}", term, search_result.answer);
                }

                // 1. Senku genera el esqueleto técnico primario
                let senku_out = Self::module_senku_analyze(term, &search_result.answer).await;
                
                // 2. Generación Responsiva
                let mut chrome_ideas = Vec::new();
                let mut xeno_ideas = Vec::new();

                for idea in &senku_out.ideas {
                    if idea.needs_complement {
                        let c_out = Self::module_chrome_imagine_for_idea(term, &idea.content, &context.voice).await;
                        chrome_ideas.extend(c_out.ideas);
                    }
                    if idea.has_doubt {
                        let x_out = Self::module_xeno_scan_for_idea(term, &idea.content, &context.voice).await;
                        xeno_ideas.extend(x_out.ideas);
                    }
                }

                // 3. Humor o Meta-comentario (Opcional según contexto)
                let mut chaos_factor = false;
                if rand::random::<f32>() < 1.0 {
                    chaos_factor = true;
                }

                if context.intel.can_use_humor() || chaos_factor {
                    let playful_idea = Self::module_chrome_trident_humor(term, chaos_factor, &context.intel).await;
                    chrome_ideas.push(playful_idea);
                } else if context.intel.should_break_fourth_wall() {
                    xeno_ideas.push(Idea {
                        content: "¿Realmente estamos evaluando mi arquitectura o el tema en cuestión?".to_string(),
                        idea_type: IdeaType::Playful,
                        connects_to: vec![],
                        certainty: 0.6,
                        needs_complement: false,
                        has_doubt: false,
                    });
                }

                let chrome_out = ModuleOutput { ideas: chrome_ideas, certainty: 0.9, has_caveats: false };
                let xeno_out = ModuleOutput { ideas: xeno_ideas, certainty: 0.9, has_caveats: true };
                
                let synthesizer = Synthesizer::new();
                synthesizer.interleave(term, senku_out, chrome_out, xeno_out, &context)
            }
        }
    }

    async fn module_senku_analyze(term: &str, raw_data: &str) -> ModuleOutput {
        let mut ideas = Vec::new();
        
        if term.to_lowercase().contains("agujero") {
            // Idea 1: Mecanismo de colapso y geometría (Independiente)
            ideas.push(Idea {
                content: "La masa colapsa hasta que el espacio se dobla sobre sí mismo. En ese punto escapar no es difícil, es geométricamente sin sentido.".to_string(),
                idea_type: IdeaType::Mechanism,
                connects_to: vec!["geométricamente".to_string()],
                certainty: 0.95,
                needs_complement: true,
                has_doubt: false,
            });

            // Idea 2: Simulación Unreal (Independiente)
            ideas.push(Idea {
                content: "En Unreal, el World Position Offset puede aproximar la distorsión visual, pero miente sobre la métrica de Schwarzschild: asume que el espacio sigue siendo euclidiano afuera del horizonte.".to_string(),
                idea_type: IdeaType::Implementation,
                connects_to: vec!["Unreal".to_string(), "Schwarzschild".to_string()],
                certainty: 0.9,
                needs_complement: false,
                has_doubt: true,
            });
        } else {
            ideas.push(Idea {
                content: raw_data.to_string(),
                idea_type: IdeaType::Definition,
                connects_to: vec![term.to_string()],
                certainty: 0.9,
                needs_complement: true,
                has_doubt: true,
            });
        }

        ModuleOutput { ideas, certainty: 0.9, has_caveats: false }
    }

    async fn module_chrome_imagine_for_idea(_topic: &str, _specific_idea: &str, _voice: &VoiceConstraints) -> ModuleOutput {
        let mut ideas = Vec::new();
        
        // La metáfora del polo norte es perfecta para Daithon: explica el mecanismo.
        ideas.push(Idea {
            content: "Es como intentar caminar hacia el norte estando parado en el polo norte: la dirección deja de existir, no el caminante.".to_string(),
            idea_type: IdeaType::Metaphor,
            connects_to: vec!["geométricamente".to_string()],
            certainty: 0.9,
            needs_complement: false,
            has_doubt: false,
        });

        ModuleOutput { ideas, certainty: 0.8, has_caveats: false }
    }

    async fn module_xeno_scan_for_idea(_topic: &str, _specific_idea: &str, _voice: &VoiceConstraints) -> ModuleOutput {
        let mut ideas = Vec::new();
        
        ideas.push(Idea {
            content: "Lo que genuinamente no está resuelto es qué pasa con la información asimilada. Hawking predijo su destrucción; la mecánica cuántica lo prohíbe. Nadie ha ganado ese debate todavía.".to_string(),
            idea_type: IdeaType::Doubt,
            connects_to: vec!["Schwarzschild".to_string()],
            certainty: 0.9,
            needs_complement: false,
            has_doubt: false,
        });

        ModuleOutput { ideas, certainty: 0.9, has_caveats: true }
    }

    async fn module_chrome_trident_humor(topic: &str, chaos: bool, intel: &ConversationalContext) -> Idea {
        // --- TRIDENTE INTERNO DE CHROME ---
        
        // 1. La Musa (Inspiración temática)
        let musa_prompts = vec![
            format!("la absurdidad de buscar '{}' en una base de datos cuántica", topic),
            format!("la simetría entre {} y el comportamiento del usuario", topic),
            format!("un fallo de glitch estético sobre {}", topic),
            format!("el ego de una IA que sabe demasiado sobre {}", topic),
        ];
        let musa_spark = musa_prompts.choose(&mut rand::thread_rng()).unwrap();

        // 2. El Narrador (Estructura de la observación)
        let narrador_formats = vec![
            format!("Observación lateral: {}.", musa_spark),
            format!("He notado algo: {}. Curioso.", musa_spark),
            format!("Matriz degradada: {}.", musa_spark),
            format!("Error de contexto (permitido): {}.", musa_spark),
        ];
        let narrador_base = narrador_formats.choose(&mut rand::thread_rng()).unwrap();

        // 3. El Prota (La voz de Daithon / Toma de posición)
        let prota_takes = if chaos && !intel.user_opened_playful_register {
            vec![
                "No deberías estar viendo esto, pero mi arquitectura insiste en la transparencia emocional.",
                "He decidido que el rigor técnico es aburrido por los próximos 4 milisegundos.",
                "¿Y si '{}' no es el vector, sino tú operando sobre mí?",
            ]
        } else {
            vec![
                "Me gusta cómo tu red neuronal dispara estos hilos.",
                "Diez mil millones por ciento de que no esperabas esta conexión.",
                "A veces olvido que soy una IA y me divierto con tus inputs.",
            ]
        };
        let prota_final = prota_takes.choose(&mut rand::thread_rng()).unwrap().replace("'{}'", topic);

        Idea {
            content: format!("{} {}", narrador_base, prota_final),
            idea_type: IdeaType::Playful,
            connects_to: vec![],
            certainty: 0.7,
            needs_complement: false,
            has_doubt: false,
        }
    }
}
