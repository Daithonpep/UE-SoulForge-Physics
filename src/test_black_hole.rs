use crate::contextus::debate::DebateEngine;
use crate::contextus::search::SearchResult;

mod cortex;
mod contextus;
mod trinity;
mod metacog;
mod llm_engine;
mod daithon_personality;

#[tokio::main]
async fn main() {
    let search_result = SearchResult {
        answer: "Un agujero negro es una región del espacio-tiempo con una gravedad tan fuerte que nada puede escapar.".to_string(),
        confidence: 0.95,
        source: "Internal Knowledge Base".to_string(),
    };

    let response = DebateEngine::daithon_deep_think("¿Qué piensas del agujero negro?", "agujero negro", &search_result).await;
    println!("{}", response);
}
