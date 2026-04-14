use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GutendexResponse {
    pub count: usize,
    pub results: Vec<GutendexBook>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GutendexBook {
    pub id: usize,
    pub title: String,
    pub authors: Vec<GutendexAuthor>,
    pub subjects: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GutendexAuthor {
    pub name: String,
}

pub struct GutendexClient {
    client: reqwest::Client,
}

impl GutendexClient {
    pub fn new() -> Self {
        Self {
            client: reqwest::Client::new(),
        }
    }

    /// Busca manuscritos y textos filosóficos o de literatura clásica
    pub async fn search_books(&self, query: &str) -> Result<Vec<GutendexBook>, String> {
        let encoded_query = urlencoding::encode(query);
        let url = format!(
            "https://gutendex.com/books/?search={}",
            encoded_query
        );

        log::info!("[GUTENDEX] Consultando Biblioteca Clásica para: {}", query);

        let response = self.client.get(&url)
            .send()
            .await
            .map_err(|e| format!("Error en red Gutendex: {}", e))?;
        
        let json: GutendexResponse = response.json().await
            .map_err(|e| format!("Error parseando Gutendex JSON: {}", e))?;

        // Return up to 3 classic books
        Ok(json.results.into_iter().take(3).collect())
    }
}
