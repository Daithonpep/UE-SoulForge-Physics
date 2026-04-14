// nexus/cross_pollination.rs
// GENESIS Bridge — Transferencia de conocimiento cruzado entre categorías
//
// Ejemplo: La forma aerodinámica de un ala de avión puede inspirar
// un techo resistente a huracanes. Las vigas de un puente pueden
// enseñar cómo distribuir peso en una mesa.

use crate::sofia::primitives::*;
use crate::sofia::universal_validator::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Sistema de transferencia de conocimiento entre categorías
pub struct GENESISBridge {
    functional_analogies: HashMap<AnalogyConcept, Vec<PrimitiveMapping>>,
    successful_transfers: Vec<TransferRecord>,
    pub transfer_probability: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum AnalogyConcept {
    LateralForceResistance,
    LoadDistribution,
    AerodynamicProfile,
    StructuralSupport,
    EnvironmentalShield,
    FluidContainment,
    ArticulatedMovement,
    MaterialEfficiency,
    StressDistribution,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimitiveMapping {
    pub donor_primitive: FunctionalPrimitive,
    pub donor_category: ObjectCategory,
    pub donor_context: String,
    pub receptor_primitive: FunctionalPrimitive,
    pub receptor_category: ObjectCategory,
    pub transformation: GeometricTransformation,
    pub success_probability: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeometricTransformation {
    pub scale_factors: [f32; 3],
    pub rotation_offset: [f32; 3],
    pub aspect_ratio_adjustment: f32,
    pub density_modifier: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransferRecord {
    pub analogy: String,
    pub from_object: String,
    pub to_object: String,
    pub fitness_improvement: f32,
    pub innovation_score: f32,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PerformanceIssue {
    StructuralFailure { reason: String },
    MaterialWaste,
    PoorAerodynamics,
    InsufficientLoadCapacity,
    Instability,
}

#[derive(Debug, Clone)]
pub struct TransferSuggestion {
    pub analogy: AnalogyConcept,
    pub mapping: PrimitiveMapping,
    pub issue_addressed: PerformanceIssue,
    pub estimated_improvement: f32,
}

#[derive(Debug, Clone)]
pub struct TransferResult {
    pub success: bool,
    pub new_primitive: PrimitiveInstance,
    pub description: String,
}

impl GENESISBridge {
    pub fn new() -> Self {
        let mut bridge = Self {
            functional_analogies: HashMap::new(),
            successful_transfers: Vec::new(),
            transfer_probability: 0.15,
        };
        bridge.initialize_analogies();
        bridge
    }

    fn initialize_analogies(&mut self) {
        // Ala → Techo (resistencia lateral)
        self.functional_analogies.insert(
            AnalogyConcept::LateralForceResistance,
            vec![
                PrimitiveMapping {
                    donor_primitive: FunctionalPrimitive::Wing,
                    donor_category: ObjectCategory::Vehicle,
                    donor_context: "Ala de avión - perfil aerodinámico que resiste vientos".into(),
                    receptor_primitive: FunctionalPrimitive::Ceiling,
                    receptor_category: ObjectCategory::Architecture,
                    transformation: GeometricTransformation {
                        scale_factors: [10.0, 1.0, 10.0],
                        rotation_offset: [0.0, 0.0, 0.0],
                        aspect_ratio_adjustment: 0.3,
                        density_modifier: 2.0,
                    },
                    success_probability: 0.75,
                },
            ],
        );

        // Puente → Mesa (distribución de carga)
        self.functional_analogies.insert(
            AnalogyConcept::LoadDistribution,
            vec![
                PrimitiveMapping {
                    donor_primitive: FunctionalPrimitive::Span,
                    donor_category: ObjectCategory::Architecture,
                    donor_context: "Viga de puente con arcos de soporte - distribuye peso".into(),
                    receptor_primitive: FunctionalPrimitive::Platform,
                    receptor_category: ObjectCategory::Furniture,
                    transformation: GeometricTransformation {
                        scale_factors: [0.1, 0.2, 0.1],
                        rotation_offset: [0.0, 0.0, 0.0],
                        aspect_ratio_adjustment: 1.2,
                        density_modifier: 0.5,
                    },
                    success_probability: 0.8,
                },
            ],
        );

        // Bisagra → Articulación
        self.functional_analogies.insert(
            AnalogyConcept::ArticulatedMovement,
            vec![
                PrimitiveMapping {
                    donor_primitive: FunctionalPrimitive::Hinge,
                    donor_category: ObjectCategory::Architecture,
                    donor_context: "Bisagra de puerta - rotación en un eje".into(),
                    receptor_primitive: FunctionalPrimitive::Support,
                    receptor_category: ObjectCategory::Tool,
                    transformation: GeometricTransformation {
                        scale_factors: [0.5, 0.5, 0.5],
                        rotation_offset: [0.0, 0.0, 0.0],
                        aspect_ratio_adjustment: 1.0,
                        density_modifier: 1.2,
                    },
                    success_probability: 0.7,
                },
            ],
        );

        // Casco de barco → Contenedor
        self.functional_analogies.insert(
            AnalogyConcept::FluidContainment,
            vec![
                PrimitiveMapping {
                    donor_primitive: FunctionalPrimitive::Enclosure,
                    donor_category: ObjectCategory::Vehicle,
                    donor_context: "Casco de barco - contención de presión de agua".into(),
                    receptor_primitive: FunctionalPrimitive::Container,
                    receptor_category: ObjectCategory::Container,
                    transformation: GeometricTransformation {
                        scale_factors: [0.01, 0.02, 0.01],
                        rotation_offset: [0.0, 0.0, 0.0],
                        aspect_ratio_adjustment: 1.5,
                        density_modifier: 0.3,
                    },
                    success_probability: 0.85,
                },
            ],
        );

        // Hueso → Columna (eficiencia material)
        self.functional_analogies.insert(
            AnalogyConcept::MaterialEfficiency,
            vec![
                PrimitiveMapping {
                    donor_primitive: FunctionalPrimitive::Support,
                    donor_category: ObjectCategory::Nature,
                    donor_context: "Estructura ósea - máxima resistencia con mínimo material".into(),
                    receptor_primitive: FunctionalPrimitive::Support,
                    receptor_category: ObjectCategory::Architecture,
                    transformation: GeometricTransformation {
                        scale_factors: [5.0, 10.0, 5.0],
                        rotation_offset: [0.0, 0.0, 0.0],
                        aspect_ratio_adjustment: 0.8,
                        density_modifier: 1.0,
                    },
                    success_probability: 0.9,
                },
            ],
        );

        // Parabrisas → Ventana
        self.functional_analogies.insert(
            AnalogyConcept::EnvironmentalShield,
            vec![
                PrimitiveMapping {
                    donor_primitive: FunctionalPrimitive::Shield,
                    donor_category: ObjectCategory::Vehicle,
                    donor_context: "Parabrisas curvado - protege manteniendo visibilidad".into(),
                    receptor_primitive: FunctionalPrimitive::Opening,
                    receptor_category: ObjectCategory::Architecture,
                    transformation: GeometricTransformation {
                        scale_factors: [3.0, 3.0, 0.5],
                        rotation_offset: [0.0, 0.0, 0.0],
                        aspect_ratio_adjustment: 1.2,
                        density_modifier: 0.8,
                    },
                    success_probability: 0.7,
                },
            ],
        );
    }

    /// Sugiere transferencias para un diseño con problemas
    pub fn suggest_transfers(
        &self,
        current_design: &UniversalDesign,
        issues: &[PerformanceIssue],
    ) -> Vec<TransferSuggestion> {
        let mut suggestions = Vec::new();

        for issue in issues {
            let relevant = self.map_issue_to_analogy(issue);
            for analogy in relevant {
                if let Some(mappings) = self.functional_analogies.get(&analogy) {
                    for mapping in mappings {
                        if self.matches_category(&current_design.object_type, &mapping.receptor_category) {
                            suggestions.push(TransferSuggestion {
                                analogy: analogy.clone(),
                                mapping: mapping.clone(),
                                issue_addressed: issue.clone(),
                                estimated_improvement: self.estimate_improvement(issue, mapping),
                            });
                        }
                    }
                }
            }
        }

        suggestions.sort_by(|a, b| b.estimated_improvement.partial_cmp(&a.estimated_improvement).unwrap_or(std::cmp::Ordering::Equal));
        suggestions
    }

    /// Aplica transferencia a un diseño
    pub fn apply_transfer(
        &mut self,
        design: &mut UniversalDesign,
        suggestion: &TransferSuggestion,
    ) -> TransferResult {
        log::info!("🧬 GENESIS: Transferencia {:?} → {:?} (ctx: {})",
            suggestion.mapping.donor_primitive,
            suggestion.mapping.receptor_primitive,
            suggestion.mapping.donor_context
        );

        let new_primitive = PrimitiveInstance {
            primitive_type: suggestion.mapping.donor_primitive.clone(),
            role_name: format!("{:?}_transferred", suggestion.mapping.donor_primitive),
            position: [0.0, 0.0, 0.0],
            rotation: suggestion.mapping.transformation.rotation_offset,
            scale: suggestion.mapping.transformation.scale_factors,
            properties: {
                let mut p = HashMap::new();
                p.insert("cross_pollinated".into(), "true".into());
                p.insert("density_modifier".into(), suggestion.mapping.transformation.density_modifier.to_string());
                p
            },
        };

        design.primitives.push(new_primitive.clone());

        TransferResult {
            success: true,
            new_primitive,
            description: format!("Aplicada lógica de {} a {}", suggestion.mapping.donor_context, design.object_type),
        }
    }

    /// Registra una transferencia exitosa para aprender
    pub fn record_success(&mut self, description: String, improvement: f32) {
        self.successful_transfers.push(TransferRecord {
            analogy: "auto".into(),
            from_object: "donor".into(),
            to_object: "receptor".into(),
            fitness_improvement: improvement,
            innovation_score: improvement * 1.5,
            description,
        });

        if improvement > 0.2 {
            self.transfer_probability = (self.transfer_probability + 0.01).min(0.5);
        }
    }

    fn map_issue_to_analogy(&self, issue: &PerformanceIssue) -> Vec<AnalogyConcept> {
        match issue {
            PerformanceIssue::StructuralFailure { reason } if reason.contains("lateral") =>
                vec![AnalogyConcept::LateralForceResistance],
            PerformanceIssue::StructuralFailure { reason } if reason.contains("weight") =>
                vec![AnalogyConcept::LoadDistribution, AnalogyConcept::MaterialEfficiency],
            PerformanceIssue::MaterialWaste =>
                vec![AnalogyConcept::MaterialEfficiency],
            PerformanceIssue::PoorAerodynamics =>
                vec![AnalogyConcept::AerodynamicProfile],
            _ => vec![],
        }
    }

    fn estimate_improvement(&self, issue: &PerformanceIssue, mapping: &PrimitiveMapping) -> f32 {
        let severity = match issue {
            PerformanceIssue::StructuralFailure { .. } => 1.0,
            PerformanceIssue::MaterialWaste => 0.6,
            PerformanceIssue::PoorAerodynamics => 0.7,
            _ => 0.5,
        };
        mapping.success_probability * severity
    }

    fn matches_category(&self, object_type: &str, category: &ObjectCategory) -> bool {
        match (object_type, category) {
            ("table" | "chair" | "bed" | "sofa", ObjectCategory::Furniture) => true,
            ("building" | "bridge" | "door", ObjectCategory::Architecture) => true,
            ("car" | "airplane" | "boat", ObjectCategory::Vehicle) => true,
            ("cup", ObjectCategory::Container) => true,
            _ => false,
        }
    }

    pub fn innovation_report(&self) -> serde_json::Value {
        serde_json::json!({
            "total_transfers": self.successful_transfers.len(),
            "transfer_probability": self.transfer_probability,
            "top_innovations": self.successful_transfers.iter()
                .rev().take(10)
                .map(|r| serde_json::json!({
                    "description": r.description,
                    "improvement": r.fitness_improvement,
                    "innovation_score": r.innovation_score,
                }))
                .collect::<Vec<_>>(),
        })
    }
}
