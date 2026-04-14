use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikidataSearchResponse {
    pub search: Vec<WikidataEntity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WikidataEntity {
    pub id: String,
    pub label: Option<String>,
    pub description: Option<String>,
    pub url: Option<String>,
}

pub struct WikidataClient {
    client: reqwest::Client,
}

impl WikidataClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    /// Search for topics in Wikidata to find definitions and relational entities
    pub async fn search_concept(&self, query: &str) -> Result<Vec<WikidataEntity>, String> {
        let encoded_query = urlencoding::encode(query);
        let url = format!(
            "https://www.wikidata.org/w/api.php?action=wbsearchentities&search={}&language=es&format=json&limit=3",
            encoded_query
        );

        log::info!("[WIKIDATA] Buscando relaciones ontológicas para: {}", query);

        let response = self.client.get(&url)
            .header("User-Agent", "DaithonAutonomousBot/1.0 (cortex@daithon.ai)")
            .send()
            .await
            .map_err(|e| format!("Error en red Wikidata: {}", e))?;
        
        let json: WikidataSearchResponse = response.json().await
            .map_err(|e| format!("Error parseando Wikidata JSON: {}", e))?;

        Ok(json.search)
    }
}
