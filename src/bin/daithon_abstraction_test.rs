use daithon_bridge::persona::integration::DaithonPersona;
use daithon_bridge::contextus::semantic_graph::SemanticGraph;

#[tokio::main]
async fn main() {
    let mut persona = DaithonPersona::new();
    let mut graph = SemanticGraph::new();

    let domain = "Física de Fluidos".to_string();
    let concept = "Tensión Superficial".to_string();
    let text = "La tensión superficial ocurre porque las moléculas en el borde no tienen a quién unirse arriba, así que se unen más fuerte a las de los lados creando una piel resistente.".to_string();

    println!("[TEST] Enviando concepto al 'Pasillo de la Muerte'...");

    // Lanzar en segundo plano
    let handle = persona.start_background_abstraction(domain.clone(), concept.clone(), text);

    println!("[TEST] Mientras Daithon piensa, podemos seguir operando...");
    println!("[TEST] (Simulando trabajo en Unreal Engine)");
    
    // Esperar el resultado (en producción, esto sería manejado por un sistema de mensajes o polling)
    if let Ok(Some(result)) = handle.await {
        if let Some(synthesis) = result.synthesis {
            println!("\n✅ ABSTRACCIÓN APROBADA POR XENO:");
            println!("   Origen: {}", domain);
            println!("   Concepto: {}", concept);
            println!("   Aplicación Unreal: {}", synthesis.aplicacion_unreal);

            // Guardar en el grafo
            graph.add_abstraction(domain, concept, synthesis);
            println!("\n[GRAFO] Nuevo nodo de abstracción sellado.");
        } else {
            println!("\n❌ ABSTRACCIÓN RECHAZADA (No pragmática o error lógico)");
            println!("   Chrome: {}", result.analysis.chrome_divergence);
            println!("   Senku: {}", result.analysis.senku_convergence);
            println!("   Xeno: {}", result.analysis.xeno_pragmatism);
        }
    }

    println!("\n[ESTADO DEL GRAFO]");
    println!("Anclas de Abstracción: {}", graph.abstraction_anchors.len());
}
