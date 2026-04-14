use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MorphologicalCategory {
    Sustantivo(NounType),
    Articulo(Definiteness),
    Adjetivo,
    Pronombre,
    Verbo(VerbFlexion),
    Preposicion,
    Conjuncion,
    Adverbio,
    Desconocido,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum NounType { Propi, Comun, Abstracto }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Definiteness { Definido, Indefinido }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct VerbFlexion {
    pub persona: Person,
    pub numero: Number,
    pub tiempo: Tense,
    pub modo: Mood,
    pub aspecto: Aspect,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Person { Primera, Segunda, Tercera }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Number { Singular, Plural }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Tense { Pasado, Presente, Futuro }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Mood { Indicativo, Subjuntivo, Imperativo }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Aspect { Perfecto, Imperfecto }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SyntacticRole {
    pub role: RoleType,
    pub words: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum RoleType {
    Sujeto,
    PredicadoNucleo,
    ObjetoDirecto,
    ObjetoIndirecto,
    Circunstancial(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedSentence {
    pub raw: String,
    pub roles: Vec<SyntacticRole>,
    pub morphological_tags: Vec<(String, MorphologicalCategory)>,
}

pub struct SyntacticEngine;

impl SyntacticEngine {
    pub fn new() -> Self {
        Self
    }

    /// Analizador de Flexión Verbal (Hack Lingüístico para Daithon)
    pub fn analyze_verb(&self, word: &str) -> Option<VerbFlexion> {
        let w = word.to_lowercase();
        // Ejemplos básicos para demostrar la capacidad de modo subjuntivo y flexión múltiple.
        if w.ends_with("mos") {
            // Ejemplo rápido 1P Plural
            let modo = if w.ends_with("emos") || w.ends_with("amos") && w != "estamos" { Mood::Subjuntivo } else { Mood::Indicativo };
            Some(VerbFlexion {
                persona: Person::Primera,
                numero: Number::Plural,
                tiempo: Tense::Presente,
                modo,
                aspecto: Aspect::Imperfecto,
            })
        } else if w.ends_with("é") || w.ends_with("í") {
            Some(VerbFlexion {
                persona: Person::Primera,
                numero: Number::Singular,
                tiempo: Tense::Pasado,
                modo: Mood::Indicativo,
                aspecto: Aspect::Perfecto, // Acción terminada
            })
        } else if w == "espero" || w == "es" {
            Some(VerbFlexion {
                persona: if w == "espero" { Person::Primera } else { Person::Tercera },
                numero: Number::Singular,
                tiempo: Tense::Presente,
                modo: Mood::Indicativo,
                aspecto: Aspect::Imperfecto,
            })
        } else if w.starts_with("aprenda") || w.starts_with("sea") {
            // Subjuntivo (Deseos/Hipotéticos)
            Some(VerbFlexion {
                persona: Person::Tercera, // Podríamos ajustarlo según s, pero dejémoslo general para la prueba
                numero: Number::Singular,
                tiempo: Tense::Presente,
                modo: Mood::Subjuntivo, 
                aspecto: Aspect::Imperfecto,
            })
        } else if w.ends_with("o") {
             Some(VerbFlexion {
                persona: Person::Primera,
                numero: Number::Singular,
                tiempo: Tense::Presente,
                modo: Mood::Indicativo,
                aspecto: Aspect::Imperfecto,
            })
        } else {
            None
        }
    }

    /// Identificación de conectores
    pub fn is_connector(&self, word: &str) -> bool {
        let preps = ["a", "ante", "bajo", "cabe", "con", "contra", "de", "desde", "en", "entre", "hacia", "hasta", "para", "por", "según", "sin", "so", "sobre", "tras"];
        preps.contains(&word.to_lowercase().as_str())
    }

    /// Asignación de Roles Sintácticos Básicos (Ingeniería de la oración)
    pub fn parse_sentence(&self, sentence: &str) -> ParsedSentence {
        let words: Vec<&str> = sentence.split_whitespace().collect();
        let mut morph_tags = Vec::new();
        let mut roles = Vec::new();

        let mut current_role = RoleType::Sujeto;
        let mut current_words = Vec::new();

        for word in &words {
            let clean = word.replace(&['.', ',', '!', '?'][..], "").to_lowercase();
            
            // Determinar Morfología
            let morph = if self.is_connector(&clean) {
                MorphologicalCategory::Preposicion
            } else if let Some(flex) = self.analyze_verb(&clean) {
                MorphologicalCategory::Verbo(flex)
            } else if ["el", "la", "los", "las"].contains(&clean.as_str()) {
                MorphologicalCategory::Articulo(Definiteness::Definido)
            } else if ["un", "una", "unos", "unas"].contains(&clean.as_str()) {
                MorphologicalCategory::Articulo(Definiteness::Indefinido)
            } else if ["yo", "tú", "él", "ella", "nosotros", "ellos"].contains(&clean.as_str()) {
                MorphologicalCategory::Pronombre
            } else {
                MorphologicalCategory::Sustantivo(NounType::Comun) // Fallback simplificado
            };

            morph_tags.push((word.to_string(), morph.clone()));

            // Mecánica Sintáctica simple basada en transiciones
            match morph {
                MorphologicalCategory::Verbo(_) => {
                    // Si encontramos verbo, cerramos lo anterior y abrimos Núcleo del Predicado
                    if !current_words.is_empty() {
                        roles.push(SyntacticRole { role: current_role.clone(), words: current_words.clone() });
                        current_words.clear();
                    }
                    current_role = RoleType::PredicadoNucleo;
                    current_words.push(word.to_string());
                    
                    // Siguiente token probablemente inicie el Objeto (Directo o Circunstancial)
                    roles.push(SyntacticRole { role: current_role.clone(), words: current_words.clone() });
                    current_words.clear();
                    current_role = RoleType::ObjetoDirecto; 
                }
                MorphologicalCategory::Preposicion => {
                    if !current_words.is_empty() {
                        roles.push(SyntacticRole { role: current_role.clone(), words: current_words.clone() });
                        current_words.clear();
                    }
                    current_role = RoleType::Circunstancial(clean.clone());
                    current_words.push(word.to_string());
                }
                _ => {
                    current_words.push(word.to_string());
                }
            }
        }

        if !current_words.is_empty() {
            roles.push(SyntacticRole { role: current_role, words: current_words });
        }

        ParsedSentence {
            raw: sentence.to_string(),
            roles,
            morphological_tags: morph_tags,
        }
    }
}
