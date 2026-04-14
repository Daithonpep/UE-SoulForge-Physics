// harmonia/aesthetics.rs
use serde::{Deserialize, Serialize};
use crate::sofia::universal_validator::*;

const PHI: f32 = 1.618033988749; // Proporción áurea

/// Motor de evaluación estética
pub struct FIBONACCIEngine;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AestheticScore {
    pub total_score: f32,
    pub symmetry_score: f32,
    pub golden_ratio_score: f32,
    pub harmony_score: f32,
    pub smoothness_score: f32,
    pub rhythm_score: f32,
    pub visual_balance: f32,
}

impl FIBONACCIEngine {
    /// Evalúa la estética de un diseño según reglas configuradas
    pub fn evaluate(
        design: &UniversalDesign,
        rules: &super::context::AestheticRules,
    ) -> AestheticScore {
        let symmetry_score = Self::evaluate_symmetry(design, &rules.symmetry_type) 
            * rules.symmetry_requirement;

        let golden_ratio_score = Self::evaluate_golden_ratio(design) 
            * rules.golden_ratio_enforcement;

        let harmony_score = Self::evaluate_harmony(design);

        let smoothness_score = Self::evaluate_smoothness(design) 
            * rules.surface_smoothness;

        let rhythm_score = Self::evaluate_rhythmic_patterns(design, &rules.rhythmic_patterns);

        let visual_balance = Self::evaluate_visual_balance(design);

        let total_score = (
            symmetry_score * 0.25 +
            golden_ratio_score * 0.25 +
            harmony_score * 0.15 +
            smoothness_score * 0.15 +
            rhythm_score * 0.10 +
            visual_balance * 0.10
        ).clamp(0.0, 1.0);

        AestheticScore {
            total_score,
            symmetry_score,
            golden_ratio_score,
            harmony_score,
            smoothness_score,
            rhythm_score,
            visual_balance,
        }
    }

    /// Evalúa simetría
    fn evaluate_symmetry(design: &UniversalDesign, symmetry_type: &super::context::SymmetryType) -> f32 {
        use super::context::SymmetryType;

        match symmetry_type {
            SymmetryType::None => 1.0,

            SymmetryType::Bilateral { axis } => {
                Self::evaluate_bilateral_symmetry(design, axis)
            }

            SymmetryType::Radial { sectors } => {
                Self::evaluate_radial_symmetry(design, *sectors)
            }

            SymmetryType::Fractal { depth } => {
                Self::evaluate_fractal_pattern(design, *depth)
            }

            SymmetryType::Combined(types) => {
                types.iter()
                    .map(|t| Self::evaluate_symmetry(design, t))
                    .sum::<f32>() / types.len().max(1) as f32
            }

            _ => 0.5,
        }
    }

    fn evaluate_bilateral_symmetry(design: &UniversalDesign, axis: &str) -> f32 {
        let axis_index = match axis {
            "X" => 0,
            "Y" => 1,
            "Z" => 2,
            _ => return 0.5,
        };

        let center = Self::calculate_centroid(design);
        let mut symmetry_error = 0.0;
        let mut comparisons = 0;

        for prim in &design.primitives {
            let distance_from_center = (prim.position[axis_index] - center[axis_index]).abs();
            
            // Buscar primitiva espejo
            let mirror_pos = center[axis_index] - (prim.position[axis_index] - center[axis_index]);
            
            let has_mirror = design.primitives.iter().any(|other| {
                let pos_diff = (other.position[axis_index] - mirror_pos).abs();
                let type_match = other.primitive_type == prim.primitive_type;
                
                pos_diff < 0.5 && type_match
            });

            if !has_mirror && distance_from_center > 0.1 {
                symmetry_error += 1.0;
            }
            comparisons += 1;
        }

        if comparisons == 0 {
            return 1.0;
        }

        1.0 - (symmetry_error / comparisons as f32).min(1.0)
    }

    fn evaluate_radial_symmetry(design: &UniversalDesign, sectors: u32) -> f32 {
        let center = Self::calculate_centroid(design);
        let angle_per_sector = 360.0 / sectors as f32;
        
        let mut symmetry_score = 0.0;

        for sector in 0..sectors {
            let target_angle = sector as f32 * angle_per_sector;
            let count_in_sector = design.primitives.iter().filter(|p| {
                let dx = p.position[0] - center[0];
                let dz = p.position[2] - center[2];
                let angle = dz.atan2(dx).to_degrees();
                
                let angle_diff = ((angle - target_angle).abs() % 360.0).min(360.0 - (angle - target_angle).abs());
                angle_diff < angle_per_sector / 2.0
            }).count();

            symmetry_score += count_in_sector as f32;
        }

        let expected_per_sector = design.primitives.len() as f32 / sectors as f32;
        let actual_avg = symmetry_score / sectors as f32;

        1.0 - ((actual_avg - expected_per_sector).abs() / expected_per_sector.max(1.0)).min(1.0)
    }

