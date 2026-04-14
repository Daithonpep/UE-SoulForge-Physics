// sofia/universal_validator.rs
// Validador Universal — Evalúa CUALQUIER diseño contra su template SOFIA
use super::primitives::*;
use super::template_library::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct UniversalValidator {
    library: UniversalTemplateLibrary,
}

impl UniversalValidator {
    pub fn new() -> Self {
        Self { library: UniversalTemplateLibrary::new() }
    }

    pub fn with_library(library: UniversalTemplateLibrary) -> Self {
        Self { library }
    }

    /// Valida CUALQUIER objeto basándose en su template
    pub fn validate_design(
        &self,
        object_type: &str,
        design: &UniversalDesign,
    ) -> UniversalValidationReport {
        let template = match self.library.get_template(object_type) {
            Some(t) => t,
            None => {
                return UniversalValidationReport {
                    object_type: object_type.to_string(),
                    is_valid: false,
                    score: 0.0,
                    violations: vec![format!("❌ Tipo '{}' desconocido en SOFIA", object_type)],
                    warnings: vec![],
                    suggestions: vec!["Añade este tipo a la biblioteca de templates".into()],
                };
            }
        };

        let mut violations = Vec::new();
        let mut warnings = Vec::new();

        // 1. Validar primitivas requeridas
        for req in &template.required_primitives {
            let count = design.primitives.iter()
                .filter(|p| p.primitive_type == req.primitive)
                .count();

            let (min, max) = match &req.quantity {
                QuantitySpec::Exact(n) => (*n as usize, *n as usize),
                QuantitySpec::Range { min, max } => (*min as usize, *max as usize),
                QuantitySpec::Variable { default } => (1, (*default as usize) * 2),
            };

            if count < min {
                violations.push(format!(
                    "❌ Faltan {:?} ({}): tiene {}, necesita min {}",
                    req.primitive, req.role_name, count, min
                ));
            } else if count > max {
                warnings.push(format!(
                    "⚠️ Exceso de {:?}: tiene {}, max recomendado {}",
                    req.primitive, count, max
                ));
            }
        }

        // 2. Validar relaciones
        for rel in &template.relations {
            if !self.validate_relation(rel, design) {
                violations.push(format!(
                    "❌ Relación violada: {} debe {:?} {}",
                    rel.from_primitive, rel.relation_type, rel.to_primitive
                ));
            }
        }

        // 3. Validar escala
        if !self.validate_scale(design, &template.scale_reference) {
            warnings.push(format!(
                "⚠️ Escala fuera del rango típico (referencia: {})",
                template.scale_reference.reference_entity
            ));
        }

        // 4. Sugerencias
        let suggestions = vec![
            format!("💡 Función: {}", template.function_description),
            format!("📏 Tamaño típico: {:?}", template.scale_reference.typical_size),
        ];

        let violation_penalty = violations.len() as f32 * 0.25;
        let warning_penalty = warnings.len() as f32 * 0.05;
        let score = (1.0 - violation_penalty - warning_penalty).max(0.0);

        UniversalValidationReport {
            object_type: object_type.to_string(),
            is_valid: violations.is_empty(),
            score,
            violations,
            warnings,
            suggestions,
        }
    }

