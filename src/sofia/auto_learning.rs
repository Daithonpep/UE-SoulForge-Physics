// sofia/auto_learning.rs
// Sistema de Auto-Aprendizaje — Daithon aprende nuevos objetos observando
use super::primitives::*;
use super::template_library::*;
use std::collections::HashMap;

pub struct AutoLearningSystem {
    pub library: UniversalTemplateLibrary,
}

impl AutoLearningSystem {
    pub fn new() -> Self {
        Self { library: UniversalTemplateLibrary::new() }
    }

    /// Aprende nuevo objeto observando un ejemplo
    pub fn learn_from_example(
        &mut self,
        name: String,
        category: ObjectCategory,
        observed_primitives: Vec<ObservedPrimitive>,
        function_description: String,
    ) -> ObjectTemplate {
        log::info!("🧠 SOFIA Auto-Learning: Aprendiendo '{}'", name);

        // 1. Contar primitivas observadas
        let mut primitive_counts: HashMap<FunctionalPrimitive, u32> = HashMap::new();
        for obs in &observed_primitives {
            *primitive_counts.entry(obs.primitive.clone()).or_insert(0) += 1;
        }

        // 2. Crear requerimientos
        let required_primitives: Vec<PrimitiveRequirement> = primitive_counts
            .into_iter()
            .map(|(prim, count)| {
                PrimitiveRequirement {
                    primitive: prim.clone(),
                    role_name: format!("{:?}_auto", prim).to_lowercase(),
                    quantity: if count == 1 {
                        QuantitySpec::Exact(1)
                    } else {
                        QuantitySpec::Range { min: count.saturating_sub(1), max: count + 2 }
                    },
                    properties: HashMap::new(),
                }
            })
            .collect();

        // 3. Inferir relaciones
        let relations = self.infer_relations(&observed_primitives);

        // 4. Calcular escala
        let scale_reference = self.calculate_scale_reference(&observed_primitives);

        // 5. Crear y registrar template
        let template = ObjectTemplate {
            name: name.clone(),
            category,
            function_description,
            required_primitives,
            relations,
            scale_reference,
        };

        self.library.add_custom_template(template.clone());
        log::info!("✅ SOFIA: Template '{}' creado y registrado", name);
        template
    }

    fn infer_relations(&self, primitives: &[ObservedPrimitive]) -> Vec<FunctionalRelation> {
        let mut relations = Vec::new();

        for (i, prim_a) in primitives.iter().enumerate() {
            for prim_b in primitives.iter().skip(i + 1) {
                let distance = self.calc_distance(&prim_a.position, &prim_b.position);

                if distance < 0.1 {
                    relations.push(FunctionalRelation {
                        relation_type: RelationType::MustConnectTo,
                        from_primitive: format!("{:?}_auto", prim_a.primitive).to_lowercase(),
                        to_primitive: format!("{:?}_auto", prim_b.primitive).to_lowercase(),
                        constraints: vec![RelationConstraint::Distance { min: 0.0, max: 0.1 }],
                    });
                }

                if prim_a.position[1] > prim_b.position[1] + 0.05 {
                    relations.push(FunctionalRelation {
                        relation_type: RelationType::MustBeAbove,
                        from_primitive: format!("{:?}_auto", prim_a.primitive).to_lowercase(),
                        to_primitive: format!("{:?}_auto", prim_b.primitive).to_lowercase(),
                        constraints: vec![],
                    });
                }
            }
        }

        relations
    }

    fn calculate_scale_reference(&self, primitives: &[ObservedPrimitive]) -> ScaleReference {
        if primitives.is_empty() {
            return ScaleReference {
                reference_entity: "unknown".into(),
                typical_size: [1.0, 1.0, 1.0],
                size_range: ([0.5, 0.5, 0.5], [2.0, 2.0, 2.0]),
            };
        }

        let mut min = [f32::INFINITY; 3];
        let mut max = [f32::NEG_INFINITY; 3];

        for prim in primitives {
            for i in 0..3 {
                min[i] = min[i].min(prim.position[i]);
                max[i] = max[i].max(prim.position[i] + prim.size[i]);
            }
        }

        let size = [max[0] - min[0], max[1] - min[1], max[2] - min[2]];

        ScaleReference {
            reference_entity: "learned_from_observation".into(),
            typical_size: size,
            size_range: (
                [size[0] * 0.7, size[1] * 0.7, size[2] * 0.7],
                [size[0] * 1.3, size[1] * 1.3, size[2] * 1.3],
            ),
        }
    }

    fn calc_distance(&self, a: &[f32; 3], b: &[f32; 3]) -> f32 {
        ((a[0]-b[0]).powi(2) + (a[1]-b[1]).powi(2) + (a[2]-b[2]).powi(2)).sqrt()
    }

    pub fn save_library(&self, path: &str) -> std::io::Result<()> {
        self.library.export_to_json(path)
    }
}

#[derive(Debug, Clone)]
pub struct ObservedPrimitive {
    pub primitive: FunctionalPrimitive,
    pub position: [f32; 3],
    pub size: [f32; 3],
}
