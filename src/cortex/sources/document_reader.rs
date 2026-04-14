use crate::cortex::extraction::knowledge_extractor::*;
use crate::cortex::comprehension::deep_understanding::*;

/// Lector y aprendiz de documentos
#[derive(Clone)]
pub struct DocumentLearner {
    extractor: KnowledgeExtractor,
    pub comprehension: DeepComprehension,
}

impl DocumentLearner {
    pub fn new(
        extractor: KnowledgeExtractor,
        comprehension: DeepComprehension,
    ) -> Self {
        Self {
            extractor,
            comprehension,
        }
    }

    /// Aprender de un archivo local (Simulado)
    pub fn learn_from_file(&mut self, file_path: &str) -> DocumentLearningReport {
        log::info!("[CORTEX] Leyendo documento: {}", file_path);

        let section = "Ejemplo de texto extraido de un documento. El sol proyecta luz que permite la fotosíntesis.";

        let mut total_entities = 0;
        let mut total_relations = 0;
        let mut total_causal = 0;

        let knowledge = self.extractor.extract_from_text(
            section,
            KnowledgeSource::Document {
                filename: file_path.to_string(),
                page: 1,
            },
        );

        let result = self.comprehension.integrate_knowledge(knowledge);

        total_entities += result.entities_added;
        total_relations += result.relations_added;
        total_causal += result.causal_chains_added;

        DocumentLearningReport {
            filename: file_path.to_string(),
            sections_processed: 1,
            entities_learned: total_entities,
            relations_learned: total_relations,
            skills_learned: 0,
            causal_chains_learned: total_causal,
        }
    }
}

#[derive(Debug, Clone)]
pub struct DocumentLearningReport {
    pub filename: String,
    pub sections_processed: usize,
    pub entities_learned: usize,
    pub relations_learned: usize,
    pub skills_learned: usize,
    pub causal_chains_learned: usize,
}
