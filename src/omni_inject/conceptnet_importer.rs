use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Importador de ConceptNet (5 millones de relaciones)
pub struct ConceptNetImporter {
    api_base: String,
    cache_dir: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptNetEdge {
    #[serde(rename = "@id")]
    pub id: String,
    pub rel: ConceptNetRelation,
    pub start: ConceptNetNode,
    pub end: ConceptNetNode,
    pub weight: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptNetRelation {
    #[serde(rename = "@id")]
    pub id: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptNetNode {
    #[serde(rename = "@id")]
    pub id: String,
    pub label: String,
    pub language: String,
}

impl ConceptNetImporter {
    pub fn new() -> Self {
        Self {
            api_base: "https://api.conceptnet.io".to_string(),
            cache_dir: "omni_inject_cache/conceptnet".to_string(),
        }
    }

    /// Descargar dataset completo de ConceptNet
    pub async fn download_full_dataset(&self) -> Result<(), Box<dyn std::error::Error>> {
        println!("[OMNI-INJECT] Descargando dataset completo de ConceptNet...");
        
        let dump_url = "https://s3.amazonaws.com/conceptnet/downloads/2019/edges/conceptnet-assertions-5.7.0.csv.gz";
        
        let response = reqwest::get(dump_url).await?;
        let bytes = response.bytes().await?;

        std::fs::create_dir_all(&self.cache_dir)?;
        let cache_path = format!("{}/conceptnet_full.csv.gz", self.cache_dir);
        
        std::fs::write(&cache_path, bytes)?;
        
        println!("✓ Dataset descargado: {}", cache_path);
        println!("  Descomprimiendo...");
        
        let compressed = std::fs::File::open(&cache_path)?;
        let mut decoder = flate2::read::GzDecoder::new(compressed);
        let output_path = format!("{}/conceptnet_full.csv", self.cache_dir);
        let mut output = std::fs::File::create(&output_path)?;
        std::io::copy(&mut decoder, &mut output)?;
        
        println!("✓ Descomprimido: {}", output_path);
        
        Ok(())
    }

    /// Importar dataset a KnowledgeBase de Daithon
    pub fn import_to_knowledge_base(
        &self,
        kb: &mut crate::cortex::comprehension::deep_understanding::KnowledgeBase,
        language_filter: &str, // "es" para español
        max_relations: Option<usize>,
    ) -> Result<ImportStats, Box<dyn std::error::Error>> {
        println!("[OMNI-INJECT] Importando ConceptNet a KnowledgeBase...");
        println!("  Filtro de idioma: {}", language_filter);
        
        let csv_path = format!("{}/conceptnet_full.csv", self.cache_dir);
        
        if !std::path::Path::new(&csv_path).exists() {
            return Err("Dataset no encontrado. Ejecuta download_full_dataset primero.".into());
        }

        let mut reader = csv::ReaderBuilder::new()
            .has_headers(false)
            .delimiter(b'\t')
            .from_path(&csv_path)?;

        let mut stats = ImportStats::default();
        let mut processed = 0;

        for result in reader.records() {
            let record = result?;
            
            let rel_uri = record.get(1).unwrap_or("");
            let start_uri = record.get(2).unwrap_or("");
            let end_uri = record.get(3).unwrap_or("");
            let weight_str = record.get(4).unwrap_or("1.0");

            let start = Self::parse_uri(start_uri);
            let end = Self::parse_uri(end_uri);
            let rel_type = Self::parse_relation(rel_uri);

            if start.language != language_filter && end.language != language_filter {
                continue;
            }

            let weight: f64 = serde_json::from_str(weight_str)
                .map(|v: serde_json::Value| {
                    v.get("weight")
                        .and_then(|w| w.as_f64())
                        .unwrap_or(1.0)
                })
                .unwrap_or(1.0);

            if weight < 1.0 { continue; }

            self.add_to_kb(kb, &start, &end, &rel_type, weight, &mut stats);

            processed += 1;

            if processed % 10000 == 0 {
                print!("\r  Procesadas: {} relaciones", processed);
                let _ = std::io::Write::flush(&mut std::io::stdout());
            }

            if let Some(max) = max_relations {
                if processed >= max {
                    break;
                }
            }
        }

        println!("\n✓ Importación completa");
        println!("  Entidades: {}", stats.entities_added);
        println!("  Relaciones: {}", stats.relations_added);
        println!("  Propiedades: {}", stats.properties_added);

        Ok(stats)
    }

    fn parse_uri(uri: &str) -> ParsedConcept {
        let parts: Vec<&str> = uri.split('/').collect();
        if parts.len() >= 4 {
            ParsedConcept { language: parts[2].to_string(), concept: parts[3].to_string() }
        } else {
            ParsedConcept { language: "unknown".to_string(), concept: uri.to_string() }
        }
    }

    fn parse_relation(uri: &str) -> String {
        let parts: Vec<&str> = uri.split('/').collect();
        parts.get(2).unwrap_or(&"Related").to_string()
    }

    fn add_to_kb(
        &self,
        kb: &mut crate::cortex::comprehension::deep_understanding::KnowledgeBase,
        start: &ParsedConcept,
        end: &ParsedConcept,
        rel_type: &str,
        weight: f64,
        stats: &mut ImportStats,
    ) {
        use crate::cortex::extraction::knowledge_extractor::*;
        use crate::cortex::comprehension::deep_understanding::*;

        if !kb.ontology.contains_key(&start.concept) {
            kb.ontology.insert(
                start.concept.clone(),
                OntologyNode {
                    name: start.concept.clone(),
                    parent: None,
                    children: vec![],
                    properties: HashMap::new(),
                    typical_behaviors: vec![],
                    confidence: 1.0,
                    last_used: 0,
                    source: None,
                },
            );
            stats.entities_added += 1;
        }

        if !kb.ontology.contains_key(&end.concept) {
            kb.ontology.insert(
                end.concept.clone(),
                OntologyNode {
                    name: end.concept.clone(),
                    parent: None,
                    children: vec![],
                    properties: HashMap::new(),
                    typical_behaviors: vec![],
                    confidence: 1.0,
                    last_used: 0,
                    source: None,
                },
            );
            stats.entities_added += 1;
        }

        let relation_type = match rel_type {
            "IsA" => RelationType::IsA,
            "PartOf" => RelationType::PartOf,
            "UsedFor" => RelationType::UsedFor,
            "CapableOf" => RelationType::UsedFor,
            "Causes" => RelationType::Causes,
            "HasProperty" => RelationType::HasProperty,
            "AtLocation" => RelationType::LocatedIn,
            "CreatedBy" => RelationType::CreatedBy,
            "DerivedFrom" => RelationType::IsA,
            "HasA" => RelationType::PartOf,
            _ => return,
        };

        match relation_type {
            RelationType::IsA => {
                if let Some(child) = kb.ontology.get_mut(&start.concept) {
                    child.parent = Some(end.concept.clone());
                }
                if let Some(parent) = kb.ontology.get_mut(&end.concept) {
                    if !parent.children.contains(&start.concept) {
                        parent.children.push(start.concept.clone());
                    }
                }
                stats.relations_added += 1;
            }
            RelationType::HasProperty => {
                if let Some(node) = kb.ontology.get_mut(&start.concept) {
                    node.properties.insert(
                        end.concept.clone(),
                        PropertyValue::Boolean(true),
                    );
                    stats.properties_added += 1;
                }
            }
            RelationType::Causes => {
                let links = kb.causal_graph.entry(start.concept.clone()).or_insert_with(Vec::new);
                links.push(CausalLink {
                    cause: start.concept.clone(),
                    effect: end.concept.clone(),
                    strength: weight,
                    mechanism: None,
                    exceptions: vec![],
                    last_used: 0,
                });
                stats.relations_added += 1;
            }
            _ => { stats.relations_added += 1; }
        }
    }

    pub async fn search_concept(&self, word: &str, language: &str) -> Result<Vec<ConceptNetEdge>, Box<dyn std::error::Error>> {
        let url = format!(
            "{}/c/{}/{}?limit=100",
            self.api_base,
            language,
            urlencoding::encode(word)
        );

        let response = reqwest::get(&url).await?;
        let json: serde_json::Value = response.json().await?;

        let mut edges = Vec::new();
        if let Some(edges_array) = json.get("edges").and_then(|e| e.as_array()) {
            for edge_value in edges_array {
                if let Ok(edge) = serde_json::from_value::<ConceptNetEdge>(edge_value.clone()) {
                    edges.push(edge);
                }
            }
        }
        Ok(edges)
    }
}

#[derive(Debug, Clone, Default)]
pub struct ImportStats {
    pub entities_added: usize,
    pub relations_added: usize,
    pub properties_added: usize,
}

#[derive(Debug, Clone)]
struct ParsedConcept {
    language: String,
    concept: String,
}
