use crate::cortex::extraction::knowledge_extractor::*;
use crate::cortex::comprehension::deep_understanding::*;
use crate::trinity::judge::naturalness_evaluator::*;

/// Sistema de aprendizaje desde fuentes web
#[derive(Clone)]
pub struct WebLearningEngine {
    extractor: KnowledgeExtractor,
    pub comprehension: DeepComprehension,
    _judge: NaturalnessJudge,
}

impl WebLearningEngine {
    pub fn new(
        extractor: KnowledgeExtractor,
        comprehension: DeepComprehension,
        judge: NaturalnessJudge,
    ) -> Self {
        Self {
            extractor,
            comprehension,
            _judge: judge,
        }
    }

    /// Aprender de Wikipedia simulado
    pub async fn learn_from_wikipedia(&mut self, topic: &str) -> Option<LearningReport> {
        log::info!("[CORTEX] Aprendiendo de Wikipedia (MOCK): {}", topic);

        let extract = "Mesa es un mueble con cuatro patas. La gravedad es una fuerza de atracción porque tira los objetos hacia el centro.";

        // Extraer conocimiento
        let knowledge = self.extractor.extract_from_text(
            extract,
            KnowledgeSource::Wikipedia {
                article: topic.to_string(),
                section: "summary".to_string(),
            },
        );

        // Integrar en base de conocimiento
        let result = self.comprehension.integrate_knowledge(knowledge);

        Some(LearningReport {
            source: format!("Wikipedia: {}", topic),
            entities_learned: result.entities_added,
            relations_learned: result.relations_added,
            inferences_made: result.relations_inferred,
            contradictions: result.contradictions_found,
        })
    }

    /// Aprender de Reddit simulado
    pub async fn learn_from_reddit(
        &mut self,
        subreddit: &str,
    ) -> Option<LearningReport> {
        log::info!("[CORTEX] Aprendiendo de Reddit: r/{}", subreddit);

        let mock_reddit_texts = [
            "La madera mojada causa roturas porque...",
            "El rojo carmesí es un color increíble",
        ];

        let mut total_entities = 0;
        let mut total_relations = 0;

        for title in mock_reddit_texts {
            let knowledge = self.extractor.extract_from_text(
                title,
                KnowledgeSource::Reddit {
                    subreddit: subreddit.to_string(),
                    thread_id: "mock_id".to_string(),
                },
            );

            let result = self.comprehension.integrate_knowledge(knowledge);
            total_entities += result.entities_added;
            total_relations += result.relations_added;
        }

        Some(LearningReport {
            source: format!("Reddit: r/{}", subreddit),
            entities_learned: total_entities,
            relations_learned: total_relations,
            inferences_made: 0,
            contradictions: 0,
        })
    }

}

#[derive(Debug, Clone)]
pub struct LearningReport {
    pub source: String,
    pub entities_learned: usize,
    pub relations_learned: usize,
    pub inferences_made: usize,
    pub contradictions: usize,
}
