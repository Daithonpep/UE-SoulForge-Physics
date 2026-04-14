use daithon_bridge::code_lab::auto_iteration::iteration_loop::AutoIterationLoop;

fn main() {
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║  🧠 DAITHON: AUTONOMOUS GENERAL ENGINEERING LAB               ║");
    println!("║  Sistema: CodeLab + DeepReasoning + IterationLoop             ║");
    println!("║  Objetivo: Resolver problemas gradualmente en bucle infinito  ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    let mut auto_lab = AutoIterationLoop::new();
    
    // Iniciar entrenamiento masivo
    let sessions = auto_lab.run_auto_session();

    println!("\n═══ RESUMEN DE COMPRENSIÓN SINTÉTICA ═══");
    println!("Sesiones intentadas: {}", sessions.len());
    let aprobadas = sessions.iter().filter(|s| s.success).count();
    println!("Ejercicios superados: {}/{}", aprobadas, sessions.len());

    println!("\nLecciones asimiladas en este ciclo:");
    for session in sessions.iter().filter(|s| s.success) {
        println!(" - {}: Superado en {} iteraciones", session.exercise.title, session.iterations.len());
        for lesson in &session.lessons_learned {
            println!("    🧠: {}", lesson);
        }
    }
}
