//! LINGUA — Motor Gramatical Autónomo
//!
//! Enseña a Daithon reglas estructurales completas (Sintaxis, Morfología, Semántica).
//! Permite que Daithon abandone frases rígidas y construya sus propias oraciones
//! conjugando verbos y ensamblando sujetos/predicados según reglas, simulando
//! el entendimiento fluido del diccionario base que adquirió.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct GrammarEngine {
    pub total_words_known: usize,
    pub grammar_rules_mastered: usize,
    pub verb_conjugations: HashMap<String, VerbConjugation>,
}

#[derive(Debug, Clone)]
pub struct VerbConjugation {
    root: String,
    present_1p: String,
    present_3p: String,
    past_1p: String,
    future_1p: String,
}

impl GrammarEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            total_words_known: 261, 
            grammar_rules_mastered: 45,
            verb_conjugations: HashMap::new(),
        };
        engine.load_irregular_verbs();
        engine
    }

    /// Carga verbos irregulares que no siguen las reglas estándar
    fn load_irregular_verbs(&mut self) {
        self.verb_conjugations.insert("hacer".into(), VerbConjugation {
            root: "hacer".into(),
            present_1p: "hago".into(),
            present_3p: "hace".into(),
            past_1p: "hice".into(),
            future_1p: "haré".into(),
        });
        
        self.verb_conjugations.insert("ser".into(), VerbConjugation {
            root: "ser".into(),
            present_1p: "soy".into(),
            present_3p: "es".into(),
            past_1p: "fui".into(),
            future_1p: "seré".into(),
        });

        self.verb_conjugations.insert("estar".into(), VerbConjugation {
            root: "estar".into(),
            present_1p: "estoy".into(),
            present_3p: "está".into(),
            past_1p: "estuve".into(),
            future_1p: "estaré".into(),
        });

        self.verb_conjugations.insert("ir".into(), VerbConjugation {
            root: "ir".into(),
            present_1p: "voy".into(),
            present_3p: "va".into(),
            past_1p: "fui".into(),
            future_1p: "iré".into(),
        });
    }

    /// Obtiene la conjugación: Primero busca irregularidades, luego aplica reglas morfológicas
    pub fn conjugate(&self, verb_root: &str, tense: &str) -> String {
        // 1. Irregulares
        if let Some(conj) = self.verb_conjugations.get(verb_root) {
            match tense {
                "present_1p" => return conj.present_1p.clone(),
                "present_3p" => return conj.present_3p.clone(),
                "past_1p" => return conj.past_1p.clone(),
                "future_1p" => return conj.future_1p.clone(),
                _ => return conj.root.clone(),
            }
        }

        // 2. Reglas Morfológicas para verbos regulares
        let v = verb_root.to_lowercase();
        if v.len() < 3 { return v; }
        
        let stem = &v[..v.len()-2];
        let ending = &v[v.len()-2..];

        match ending {
            "ar" => match tense {
                "present_1p" => format!("{}o", stem),
                "present_3p" => format!("{}a", stem),
                "past_1p"    => format!("{}é", stem),
                "future_1p"  => format!("{}aré", stem),
                _ => v,
            },
            "er" => match tense {
                "present_1p" => format!("{}o", stem),
                "present_3p" => format!("{}e", stem),
                "past_1p"    => format!("{}í", stem),
                "future_1p"  => format!("{}eré", stem),
                _ => v,
            },
            "ir" => match tense {
                "present_1p" => format!("{}o", stem),
                "present_3p" => format!("{}e", stem),
                "past_1p"    => format!("{}í", stem),
                "future_1p"  => format!("{}iré", stem),
                _ => v,
            },
            _ => v,
        }
    }

    /// Ajusta un adjetivo para que coincida con el género y número del sustantivo (Concordancia)
    pub fn agree_adjective(&self, adjective: &str, gender: &crate::lingua::acquisition::Gender, number: &crate::lingua::acquisition::WordNumber) -> String {
        let mut adj = adjective.to_lowercase();
        if adj.is_empty() { return adj; }

        // Regla general de género (o -> a)
        if *gender == crate::lingua::acquisition::Gender::Feminine {
            if adj.ends_with('o') {
                adj.pop();
                adj.push('a');
            } else if adj.ends_with("dor") {
                adj.push('a');
            }
        }

        // Regla general de número (singular -> plural)
        if *number == crate::lingua::acquisition::WordNumber::Plural {
            if adj.ends_with('a') || adj.ends_with('e') || adj.ends_with('o') {
                adj.push('s');
            } else if adj.ends_with('r') || adj.ends_with('l') || adj.ends_with('n') {
                adj.push_str("es");
            } else if adj.ends_with('z') {
                adj.pop();
                adj.push_str("ces");
            }
        }

        adj
    }

    /// Ajusta un artículo (el, la, un, una) según género y número
    pub fn agree_article(&self, article_root: &str, gender: &crate::lingua::acquisition::Gender, number: &crate::lingua::acquisition::WordNumber) -> String {
        let root = article_root.to_lowercase();
        match (root.as_str(), gender, number) {
            ("el" | "la", crate::lingua::acquisition::Gender::Masculine, crate::lingua::acquisition::WordNumber::Singular) => "el".into(),
            ("el" | "la", crate::lingua::acquisition::Gender::Feminine, crate::lingua::acquisition::WordNumber::Singular) => "la".into(),
            ("el" | "la", crate::lingua::acquisition::Gender::Masculine, crate::lingua::acquisition::WordNumber::Plural) => "los".into(),
            ("el" | "la", crate::lingua::acquisition::Gender::Feminine, crate::lingua::acquisition::WordNumber::Plural) => "las".into(),
            
            ("un" | "una", crate::lingua::acquisition::Gender::Masculine, crate::lingua::acquisition::WordNumber::Singular) => "un".into(),
            ("un" | "una", crate::lingua::acquisition::Gender::Feminine, crate::lingua::acquisition::WordNumber::Singular) => "una".into(),
            ("un" | "una", crate::lingua::acquisition::Gender::Masculine, crate::lingua::acquisition::WordNumber::Plural) => "unos".into(),
            ("un" | "una", crate::lingua::acquisition::Gender::Feminine, crate::lingua::acquisition::WordNumber::Plural) => "unas".into(),
            
            _ => root,
        }
    }

    /// Construye una frase orgánica usando sintaxis S-V-O con concordancia
    pub fn construct_sentence(&self, subject: &str, verb: &str, tense: &str, object: &str, modifier: &str) -> String {
        log::info!("[GRAMMAR] Ensamblando: S='{}', V='{}', O='{}', Mod='{}'", subject, verb, object, modifier);
        
        let verb_str = self.conjugate(verb, tense);
        
        let mut sentence = if subject.is_empty() {
            format!("{} {}", verb_str, object)
        } else {
            format!("{} {} {}", subject, verb_str, object)
        };

        if !modifier.is_empty() {
            sentence = format!("{} {}.", sentence.trim(), modifier);
        } else {
            sentence = format!("{}.", sentence.trim());
        }

        // Capitalizar
        let mut chars = sentence.chars();
        match chars.next() {
            None => String::new(),
            Some(f) => f.to_uppercase().collect::<String>() + chars.as_str(),
        }
    }
    /// Determina si una palabra es una partícula gramatical común que no requiere explicación
    pub fn is_common_particle(&self, word: &str) -> bool {
        let common = ["el", "la", "los", "las", "un", "una", "unos", "unas", "de", "del", "que", "qué", "y", "o", "a", "ante", "bajo", "con", "contra", "en", "entre", "hacia", "hasta", "para", "por", "según", "sin", "sobre", "tras"];
        common.contains(&word.to_lowercase().as_str())
    }
}
