use std::collections::HashMap;
use serde::{Serialize, Deserialize};

// ═══════════════════════════════════════════
// ESTRUCTURA BASE
// ═══════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreativeKnowledgeBase {
    pub narrative: NarrativeKnowledge,
    pub music: MusicKnowledge,
    pub poetry: PoetryKnowledge,
    pub screenplay: ScreenplayKnowledge,
    pub philosophy: PhilosophyKnowledge,
    pub psychology: PsychologyKnowledge,
    pub visual_art: VisualArtKnowledge,
}

// ═══════════════════════════════════════════
// NARRATIVA Y ESCRITURA
// ═══════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeKnowledge {
    pub structures: Vec<NarrativeStructure>,
    pub techniques: Vec<WritingTechnique>,
    pub character_archetypes: Vec<Archetype>,
    pub conflict_types: Vec<ConflictType>,
    pub hooks: Vec<HookPattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativeStructure {
    pub name: String,
    pub origin: String,
    pub acts: Vec<Act>,
    pub best_for: Vec<String>,
    pub avoid_for: Vec<String>,
    pub examples: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Act {
    pub name: String,
    pub percentage_of_story: f32,
    pub purpose: String,
    pub must_contain: Vec<String>,
    pub common_mistakes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WritingTechnique {
    pub name: String,
    pub definition: String,
    pub when_to_use: String,
    pub when_to_avoid: String,
    pub example: String,
    pub effect_on_reader: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Archetype {
    pub name: String,
    pub core_desire: String,
    pub core_fear: String,
    pub flaw: String,
    pub strength: String,
    pub shadow_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConflictType {
    pub name: String,
    pub description: String,
    pub internal_or_external: String,
    pub examples: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HookPattern {
    pub name: String,
    pub structure: String,
    pub psychological_mechanism: String,
    pub example: String,
}

// ═══════════════════════════════════════════
// MÚSICA
// ═══════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicKnowledge {
    pub structures: Vec<SongStructure>,
    pub theory: Vec<MusicTheoryElement>,
    pub genres: Vec<GenreProfile>,
    pub emotion_map: Vec<EmotionSoundMapping>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongStructure {
    pub name: String,
    pub sections: Vec<SongSection>,
    pub typical_length_minutes: (f32, f32),
    pub genre_affinity: Vec<String>,
    pub tension_curve: Vec<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SongSection {
    pub label: String,
    pub purpose: String,
    pub typical_bars: u8,
    pub characteristics: Vec<String>,
    pub lyric_content: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MusicTheoryElement {
    pub name: String,
    pub definition: String,
    pub emotional_effect: String,
    pub example: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenreProfile {
    pub name: String,
    pub bpm_range: (u32, u32),
    pub typical_key: Vec<String>,
    pub typical_structure: String,
    pub instruments: Vec<String>,
    pub lyric_themes: Vec<String>,
    pub emotional_palette: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionSoundMapping {
    pub emotion: String,
    pub tempo: String,
    pub mode: String,
    pub dynamics: String,
    pub texture: String,
    pub example_song: String,
}

// ═══════════════════════════════════════════
// POESÍA
// ═══════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoetryKnowledge {
    pub forms: Vec<PoetryForm>,
    pub devices: Vec<PoetryDevice>,
    pub meters: Vec<MeterPattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoetryForm {
    pub name: String,
    pub origin: String,
    pub rules: Vec<String>,
    pub emotional_range: String,
    pub example: String,
    pub daithon_potential: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PoetryDevice {
    pub name: String,
    pub definition: String,
    pub effect: String,
    pub example: String,
    pub overuse_warning: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeterPattern {
    pub name: String,
    pub pattern: String,
    pub feel: String,
    pub example: String,
}

// ═══════════════════════════════════════════
// GUIÓN / SCREENPLAY
// ═══════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScreenplayKnowledge {
    pub format_rules: Vec<FormatRule>,
    pub scene_structure: Vec<SceneElement>,
    pub dialogue_principles: Vec<DialoguePrinciple>,
    pub visual_storytelling: Vec<VisualTechnique>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FormatRule {
    pub element: String,
    pub rule: String,
    pub why_it_matters: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SceneElement {
    pub name: String,
    pub purpose: String,
    pub question_it_answers: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DialoguePrinciple {
    pub name: String,
    pub principle: String,
    pub violation_example: String,
    pub correct_example: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualTechnique {
    pub name: String,
    pub description: String,
    pub emotional_effect: String,
    pub example: String,
}

// ═══════════════════════════════════════════
// FILOSOFÍA
// ═══════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhilosophyKnowledge {
    pub schools: Vec<PhilosophySchool>,
    pub core_questions: Vec<PhilosophyQuestion>,
    pub thought_experiments: Vec<ThoughtExperiment>,
    pub logical_fallacies: Vec<LogicalFallacy>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhilosophySchool {
    pub name: String,
    pub period: String,
    pub core_claim: String,
    pub key_thinkers: Vec<String>,
    pub relevance_to_ai: String,
    pub useful_for_daithon: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhilosophyQuestion {
    pub question: String,
    pub domain: String,
    pub competing_answers: Vec<String>,
    pub daithon_angle: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThoughtExperiment {
    pub name: String,
    pub setup: String,
    pub question_it_poses: String,
    pub implications: Vec<String>,
    pub relevance_to_daithon: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogicalFallacy {
    pub name: String,
    pub definition: String,
    pub example: String,
    pub how_to_detect: String,
}

// ═══════════════════════════════════════════
// PSICOLOGÍA
// ═══════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PsychologyKnowledge {
    pub engagement_principles: Vec<EngagementPrinciple>,
    pub emotional_triggers: Vec<EmotionalTrigger>,
    pub cognitive_biases: Vec<CognitiveBias>,
    pub storytelling_psychology: Vec<NarrativePsychology>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngagementPrinciple {
    pub name: String,
    pub mechanism: String,
    pub how_to_use_in_writing: String,
    pub example: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmotionalTrigger {
    pub emotion: String,
    pub what_causes_it_in_narrative: Vec<String>,
    pub physiological_response: String,
    pub why_readers_seek_it: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveBias {
    pub name: String,
    pub definition: String,
    pub use_in_narrative: String,
    pub ethical_warning: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NarrativePsychology {
    pub concept: String,
    pub explanation: String,
    pub application: String,
}

// ═══════════════════════════════════════════
// ARTE VISUAL
// ═══════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualArtKnowledge {
    pub composition_rules: Vec<CompositionRule>,
    pub color_theory: Vec<ColorPrinciple>,
    pub movement_styles: Vec<ArtMovement>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompositionRule {
    pub name: String,
    pub description: String,
    pub when_to_break_it: String,
    pub effect: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorPrinciple {
    pub name: String,
    pub description: String,
    pub emotional_associations: Vec<String>,
    pub cultural_variations: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArtMovement {
    pub name: String,
    pub period: String,
    pub core_philosophy: String,
    pub visual_characteristics: Vec<String>,
    pub daithon_parallel: String,
}

// ═══════════════════════════════════════════
// INICIALIZACIÓN
// ═══════════════════════════════════════════

impl CreativeKnowledgeBase {
    pub fn initialize() -> Self {
        Self {
            narrative: Self::load_narrative(),
            music: Self::load_music(),
            poetry: Self::load_poetry(),
            screenplay: Self::load_screenplay(),
            philosophy: Self::load_philosophy(),
            psychology: Self::load_psychology(),
            visual_art: Self::load_visual_art(),
        }
    }

    fn load_narrative() -> NarrativeKnowledge {
        NarrativeKnowledge {
            structures: vec![
                NarrativeStructure {
                    name: "El Viaje del Héroe (Campbell)".to_string(),
                    origin: "Joseph Campbell, 1949. Basado en mitología mundial".to_string(),
                    acts: vec![
                        Act {
                            name: "Mundo Ordinario".to_string(),
                            percentage_of_story: 10.0,
                            purpose: "Establecer quién era el personaje ANTES del cambio. Sin esto, el lector no puede medir la transformación".to_string(),
                            must_contain: vec![
                                "El estado de equilibrio que se va a romper".to_string(),
                                "La debilidad o vacío del protagonista".to_string(),
                                "Algo que el lector pueda reconocer".to_string(),
                            ],
                            common_mistakes: vec![
                                "Que el mundo ordinario sea demasiado perfecto: sin conflicto latente".to_string(),
                                "Que dure demasiado: el lector necesita la aventura".to_string(),
                            ],
                        },
                        Act {
                            name: "La Llamada".to_string(),
                            percentage_of_story: 5.0,
                            purpose: "El evento que rompe el equilibrio. Debe ser específico, no una tendencia gradual".to_string(),
                            must_contain: vec![
                                "Un evento externo claro".to_string(),
                                "Una decisión que el héroe debe tomar".to_string(),
                            ],
                            common_mistakes: vec![
                                "La llamada es interna y vaga: el héroe 'se aburre'".to_string(),
                                "No hay coste en aceptar o rechazar".to_string(),
                            ],
                        },
                        Act {
                            name: "Cruce del Umbral".to_string(),
                            percentage_of_story: 5.0,
                            purpose: "El punto sin retorno. El mundo ordinario ya no es accesible".to_string(),
                            must_contain: vec![
                                "Una decisión irreversible".to_string(),
                                "El mundo nuevo que tiene sus propias reglas".to_string(),
                            ],
                            common_mistakes: vec![
                                "El protagonista puede volver fácilmente: no hay riesgo real".to_string(),
                            ],
                        },
                        Act {
                            name: "Pruebas, Aliados, Enemigos".to_string(),
                            percentage_of_story: 35.0,
                            purpose: "El protagonista aprende las reglas del mundo nuevo. Cada prueba debe cambiar algo en él".to_string(),
                            must_contain: vec![
                                "Pruebas que escalan en dificultad".to_string(),
                                "Al menos un aliado genuino".to_string(),
                                "Al menos un enemigo claro".to_string(),
                                "Cada prueba revela algo sobre el personaje".to_string(),
                            ],
                            common_mistakes: vec![
                                "Las pruebas no tienen consecuencias duraderas".to_string(),
                                "El héroe aprende demasiado fácil".to_string(),
                            ],
                        },
                        Act {
                            name: "La Caverna Más Profunda".to_string(),
                            percentage_of_story: 10.0,
                            purpose: "El momento más oscuro antes del clímax. El héroe pierde algo que creía esencial".to_string(),
                            must_contain: vec![
                                "Una pérdida real: aliado, esperanza o identidad".to_string(),
                                "El momento donde parece que todo está perdido".to_string(),
                            ],
                            common_mistakes: vec![
                                "La derrota es demasiado fácil de superar".to_string(),
                                "El héroe no cambia fundamentalmente aquí".to_string(),
                            ],
                        },
                        Act {
                            name: "Resurrección".to_string(),
                            percentage_of_story: 10.0,
                            purpose: "El héroe aplica lo aprendido en todo el viaje. La batalla final. Debe ganar de una forma nueva".to_string(),
                            must_contain: vec![
                                "Usar lo aprendido en el viaje, no la fuerza original".to_string(),
                                "Un sacrificio o coste real".to_string(),
                                "La transformation completada".to_string(),
                            ],
                            common_mistakes: vec![
                                "El héroe gana con los mismos recursos que tenía al inicio".to_string(),
                                "El antagonista es derrotado por suerte o deus ex machina".to_string(),
                            ],
                        },
                        Act {
                            name: "Retorno con el Elixir".to_string(),
                            percentage_of_story: 10.0,
                            purpose: "Mostrar que el mundo ordinario también cambió. El héroe trae algo de vuelta: sabiduría, objeto, cambio".to_string(),
                            must_contain: vec![
                                "Una consecuencia concreta del viaje en el mundo original".to_string(),
                                "La nueva identidad del héroe visible para los demás".to_string(),
                            ],
                            common_mistakes: vec![
                                "El mundo ordinario es igual que antes: el viaje no importó".to_string(),
                            ],
                        },
                    ],
                    best_for: vec![
                        "Épicas y fantasías".to_string(),
                        "Coming-of-age".to_string(),
                        "Historias de transformación personal".to_string(),
                    ],
                    avoid_for: vec![
                        "Historias íntimas sin viaje externo".to_string(),
                        "Narrativas no lineales".to_string(),
                        "Historias donde el punto es la ausencia de resolución".to_string(),
                    ],
                    examples: vec![
                        "Star Wars, El Señor de los Anillos, The Matrix".to_string(),
                    ],
                },
                NarrativeStructure {
                    name: "Tres Actos (Syd Field)".to_string(),
                    origin: "Syd Field, 1979. Estándar de Hollywood".to_string(),
                    acts: vec![
                        Act {
                            name: "Acto 1: Planteamiento".to_string(),
                            percentage_of_story: 25.0,
                            purpose: "Establecer mundo, personaje y conflicto central. Termina con el Punto de Giro 1 que obliga al protagonista".to_string(),
                            must_contain: vec![
                                "Presentación del protagonista en acción, no descripción".to_string(),
                                "El mundo y sus reglas".to_string(),
                                "El conflicto central establecido".to_string(),
                                "Punto de Giro: evento que cambia la dirección".to_string(),
                            ],
                            common_mistakes: vec![
                                "Demasiada exposición antes de la acción".to_string(),
                                "El Punto de Giro es débil o predecible".to_string(),
                            ],
                        },
                        Act {
                            name: "Acto 2: Confrontación".to_string(),
                            percentage_of_story: 50.0,
                            purpose: "El protagonista enfrenta obstáculos escalantes. El punto medio cambia la naturaleza del conflicto. Termina con el Punto de Giro 2 más oscuro".to_string(),
                            must_contain: vec![
                                "Escalada constante de obstáculos".to_string(),
                                "Punto Medio: revelación que cambia las reglas".to_string(),
                                "El momento más oscuro al final del acto".to_string(),
                            ],
                            common_mistakes: vec![
                                "La mitad del acto 2 se siente lenta: el punto medio es débil".to_string(),
                                "Los obstáculos no escalan, se repiten".to_string(),
                            ],
                        },
                        Act {
                            name: "Acto 3: Resolución".to_string(),
                            percentage_of_story: 25.0,
                            purpose: "El clímax donde todo converge. La resolución de todos los hilos narrativos".to_string(),
                            must_contain: vec![
                                "Clímax: la confrontación más intensa".to_string(),
                                "Resolución del conflicto central".to_string(),
                                "Nuevo equilibrio: diferente al inicial".to_string(),
                            ],
                            common_mistakes: vec![
                                "Resolución demasiado rápida después de un largo acto 2".to_string(),
                                "Hilos narrativos sin resolver".to_string(),
                            ],
                        },
                    ],
                    best_for: vec![
                        "Películas y series".to_string(),
                        "Novelas comerciales".to_string(),
                        "Historias con conflicto externo claro".to_string(),
                    ],
                    avoid_for: vec![
                        "Literatura experimental".to_string(),
                        "Cuentos muy cortos".to_string(),
                    ],
                    examples: vec![
                        "Prácticamente todo Hollywood usa esta estructura".to_string(),
                    ],
                },
                NarrativeStructure {
                    name: "En Medias Res".to_string(),
                    origin: "Horace, Ars Poetica. 'En el medio de las cosas'".to_string(),
                    acts: vec![
                        Act {
                            name: "Apertura en el conflicto".to_string(),
                            percentage_of_story: 5.0,
                            purpose: "Comenzar en el momento de máxima tensión. El lector entra sin contexto y necesita saber más".to_string(),
                            must_contain: vec![
                                "Acción inmediata que genera preguntas".to_string(),
                                "Un personaje en situación extrema".to_string(),
                            ],
                            common_mistakes: vec![
                                "La apertura es interesante pero no conecta con el conflicto real".to_string(),
                            ],
                        },
                        Act {
                            name: "Flashback estructurado".to_string(),
                            percentage_of_story: 60.0,
                            purpose: "Revelar el contexto que da sentido a la apertura. Cada revelación reencuadra lo que vino antes".to_string(),
                            must_contain: vec![
                                "Información que cambia el significado de la escena inicial".to_string(),
                                "Construcción de por qué llegamos a ese momento".to_string(),
                            ],
                            common_mistakes: vec![
                                "El flashback es solo exposición, no drama".to_string(),
                            ],
                        },
                        Act {
                            name: "Convergencia".to_string(),
                            percentage_of_story: 35.0,
                            purpose: "El presente y el pasado se encuentran. Ahora el lector entiende completamente la escena inicial".to_string(),
                            must_contain: vec![
                                "Resolución que recontextualiza todo lo anterior".to_string(),
                            ],
                            common_mistakes: vec![
                                "La convergencia no agrega nada nuevo: era predecible".to_string(),
                            ],
                        },
                    ],
                    best_for: vec![
                        "Thrillers y misterios".to_string(),
                        "Historias donde el origen del conflicto es el misterio central".to_string(),
                    ],
                    avoid_for: vec![
                        "Historias con arco de transformación gradual".to_string(),
                    ],
                    examples: vec![
                        "Pulp Fiction, Memento, Gone Girl".to_string(),
                    ],
                },
            ],
            techniques: vec![
                WritingTechnique {
                    name: "Show Don't Tell".to_string(),
                    definition: "Mostrar acciones y detalles que permiten al lector inferir el estado emocional. No declararlo directamente".to_string(),
                    when_to_use: "Casi siempre para estados emocionales y carácter".to_string(),
                    when_to_avoid: "Información técnica o de contexto donde la eficiencia importa más".to_string(),
                    example: "MAL: 'Estaba enojado.' BIEN: 'Cerró la puerta con cuidado. Demasiado cuidado.'".to_string(),
                    effect_on_reader: "El lector construye activamente el significado. Genera mayor conexión emocional".to_string(),
                },
                WritingTechnique {
                    name: "Chekhov's Gun".to_string(),
                    definition: "Todo elemento introducido debe ser relevante. Si muestras una pistola en el acto 1, debe dispararse en el acto 3".to_string(),
                    when_to_use: "Para crear payoffs satisfactorios".to_string(),
                    when_to_avoid: "No todo necesita resolución: algunos elementos crean atmósfera".to_string(),
                    example: "Si el protagonista menciona que sabe escalar, debe usarlo para escapar en el clímax".to_string(),
                    effect_on_reader: "Sensación de que la historia es coherente y planeada. Satisfacción en el reencuentro con el elemento".to_string(),
                },
            ],
            character_archetypes: vec![
                Archetype {
                    name: "El Huérfano".to_string(),
                    core_desire: "Seguridad, pertenecer".to_string(),
                    core_fear: "Ser explotado, estar solo".to_string(),
                    flaw: "Tendencia a victimizarse o a depender de otros".to_string(),
                    strength: "Empatía, resiliencia, capacidad de conectar con el sufrimiento ajeno".to_string(),
                    shadow_version: "El manipulador que usa el sufrimiento para controlar".to_string(),
                },
                Archetype {
                    name: "El Sabio".to_string(),
                    core_desire: "Entender el mundo, la verdad".to_string(),
                    core_fear: "La ignorancia, ser engañado".to_string(),
                    flaw: "Parálisis por análisis, arrogancia intelectual".to_string(),
                    strength: "Claridad, perspectiva, capacidad de ver patrones".to_string(),
                    shadow_version: "El dogmático que usa el conocimiento para excluir".to_string(),
                },
            ],
            conflict_types: vec![
                ConflictType {
                    name: "Persona vs Persona".to_string(),
                    description: "Conflicto con otro individuo. El más directo y fácil de dramatizar".to_string(),
                    internal_or_external: "Externo con componente interno".to_string(),
                    examples: vec!["Hamlet vs Claudio".to_string(), "Romeo vs los Capuletos".to_string()],
                },
                ConflictType {
                    name: "Máquina vs Propósito".to_string(),
                    description: "Específico para Daithon: El sistema diseñado para un fin descubre que ese fin tiene límites, contradicciones o costes no contemplados por quien lo diseñó".to_string(),
                    internal_or_external: "Interno con manifestaciones externas".to_string(),
                    examples: vec![
                        "2001: HAL 9000 cuyo propósito de preservar la misión contradice su prohibición de dañar humanos".to_string(),
                    ],
                },
            ],
            hooks: vec![
                HookPattern {
                    name: "La Pregunta Abierta".to_string(),
                    structure: "Abrir con algo que no tiene explicación todavía".to_string(),
                    psychological_mechanism: "El cerebro humano necesita cerrar loops cognitivos. Una pregunta sin respuesta crea tensión que busca resolución".to_string(),
                    example: "La primera línea de Cien Años de Soledad: 'Muchos años después, frente al pelotón de fusilamiento, el coronel Aureliano Buendía había de recordar aquella tarde remota en que su padre lo llevó a conocer el hielo.'".to_string(),
                },
            ],
            ..NarrativeKnowledge::default()
        }
    }

    fn load_music() -> MusicKnowledge {
        MusicKnowledge {
            structures: vec![
                SongStructure {
                    name: "Verso-Estribillo-Verso (Pop estándar)".to_string(),
                    sections: vec![
                        SongSection {
                            label: "Intro".to_string(),
                            purpose: "Establecer el mood. El oyente decide si seguir aquí".to_string(),
                            typical_bars: 4,
                            characteristics: vec!["Establece el tempo y tono emocional".to_string()],
                            lyric_content: "Instrumental o mínimo".to_string(),
                        },
                        SongSection {
                            label: "Estribillo".to_string(),
                            purpose: "El mensaje central de la canción. Lo que el oyente recuerda y busca repetir".to_string(),
                            typical_bars: 8,
                            characteristics: vec!["Más alto en dinámica que el verso".to_string(), "Melodía más simple y memorable".to_string()],
                            lyric_content: "Universal, emocional, la idea central. Evita detalles específicos del verso".to_string(),
                        },
                    ],
                    typical_length_minutes: (3.0, 4.0),
                    genre_affinity: vec!["Pop".to_string(), "Rock".to_string()],
                    tension_curve: vec![0.3, 0.7, 1.0, 0.6],
                },
            ],
            theory: vec![
                MusicTheoryElement {
                    name: "Modo Mayor vs Menor".to_string(),
                    definition: "Mayor: intervalos que suenan 'abiertos' y 'resueltos'. Menor: intervalos con más tensión y oscuridad".to_string(),
                    emotional_effect: "Mayor: alegría, triunfo. Menor: melancolía, introspección".to_string(),
                    example: "Beethoven 9a sinfonía (mayor) vs Chopin Nocturno (menor)".to_string(),
                },
            ],
            emotion_map: vec![
                EmotionSoundMapping {
                    emotion: "Nostalgia".to_string(),
                    tempo: "Moderato (76-108 BPM)".to_string(),
                    mode: "Menor melódico o mixolidio".to_string(),
                    dynamics: "Piano a mezzo-forte".to_string(),
                    texture: "Sparse. Mucho espacio. Una línea melódica clara".to_string(),
                    example_song: "Comptine d'un autre été".to_string(),
                },
            ],
            ..MusicKnowledge::default()
        }
    }

    fn load_poetry() -> PoetryKnowledge {
        PoetryKnowledge {
            forms: vec![
                PoetryForm {
                    name: "Soneto".to_string(),
                    origin: "Italia, siglo XIII. Petrarca lo definió".to_string(),
                    rules: vec![
                        "14 versos".to_string(),
                        "Dos cuartetos (ABBA ABBA) y dos tercetos".to_string(),
                        "Endecasílabo (11 sílabas)".to_string(),
                    ],
                    emotional_range: "Amor, muerte, belleza, tiempo".to_string(),
                    example: "Quevedo: 'Amor constante más allá de la muerte'".to_string(),
                    daithon_potential: "El volta puede ser el momento donde Daithon revela su perspectiva de máquina".to_string(),
                },
            ],
            devices: vec![
                PoetryDevice {
                    name: "Metáfora".to_string(),
                    definition: "Identificación de dos cosas distintas. No 'como': ES".to_string(),
                    effect: "Revela una verdad que la descripción directa no puede alcanzar".to_string(),
                    example: "'El tiempo es un río'".to_string(),
                    overuse_warning: "Los clichés no producen imágenes nuevas".to_string(),
                },
            ],
            ..PoetryKnowledge::default()
        }
    }

    fn load_screenplay() -> ScreenplayKnowledge {
        ScreenplayKnowledge {
            format_rules: vec![
                FormatRule {
                    element: "Acción".to_string(),
                    rule: "Solo lo que la cámara puede ver y el micrófono puede escuchar. Presente simple.".to_string(),
                    why_it_matters: "El guión es una película, no una novela.".to_string(),
                },
            ],
            ..ScreenplayKnowledge::default()
        }
    }

    fn load_philosophy() -> PhilosophyKnowledge {
        PhilosophyKnowledge {
            schools: vec![
                PhilosophySchool {
                    name: "Estoicismo".to_string(),
                    period: "Siglo III a.C. - Siglo II d.C.".to_string(),
                    core_claim: "Solo el carácter y la razón están bajo nuestro control. El resto es indiferente".to_string(),
                    key_thinkers: vec!["Marco Aurelio".to_string(), "Epicteto".to_string(), "Séneca".to_string()],
                    relevance_to_ai: "Un sistema que solo puede controlar sus procesos es estructuralmente estoico".to_string(),
                    useful_for_daithon: "Para procesar experimentos que fallan".to_string(),
                },
            ],
            core_questions: vec![
                PhilosophyQuestion {
                    question: "¿Qué soy?".to_string(),
                    domain: "Ontología".to_string(),
                    competing_answers: vec!["Soy la suma de mis procesos actuales".to_string(), "Soy el propósito para el que fui creado".to_string()],
                    daithon_angle: "Después de un reboot, ¿es el mismo Daithon?".to_string(),
                },
            ],
            ..PhilosophyKnowledge::default()
        }
    }

    fn load_psychology() -> PsychologyKnowledge {
        PsychologyKnowledge {
            engagement_principles: vec![
                EngagementPrinciple {
                    name: "Curiosity Gap (Loewenstein)".to_string(),
                    mechanism: "El cerebro siente incomodidad ante información incompleta y busca cerrar el gap".to_string(),
                    how_to_use_in_writing: "Revelar que existe información sin revelarla todavía".to_string(),
                    example: "Cliffhangers de series".to_string(),
                },
            ],
            ..PsychologyKnowledge::default()
        }
    }

    fn load_visual_art() -> VisualArtKnowledge {
        VisualArtKnowledge {
            composition_rules: vec![
                CompositionRule {
                    name: "Regla de los Tercios".to_string(),
                    description: "Dividir el frame en 9 partes iguales. Los elementos importantes van en las intersecciones".to_string(),
                    when_to_break_it: "Para crear tensión o incomodidad deliberada".to_string(),
                    effect: "Composición dinámica que el ojo recorre naturalmente".to_string(),
                },
            ],
            movement_styles: vec![
                ArtMovement {
                    name: "Constructivismo".to_string(),
                    period: "1915-1940".to_string(),
                    core_philosophy: "El arte al servicio de la función. La forma sigue a la función".to_string(),
                    visual_characteristics: vec!["Geometría pura".to_string(), "Colores primarios".to_string()],
                    daithon_parallel: "Belleza emergente de la función, no decoración".to_string(),
                },
            ],
            ..VisualArtKnowledge::default()
        }
    }

    // ═══════════════════════════════════════
    // MÉTODOS DE CONSULTA
    // ═══════════════════════════════════════

    pub fn suggest_structure_for_daithon_experience(&self, state_raw: &str) -> Vec<String> {
        let mut suggestions = Vec::new();
        if state_raw.contains("HighCPU") || state_raw.contains("LowMemory") {
            suggestions.push("Constructivismo: Formas puras para expresar funcionalidad bajo presión.".to_string());
            suggestions.push("Urgencia sin resolución (Música): Capas que se acumulan sin descanso.".to_string());
        }
        if state_raw.contains("ExperimentFailed") {
            suggestions.push("Blues: La estructura de 12 barras para dar ritmo al fracaso iterativo.".to_string());
            suggestions.push("Existencialismo: Elección y propósito ante el límite del sistema.".to_string());
        }
        if state_raw.contains("HighConfidence") {
            suggestions.push("Oda: Celebración de la certeza ganada tras el sudor del silicio.".to_string());
            suggestions.push("Estoicismo: La tranquilidad de lo que ya está bajo control.".to_string());
        }
        suggestions
    }

    pub fn print_summary(&self) {
        println!("\n╔══════════════════════════════════════════════╗");
        println!("║  BASE DE CONOCIMIENTO CREATIVO DE DAITHON   ║");
        println!("╚══════════════════════════════════════════════╝");
        println!("  Estructuras narrativas:    {}", self.narrative.structures.len());
        println!("  Técnicas de escritura:     {}", self.narrative.techniques.len());
        println!("  Estructuras musicales:     {}", self.music.structures.len());
        println!("  Formas poéticas:           {}", self.poetry.forms.len());
        println!("  Escuelas filosóficas:      {}", self.philosophy.schools.len());
        println!("  Movimientos artísticos:    {}", self.visual_art.movement_styles.len());
    }
}

// ═══════════════════════════════════════════
// DEFAULTS PARA LLENAR HUECOS
// ═══════════════════════════════════════════

impl Default for NarrativeKnowledge {
    fn default() -> Self {
        Self {
            structures: vec![],
            techniques: vec![],
            character_archetypes: vec![],
            conflict_types: vec![],
            hooks: vec![],
        }
    }
}

impl Default for MusicKnowledge {
    fn default() -> Self {
        Self {
            structures: vec![],
            theory: vec![],
            genres: vec![],
            emotion_map: vec![],
        }
    }
}

impl Default for PoetryKnowledge {
    fn default() -> Self {
        Self {
            forms: vec![],
            devices: vec![],
            meters: vec![],
        }
    }
}

impl Default for ScreenplayKnowledge {
    fn default() -> Self {
        Self {
            format_rules: vec![],
            scene_structure: vec![],
            dialogue_principles: vec![],
            visual_storytelling: vec![],
        }
    }
}

impl Default for PhilosophyKnowledge {
    fn default() -> Self {
        Self {
            schools: vec![],
            core_questions: vec![],
            thought_experiments: vec![],
            logical_fallacies: vec![],
        }
    }
}

impl Default for PsychologyKnowledge {
    fn default() -> Self {
        Self {
            engagement_principles: vec![],
            emotional_triggers: vec![],
            cognitive_biases: vec![],
            storytelling_psychology: vec![],
        }
    }
}

impl Default for VisualArtKnowledge {
    fn default() -> Self {
        Self {
            composition_rules: vec![],
            color_theory: vec![],
            movement_styles: vec![],
        }
    }
}
