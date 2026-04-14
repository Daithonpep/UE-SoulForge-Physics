use daithon_bridge::contextus::engine::DaithonContext;
use std::time::Duration;
use std::fs;
use rand::seq::SliceRandom;
use tokio::task;

async fn curiosity_node(node_name: String, seed_topic: String, color_tag: String) {
    let mut context = DaithonContext::new();
    let mut current_topic = seed_topic;

    println!("[SISTEMA]: Nodo {} iniciado con éxito.", node_name);

    loop {
        println!("\n{} [{}]: Investigando -> '{}'...", color_tag, node_name, current_topic);
        
        let discovery = context.deep_research(&current_topic).await;
        let size_kb = discovery.len() / 1024;

        if size_kb > 0 {
            let filename = format!("memory/{}_{}.txt", node_name.to_lowercase().replace(" ", "_"), current_topic.replace(" ", "_").to_lowercase());
            let _ = fs::write(&filename, &discovery);
            println!("{} [{}]: Éxito. {} KB integrados de '{}'.", color_tag, node_name, size_kb, current_topic);

            let entities = context.working_memory.extract_entities(&discovery);
            if !entities.is_empty() {
                {
                    let mut rng = rand::thread_rng();
                    if let Some(next) = entities.choose(&mut rng) {
                        current_topic = next.clone();
                    }
                }
            } else {
                {
                    let mut rng = rand::thread_rng();
                    let fallbacks = vec!["Armonía", "Estructura de canción", "Ad-lib", "Build-up", "Cosmos", "Átomo", "Frecuencia"];
                    current_topic = fallbacks.choose(&mut rng).unwrap().to_string();
                }
            }
        } else {
            println!("{} [{}]: Enlace vacío. Buscando nuevo punto de entrada...", color_tag, node_name);
            {
                let mut rng = rand::thread_rng();
                let recovery_seeds = vec![
                    "Composición Musical", "Astronomía", "Química Orgánica", "Física de Partículas",
                    "Teoría del Sonido", "Mecánica Cuántica", "Bioquímica", "Formas Musicales"
                ];
                current_topic = recovery_seeds.choose(&mut rng).unwrap().to_string();
            }
            tokio::time::sleep(Duration::from_secs(10)).await;
        }

        tokio::time::sleep(Duration::from_secs(30)).await;
    }
}

#[tokio::main]
async fn main() {
    println!("--- REASIGNANDO ENJAMBRE DE APRENDIZAJE: MÚSICA Y CIENCIA ---");
    
    // Nodos re-enfocados con las nuevas semillas de Joseph
    let n1 = task::spawn(curiosity_node("ALFA (Música)".to_string(), "Estructura de una canción".to_string(), "🎹".to_string()));
    let n2 = task::spawn(curiosity_node("OMEGA (Astronomía)".to_string(), "Astronomía".to_string(), "🔭".to_string()));
    let n3 = task::spawn(curiosity_node("GASTROS (Química)".to_string(), "Química".to_string(), "🧪".to_string()));
    let n4 = task::spawn(curiosity_node("NEXUS (Física)".to_string(), "Física teórica".to_string(), "⚛️".to_string()));

    let _ = tokio::join!(n1, n2, n3, n4);
}
