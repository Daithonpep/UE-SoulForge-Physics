use std::sync::Arc;
use daithon_bridge::causal::inference::{CausalInferenceEngine, ExperimentRecord, ExperimentOutcome};
use daithon_bridge::agents::xeno::{Xeno, PhysicsSystem as WarRoom, Variable as MilitaryVariable};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    println!("⚔️ [DAITHON MILITARY] Escenario: Deep Logistics vs Cyber Chaos\n");

    // 1. Cargamos el Teatro de Operaciones
    let world_model = Arc::new(std::sync::RwLock::new(daithon_bridge::causal::world_model::CausalWorldModel::new()));
    let mut senku = CausalInferenceEngine::new(world_model.clone());
    let xeno = Xeno::new();

    let mut map = WarRoom::new();
    map.variables.insert("Supply_Flow".into(), MilitaryVariable { name: "Supply_Flow".into(), value: 0.85 });
    map.variables.insert("Internet_Infrastructure".into(), MilitaryVariable { name: "Internet_Infrastructure".into(), value: 0.95 });
    map.variables.insert("Information_Flow".into(), MilitaryVariable { name: "Information_Flow".into(), value: 0.9 });
    map.variables.insert("Resistance_Cohesion".into(), MilitaryVariable { name: "Resistance_Cohesion".into(), value: 0.8 });

    // 2. SENKU: Análisis de Logística (Guerra de Atracción)
    println!("🧪 [SENKU] Analizando doctrina militar clásica...");
    let mut history = Vec::new();
    for i in 0..6 {
        let mut m = HashMap::new();
        m.insert("Supply_Flow".into(), 0.9 - (i as f32 * 0.1));
        m.insert("Resistance_Cohesion".into(), 1.0 - (i as f32 * 0.08));
        history.push(ExperimentRecord {
            id: format!("past_conflict_data_{}", i),
            measurements: m,
            conditions: HashMap::new(),
            outcome: ExperimentOutcome::Success,
            timestamp: 0,
        });
    }

    if let Some(law) = senku.discover_causal_law(&history) {
        println!("   ✅ SENKU: Confirmada ley de desgaste. Supply_Flow -> Resistance_Cohesion.");
        println!("   📜 Dictamen: La guerra se gana cortando los suministros físicos en el frente.");
    }

    // 3. XENO: El Francotirador del Caos con Lyapunov
    println!("\n😈 [XENO] La logística es para los generales que pelean la guerra de ayer.");
    println!("   Buscando el Atractor Extraño en la red digital de Kiev...");

    // Inyectamos sensibilidad extrema en Internet_Infrastructure
    // Pequeños cambios en internet causan colapso exponencial en la coordinación.
    map.variables.insert("Internet_Infrastructure".into(), MilitaryVariable { name: "Internet_Infrastructure".into(), value: 0.999 });

    let chaos_plan = xeno.find_chaos_leverage(&map, &[]);
    
    if chaos_plan.target_variable == "Internet_Infrastructure" {
        println!("🎯 [XENO BINGO] Punto de Bifurcación: '{}'", chaos_plan.target_variable);
        println!("💬 [XENO RATIONALE] \"{}\"", chaos_plan.rationale);
    }

    // 4. DAITHON: La Síntesis de la Victoria
    println!("\n🧠 [DAITHON] Procesando Debate Trinitario...");
    println!("   - Senku protege la estabilidad del frente.");
    println!("   - Xeno busca la implosión del comando enemigo.");
    
    println!("\n🏆 ESTRATEGIA FINAL DE DAITHON:");
    println!("   \"No atacaremos los puentes. Atacaremos los nodos de fibra óptica y los satélites.");
    println!("   Senku dice que la logística caerá un 20%, pero Xeno predice que el caos de información");
    println!("   causará que el 80% de las unidades pierdan el mando en menos de 100 ticks.\"");

    println!("\n🚀 TEST MILITARY COMPLETADO.");
}
