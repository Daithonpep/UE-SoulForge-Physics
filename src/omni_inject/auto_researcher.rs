use super::semantic_vectors::*;
use super::conceptnet_importer::*;

/// Sistema de investigación automática
pub struct ActiveResearchEngine {
    pub embeddings: SemanticEmbeddingEngine,
    pub conceptnet: ConceptNetImporter,
    confidence_threshold: f32,
}

impl ActiveResearchEngine {
    pub fn new(embeddings: SemanticEmbeddingEngine) -> Self {
        Self {
            embeddings,
            conceptnet: ConceptNetImporter::new(),
            confidence_threshold: 0.3,
        }
    }

    /// Detectar palabra desconocida y activar investigación
    pub async fn handle_unknown_word(
        &mut self,
        word: &str,
        kb: &mut crate::cortex::comprehension::deep_understanding::KnowledgeBase,
    ) -> ResearchResult {
        println!("[RESEARCH] Palabra desconocida detectada: '{}'", word);

        // 1. Intentar inferir por embeddings
        if let Some(inference) = self.embeddings.infer_unknown_word(word) {
            if inference.confidence > self.confidence_threshold {
                println!("  ✓ Inferencia por similitud: {} (confianza: {:.2})", 
                         inference.inferred_meaning, inference.confidence);
                
                return ResearchResult {
                    word: word.to_string(),
                    learned: true,
                    method: "semantic_inference".to_string(),
                    definition: inference.inferred_meaning,
                    confidence: inference.confidence as f64,
                };
            }
        }

        // 2. Buscar en ConceptNet
        println!("  → Consultando ConceptNet API...");
        match self.conceptnet.search_concept(word, "es").await {
            Ok(edges) if !edges.is_empty() => {
                println!("  ✓ Encontrado en ConceptNet: {} relaciones", edges.len());
                
                let mut definition = format!("'{}' ", word);
                for edge in edges.iter().take(3) {
                    if edge.rel.label == "IsA" {
                        definition.push_str(&format!("es un tipo de {}. ", edge.end.label));
                    } else if edge.rel.label == "UsedFor" {
                        definition.push_str(&format!("se usa para {}. ", edge.end.label));
                    }
                }

                let _embedding = self.embeddings.synthesize_embedding(word, &definition);
                self.add_to_kb(word, &edges, kb);

                return ResearchResult {
                    word: word.to_string(),
                    learned: true,
                    method: "conceptnet_api".to_string(),
                    definition,
                    confidence: 0.9,
                };
            }
            _ => {}
        }

        // 3. Fallback: Buscar en Wikipedia
        println!("  → Consultando Wikipedia...");
        if let Ok(wiki_def) = self.search_wikipedia(word).await {
            println!("  ✓ Encontrado en Wikipedia");
            let _embedding = self.embeddings.synthesize_embedding(word, &wiki_def);

            return ResearchResult {
                word: word.to_string(),
                learned: true,
                method: "wikipedia".to_string(),
                definition: wiki_def,
                confidence: 0.8,
            };
        }

        println!("  ✗ No se encontró información sobre '{}'", word);
        ResearchResult {
            word: word.to_string(),
            learned: false,
            method: "none".to_string(),
            definition: format!("No tengo información sobre '{}'", word),
            confidence: 0.0,
        }
    }

    async fn search_wikipedia(&self, word: &str) -> Result<String, Box<dyn std::error::Error>> {
        let url = format!(
            "https://es.wikipedia.org/api/rest_v1/page/summary/{}",
            urlencoding::encode(word)
        );

        let response = reqwest::get(&url).await?;
        let json: serde_json::Value = response.json().await?;

        let extract = json.get("extract")
            .and_then(|e| e.as_str())
            .ok_or("No extract found")?;

        Ok(extract.to_string())
    }

    fn add_to_kb(
        &self,
        word: &str,
        edges: &[ConceptNetEdge],
        kb: &mut crate::cortex::comprehension::deep_understanding::KnowledgeBase,
    ) {
        use crate::cortex::comprehension::deep_understanding::*;

        if !kb.ontology.contains_key(word) {
            kb.ontology.insert(
                word.to_string(),
                OntologyNode {
                    name: word.to_string(),
                    parent: None,
                    children: vec![],
                    properties: std::collections::HashMap::new(),
                    typical_behaviors: vec![],
                    confidence: 1.0,
                    last_used: 0,
                    source: None,
                },
            );
        }

        for edge in edges.iter().take(10) {
            if edge.rel.label == "IsA" {
                if let Some(node) = kb.ontology.get_mut(word) {
                    node.parent = Some(edge.end.label.clone());
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct ResearchResult {
    pub word: String,
    pub learned: bool,
    pub method: String,
    pub definition: String,
    pub confidence: f64,
}