    /// Convierte un DesignGenome del motor evolutivo en un UniversalDesign para validar
    pub fn genome_to_universal_design(
        &self,
        object_type: &str,
        genes: &[crate::design_evolution::mutation_engine::Gene],
    ) -> UniversalDesign {
        use crate::design_evolution::mutation_engine::GeneValue;

        let mut primitives = Vec::new();
        let mut width = 1.0_f32;
        let mut height = 1.0_f32;
        let mut depth = 1.0_f32;

        // Extraer escala
        for g in genes {
            if g.trait_name == "base_scale" {
                if let GeneValue::Vector(v) = &g.value {
                    width = v[0]; height = v[1]; depth = v[2];
                }
            }
        }

        // Crear primitivas según categoría
        match object_type {
            "table" | "mesa" => {
                // Superficie
                primitives.push(PrimitiveInstance {
                    primitive_type: FunctionalPrimitive::Platform,
                    role_name: "top_surface".into(),
                    position: [0.0, height, 0.0],
                    rotation: [0.0, 0.0, 0.0],
                    scale: [width, 0.05, depth],
                    properties: HashMap::new(),
                });
                // Patas
                let mut leg_count = 4;
                let mut leg_inset = 0.0_f32;
                for g in genes {
                    if g.trait_name == "leg_count" {
                        if let GeneValue::Integer(v) = g.value { leg_count = v; }
                    }
                    if g.trait_name == "leg_inset" {
                        if let GeneValue::Scalar(v) = g.value { leg_inset = v; }
                    }
                }
                for _ in 0..leg_count {
                    primitives.push(PrimitiveInstance {
                        primitive_type: FunctionalPrimitive::Support,
                        role_name: "legs".into(),
                        position: [0.0, 0.0, 0.0],
                        rotation: [0.0, 0.0, 0.0],
                        scale: [0.05, height, 0.05],
                        properties: {
                            let mut p = HashMap::new();
                            p.insert("inset".into(), format!("{:.2}", leg_inset));
                            p
                        },
                    });
                }
            }
            "chair" | "silla" => {
                primitives.push(PrimitiveInstance {
                    primitive_type: FunctionalPrimitive::Seat,
                    role_name: "seat_surface".into(),
                    position: [0.0, height * 0.5, 0.0],
                    rotation: [0.0, 0.0, 0.0],
                    scale: [width, 0.05, depth],
                    properties: HashMap::new(),
                });
                primitives.push(PrimitiveInstance {
                    primitive_type: FunctionalPrimitive::Rest,
                    role_name: "backrest".into(),
                    position: [0.0, height * 0.75, -depth * 0.5],
                    rotation: [0.0, 0.0, 0.0],
                    scale: [width, height * 0.5, 0.05],
                    properties: HashMap::new(),
                });
                let mut leg_count = 4;
                for g in genes {
                    if g.trait_name == "leg_count" {
                        if let GeneValue::Integer(v) = g.value { leg_count = v; }
                    }
                }
                for _ in 0..leg_count {
                    primitives.push(PrimitiveInstance {
                        primitive_type: FunctionalPrimitive::Support,
                        role_name: "legs".into(),
                        position: [0.0, 0.0, 0.0],
                        rotation: [0.0, 0.0, 0.0],
                        scale: [0.04, height * 0.5, 0.04],
                        properties: HashMap::new(),
                    });
                }
            }
            _ => {
                // Diseño genérico: una caja
                primitives.push(PrimitiveInstance {
                    primitive_type: FunctionalPrimitive::Container,
                    role_name: "main_body".into(),
                    position: [0.0, height * 0.5, 0.0],
                    rotation: [0.0, 0.0, 0.0],
                    scale: [width, height, depth],
                    properties: HashMap::new(),
                });
            }
        }

        UniversalDesign {
            object_type: object_type.to_string(),
            primitives,
            bounding_box: BoundingBox { width, height, depth },
        }
    }

    /// Genera un score de fitness basado en SOFIA para el motor evolutivo
    pub fn sofia_fitness_score(
        &self,
        object_type: &str,
        genes: &[crate::design_evolution::mutation_engine::Gene],
    ) -> f32 {
        let design = self.genome_to_universal_design(object_type, genes);
        let report = self.validate_design(object_type, &design);
        report.score
    }

    fn validate_relation(&self, relation: &FunctionalRelation, design: &UniversalDesign) -> bool {
        let from_exists = design.primitives.iter().any(|p| p.role_name == relation.from_primitive);
        let to_exists = design.primitives.iter().any(|p| p.role_name == relation.to_primitive);

        if !from_exists || !to_exists {
            return false;
        }

        match relation.relation_type {
            RelationType::MustBeAbove => {
                let from_y: f32 = design.primitives.iter()
                    .filter(|p| p.role_name == relation.from_primitive)
                    .map(|p| p.position[1]).sum::<f32>();
                let to_y: f32 = design.primitives.iter()
                    .filter(|p| p.role_name == relation.to_primitive)
                    .map(|p| p.position[1]).sum::<f32>();
                from_y > to_y
            }
            _ => true, // Simplificado por ahora
        }
    }

    fn validate_scale(&self, design: &UniversalDesign, reference: &ScaleReference) -> bool {
        let (min, max) = reference.size_range;
        design.bounding_box.width >= min[0] * 0.5 && design.bounding_box.width <= max[0] * 2.0 &&
        design.bounding_box.height >= min[1] * 0.5 && design.bounding_box.height <= max[1] * 2.0 &&
        design.bounding_box.depth >= min[2] * 0.5 && design.bounding_box.depth <= max[2] * 2.0
    }

    pub fn get_available_types(&self) -> Vec<String> {
        self.library.all_template_names().iter().map(|s| s.to_string()).collect()
    }
}

// ============ TIPOS DE DATOS ============

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalDesign {
    pub object_type: String,
    pub primitives: Vec<PrimitiveInstance>,
    pub bounding_box: BoundingBox,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimitiveInstance {
    pub primitive_type: FunctionalPrimitive,
    pub role_name: String,
    pub position: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
    pub properties: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBox {
    pub width: f32,
    pub height: f32,
    pub depth: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UniversalValidationReport {
    pub object_type: String,
    pub is_valid: bool,
    pub score: f32,
    pub violations: Vec<String>,
    pub warnings: Vec<String>,
    pub suggestions: Vec<String>,
}
