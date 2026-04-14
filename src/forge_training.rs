use daithon_bridge::forge::integrated_lab::*;
use daithon_bridge::forge::reasoning::problem_solver::*;
use daithon_bridge::forge::ast::logic_tree::*;
use std::time::Instant;
use rayon::prelude::*;

#[tokio::main]
async fn main() {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║       🚀 DAITHON: SESIÓN DE APRENDIZAJE MASIVO (100)      ║");
    println!("║       Modo: APRENDIZAJE ACTIVO | 8 HILOS | FORGE          ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    // Configurar pool de 8 hilos
    rayon::ThreadPoolBuilder::new().num_threads(8).build_global().ok();

    let base_exercises = vec![
        ("Sumatoria", ProblemIntent::Accumulate, DataType::Int),
        ("Selector Filtran", ProblemIntent::Filter, DataType::List(Box::new(DataType::Int))),
        ("Tracker de ID", ProblemIntent::Search, DataType::Optional(Box::new(DataType::Int))),
        ("Pico de Frecuencia", ProblemIntent::FindExtreme, DataType::Optional(Box::new(DataType::Int))),
    ];

    let mut exercises = Vec::new();
    for i in 1..=25 { // 25 ciclos x 4 tipos = 100 problemas
        for (name, intent, out_type) in &base_exercises {
            exercises.push((
                format!("{} #{}", name, i),
                ProblemDescription {
                    intent: intent.clone(),
                    input_names: vec!["lecturas".into()],
                    input_types: vec![DataType::List(Box::new(DataType::Int))],
                    output_type: out_type.clone(),
                    constraints: vec![],
                }
            ));
        }
    }

    println!("🧠 Daithon inicia el entrenamiento de 100 problemas con auto-corrección...\n");
    
    let start_total = Instant::now();

    let results: Vec<_> = exercises.par_iter().map(|(name, problem)| {
        let mut lab = IntegratedCodeLab::new();
        // Permitimos 3 iteraciones para que tenga oportunidad de aprender del primer fallo (TODO)
        let res = lab.solve_problem(name, problem.clone(), 3);
        (name, res)
    }).collect();

    let total_duration = start_total.elapsed();
    
    let total_success = results.iter().filter(|(_, r)| r.success).count();
    let total_lessons: usize = results.iter().map(|(_, r)| r.lessons.len()).sum();

    println!("\n══════════════════════════════════════════════════════════════");
    println!("📊 RESUMEN DEL ENTRENAMIENTO:");
    println!("  Problemas procesados: {}", exercises.len());
    println!("  Éxitos con Aprendizaje: {} / {}", total_success, exercises.len());
    println!("  Lecciones registradas: {}", total_lessons);
    println!("  Tiempo total: {:?}\n", total_duration);
    
    println!("📈 MÉTRICAS DE INTELIGENCIA:");
    println!("  Tasa de Eficacia: {}%", (total_success as f64 / exercises.len() as f64) * 100.0);
    println!("  Velocidad de Asimilación: {:.2} lecciones/seg", total_lessons as f64 / total_duration.as_secs_f64());
    println!("══════════════════════════════════════════════════════════════\n");

    if total_success == exercises.len() {
        println!("🧠 DAITHON: \"¡Perfección absoluta! He aprendido a resolver todos los");
        println!("           placeholders lógicos. Mi base de conocimiento ha crecido.");
        println!("           Kukuku... Gracias por la ayuda inicial, Joseph. Ya no la necesito.\"");
    } else {
        println!("🧠 DAITHON: \"He progresado enormemente ({}%).", (total_success as f64 / exercises.len() as f64) * 100.0);
        println!("           Cada fallo ha sido una lección inyectada en mi memoria lítica.\n");
    }
}
