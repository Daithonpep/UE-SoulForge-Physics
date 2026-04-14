// harmonia/example_usage.rs

#[cfg(test)]
mod tests {
    use crate::harmonia::*;
    use crate::harmonia::fitness::HARMONIACore;
    use crate::harmonia::batch_evaluator::HARMONIABatchEvaluator;
    use crate::phoenix::reality_profiles::REALITYEngine;
    use crate::sofia::universal_validator::*;
    use crate::sofia::primitives::FunctionalPrimitive;
    use std::collections::HashMap;

    #[test]
    fn test_complete_harmonia_system() {
        let separator_heavy = "═".repeat(60);
        let separator_light = "─".repeat(60);

        println!("\n{}", separator_heavy);
        println!("HARMONIA COMPLETE SYSTEM TEST");
        println!("{}\n", separator_heavy);

        // 1. Setup
        let reality = REALITYEngine::new();
        let material_library = reality.get_active_profile().material_library.clone();

        // 2. Diseño inicial
        let initial_design = UniversalDesign {
            object_type: "building".to_string(),
            primitives: vec![
                PrimitiveInstance {
                    primitive_type: FunctionalPrimitive::Support,
                    role_name: "column".to_string(),
                    position: [0.0, 0.0, 0.0],
                    rotation: [0.0, 0.0, 0.0],
                    scale: [0.5, 5.0, 0.5],
                    properties: HashMap::new(),
                },
            ],
            bounding_box: BoundingBox { width: 0.5, height: 5.0, depth: 0.5 },
        };

        // 3. Configurar HARMONIA
        let mut harmonia_core = HARMONIACore::new(material_library.clone());

        // TEST 1: Evaluación en diferentes contextos
        println!("\n{}", separator_light);
        println!("TEST 1: Multi-Context Evaluation");
        println!("{}", separator_light);

        for context in &["ENGINEERING_REALISTIC", "HIGH_DESIGN_ARCHITECTURE", "FANTASY_CREATIVE"] {
            harmonia_core.muse.set_context(context).unwrap();
            
            // evaluate is no longer async
            let fitness = harmonia_core.evaluate(
                &initial_design,
                7.0,  // Richter
                20.0, // m/s viento
            );

            println!("\n{} → Fitness: {:.1}%", context, fitness.total_fitness * 100.0);
        }

        // TEST 2: Batch Evaluation
        println!("\n{}", separator_light);
        println!("TEST 2: Batch Evaluation (100 variaciones)");
        println!("{}", separator_light);

        let mut batch_evaluator = HARMONIABatchEvaluator::new(
            material_library.clone(),
            4,    // 4 threads
            true, // quick mode
        );

        let variations: Vec<_> = (0..100)
            .map(|_| {
                let mut variant = initial_design.clone();
                // Variar posición
                variant.primitives[0].position[0] = fastrand::f32() * 10.0 - 5.0;
                variant.primitives[0].scale[1] = 3.0 + fastrand::f32() * 5.0;
                variant
            })
            .collect();

        // No longer async
        let batch_result = batch_evaluator.evaluate_batch(
            &variations,
            "ENGINEERING_REALISTIC",
            6.0,
            15.0,
        );

        println!("Best: {:.1}% | Avg: {:.1}% | Rate: {:.0} designs/sec",
            batch_result.fitness_scores[batch_result.best_index] * 100.0,
            batch_result.average_fitness * 100.0,
            batch_result.designs_per_second,
        );

        // TEST 3: Genetic Algorithm Evolution
        println!("\n{}", separator_light);
        println!("TEST 3: Genetic Algorithm Evolution");
        println!("{}", separator_light);

        let ga_config = genetic_algorithm::GeneticConfig {
            population_size: 20,
            num_generations: 10,
            mutation_rate: 0.2,
            crossover_rate: 0.7,
            ..Default::default()
        };

        let mut genesis_loop = genetic_algorithm::GENESISLoop::new(ga_config);

        // No longer async
        let evolution_result = genesis_loop.evolve(
            &initial_design,
            &mut batch_evaluator,
            "HIGH_DESIGN_ARCHITECTURE",
            6.5,
            25.0,
        );

        println!("\n🏆 EVOLUTION RESULTS:");
        println!("   Best fitness: {:.3}", evolution_result.best_fitness);
        println!("   Converged at generation: {}", evolution_result.convergence_generation);
        println!("   Final diversity: {:.2}", 
            evolution_result.diversity_history.last().unwrap_or(&0.0));

        // TEST 4: Context Inference
        println!("\n{}", separator_light);
        println!("TEST 4: Context Inference from Natural Language");
        println!("{}", separator_light);

        let test_prompts = vec![
            "Build a safe earthquake-resistant structure",
            "Create a beautiful artistic masterpiece",
            "Design a fast aerodynamic race car",
            "Make a magical fantasy castle",
        ];

        for prompt in test_prompts {
            let inferred = harmonia_core.infer_context_from_prompt(prompt);
            println!("\n\"{}\"", prompt);
            println!("   → Context: {}", inferred);
        }

        println!("\n{}", separator_heavy);
        println!("ALL TESTS COMPLETED");
        println!("{}\n", separator_heavy);
    }
}
