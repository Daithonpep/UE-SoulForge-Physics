// nexus/entropy.rs
// ENTROPY — Motor de Caos Controlado
//
// Introduce mutaciones "imposibles" que violan las reglas establecidas.
// Si por casualidad una de estas mutaciones MEJORA el fitness a pesar de
// violar reglas, se registra como "Innovación Descubierta" y se integra
// al conocimiento de Daithon.

use crate::sofia::primitives::*;
use crate::sofia::universal_validator::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct ENTROPY {
    pub chaos_probability: f32,
    pub discovered_innovations: Vec<Innovation>,
    violation_tolerance: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Innovation {
    pub name: String,
    pub description: String,
    pub violated_rules: Vec<String>,
    pub survival_reason: String,
    pub fitness_score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChaosDecision {
    pub allow_chaos: bool,
    pub chaos_type: ChaosType,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ChaosType {
    IgnoreQuantityConstraint,
    ViolateSymmetry,
    UnconventionalPrimitive,
    ExtremeScaling,
    MaterialMixing,
    InvertedLogic,
}

impl ENTROPY {
    pub fn new() -> Self {
        Self {
            chaos_probability: 0.05,
            discovered_innovations: Vec::new(),
            violation_tolerance: 0.3,
        }
    }

    /// Decide si inyectar caos en esta generación
    pub fn should_introduce_chaos(&self) -> ChaosDecision {
        let roll = fastrand::f32();

        if roll < self.chaos_probability {
            let chaos_type = Self::random_chaos_type();
            log::info!("🌀 ENTROPY: Caos {:?} activado", chaos_type);

            ChaosDecision {
                allow_chaos: true,
                chaos_type,
                reason: "Explorando más allá de las reglas".into(),
            }
        } else {
            ChaosDecision {
                allow_chaos: false,
                chaos_type: ChaosType::IgnoreQuantityConstraint,
                reason: "Siguiendo reglas".into(),
            }
        }
    }

    fn random_chaos_type() -> ChaosType {
        match fastrand::usize(0..6) {
            0 => ChaosType::IgnoreQuantityConstraint,
            1 => ChaosType::ViolateSymmetry,
            2 => ChaosType::UnconventionalPrimitive,
            3 => ChaosType::ExtremeScaling,
            4 => ChaosType::MaterialMixing,
            _ => ChaosType::InvertedLogic,
        }
    }

    /// Aplica mutación caótica a un diseño
    pub fn apply_chaos(&self, design: &mut UniversalDesign, chaos_type: &ChaosType) {
        match chaos_type {
            ChaosType::IgnoreQuantityConstraint => {
                let extra = fastrand::u32(3..10);
                for _ in 0..extra {
                    design.primitives.push(PrimitiveInstance {
                        primitive_type: FunctionalPrimitive::Support,
                        role_name: "chaos_support".into(),
                        position: [
                            (fastrand::f32() - 0.5) * 2.0,
                            fastrand::f32(),
                            (fastrand::f32() - 0.5) * 2.0,
                        ],
                        rotation: [0.0; 3],
                        scale: [0.1, 0.5, 0.1],
                        properties: {
                            let mut p = HashMap::new();
                            p.insert("chaos".into(), "true".into());
                            p
                        },
                    });
                }
            }

            ChaosType::ViolateSymmetry => {
                for prim in design.primitives.iter_mut() {
                    if prim.role_name.contains("leg") || prim.role_name.contains("support") {
                        prim.position[0] += (fastrand::f32() - 0.5) * 0.5;
                        prim.position[2] += (fastrand::f32() - 0.5) * 0.5;
                    }
                }
            }

            ChaosType::UnconventionalPrimitive => {
                if let Some(platform) = design.primitives.iter_mut()
                    .find(|p| p.primitive_type == FunctionalPrimitive::Platform)
                {
                    platform.primitive_type = FunctionalPrimitive::Wing;
                }
            }

            ChaosType::ExtremeScaling => {
                let factor = if fastrand::bool() { 0.2 } else { 5.0 };
                design.bounding_box.height *= factor;
                for prim in design.primitives.iter_mut() {
                    prim.scale[1] *= factor;
                }
            }

            ChaosType::MaterialMixing => {
                // Mezclar tipos de primitivas aleatoriamente
                let all_types = vec![
                    FunctionalPrimitive::Support,
                    FunctionalPrimitive::Wing,
                    FunctionalPrimitive::Span,
                    FunctionalPrimitive::Container,
                    FunctionalPrimitive::Grip,
                ];
                if let Some(prim) = design.primitives.iter_mut().last() {
                    prim.primitive_type = all_types[fastrand::usize(0..all_types.len())].clone();
                }
            }

            ChaosType::InvertedLogic => {
                for prim in design.primitives.iter_mut() {
                    prim.position[1] *= -1.0;
                    prim.rotation[0] += 180.0;
                }
            }
        }
    }

    /// Evalúa si el caos produjo innovación
    pub fn evaluate_chaos_result(
        &mut self,
        design: &UniversalDesign,
        fitness_before: f32,
        fitness_after: f32,
        violated_rules: Vec<String>,
    ) -> bool {
        let improvement = fitness_after - fitness_before;

        if improvement > self.violation_tolerance && !violated_rules.is_empty() {
            let prim_types: std::collections::HashSet<_> = design.primitives.iter()
                .map(|p| format!("{:?}", p.primitive_type))
                .collect();

            let innovation = Innovation {
                name: format!("Chaos Discovery #{}", self.discovered_innovations.len() + 1),
                description: format!("{} con {} primitivas: {}",
                    design.object_type,
                    prim_types.len(),
                    prim_types.into_iter().collect::<Vec<_>>().join(", ")
                ),
                violated_rules,
                survival_reason: format!("Mejoró fitness en {:.1}%", improvement * 100.0),
                fitness_score: fitness_after,
            };

            log::info!("🎉 ENTROPY INNOVACIÓN: {} — {}", innovation.name, innovation.survival_reason);
            self.discovered_innovations.push(innovation);
            self.chaos_probability = (self.chaos_probability * 1.1).min(0.25);
            true
        } else if improvement < -0.5 {
            self.chaos_probability = (self.chaos_probability * 0.9).max(0.02);
            false
        } else {
            false
        }
    }

    /// Aplica caos a nivel de genoma (modifica genes directamente)
    pub fn apply_chaos_to_genes(&self, genes: &mut Vec<crate::design_evolution::mutation_engine::Gene>, chaos_type: &ChaosType) {
        use crate::design_evolution::mutation_engine::GeneValue;

        match chaos_type {
            ChaosType::IgnoreQuantityConstraint => {
                // Forzar conteo extremo de patas/soportes
                for gene in genes.iter_mut() {
                    if gene.trait_name == "leg_count" {
                        if let GeneValue::Integer(ref mut v) = gene.value {
                            *v = fastrand::i32(1..12); // De 1 a 12 patas
                        }
                    }
                }
            }
            ChaosType::ExtremeScaling => {
                for gene in genes.iter_mut() {
                    if gene.trait_name == "base_scale" {
                        if let GeneValue::Vector(ref mut v) = gene.value {
                            let factor = if fastrand::bool() { 0.3 } else { 3.0 };
                            v[0] *= factor;
                            v[1] *= factor;
                            v[2] *= factor;
                        }
                    }
                }
            }
            ChaosType::ViolateSymmetry => {
                for gene in genes.iter_mut() {
                    if gene.trait_name == "leg_inset" {
                        if let GeneValue::Scalar(ref mut v) = gene.value {
                            *v = fastrand::f32(); // Cualquier valor de inset
                        }
                    }
                }
            }
            _ => {
                // Para otros tipos, mutar todos los scalars agresivamente
                for gene in genes.iter_mut() {
                    if let GeneValue::Scalar(ref mut v) = gene.value {
                        *v = (*v + (fastrand::f32() - 0.5) * 0.8).clamp(0.0, 1.0);
                    }
                }
            }
        }
    }

    pub fn innovation_report(&self) -> serde_json::Value {
        serde_json::json!({
            "total_innovations": self.discovered_innovations.len(),
            "chaos_probability": self.chaos_probability,
            "innovations": self.discovered_innovations.iter()
                .rev().take(10)
                .map(|i| serde_json::json!({
                    "name": i.name,
                    "description": i.description,
                    "fitness": i.fitness_score,
                    "reason": i.survival_reason,
                    "rules_broken": i.violated_rules.len(),
                }))
                .collect::<Vec<_>>(),
        })
    }
}
