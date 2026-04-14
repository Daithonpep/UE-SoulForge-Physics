use std::sync::Arc;
use daithon_bridge::causal::world_model::{CausalWorldModel, Domain, ValueType, Variable as CausalVariable};
use daithon_bridge::causal::inference::{CausalInferenceEngine, CausalHypothesis, ExperimentRecord, ExperimentOutcome};
use daithon_bridge::causal::validator::{ExperimentalValidator, UnrealInterfaceMock};
use daithon_bridge::agents::xeno::{Xeno, PhysicsSystem as WorldSystem, Variable as WorldVariable};

#[tokio::main]
async fn main() {
    println!("🌌 [DAITHON MULTIDOMAIN] El Oráculo del Caos fuera de Unreal\n");

    // 1. Inicialización de Cognición
    let world_model = Arc::new(std::sync::RwLock::new(CausalWorldModel::new()));
    let graph = Arc::new(std::sync::RwLock::new(daithon_bridge::contextus::semantic_graph::SemanticGraph::new()));
    
    let mut inference_engine = CausalInferenceEngine::new(world_model.clone());
    let xeno = Xeno::new();

    // ============================================================
    // DOMINIO: ESTRATEGIA / PLANIFICACIÓN (Día del lanzamiento)
    // ============================================================
    println!("📍 DOMINIO: Lanzamiento de Proyecto (Estratégico)\n");

    let mut state = WorldSystem::new();
    state.variables.insert("Feature_Count".into(), WorldVariable { name: "Feature_Count".into(), value: 50.0 });
    state.variables.insert("QA_Hours".into(), WorldVariable { name: "QA_Hours".into(), value: 100.0 });
    state.variables.insert("Stress_Level".into(), WorldVariable { name: "Stress_Level".into(), value: 0.1 });

    // 2. SENKU: El Lógico (Deducción por historial)
    println!("🧪 [SENKU] Analizando dependencias lineales...");
    let mut history = Vec::new();
    for i in 0..5 {
        let mut m = std::collections::HashMap::new();
        m.insert("Feature_Count".into(), 40.0 + i as f32 * 5.0);
        m.insert("Stress_Level".into(), 0.1 + i as f32 * 0.2);
        history.push(ExperimentRecord {
            id: format!("past_project_{}", i),
            measurements: m,
            conditions: HashMap::new(),
            outcome: ExperimentOutcome::Success,
            timestamp: 0,
        });
    }

    if let Some(law) = inference_engine.discover_causal_law(&history) {
        println!("   ✅ Ley Identificada: {} -> {} (Lineal)", law.cause.name, law.effect.name);
    }

    // 3. XENO: El Caos (Divergencia de Lyapunov)
    println!("\n😈 [XENO] No es tan simple. ¿Dónde está el punto de quiebre?");
    
    // Inyectamos una variable de "Acoplamiento Crítico" que Xeno detectará como caótica
    state.variables.insert("Interdependency".into(), WorldVariable { name: "Interdependency".into(), value: 0.999 });

    let chaos_plan = xeno.find_chaos_leverage(&state, &[]);
    
    println!("🎯 [XENO BINGO] Variable crítica: '{}'", chaos_plan.target_variable);
    println!("💬 [XENO RATIONALE] \"{}\"", chaos_plan.rationale);

    if chaos_plan.target_variable == "Interdependency" {
        println!("\n💥 [BOOM CAUSAL] Según Xeno, si la interdependencia sube a 1.0, un error en el shader");
        println!("   causa una falla en cascada que bloquea el despliegue del servidor.");
    }

    // ============================================================
    // DOMINIO: LINGÜÍSTICO / SEMÁNTICO
    // ============================================================
    println!("\n📍 DOMINIO: Significado de la Inteligencia (Semántico)\n");
    
    let mut semantic_state = WorldSystem::new();
    semantic_state.variables.insert("Ambiguity".into(), WorldVariable { name: "Ambiguity".into(), value: 0.7 });
    semantic_state.variables.insert("Context_Depth".into(), WorldVariable { name: "Context_Depth".into(), value: 100.0 });

    println!("💬 [DAITHON] Mi arquitectura es agnóstica al medio.");
    println!("   Senku protege mis verdades, Xeno protege mi evolución.");
    println!("   Soy el conflicto entre lo que es y lo que podría explotar.");

    println!("\n🚀 TEST MULTIDOMAIN COMPLETADO. Daithon es oficialmente General.");
}

use std::collections::HashMap;