    fn evaluate_fractal_pattern(design: &UniversalDesign, depth: u32) -> f32 {
        // Simplificado: busca auto-similitud en múltiples escalas
        let scales = (1..=depth).map(|d| 2.0_f32.powi(d as i32)).collect::<Vec<_>>();
        
        let mut similarity_score = 0.0;

        for scale in scales {
            let scaled_positions: Vec<[f32; 3]> = design.primitives.iter()
                .map(|p| [
                    p.position[0] / scale,
                    p.position[1] / scale,
                    p.position[2] / scale,
                ])
                .collect();

            let matches = design.primitives.iter().filter(|p| {
                scaled_positions.iter().any(|sp| {
                    let dist = ((p.position[0] - sp[0]).powi(2) +
                               (p.position[1] - sp[1]).powi(2) +
                               (p.position[2] - sp[2]).powi(2)).sqrt();
                    dist < 0.5
                })
            }).count();

            similarity_score += matches as f32 / design.primitives.len().max(1) as f32;
        }

        similarity_score / depth.max(1) as f32
    }

    /// Evalúa proporción áurea
    fn evaluate_golden_ratio(design: &UniversalDesign) -> f32 {
        let bbox = &design.bounding_box;
        let dimensions = [bbox.width, bbox.height, bbox.depth];
        
        let mut phi_scores = Vec::new();

        // Comparar ratios entre dimensiones
        for i in 0..3 {
            for j in (i+1)..3 {
                if dimensions[j] > 1e-6 {
                    let ratio = dimensions[i] / dimensions[j];
                    let distance_from_phi = (ratio - PHI).abs();
                    let distance_from_inverse = (ratio - 1.0/PHI).abs();
                    
                    let score = 1.0 - distance_from_phi.min(distance_from_inverse).min(1.0);
                    phi_scores.push(score);
                }
            }
        }

        // Evaluar distribución de primitivas
        let positions: Vec<f32> = design.primitives.iter()
            .flat_map(|p| p.position.iter().copied())
            .collect();

        if positions.len() > 1 {
            positions.windows(2).for_each(|window| {
                if window[1] > 1e-6 {
                    let ratio = (window[0] / window[1]).abs();
                    let distance = ((ratio - PHI).abs()).min((ratio - 1.0/PHI).abs());
                    phi_scores.push(1.0 - distance.min(1.0));
                }
            });
        }

        if phi_scores.is_empty() {
            0.5
        } else {
            phi_scores.iter().sum::<f32>() / phi_scores.len() as f32
        }
    }

    /// Evalúa armonía general
    fn evaluate_harmony(design: &UniversalDesign) -> f32 {
        // Diversidad balanceada de primitivas
        let primitive_types: std::collections::HashSet<_> = design.primitives.iter()
            .map(|p| format!("{:?}", p.primitive_type))
            .collect();

        let diversity = primitive_types.len() as f32 / design.primitives.len().max(1) as f32;
        let diversity_score = if diversity > 0.3 && diversity < 0.7 {
            1.0
        } else {
            0.5
        };

        // Distribución espacial uniforme
        let center = Self::calculate_centroid(design);
        let avg_distance: f32 = design.primitives.iter()
            .map(|p| {
                ((p.position[0] - center[0]).powi(2) +
                 (p.position[1] - center[1]).powi(2) +
                 (p.position[2] - center[2]).powi(2)).sqrt()
            })
            .sum::<f32>() / design.primitives.len().max(1) as f32;

        let distances: Vec<f32> = design.primitives.iter()
            .map(|p| {
                ((p.position[0] - center[0]).powi(2) +
                 (p.position[1] - center[1]).powi(2) +
                 (p.position[2] - center[2]).powi(2)).sqrt()
            })
            .collect();

        let variance = distances.iter()
            .map(|d| (d - avg_distance).powi(2))
            .sum::<f32>() / distances.len().max(1) as f32;

        let distribution_score = 1.0 - (variance / avg_distance.max(1.0)).min(1.0);

        (diversity_score + distribution_score) / 2.0
    }

    /// Evalúa suavidad de superficies
    fn evaluate_smoothness(design: &UniversalDesign) -> f32 {
        // Basado en cambios abruptos de escala
        let mut smoothness = 1.0_f32;

        for i in 0..design.primitives.len().saturating_sub(1) {
            let scale_diff = (
                (design.primitives[i].scale[0] - design.primitives[i+1].scale[0]).abs() +
                (design.primitives[i].scale[1] - design.primitives[i+1].scale[1]).abs() +
                (design.primitives[i].scale[2] - design.primitives[i+1].scale[2]).abs()
            ) / 3.0;

            if scale_diff > 2.0 {
                smoothness -= 0.1;
            }
        }

        smoothness.max(0.0_f32)
    }

