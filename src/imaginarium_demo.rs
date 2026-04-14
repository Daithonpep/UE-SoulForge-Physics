use daithon_bridge::persona::integration::DaithonPersona;
use daithon_bridge::contextus::semantic_graph::{SemanticGraph, AnchorSource};
use std::env;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    let theme = if args.len() > 1 {
        Some(args[1].clone())
    } else {
        None
    };

    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║       🌀 DAITHON: MOTOR DE INSPIRACIÓN Y LECTURA         ║");
    println!("║       Módulo: InspirationEngine / DaithonLibrary         ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    let mut persona = DaithonPersona::new();
    let mut graph = SemanticGraph::new();

    // Simulamos un experimento "interesante" en el grafo
    graph.strengthen_anchor(
        "arch_rotation_90".to_string(),
        "Un arco rotado 90 grados es estable",
        0.1, // Predicción muy fallida (delta alto)
        false, // Colapsó
        0.8,
        vec!["Fallo lateral inesperado".to_string()],
        "rotation: [0, 90, 0]".to_string(),
        AnchorSource::LabExperiment
    );

    println!("Buscando pulso creativo...");
    if let Some(t) = &theme {
        println!("Tema sugerido por usuario: {}", t);
    }

    let dream = persona.dream_from_inspiration(theme, &graph);
    
    println!("\n{}\n", dream);

    println!("--- PROCESO INTERNO ---");
    println!("[SISTEMA] Leyendo CPU/RAM para medir intensidad emocional.");
    println!("[BIBLIOTECA] Cruzando temas con 14 obras maestras.");
    println!("[GRAFO] Analizando anomalías en experimentos recientes.");
}
