use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAlexResponse {
    pub meta: OpenAlexMeta,
    pub results: Vec<OpenAlexWork>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAlexMeta {
    pub count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAlexWork {
    pub id: String,
    pub title: Option<String>,
    pub publication_year: Option<u32>,
    pub concepts: Option<Vec<OpenAlexConcept>>,
    pub abstract_inverted_index: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenAlexConcept {
    pub id: String,
    pub wikidata: Option<String>,
    pub display_name: Option<String>,
    pub score: Option<f32>,
    pub level: Option<u32>,
}

pub struct OpenAlexClient {
    client: reqwest::Client,
    email: String, // Contact email allows faster API limits
}

impl OpenAlexClient {
    pub fn new(email: &str) -> Self {
        Self {
            client: reqwest::Client::new(),
            email: email.to_string(),
        }
    }

    /// Search academic papers matching the topic.
    pub async fn search_works(&self, query: &str, limit: usize) -> Result<Vec<OpenAlexWork>, String> {
        let encoded_query = urlencoding::encode(query);
        let url = format!(
            "https://api.openalex.org/works?search={}&per-page={}&mailto={}",
            encoded_query, limit, self.email
        );

        log::info!("[OPENALEX] Buscando papers académicos para: {}", query);

        let response = self.client.get(&url)
            .send()
            .await
            .map_err(|e| format!("Error en red OpenAlex: {}", e))?;
        
        let json: OpenAlexResponse = response.json().await
            .map_err(|e| format!("Error parseando OpenAlex JSON: {}", e))?;

        Ok(json.results)
    }

    /// Analizador para buscar constantes físicas dentro de un abstract usando expresiones o palabras clave
    pub fn extract_action_variables(&self, work: &OpenAlexWork) -> Vec<crate::cortex::extraction::knowledge_extractor::Entity> {
        let mut constants = Vec::new();
        
        // Simulación: Si el subject trata sobre 'Young's modulus' o 'elasticidad', creamos una entidad hardcodeada
        // En un parser real se reconstruye el texto del invert_index y se busca con Regex o con MATHESIS.
        if let Some(title) = &work.title {
            let lower = title.to_lowercase();
            if lower.contains("módulo de young") || lower.contains("young's modulus") || lower.contains("elasticity") {
                constants.push(crate::cortex::extraction::knowledge_extractor::Entity {
                    name: "módulo de young".to_string(),
                    entity_type: crate::cortex::extraction::knowledge_extractor::EntityType::Quantity,
                    aliases: vec!["E".to_string(), "elasticidad".to_string()],
                    description: Some("is_constant: true | value: ~200 GPa".to_string()),
                });
            } else if lower.contains("gravedad") || lower.contains("gravity") {
                constants.push(crate::cortex::extraction::knowledge_extractor::Entity {
                    name: "aceleración gravitacional".to_string(),
                    entity_type: crate::cortex::extraction::knowledge_extractor::EntityType::Quantity,
                    aliases: vec!["g".to_string()],
                    description: Some("is_constant: true | value: 9.8 m/s^2".to_string()),
                });
            }
        }
        constants
    }
}
