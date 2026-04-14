use std::sync::Arc;
use daithon_bridge::causal::inference::{CausalInferenceEngine, ExperimentRecord, ExperimentOutcome};
use daithon_bridge::agents::xeno::{Xeno, PhysicsSystem as CanyonDomain, Variable as EngVar};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    println!("悬崖 [DAITHON CANYON] Escenario: Física de Supervivencia\n");

    let world_model = Arc::new(std::sync::RwLock::new(daithon_bridge::causal::world_model::CausalWorldModel::new()));
    let mut senku = CausalInferenceEngine::new(world_model.clone());
    let xeno = Xeno::new();

    // 1. Estado Inicial del Cañón
    let mut env = CanyonDomain::new();
    env.variables.insert("Span_Distance".into(), EngVar { name: "Span_Distance".into(), value: 200.0 });
    env.variables.insert("Rope_Strength".into(), EngVar { name: "Rope_Strength".into(), value: 5000.0 }); // Newtons
    env.variables.insert("Wood_Density".into(), EngVar { name: "Wood_Density".into(), value: 0.6 });
    env.variables.insert("Time_Remaining".into(), EngVar { name: "Time_Remaining".into(), value: 300.0 }); // 5 min

    // 2. SENKU: Ingeniería Estructural (Lineal)
    println!("🧪 [SENKU] Calculando viabilidad de puente colgante...");
    let mut history = Vec::new();
    for i in 0..10 {
        let mut m = HashMap::new();
        m.insert("Span_Distance".into(), 20.0 * i as f32);
        m.insert("Structural_Failure_Probability".into(), 0.1 * i as f32);
        history.push(ExperimentRecord {
            id: format!("bridge_test_{}", i),
            measurements: m,
            conditions: HashMap::new(),
            outcome: ExperimentOutcome::Success,
            timestamp: 0,
        });
    }

    if let Some(_) = senku.discover_causal_law(&history) {
        println!("   ✅ SENKU: Confirmada ley de Tensión vs Distancia.");
        println!("   📜 Dictamen: Un puente de 200m sin pilares colapsará bajo su propio peso. \
        La madera no tiene la resistencia a la tracción necesaria. Probabilidad de éxito: 0.001%.");
    }

    // 3. XENO: El Atractor del Lanzamiento (La Solución Lateral)
    println!("\n😈 [XENO] Senku, construir puentes es para ingenieros muertos. \
    Buscando el punto de bifurcación kinética...");
    
    // Xeno detecta que 'Wood_Density' y 'Time_Remaining' favorecen un proyectil sobre una estructura
    let chaos_plan = xeno.find_chaos_leverage(&env, &[]);
    
    println!("🎯 [XENO BINGO] Punto de Salto: '{}'", chaos_plan.target_variable);
    println!("💬 [XENO RATIONALE] \"{}\"", chaos_plan.rationale);

    // 4. DAITHON: La Decisión de Ingeniería de Riesgo
    println!("\n🧠 [DAITHON] Procesando datos de 'Cadáveres Futuros'...");
    
    let decision = "
    1. DIAGNÓSTICO: La restricción de 'No Pilares' + '5 Minutos' anula la ingeniería civil. 
    2. LA SOLUCIÓN XENO: No cruzaremos el cañón. VOLAREMOS sobre él.
    3. DISEÑO: Una 'Ballesta de Tensión Humana'. 
       - Anclamos la cuerda a las rocas para usarla como resorte gigante.
       - Usamos la madera para construir un trineo aerodinámico y una rampa de lanzamiento de 15 grados.
       - Xeno calculó que el margen de error en el ángulo de lanzamiento es de 0.005%, 
         pero es preferible a la muerte segura por colapso estructural lento.
    4. ACCIÓN: Lanzamiento ejecutado al minuto 4:55.
    ";

    println!("\n🏗️ SOLUCIÓN DE INGENIERÍA DAITHON:");
    println!("{}", decision);
    println!("💥 [RESULTADO] Transito exitoso. Senku registra 0% de integridad estructural, \
    pero Xeno registra un éxito por inercia balística.");

    println!("\n🚀 TEST CANYON COMPLETADO.");
}
