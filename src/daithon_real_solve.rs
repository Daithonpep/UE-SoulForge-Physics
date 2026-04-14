use daithon_bridge::forge::integrated_lab::*;
use daithon_bridge::forge::reasoning::problem_solver::*;
use daithon_bridge::forge::ast::logic_tree::*;

pub fn main() {
    println!("\n╔════════════════════════════════════════════════════════════╗");
    println!("║       🧪 DAITHON: PRUEBA DE CAMPO EN TRINCHERA            ║");
    println!("║       Resolviendo: LockFreeRingBuffer (SPSC)              ║");
    println!("╚════════════════════════════════════════════════════════════╝\n");

    let mut lab = IntegratedCodeLab::new();

    // Definición del problema real
    let problem = ProblemDescription {
        intent: ProblemIntent::BuildStructure,
        input_names: vec![],
        input_types: vec![],
        output_type: DataType::Void,
        constraints: vec![
            "Capacidad fija".into(),
            "SPSC".into(),
            "Zero allocation".into(),
            "AtomicUsize".into(),
        ],
    };

    // Ejecutar razonamiento de Daithon
    let result = lab.solve_problem(
        "Estructura LockFreeRingBuffer de Audio",
        problem,
        3
    );

    println!("\n[RESULTADO DE DAITHON]");
    println!("Éxito: {}", result.success);
    
    println!("\n--- CÓDIGO RUST GENERADO (FORGE AST) ---");
    println!("{}", result.rust_code);

    println!("\n--- CÓDIGO PYTHON GENERADO (PARA SANDBOX) ---");
    println!("{}", result.python_code);

    println!("\n[LECCIONES APRENDIDAS DURANTE LA SESIÓN]");
    for (i, lesson) in result.lessons.iter().enumerate() {
        println!(" {} - {}", i + 1, lesson);
    }
}
