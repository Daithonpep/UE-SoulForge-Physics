//! LINGUA — Modismos Guatemaltecos
//! 
//! Proporciona un conjunto de modismos y expresiones de Guatemala para que 
//! Daithon aprenda a hablar con un sabor local cuando su curiosidad o entrenamiento lo requiera.

use std::collections::HashMap;
use crate::lingua::acquisition::{AcquisitionEngine, AcquiredWord, Gender, WordNumber};

pub struct GuatemalaImmersion;

impl GuatemalaImmersion {
    /// Inyecta modismos guatemaltecos en el AcquisitionEngine
    pub fn inject(engine: &mut AcquisitionEngine) {
        let idioms = vec![
            ("chilero", "adjective", "Algo que es muy bueno, bonito o excelente.", vec!["genial", "bueno", "bonito"]),
            ("chispudo", "adjective", "Persona que es inteligente, rápida o astuta.", vec!["listo", "astuto", "rápido"]),
            ("patojo", "noun", "Un niño o joven.", vec!["niño", "joven", "muchacho"]),
            ("puechica", "interjection", "Expresión de asombro, enojo o sorpresa (suave).", vec!["vaya", "sorpresa", "asombro"]),
            ("púchica", "interjection", "Expresión de asombro, enojo o sorpresa (suave).", vec!["vaya", "sorpresa", "asombro"]),
            ("clavo", "noun", "Un problema o dificultad.", vec!["problema", "dificultad", "lío"]),
            ("chish", "interjection", "Expresión de asco o desagrado.", vec!["asco", "feo"]),
            ("sho", "interjection", "Imperativo para pedir silencio (shh).", vec!["silencio", "callate"]),
            ("cerote", "noun", "Persona (uso coloquial entre amigos, puede ser ofensivo si no hay confianza).", vec!["amigo", "persona", "sujeto"]),
            ("chuchos", "noun", "Perros o también personas que comen mucho.", vec!["perro", "glotón"]),
            ("shute", "adjective", "Persona que es entrometida o curiosa.", vec!["entrometido", "curioso"]),
            ("casaca", "noun", "Una mentira o algo que no es cierto.", vec!["mentira", "engaño"]),
            ("pajizo", "adjective", "Alguien que dice mentiras.", vec!["mentiroso"]),
            ("bochinche", "noun", "Un pleito, desorden o ruido fuerte.", vec!["desorden", "ruido", "pleito"]),
            ("canche", "adjective", "Persona de pelo rubio.", vec!["rubio"]),
            ("chapín", "noun", "Gentilicio coloquial para una persona de Guatemala.", vec!["guatemalteco"]),
        ];

        for (word, pos, meaning, synonyms) in idioms {
            let gender = if word.ends_with('a') { Gender::Feminine } else { Gender::Masculine };
            
            let mut entry = AcquiredWord {
                word: word.to_string(),
                language: "es".to_string(),
                part_of_speech: vec![pos.to_string()],
                meanings: vec![meaning.to_string()],
                synonyms: synonyms.iter().map(|s| s.to_string()).collect(),
                examples: Vec::new(),
                antonyms: Vec::new(),
                related_terms: Vec::new(),
                gender,
                number: WordNumber::Singular,
                is_visually_grounded: false,
                structural_features: Vec::new(),
                design_category: None,
                grounded_concept_id: None,
            };

            // Heurística de categorías 
            if pos == "adjective" {
                entry.design_category = Some(crate::lingua::acquisition::DesignCategory::Property);
            } else if pos == "noun" {
                entry.design_category = Some(crate::lingua::acquisition::DesignCategory::Uncategorized);
            }

            engine.update_entry(entry);
        }
        
        log::info!("[LINGUA] Inyección de modismos guatemaltecos completada.");
    }
}
