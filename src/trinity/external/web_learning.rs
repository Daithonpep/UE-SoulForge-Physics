use crate::trinity::judge::naturalness_evaluator::*;

/// Sistema de aprendizaje desde fuentes web (Simulado temporalmente sin peticiones HTTP reales para evitar dependencias extra)
pub struct WebLearningSystem {
    judge: NaturalnessJudge,
    _approved_sources: Vec<String>,
}

impl WebLearningSystem {
    pub fn new(judge: NaturalnessJudge) -> Self {
        Self {
            judge,
            _approved_sources: vec![
                "https://en.wikipedia.org".to_string(),
                "https://arxiv.org".to_string(),
                "https://www.reddit.com/r/science".to_string(),
            ],
        }
    }

    /// Aprender de Wikipedia (Simulado)
    pub async fn learn_from_wikipedia(&mut self, topic: &str) -> Result<Vec<String>, Box<dyn std::error::Error>> {
        // Simulación: En lugar de hacer scraping real (que requiere reqwest + scraper),
        // proveemos algunas frases precargadas como si hubieran sido extraidas.
        
        let sim_text = vec![
            "El diseño estructural requiere consideración cuidadosa de las fuerzas físicas.",
            "Una mesa típica tiene cuatro patas para maximizar la estabilidad asumiendo una superficie plana.",
            "En la arquitectura, la forma frecuentemente sigue a la función de manera intrínseca.",
            "Este es un texto corto ignorado.",
            "Los objetos aislados en 3D presentan desafíos distintos respecto a la colisión ambiental."
        ];

        let mut approved_sentences = Vec::new();

        for sentence in &sim_text {
            let trimmed = sentence.trim();
            if trimmed.len() < 10 {
                continue;
            }

            let context = ConversationContext {
                previous_exchanges: vec![],
                topic: topic.to_string(),
                emotional_tone: "informative".to_string(),
                formality_level: 0.7,
            };

            let eval = self.judge.evaluate_naturalness("WebSource", trimmed, &context);

            if eval.overall_score > 0.6 {
                approved_sentences.push(trimmed.to_string());
            }
        }

        log::info!("[WEB LEARNING] Aprobadas {} oraciones de Wikipedia sobre '{}'", 
                 approved_sentences.len(), topic);

        Ok(approved_sentences)
    }
}
