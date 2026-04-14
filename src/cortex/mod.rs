pub mod extraction;
pub mod comprehension;
pub mod sources;

use extraction::knowledge_extractor::*;
use comprehension::deep_understanding::*;
use sources::web_sources::*;
use sources::document_reader::*;
use crate::trinity::judge::naturalness_evaluator::*;

/// Motor CORTEX completo
#[derive(Clone)]
pub struct CortexEngine {
    pub extractor: KnowledgeExtractor,
    pub comprehension: DeepComprehension,
    pub web_learner: WebLearningEngine,
    pub document_learner: DocumentLearner,
}

impl CortexEngine {
    pub fn new(judge: NaturalnessJudge) -> Self {
        let extractor = KnowledgeExtractor::new();
        let comprehension = DeepComprehension::new();

        Self {
            extractor: extractor.clone(),
            comprehension: comprehension.clone(),
            web_learner: WebLearningEngine::new(
                extractor.clone(),
                comprehension.clone(),
                judge,
            ),
            document_learner: DocumentLearner::new(extractor.clone(), comprehension.clone()),
        }
    }

    /// Aprender de múltiples fuentes sobre un tema
    pub async fn deep_learn_topic(&mut self, topic: &str) -> TopicLearningReport {
        log::info!("\n════════════════════════════════════════");
        log::info!("[CORTEX] Aprendizaje profundo: {}", topic);
        log::info!("════════════════════════════════════════\n");

        let mut reports = Vec::new();

        // 1. Wikipedia
        if let Some(report) = self.web_learner.learn_from_wikipedia(topic).await {
            reports.push(("Wikipedia".to_string(), report));
        }

        // 2. Reddit (para lenguaje natural sobre el tema)
        if let Some(report) = self.web_learner.learn_from_reddit(topic).await {
            reports.push(("Reddit".to_string(), report));
        }

        // Calcular totales
        let total_entities: usize = reports.iter().map(|(_, r)| r.entities_learned).sum();
        let total_relations: usize = reports.iter().map(|(_, r)| r.relations_learned).sum();
        let total_inferences: usize = reports.iter().map(|(_, r)| r.inferences_made).sum();

        log::info!("\n════════════════════════════════════════");
        log::info!("✅ Aprendizaje completado");
        log::info!("   Entidades: {}", total_entities);
        log::info!("   Relaciones: {}", total_relations);
        log::info!("   Inferencias: {}", total_inferences);
        log::info!("════════════════════════════════════════\n");

        TopicLearningReport {
            topic: topic.to_string(),
            sources_used: reports.len(),
            total_entities,
            total_relations,
            total_inferences,
        }
    }

    /// Proveer el laboratorio empírico con Unreal Engine (GLYPHICA)
    pub fn verify_empirically(&mut self, entity: &str, valid: bool) {
        if let Some(node) = self.comprehension.knowledge_base.ontology.get_mut(entity) {
            if valid {
                node.confidence = 0.99;
                log::info!("[CORTEX Laboratorio] '{}' ha sido verificado empíricamente con simulador físico.", entity);
            } else {
                node.confidence = 0.1;
                log::warn!("[CORTEX Laboratorio] La simulación desmiente a '{}'.", entity);
            }
        }
    }

    pub fn execute_maintenance(&mut self) {
        self.comprehension.execute_synaptic_pruning();
    }
}

#[derive(Debug)]
pub struct TopicLearningReport {
    pub topic: String,
    pub sources_used: usize,
    pub total_entities: usize,
    pub total_relations: usize,
    pub total_inferences: usize,
}
