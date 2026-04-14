use daithon_bridge::forge::training::concept_verifier::ConceptVerifier;
use daithon_bridge::forge::training::deep_reasoning::DeepReasoner;
use daithon_bridge::forge::training::pattern_library::PatternLibrary;

fn main() {
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║  🧠 DAITHON V8.3: ENTRENAMIENTO SENIOR DE RUST                ║");
    println!("║  Sistema: FORGE + DIALECTICA + DeepReasoning + PatternLibrary ║");
    println!("║  Objetivo: Comprensión profunda, no memorización              ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    // 1. Inicializar sistemas
    let reasoner = DeepReasoner::new();
    let library = PatternLibrary::new();
    let mut verifier = ConceptVerifier::new();

    // 2. Razonamiento profundo sobre temas clave
    println!("═══ PRUEBA DE RAZONAMIENTO PROFUNDO ═══\n");

    let topics = ["cache sharing CPU", "unsafe cell Rust ownership", "atomic ordering concurrency"];
    for topic in &topics {
        let result = reasoner.reason_about(topic);
        println!("📖 '{}' (confianza: {:.0}%)", topic, result.confidence * 100.0);
        for step in &result.steps {
            println!("   {}", step);
        }
        println!("   Puede explicar el POR QUÉ: {}\n", if result.can_explain_why { "SÍ ✅" } else { "NO ❌" });
    }

    // 3. Verificar el SPSC de referencia del PatternLibrary
    println!("═══ VERIFICACIÓN DEL SPSC DE REFERENCIA ═══\n");
    if let Some(spsc_pattern) = library.patterns.get("complete_spsc_ringbuffer") {
        let reqs = vec![
            "unsafe_cell", "atomic_ordering", "no_vec_hot_path", "power_of_two",
            "const_generics", "initialization", "shared_ref_push", "send_sync",
            "cache_padding",
        ];
        let result = verifier.verify_code("SPSC Referencia", &spsc_pattern.correct_code, &reqs);
        println!("📊 Score del código de referencia: {:.1}/10", result.score);
        for c in &result.correct_concepts { println!("  {}", c); }
        for v in &result.violations { println!("  {}", v); }
    }

    // 4. Mostrar modelos mentales internalizados
    println!("\n═══ MODELOS MENTALES INTERNALIZADOS ═══");
    println!("  Total: {} modelos | {} cadenas causales", reasoner.mental_models.len(), reasoner.causal_chains.len());
    for (name, model) in &reasoner.mental_models {
        println!("  🧠 {} [{}]: {}", name, model.domain, model.what_it_is.chars().take(80).collect::<String>());
    }

    println!("\n═══ MAESTRÍA POR DOMINIO ═══");
    for (domain, level) in &reasoner.domain_mastery {
        let bar_len = (*level * 20.0) as usize;
        let bar: String = "█".repeat(bar_len) + &"░".repeat(20 - bar_len);
        println!("  {:>14}: {} {:.0}%", domain, bar, level * 100.0);
    }
}