    /// Evalúa patrones rítmicos
    fn evaluate_rhythmic_patterns(design: &UniversalDesign, patterns: &[super::context::RhythmicPattern]) -> f32 {
        use super::context::RhythmicPattern;

        if patterns.is_empty() {
            return 1.0;
        }

        let mut scores = Vec::new();

        for pattern in patterns {
            let score = match pattern {
                RhythmicPattern::Fibonacci { sequence_depth } => {
                    Self::evaluate_fibonacci_spacing(design, *sequence_depth)
                }

                RhythmicPattern::Modular { module_size, repetitions } => {
                    Self::evaluate_modular_repetition(design, *module_size, *repetitions)
                }

                RhythmicPattern::Gradient { .. } => {
                    Self::evaluate_gradient_progression(design)
                }

                RhythmicPattern::Random { .. } => {
                    0.5 // Neutral para patrones aleatorios
                }
            };

            scores.push(score);
        }

        scores.iter().sum::<f32>() / scores.len().max(1) as f32
    }

    fn evaluate_fibonacci_spacing(design: &UniversalDesign, depth: u32) -> f32 {
        let fib_sequence: Vec<u32> = (0..depth)
            .scan((0, 1), |state, _| {
                let result = state.1;
                *state = (state.1, state.0 + state.1);
                Some(result)
            })
            .collect();

        let positions_y: Vec<f32> = design.primitives.iter()
            .map(|p| p.position[1])
            .collect();

        if positions_y.len() < 2 {
            return 0.5;
        }

        let mut distances: Vec<f32> = positions_y.windows(2)
            .map(|w| (w[1] - w[0]).abs())
            .collect();

        distances.sort_by(|a, b| a.partial_cmp(b).unwrap());

        let fib_ratios: Vec<f32> = fib_sequence.windows(2)
            .map(|w| w[1] as f32 / w[0].max(1) as f32)
            .collect();

        let actual_ratios: Vec<f32> = distances.windows(2)
            .map(|w| w[1] / w[0].max(0.001))
            .collect();

        if actual_ratios.is_empty() || fib_ratios.is_empty() {
            return 0.5;
        }

        let mut match_score = 0.0;
        for actual in &actual_ratios {
            let closest_fib = fib_ratios.iter()
                .map(|fib| (actual - fib).abs())
                .fold(f32::INFINITY, f32::min);

            match_score += 1.0 - (closest_fib / PHI).min(1.0);
        }

        match_score / actual_ratios.len() as f32
    }

    fn evaluate_modular_repetition(design: &UniversalDesign, module_size: f32, repetitions: u32) -> f32 {
        let positions: Vec<f32> = design.primitives.iter()
            .map(|p| p.position[0])
            .collect();

        let expected_positions: Vec<f32> = (0..repetitions)
            .map(|i| i as f32 * module_size)
            .collect();

        let mut match_count = 0;

        for expected in &expected_positions {
            if positions.iter().any(|&p| (p - expected).abs() < module_size * 0.1) {
                match_count += 1;
            }
        }

        match_count as f32 / repetitions.max(1) as f32
    }

    fn evaluate_gradient_progression(design: &UniversalDesign) -> f32 {
        let scales: Vec<f32> = design.primitives.iter()
            .map(|p| (p.scale[0] + p.scale[1] + p.scale[2]) / 3.0)
            .collect();

        if scales.len() < 2 {
            return 0.5;
        }

        let differences: Vec<f32> = scales.windows(2)
            .map(|w| w[1] - w[0])
            .collect();

        let avg_diff = differences.iter().sum::<f32>() / differences.len() as f32;
        let variance = differences.iter()
            .map(|d| (d - avg_diff).powi(2))
            .sum::<f32>() / differences.len() as f32;

        1.0 - (variance / avg_diff.abs().max(0.1)).min(1.0)
    }

    fn evaluate_visual_balance(design: &UniversalDesign) -> f32 {
        let center = Self::calculate_centroid(design);
        
        // Calcular momentos en cada eje
        let moments: [f32; 3] = [0, 1, 2].map(|axis| {
            design.primitives.iter()
                .map(|p| {
                    let mass = p.scale[0] * p.scale[1] * p.scale[2];
                    (p.position[axis] - center[axis]) * mass
                })
                .sum::<f32>()
                .abs()
        });

        let total_mass: f32 = design.primitives.iter()
            .map(|p| p.scale[0] * p.scale[1] * p.scale[2])
            .sum();

        let balance_score = moments.iter()
            .map(|&m| 1.0 - (m / total_mass.max(1.0)).min(1.0))
            .sum::<f32>() / 3.0;

        balance_score
    }

    fn calculate_centroid(design: &UniversalDesign) -> [f32; 3] {
        let sum: [f32; 3] = design.primitives.iter()
            .fold([0.0, 0.0, 0.0], |acc, p| {
                [acc[0] + p.position[0], acc[1] + p.position[1], acc[2] + p.position[2]]
            });

        let count = design.primitives.len().max(1) as f32;
        [sum[0] / count, sum[1] / count, sum[2] / count]
    }
}
