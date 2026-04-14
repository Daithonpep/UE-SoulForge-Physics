// harmonia/batch_evaluator.rs
use super::fitness::*;
use crate::sofia::universal_validator::*;
use crate::phoenix::reality_profiles::MaterialLibrary;
use rayon::prelude::*;
use serde::{Deserialize, Serialize};
use std::time::Instant;

/// Evaluador batch ultra-optimizado para entrenamiento RL
pub struct HARMONIABatchEvaluator {
    cores: Vec<HARMONIACore>,
    quick_mode: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchFitnessResult {
    pub fitness_scores: Vec<f32>,
    pub full_evaluations: Vec<MultiObjectiveFitness>,
    pub best_index: usize,
    pub worst_index: usize,
    pub average_fitness: f32,
    pub std_deviation: f32,
    pub evaluation_time_ms: u128,
    pub designs_per_second: f32,
}

impl HARMONIABatchEvaluator {
    pub fn new(material_library: MaterialLibrary, num_threads: usize, quick_mode: bool) -> Self {
        let cores = (0..num_threads)
            .map(|_| HARMONIACore::new(material_library.clone()))
            .collect();

        Self {
            cores,
            quick_mode,
        }
    }

    /// Evaluación batch paralela
    pub fn evaluate_batch(
        &mut self,
        designs: &[UniversalDesign],
        context_name: &str,
        seismic_intensity: f32,
        airflow_velocity: f32,
    ) -> BatchFitnessResult {
        let start = Instant::now();

        println!("\n⚡ HARMONIA BATCH EVALUATION");
        println!("   Designs: {}", designs.len());
        println!("   Context: {}", context_name);
        println!("   Mode: {}", if self.quick_mode { "QUICK" } else { "FULL" });

        // Configurar contexto en todos los cores
        for core in &mut self.cores {
            let _ = core.muse.set_context(context_name);
        }

        let (fitness_scores, full_evaluations) = if self.quick_mode {
            // Modo rápido: solo fitness numérico
            let scores = designs.par_iter()
                .map(|design| {
                    self.quick_fitness(design, seismic_intensity, airflow_velocity)
                })
                .collect::<Vec<f32>>();

            (scores, vec![])
        } else {
            // Modo completo: evaluación detallada
            let evals: Vec<MultiObjectiveFitness> = designs.par_iter()
                .enumerate()
                .map(|(i, design)| {
                    let core_idx = i % self.cores.len();
                    let core = &self.cores[core_idx];
                    
                    // Crear core temporal para evaluación
                    let mut temp_core = HARMONIACore::new(
                        MaterialLibrary { materials: std::collections::HashMap::new() }
                    );
                    temp_core.muse = core.muse.clone();

                    (
                        temp_core.evaluate(design, seismic_intensity, airflow_velocity)
                    )
                })
                .collect();

            let scores = evals.iter().map(|e| e.total_fitness).collect();
            (scores, evals)
        };

        let elapsed = start.elapsed();

        // Estadísticas
        let best_index = fitness_scores.iter()
            .enumerate()
            .max_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(0);

        let worst_index = fitness_scores.iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.partial_cmp(b).unwrap())
            .map(|(i, _)| i)
            .unwrap_or(0);

        let average_fitness = fitness_scores.iter().sum::<f32>() / fitness_scores.len() as f32;

        let variance: f32 = fitness_scores.iter()
            .map(|&f| (f - average_fitness).powi(2))
            .sum::<f32>() / fitness_scores.len() as f32;
        let std_deviation = variance.sqrt();

        let designs_per_second = designs.len() as f32 / elapsed.as_secs_f32();

        println!("   ✓ Completed in {:?}", elapsed);
        println!("   ✓ Rate: {:.0} designs/sec", designs_per_second);
        println!("   ✓ Best: {:.1}% (#{}) | Worst: {:.1}% (#{})", 
            fitness_scores[best_index] * 100.0, best_index,
            fitness_scores[worst_index] * 100.0, worst_index);
        println!("   ✓ Average: {:.1}% ± {:.1}%", 
            average_fitness * 100.0, std_deviation * 100.0);

        BatchFitnessResult {
            fitness_scores,
            full_evaluations,
            best_index,
            worst_index,
            average_fitness,
            std_deviation,
            evaluation_time_ms: elapsed.as_millis(),
            designs_per_second,
        }
    }

    /// Evaluación rápida simplificada (>1000 diseños/seg)
    fn quick_fitness(&self, design: &UniversalDesign, _seismic: f32, _airflow: f32) -> f32 {
        // Aproximaciones rápidas sin simulaciones completas
        
        // 1. Aproximación estructural (masa vs geometría)
        let total_volume: f32 = design.primitives.iter()
            .map(|p| p.scale[0] * p.scale[1] * p.scale[2])
            .sum();
        
        let support_count = design.primitives.iter()
            .filter(|p| matches!(
                p.primitive_type,
                crate::sofia::primitives::FunctionalPrimitive::Support | crate::sofia::primitives::FunctionalPrimitive::Span
            ))
            .count();

        let structural_approx = (support_count as f32 / design.primitives.len().max(1) as f32).min(1.0_f32);

        // 2. Aproximación económica (menos volumen = mejor)
        let economic_approx = 1.0 - (total_volume / 100.0).min(1.0_f32);

        // 3. Aproximación aerodinámica (suavidad de superficies)
        let mut smoothness = 1.0_f32;
        for i in 0..design.primitives.len().saturating_sub(1) {
            let scale_jump = (design.primitives[i].scale[0] - design.primitives[i+1].scale[0]).abs();
            if scale_jump > 1.0 {
                smoothness -= 0.1;
            }
        }
        let aerodynamic_approx = smoothness.max(0.0_f32);

        // 4. Aproximación estética (simetría básica)
        let center_x = design.primitives.iter().map(|p| p.position[0]).sum::<f32>() 
            / design.primitives.len().max(1) as f32;
        
        let mut symmetry_score = 0.0;
        for prim in &design.primitives {
            let mirror_x = center_x - (prim.position[0] - center_x);
            let has_mirror = design.primitives.iter().any(|other| {
                (other.position[0] - mirror_x).abs() < 0.5
            });
            if has_mirror {
                symmetry_score += 1.0;
            }
        }
        let aesthetic_approx = symmetry_score / design.primitives.len().max(1) as f32;

        // Combinar con pesos neutros
        (structural_approx * 0.35 +
         economic_approx * 0.25 +
         aerodynamic_approx * 0.20 +
         aesthetic_approx * 0.20).clamp(0.0, 1.0)
    }
}
