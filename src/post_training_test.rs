use daithon_bridge::forge::training::concept_verifier::ConceptVerifier;
use daithon_bridge::forge::integrated_lab::*;
use daithon_bridge::forge::reasoning::problem_solver::*;
use daithon_bridge::forge::ast::logic_tree::*;

fn main() {
    println!("\n╔════════════════════════════════════════════════════════════════╗");
    println!("║  🧪 DAITHON: PRUEBA POST-ENTRENAMIENTO                       ║");
    println!("║  Verificando si las 7 reglas asimiladas corrigieron su SPSC   ║");
    println!("╚════════════════════════════════════════════════════════════════╝\n");

    let mut lab = IntegratedCodeLab::new();
    let problem = ProblemDescription {
        intent: ProblemIntent::BuildStructure,
        input_names: vec![],
        input_types: vec![],
        output_type: DataType::Void,
        constraints: vec!["SPSC".into(), "Lock-Free".into(), "UnsafeCell".into(), "AtomicUsize".into()],
    };

    let result = lab.solve_problem("SpscRingBuffer Post-Training", problem, 1);
    let generated_rust = &result.rust_code;

    println!("═══ CÓDIGO GENERADO POR FORGE ═══");
    println!("{}", generated_rust);

    let mut verifier = ConceptVerifier::new();
    let requirements = vec![
        "unsafe_cell", "atomic_ordering", "no_vec_hot_path", "power_of_two",
        "const_generics", "initialization", "shared_ref_push", "cache_padding",
    ];

    let verification = verifier.verify_code("SPSC Post-Training", generated_rust, &requirements);

    println!("\n═══ VERIFICACIÓN ═══");
    println!("📊 Puntuación: {:.1}/10", verification.score);
    for c in &verification.correct_concepts { println!("  {}", c); }
    for v in &verification.violations { println!("  {}", v); }
    println!("\n¿Aprobado? {}", if verification.needs_retry { "NO 🔄" } else { "SÍ ✅" });
}
