use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Nodo en el árbol de conocimiento
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConceptNode {
    pub id: String,
    pub name: String,
    pub category: ConceptCategory,
    pub parent: Option<String>,
    pub children: Vec<String>,
    /// Principios fundamentales que definen este concepto
    pub essence: Essence,
    /// Referencias mínimas (20 max)
    pub seed_examples: Vec<SeedExample>,
    /// Constraints físicos
    pub constraints: ConstraintSet,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConceptCategory {
    /// Nivel 0: Conceptos raíz
    Root,
    /// Nivel 1: Superfamilias (Mobiliario, Vehículos, Edificios, etc.)
    Superfamily,
    /// Nivel 2: Familias (Asientos, Mesas, Almacenamiento, etc.)
    Family,
    /// Nivel 3: Tipos (Silla de comedor, Silla de oficina, etc.)
    Type,
    /// Nivel 4: Variantes (Minimalista, Barroco, Industrial, etc.)
    Variant,
}

/// La "esencia" de un objeto — sus principios fundamentales
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Essence {
    /// Función primaria
    pub primary_function: Function,
    /// Funciones secundarias opcionales
    pub secondary_functions: Vec<Function>,
    /// Restricciones estructurales
    pub structural_principles: Vec<StructuralPrinciple>,
    /// Restricciones ergonómicas
    pub ergonomic_constraints: Vec<ErgonomicConstraint>,
    /// Propiedades geométricas esperadas
    pub geometric_properties: GeometricProperties,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Function {
    /// Soportar peso vertical
    SupportWeight { min_kg: f64, max_kg: f64, distributed: bool },
    /// Proveer superficie horizontal
    ProvideSurface { min_area_m2: f64, height_range: (f64, f64) },
    /// Permitir sentarse
    EnableSeating { capacity: usize, back_support: bool },
    /// Contener/almacenar
    ContainItems { volume_m3: f64, access_type: AccessType },
    /// Transportar
    Transport { cargo_kg: f64, passengers: usize },
    /// Proteger/albergar
    Shelter { area_m2: f64, weather_resistant: bool },
    /// Iluminar
    Illuminate { lumens: f64, area_coverage_m2: f64 },
    /// Procesar datos
    Compute { performance_metric: String },
    /// Custom
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AccessType {
    Open,           // Estantería abierta
    Door,           // Armario con puerta
    Drawer,         // Cajones
    Sliding,        // Puertas corredizas
    HingedLid,      // Tapa con bisagra
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StructuralPrinciple {
    /// Debe tener base estable (polígono de soporte contiene CoG)
    StableBase,
    /// Debe distribuir cargas uniformemente
    LoadDistribution,
    /// Requiere estructura de soporte vertical
    VerticalSupport { min_points: usize, max_points: usize },
    /// Puede ser voladizo (cantilever)
    CantileveredAllowed,
    /// Requiere conexiones rígidas
    RigidConnections,
    /// Permite articulaciones/movimiento
    ArticulatedJoints,
    /// Debe ser apilable
    Stackable,
    /// Resistente a volcaduras
    TipResistant { min_force_newtons: f64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErgonomicConstraint {
    pub constraint_type: ErgonomicType,
    pub value_range: (f64, f64),
    pub units: String,
    pub critical: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ErgonomicType {
    SeatHeight,
    TableHeight,
    ReachDistance,
    ClearanceHeight,
    WalkwayWidth,
    GraspDiameter,
    ViewingAngle,
    Custom(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeometricProperties {
    /// Rango de dimensiones típicas (ancho, profundo, alto) en metros
    pub typical_dimensions: BoundingBoxRange,
    /// Proporciones esperadas
    pub aspect_ratios: Vec<AspectRatioHint>,
    /// Simetría típica
    pub typical_symmetry: SymmetryType,
    /// Complejidad topológica
    pub topology_complexity: TopologyComplexity,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBoxRange {
    pub width: (f64, f64),
    pub depth: (f64, f64),
    pub height: (f64, f64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AspectRatioHint {
    pub ratio: f64,
    pub tolerance: f64,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SymmetryType {
    Bilateral,
    Radial { order: usize },
    Asymmetric,
    Mixed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TopologyComplexity {
    Simple,      // Mesa básica: tabla + patas
    Moderate,    // Silla: asiento + respaldo + patas
    Complex,     // Sofá: múltiples cojines, brazos, patas
    VeryComplex, // Auto: cientos de componentes
}

/// Un ejemplo semilla del mundo real
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeedExample {
    pub name: String,
    /// Path a GLB o datos de geometría simplificados
    pub reference_geometry: GeometryReference,
    /// Tags descriptivos
    pub tags: Vec<String>,
    /// Score de complejidad (de Objaverse)
    pub complexity_score: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GeometryReference {
    GLB { path: String },
    Simplified { point_cloud: Vec<[f64; 3]> },
    Parametric { params: HashMap<String, f64> },
}

/// Conjunto de constraints
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintSet {
    pub hard_constraints: Vec<HardConstraint>,
    pub soft_constraints: Vec<SoftConstraint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HardConstraint {
    pub name: String,
    pub validator: ConstraintValidator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SoftConstraint {
    pub name: String,
    pub weight: f64,
    pub evaluator: ConstraintValidator,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConstraintValidator {
    MinDimension { axis: String, min_value: f64 },
    MaxDimension { axis: String, max_value: f64 },
    HeightInRange { min: f64, max: f64 },
    AreaInRange { min: f64, max: f64 },
    VolumeInRange { min: f64, max: f64 },
    StabilityMargin { min_margin: f64 },
    SymmetryScore { min_score: f64 },
    MaterialUsage { max_volume: f64 },
    Custom(String),
}

/// El árbol completo de taxonomía
#[derive(Clone)]
pub struct TaxonomyTree {
    pub nodes: HashMap<String, ConceptNode>,
    pub root_id: String,
}

impl TaxonomyTree {
    pub fn new() -> Self {
        let mut tree = Self {
            nodes: HashMap::new(),
            root_id: "design_objects".to_string(),
        };

        tree.build_default_taxonomy();
        tree
    }

    fn build_default_taxonomy(&mut self) {
        // ROOT
        self.add_node(ConceptNode {
            id: "design_objects".into(),
            name: "Design Objects".into(),
            category: ConceptCategory::Root,
            parent: None,
            children: vec![
                "furniture".into(),
                "vehicles".into(),
                "buildings".into(),
                "electronics".into(),
                "appliances".into(),
            ],
            essence: Essence {
                primary_function: Function::Custom("Root category".into()),
                secondary_functions: vec![],
                structural_principles: vec![],
                ergonomic_constraints: vec![],
                geometric_properties: GeometricProperties {
                    typical_dimensions: BoundingBoxRange {
                        width: (0.1, 100.0),
                        depth: (0.1, 100.0),
                        height: (0.1, 100.0),
                    },
                    aspect_ratios: vec![],
                    typical_symmetry: SymmetryType::Mixed,
                    topology_complexity: TopologyComplexity::Moderate,
                },
            },
            seed_examples: vec![],
            constraints: ConstraintSet {
                hard_constraints: vec![],
                soft_constraints: vec![],
            },
        });

        // SUPERFAMILY: Furniture
        self.add_node(ConceptNode {
            id: "furniture".into(),
            name: "Furniture".into(),
            category: ConceptCategory::Superfamily,
            parent: Some("design_objects".into()),
            children: vec![
                "seating".into(),
                "tables".into(),
                "storage".into(),
                "beds".into(),
            ],
            essence: Essence {
                primary_function: Function::Custom("Interior furnishing".into()),
                secondary_functions: vec![],
                structural_principles: vec![
                    StructuralPrinciple::StableBase,
                    StructuralPrinciple::LoadDistribution,
                ],
                ergonomic_constraints: vec![],
                geometric_properties: GeometricProperties {
                    typical_dimensions: BoundingBoxRange {
                        width: (0.3, 3.0),
                        depth: (0.3, 2.0),
                        height: (0.3, 2.5),
                    },
                    aspect_ratios: vec![],
                    typical_symmetry: SymmetryType::Bilateral,
                    topology_complexity: TopologyComplexity::Moderate,
                },
            },
            seed_examples: vec![],
            constraints: ConstraintSet {
                hard_constraints: vec![
                    HardConstraint {
                        name: "Must be stable".into(),
                        validator: ConstraintValidator::StabilityMargin { min_margin: 0.05 },
                    }
                ],
                soft_constraints: vec![],
            },
        });

        // FAMILY: Tables
        self.add_node(ConceptNode {
            id: "tables".into(),
            name: "Tables".into(),
            category: ConceptCategory::Family,
            parent: Some("furniture".into()),
            children: vec![
                "dining_table".into(),
                "coffee_table".into(),
                "desk".into(),
                "nightstand".into(),
                "console_table".into(),
            ],
            essence: Essence {
                primary_function: Function::ProvideSurface {
                    min_area_m2: 0.2,
                    height_range: (0.4, 1.2),
                },
                secondary_functions: vec![
                    Function::SupportWeight {
                        min_kg: 5.0,
                        max_kg: 100.0,
                        distributed: true,
                    }
                ],
                structural_principles: vec![
                    StructuralPrinciple::StableBase,
                    StructuralPrinciple::VerticalSupport { min_points: 1, max_points: 8 },
                    StructuralPrinciple::TipResistant { min_force_newtons: 50.0 },
                ],
                ergonomic_constraints: vec![
                    ErgonomicConstraint {
                        constraint_type: ErgonomicType::TableHeight,
                        value_range: (0.4, 1.2),
                        units: "meters".into(),
                        critical: true,
                    }
                ],
                geometric_properties: GeometricProperties {
                    typical_dimensions: BoundingBoxRange {
                        width: (0.4, 3.0),
                        depth: (0.4, 1.5),
                        height: (0.4, 1.2),
                    },
                    aspect_ratios: vec![
                        AspectRatioHint {
                            ratio: 1.618, // Golden ratio
                            tolerance: 0.3,
                            description: "Width to depth".into(),
                        }
                    ],
                    typical_symmetry: SymmetryType::Bilateral,
                    topology_complexity: TopologyComplexity::Simple,
                },
            },
            seed_examples: vec![],
            constraints: ConstraintSet {
                hard_constraints: vec![
                    HardConstraint {
                        name: "Minimum usable surface".into(),
                        validator: ConstraintValidator::AreaInRange { min: 0.2, max: 10.0 },
                    },
                    HardConstraint {
                        name: "Height ergonomics".into(),
                        validator: ConstraintValidator::HeightInRange { min: 0.4, max: 1.2 },
                    }
                ],
                soft_constraints: vec![
                    SoftConstraint {
                        name: "Golden ratio proportions".into(),
                        weight: 0.3,
                        evaluator: ConstraintValidator::Custom("aspect_ratio_phi".into()),
                    }
                ],
            },
        });

        // TYPE: Dining Table
        self.add_node(ConceptNode {
            id: "dining_table".into(),
            name: "Dining Table".into(),
            category: ConceptCategory::Type,
            parent: Some("tables".into()),
            children: vec![],
            essence: Essence {
                primary_function: Function::ProvideSurface {
                    min_area_m2: 0.8,
                    height_range: (0.72, 0.78),
                },
                secondary_functions: vec![
                    Function::EnableSeating {
                        capacity: 4,
                        back_support: false, // Las sillas dan el soporte
                    }
                ],
                structural_principles: vec![
                    StructuralPrinciple::StableBase,
                    StructuralPrinciple::VerticalSupport { min_points: 1, max_points: 6 },
                    StructuralPrinciple::LoadDistribution,
                ],
                ergonomic_constraints: vec![
                    ErgonomicConstraint {
                        constraint_type: ErgonomicType::TableHeight,
                        value_range: (0.72, 0.78),
                        units: "meters".into(),
                        critical: true,
                    },
                    ErgonomicConstraint {
                        constraint_type: ErgonomicType::ClearanceHeight,
                        value_range: (0.65, 0.70),
                        units: "meters".into(),
                        critical: true,
                    }
                ],
                geometric_properties: GeometricProperties {
                    typical_dimensions: BoundingBoxRange {
                        width: (0.8, 2.4),
                        depth: (0.8, 1.2),
                        height: (0.72, 0.78),
                    },
                    aspect_ratios: vec![
                        AspectRatioHint {
                            ratio: 1.5,
                            tolerance: 0.5,
                            description: "Rectangular dining surface".into(),
                        }
                    ],
                    typical_symmetry: SymmetryType::Bilateral,
                    topology_complexity: TopologyComplexity::Simple,
                },
            },
            seed_examples: vec![
                SeedExample {
                    name: "Modern rectangular dining table".into(),
                    reference_geometry: GeometryReference::GLB {
                        path: "seeds/tables/dining_modern_rect.glb".into(),
                    },
                    tags: vec!["modern".into(), "rectangular".into(), "4-seater".into()],
                    complexity_score: 0.65,
                },
                SeedExample {
                    name: "Round pedestal dining table".into(),
                    reference_geometry: GeometryReference::GLB {
                        path: "seeds/tables/dining_round_pedestal.glb".into(),
                    },
                    tags: vec!["classic".into(), "round".into(), "single_base".into()],
                    complexity_score: 0.55,
                },
                // ... hasta 20 ejemplos variados
            ],
            constraints: ConstraintSet {
                hard_constraints: vec![
                    HardConstraint {
                        name: "Dining height standard".into(),
                        validator: ConstraintValidator::HeightInRange { min: 0.72, max: 0.78 },
                    },
                    HardConstraint {
                        name: "Sufficient surface for 4".into(),
                        validator: ConstraintValidator::AreaInRange { min: 0.8, max: 6.0 },
                    }
                ],
                soft_constraints: vec![],
            },
        });

        // FAMILY: Seating
        self.add_node(ConceptNode {
            id: "seating".into(),
            name: "Seating".into(),
            category: ConceptCategory::Family,
            parent: Some("furniture".into()),
            children: vec![
                "chair".into(),
                "stool".into(),
                "sofa".into(),
                "bench".into(),
            ],
            essence: Essence {
                primary_function: Function::EnableSeating {
                    capacity: 1,
                    back_support: true,
                },
                secondary_functions: vec![],
                structural_principles: vec![
                    StructuralPrinciple::StableBase,
                    StructuralPrinciple::TipResistant { min_force_newtons: 100.0 },
                ],
                ergonomic_constraints: vec![
                    ErgonomicConstraint {
                        constraint_type: ErgonomicType::SeatHeight,
                        value_range: (0.42, 0.50),
                        units: "meters".into(),
                        critical: true,
                    }
                ],
                geometric_properties: GeometricProperties {
                    typical_dimensions: BoundingBoxRange {
                        width: (0.4, 0.6),
                        depth: (0.4, 0.6),
                        height: (0.7, 1.2),
                    },
                    aspect_ratios: vec![],
                    typical_symmetry: SymmetryType::Bilateral,
                    topology_complexity: TopologyComplexity::Moderate,
                },
            },
            seed_examples: vec![],
            constraints: ConstraintSet {
                hard_constraints: vec![
                    HardConstraint {
                        name: "Seat height ergonomics".into(),
                        validator: ConstraintValidator::HeightInRange { min: 0.42, max: 0.50 },
                    }
                ],
                soft_constraints: vec![],
            },
        });

        // SUPERFAMILY: Vehicles
        self.add_node(ConceptNode {
            id: "vehicles".into(),
            name: "Vehicles".into(),
            category: ConceptCategory::Superfamily,
            parent: Some("design_objects".into()),
            children: vec![
                "ground_vehicles".into(),
                "watercraft".into(),
                "aircraft".into(),
            ],
            essence: Essence {
                primary_function: Function::Transport {
                    cargo_kg: 100.0,
                    passengers: 1,
                },
                secondary_functions: vec![],
                structural_principles: vec![
                    StructuralPrinciple::RigidConnections,
                    StructuralPrinciple::LoadDistribution,
                ],
                ergonomic_constraints: vec![],
                geometric_properties: GeometricProperties {
                    typical_dimensions: BoundingBoxRange {
                        width: (1.0, 3.0),
                        depth: (2.0, 6.0),
                        height: (1.0, 2.5),
                    },
                    aspect_ratios: vec![],
                    typical_symmetry: SymmetryType::Bilateral,
                    topology_complexity: TopologyComplexity::VeryComplex,
                },
            },
            seed_examples: vec![],
            constraints: ConstraintSet {
                hard_constraints: vec![],
                soft_constraints: vec![],
            },
        });
    }

    pub fn add_node(&mut self, node: ConceptNode) {
        self.nodes.insert(node.id.clone(), node);
    }

    pub fn get_node(&self, id: &str) -> Option<&ConceptNode> {
        self.nodes.get(id)
    }

    pub fn get_children(&self, id: &str) -> Vec<&ConceptNode> {
        if let Some(node) = self.get_node(id) {
            node.children.iter()
                .filter_map(|child_id| self.get_node(child_id))
                .collect()
        } else {
            vec![]
        }
    }

    pub fn get_ancestors(&self, id: &str) -> Vec<&ConceptNode> {
        let mut ancestors = vec![];
        let mut current_id = id;

        while let Some(node) = self.get_node(current_id) {
            if let Some(parent_id) = &node.parent {
                if let Some(parent) = self.get_node(parent_id) {
                    ancestors.push(parent);
                    current_id = parent_id;
                } else {
                    break;
                }
            } else {
                break;
            }
        }

        ancestors
    }
}
