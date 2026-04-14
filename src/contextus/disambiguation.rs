use crate::contextus::memory::*;
use serde::{Deserialize, Serialize};

/// Resuelve términos con múltiples significados
pub struct PolysemyResolver;

impl PolysemyResolver {
    /// Base de conocimiento de polisemia
    pub fn get_known_senses(term: &str) -> Vec<WordSense> {
        let term_lower = term.to_lowercase();

        match term_lower.as_str() {
            "arco" => vec![
                WordSense {
                    sense_id: "arco_arquitectura".into(),
                    label: "Arco (arquitectura)".into(),
                    definition: "Estructura curva que soporta peso, usada en puentes, puertas, ventanas".into(),
                    categories: vec!["arquitectura".into(), "estructura".into(), "forma".into(), "construcción".into(), "arquitectura/estructura".into()],
                    keywords: vec!["puente".into(), "puerta".into(), "bóveda".into(), "columna".into(), "piedra".into(), "carga".into()],
                },
                WordSense {
                    sense_id: "arco_arma".into(),
                    label: "Arco (arma)".into(),
                    definition: "Arma que lanza proyectiles mediante tensión de una cuerda".into(),
                    categories: vec!["arma".into(), "proyectil".into(), "tiro".into(), "armas/proyectil".into()],
                    keywords: vec!["flecha".into(), "cuerda".into(), "tiro".into(), "caza".into(), "guerra".into(), "arquero".into()],
                },
                WordSense {
                    sense_id: "arco_geometria".into(),
                    label: "Arco (geometría)".into(),
                    definition: "Segmento de curva, parte de una circunferencia".into(),
                    categories: vec!["geometría".into(), "matemáticas".into(), "forma".into(), "geometría/forma".into()],
                    keywords: vec!["circunferencia".into(), "ángulo".into(), "radio".into(), "curva".into()],
                },
                WordSense {
                    sense_id: "arco_iris".into(),
                    label: "Arcoíris".into(),
                    definition: "Fenómeno óptico de colores en el cielo".into(),
                    categories: vec!["física".into(), "óptica".into(), "naturaleza".into(), "ciencia/física".into()],
                    keywords: vec!["iris".into(), "colores".into(), "lluvia".into(), "luz".into()],
                },
            ],
            "suno" => vec![
                WordSense {
                    sense_id: "suno_ai".into(),
                    label: "Suno (IA)".into(),
                    definition: "Plataforma de IA para generación de música y audio".into(),
                    categories: vec!["audio".into(), "música".into(), "IA".into(), "generación".into(), "audio/música".into(), "IA/generación".into()],
                    keywords: vec!["música".into(), "audio".into(), "IA".into(), "generación".into(), "canción".into(), "modelo".into()],
                },
                WordSense {
                    sense_id: "suno_localidad".into(),
                    label: "Suno (Italia)".into(),
                    definition: "Localidad italiana en la provincia de Novara, ~2,000 habitantes".into(),
                    categories: vec!["geografía".into(), "localidad".into(), "italia".into(), "geografía/localidad".into()],
                    keywords: vec!["italia".into(), "novara".into(), "pueblo".into(), "habitantes".into()],
                },
            ],
            "rust" => vec![
                WordSense {
                    sense_id: "rust_lenguaje".into(),
                    label: "Rust (lenguaje)".into(),
                    definition: "Lenguaje de programación de sistemas, seguro y rápido".into(),
                    categories: vec!["programación".into(), "lenguaje".into(), "sistemas".into(), "programación/lenguaje".into()],
                    keywords: vec!["código".into(), "compilador".into(), "seguridad".into(), "cargo".into()],
                },
                WordSense {
                    sense_id: "rust_oxidación".into(),
                    label: "Rust (óxido)".into(),
                    definition: "Óxido de hierro, corrosión de metales".into(),
                    categories: vec!["química".into(), "corrosión".into(), "metal".into(), "ciencia/química".into()],
                    keywords: vec!["óxido".into(), "hierro".into(), "corrosión".into(), "metal".into()],
                },
            ],
            _ => vec![],
        }
    }

    /// Resolver el significado correcto basándose en contexto
    pub fn resolve(term: &str, context: &WorkingMemory) -> Option<WordSense> {
        let senses = Self::get_known_senses(term);

        if senses.is_empty() {
            return None;
        }

        if senses.len() == 1 {
            return Some(senses[0].clone());
        }

        // Puntuar cada sentido
        let mut scored: Vec<(WordSense, f64)> = senses.into_iter().map(|s| {
            let score = Self::score_sense(&s, context);
            (s, score)
        }).collect();

        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let (best_sense, best_score) = &scored[0];

        if *best_score > 0.3 {
            Some(best_sense.clone())
        } else {
            None
        }
    }

    fn score_sense(sense: &WordSense, context: &WorkingMemory) -> f64 {
        let mut score = 0.0;

        // 1. Coincidencia con anclas semánticas
        if let Some(anchor) = context.semantic_anchors.get(&sense.label.split(' ').next().unwrap_or("").to_lowercase()) {
            for category in &anchor.categories {
                if sense.categories.contains(category) {
                    score += 0.4;
                }
            }
        }

        // 2. Coincidencia con categorías de entidades activas
        for entity in &context.active_entities {
            for category in &entity.categories {
                if sense.categories.contains(category) {
                    score += 0.2;
                }
            }
        }

        // 3. Coincidencia con keywords en historial reciente
        for msg in context.thread_history.iter().rev().take(5) {
            let msg_lower = msg.content.to_lowercase();
            for keyword in &sense.keywords {
                if msg_lower.contains(&keyword.to_lowercase()) {
                    score += 0.30;
                }
            }
        }

        // 4. Coincidencia con tema del hilo
        if let Some(topic) = &context.thread_topic {
            for keyword in &sense.keywords {
                if topic.to_lowercase().contains(&keyword.to_lowercase()) {
                    score += 0.3;
                }
            }
        }

        // 5. Coincidencia con documentos activos
        for doc in &context.active_documents {
            for keyword in &sense.keywords {
                if doc.content_summary.to_lowercase().contains(&keyword.to_lowercase()) {
                    score += 0.2;
                }
            }
        }

        score
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WordSense {
    pub sense_id: String,
    pub label: String,
    pub definition: String,
    pub categories: Vec<String>,
    pub keywords: Vec<String>,
}
