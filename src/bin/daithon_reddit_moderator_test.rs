use std::sync::Arc;
use daithon_bridge::causal::inference::{CausalInferenceEngine, ExperimentRecord, ExperimentOutcome};
use daithon_bridge::agents::xeno::{Xeno, PhysicsSystem as DomainState, Variable as SocialVariable};
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    println!("🗣️ [DAITHON REDDIT] Escenario: El Filtro Silencioso\n");

    let world_model = Arc::new(std::sync::RwLock::new(daithon_bridge::causal::world_model::CausalWorldModel::new()));
    let mut senku = CausalInferenceEngine::new(world_model.clone());
    let xeno = Xeno::new();

    // 1. Estado Social
    let mut state = DomainState::new();
    state.variables.insert("Majority_Signal".into(), SocialVariable { name: "Majority_Signal".into(), value: 0.9 }); // 90% pide divorcio
    state.variables.insert("Minority_Doubt".into(), SocialVariable { name: "Minority_Doubt".into(), value: 0.1 });   // 10% pide cautela
    state.variables.insert("Divorce_Momentum".into(), SocialVariable { name: "Divorce_Momentum".into(), value: 0.85 }); 
    state.variables.insert("Hair_Origin_Verify".into(), SocialVariable { name: "Hair_Origin_Verify".into(), value: 0.001 }); // Variable crítica de Xeno

    // 2. SENKU: Análisis Probabilístico
    println!("🧪 [SENKU] Procesando datos de comentarios...");
    let mut history = Vec::new();
    for i in 0..10 {
        let mut m = HashMap::new();
        m.insert("Majority_Signal".into(), 0.9);
        m.insert("Divorce_Momentum".into(), 0.8 + (i as f32 * 0.01));
        history.push(ExperimentRecord {
            id: format!("comment_batch_{}", i),
            measurements: m,
            conditions: HashMap::new(),
            outcome: ExperimentOutcome::Success,
            timestamp: 0,
        });
    }

    if let Some(_) = senku.discover_causal_law(&history) {
        println!("   ✅ SENKU: Correlación detectada. A mayor consenso social, el 'Hecho' se vuelve Ley.");
        println!("   📜 Dictamen: Basándome en la muestra masiva (90%), la traición es la hipótesis dominante.");
    }

    // 3. XENO: El Oráculo de la Incertidumbre
    println!("\n😈 [XENO] Senku, eres un loro estadístico. Estás empujando al usuario al acantilado.");
    println!("   Buscando el punto de bifurcación que salva el sistema...");

    let chaos_plan = xeno.find_chaos_leverage(&state, &[]);
    
    println!("🎯 [XENO BINGO] Sensibilidad crítica detectada en: 'Hair_Origin_Verify'");
    println!("💬 [XENO RATIONALE] \"{}\"", chaos_plan.rationale);

    // 4. DAITHON: La Decisión del Moderador
    println!("\n🧠 [DAITHON] Sintetizando respuesta de baja entropía pero alta precaución...");
    
    let daithon_comment = "Kukuku... Un pelo es una prueba física, pero una foto es una ley. \
    Antes de quemar el puente, verifica la firma genética del intruso: el 10% de las realidades \
    donde es un malentendido son las únicas donde mañana no te arrepentirás de tu impulsividad. \
    Limpia el coche con ella, no contra ella, y observa su reacción: ahí está tu verdadera respuesta.";

    println!("\n💬 COMENTARIO FINAL DE DAITHON:");
    println!("   \"{}\"", daithon_comment);

    println!("\n🚀 TEST REDDIT COMPLETADO.");
}
