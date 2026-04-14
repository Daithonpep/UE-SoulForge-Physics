use daithon_bridge::persona::integration::DaithonPersona;
use daithon_bridge::contextus::semantic_graph::SemanticGraph;
use daithon_bridge::forge::falsification_engine::FalsificationEngine;

#[tokio::main]
async fn main() {
    let mut persona = DaithonPersona::new();
    let mut graph = SemanticGraph::new();

    // 1. FASE DE ABSTRACCIÓN (Topological Isomorphism)
    let domain = "Biología Celular".to_string();
    let concept = "Corrección de lectura de ARN".to_string();
    let text = "Durante la síntesis de proteínas, el ARN tiene mecanismos para corregir errores de emparejamiento, evitando mutaciones fatales en el organismo.".to_string();

    println!("🧪 [CICLO CIENTÍFICO] Iniciando Abstracción para: {}", concept);
    
    // Para el test, ejecutamos síncrono para ver el flujo.
    let abstraction = persona.abstraction.abstract_concept(&domain, &text).await.unwrap();

    if let Some(synthesis) = &abstraction.synthesis {
        println!("\n✅ [FASE 1-3] Abstracción Generada!");
        println!("   Aplicación Unreal: {}", synthesis.aplicacion_unreal);

        // Guardar en el grafo como HIPÓTESIS
        graph.add_abstraction(domain.clone(), concept.clone(), synthesis.clone());
        let key = format!("abs_{}_{}", domain, concept).replace(' ', "_").to_lowercase();

        // 2. FASE DE FALSACIÓN (Diseño de Experimento)
        println!("\n🔬 [FASE 4] Diseñando experimento de validación...");
        if let Some(session) = FalsificationEngine::design_experiment(&abstraction) {
            println!("   Experimento diseñado: {:?}", session.hypothesis_id);
            println!("   Predicción: {}", session.prediction.reasoning);

            // SIMULAMOS UN FALLO (La simulación de Unreal dice que colapsó)
            println!("\n⚠️ [RESULTADO] El experimento ha FALLADO en Unreal.");
            let error_context = "El crash ocurrió por una recursión infinita en el validador de bordes, a diferencia del ARN que es lineal.";
            
            // 3. APRENDIZAJE CAUSAL (Fase 6)
            println!("\n🔍 [FASE 6] Senku analizando matriz de restricciones (R)...");
            match persona.abstraction.analyze_failure(&abstraction, error_context).await {
                Ok(gap) => {
                    println!("   Fallo detectado en R: {}", gap.restriction_r);
                    println!("   Divergencia: {}", gap.why_diverged);

                    // Actualizamos el grafo: bajamos confianza
                    graph.mark_abstraction_use(&key, false);
                },
                Err(e) => println!("   Error en análisis causal: {}", e),
            }
        }
    }

    // 4. CONTROL DE ENTROPÍA (Fase 5)
    println!("\n✂️ [FASE 5] Ejecutando Poda Semántica...");
    graph.prune_abstractions(0.5); // Borrar si confianza < 0.5 (el fallo bajó la confianza considerablemente)

    println!("\n📊 [ESTADO FINAL DEL CEREBRO]");
    println!("   Abstracciones en memoria: {}", graph.abstraction_anchors.len());
    if graph.abstraction_anchors.is_empty() {
        println!("   Resultado: La analogía fue purgada por falta de pragmatismo técnico.");
    }
}
