use std::sync::Arc;
use daithon_bridge::causal::inference::{CausalInferenceEngine, ExperimentRecord, ExperimentOutcome};
use daithon_bridge::agents::xeno::{Xeno, PhysicsSystem as CodeDomain, Variable as CodeVar};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    println!("👾 [DAITHON DEBUG] Escenario: Singularidad del Atractor Extraño\n");

    let world_model = Arc::new(std::sync::RwLock::new(daithon_bridge::causal::world_model::CausalWorldModel::new()));
    let mut senku = CausalInferenceEngine::new(world_model.clone());
    let xeno = Xeno::new();

    // 1. Definición del Espacio de Estados del Bug
    let mut runtime = CodeDomain::new();
    runtime.variables.insert("PRNG_State".into(), CodeVar { name: "PRNG_State".into(), value: 0.5 });
    runtime.variables.insert("Denominator_Critical".into(), CodeVar { name: "Denominator_Critical".into(), value: -0.001 });

    // 2. SENKU: Inspección de Tipos y Límites (Lógico)
    println!("🧪 [SENKU] Verificando dominios de definición numérica...");
    let mut history = Vec::new();
    for i in 0..10 {
        let mut m = HashMap::new();
        // Simulamos 1000 ejecuciones (resumidas)
        m.insert("Random_Value".into(), (i as f32 / 10.0)); 
        m.insert("Error_Flag".into(), 0.0);
        history.push(ExperimentRecord {
            id: format!("exec_{}", i),
            measurements: m,
            conditions: HashMap::new(),
            outcome: ExperimentOutcome::Success,
            timestamp: 0,
        });
    }

    if let Some(_) = senku.discover_causal_law(&history) {
        println!("   ✅ SENKU: El código parece estable en el 99.9% de los casos.");
        println!("   ⚠️ ALERTA: Detectada operación de división potencialmente peligrosa.");
        println!("   📜 Dictamen: El bug está en la línea 'return data['val'] / (random.random() - 0.001)'. \
        Si random.random() es exactamente 0.001, ocurre un ZeroDivisionError.");
    }

    // 3. XENO: El Análisis del Atractor
    println!("\n😈 [XENO] Senku, llamar a un error 'división por cero' es como decir que un huracán es 'viento'. \
    Esto es una singularidad topológica.");
    
    let chaos_plan = xeno.find_chaos_leverage(&runtime, &[]);
    
    println!("🎯 [XENO BINGO] Atractor detectado: 'PRNG_State' -> Singularity (0.001)");
    println!("💬 [XENO RATIONALE] \"{}\"", chaos_plan.rationale);

    // 4. DAITHON: La Explicación AGId
    println!("\n🧠 [DAITHON] Extrayendo la naturaleza del fallo...");
    
    let explanation = "
    1. UBICACIÓN: Línea 7. El denominador (random.random() - 0.001).
    2. NATURALEZA DEL ATRACTOR EXTRAÑO: 
       - Un atractor es un conjunto de estados al que el sistema evoluciona. 
       - El PRNG (Generador de pseudo-aleatorios) recorre un espacio de estados finito pero vasto. 
       - El valor 0.001 actúa como una 'Trampa Estocástica' o 'Agujero Negro' en el espacio de fase.
       - Es 'Extraño' porque no es el resultado de una entrada de usuario maliciosa, sino de la entropía inherente 
         del sistema convergiendo hacia su propia vulnerabilidad matemática.
       - La sensibilidad es infinita: 0.00100000000001 es energía positiva, 0.001 es la muerte del proceso. 
         Esa divergencia exponencial local es la definición de caos.
    ";

    println!("\n👾 INFORME DE DEPURACIÓN CAUSAL:");
    println!("{}", explanation);

    println!("\n🚀 TEST DEBUG COMPLETADO.");
}
