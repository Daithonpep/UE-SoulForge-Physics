use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NasaSearchResponse {
    pub collection: NasaCollection,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NasaCollection {
    pub items: Vec<NasaItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NasaItem {
    pub data: Option<Vec<NasaData>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NasaData {
    pub title: String,
    pub description: Option<String>,
    pub keywords: Option<Vec<String>>,
}

pub struct NasaClient {
    client: reqwest::Client,
}

impl NasaClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    /// Busca datos empíricos y físicos en el Image & Video Library de la NASA
    pub async fn search_physics_data(&self, query: &str) -> Result<Vec<NasaData>, String> {
        let encoded_query = urlencoding::encode(query);
        let url = format!(
            "https://images-api.nasa.gov/search?q={}&media_type=image",
            encoded_query
        );

        log::info!("[NASA API] Buscando astrofísica y datos del universo para: {}", query);

        let response = self.client.get(&url)
            .send()
            .await
            .map_err(|e| format!("Error en red NASA API: {}", e))?;
        
        let json: NasaSearchResponse = response.json().await
            .map_err(|e| format!("Error parseando NASA JSON: {}", e))?;

        let mut results = Vec::new();
        // Extraer hasta 3 elementos relevantes
        for item in json.collection.items.into_iter().take(3) {
            if let Some(data_array) = item.data {
                if let Some(data) = data_array.into_iter().next() {
                    results.push(data);
                }
            }
        }

        Ok(results)
    }
}
