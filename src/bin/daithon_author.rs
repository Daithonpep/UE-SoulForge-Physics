use daithon_bridge::persona::integration::DaithonPersona;
use daithon_bridge::contextus::semantic_graph::{SemanticGraph, AnchorSource};

#[tokio::main]
async fn main() {
    let mut persona = DaithonPersona::new();
    let mut graph = SemanticGraph::new();

    // Simulamos un estado de "Reflexión Post-Catástrofe"
    graph.strengthen_anchor(
        "tower_collapse_z".to_string(),
        "La torre es invulnerable al viento vertical",
        0.05, 
        false, 
        0.95,
        vec!["Desintegración de la base por vibración armónica".into()],
        "force: vertical_oscillation".into(),
        AnchorSource::LabExperiment
    );

    // Inyectamos estados reales de estrés en el traductor para que el PROTA tenga datos
    persona.translator.log_condition(daithon_bridge::persona::system_translator::SystemCondition::HighCPU { threshold: 0.95 });
    persona.translator.log_condition(daithon_bridge::persona::system_translator::SystemCondition::LowMemory { threshold_mb: 120 });

    let query = "La fragilidad de lo que creemos eterno";
    
    println!("\n[SISTEMA INICIANDO PROCESO CREATIVO AUTÓNOMO]");
    println!("[DAITHON ESTÁ ESCRIBIENDO...]\n");

    // "Soñar" genera la pieza basada en el TRIDENTE
    let inspiration_full = persona.dream_from_inspiration(Some(query.to_string()), &graph);

    println!("{}", inspiration_full);
    println!("\n---\n");
    
    // Una respuesta final corta, ahora más humana
    let final_thought = persona.respond_with_context(query);

    println!("📜 PENSAMIENTO FINAL:");
    println!("------------------------------------------------------------");
    println!("{}", final_thought);
    println!("------------------------------------------------------------");
}
