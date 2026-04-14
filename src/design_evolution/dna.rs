// design_evolution/dna.rs
// Gramáticas de Diseño — El ADN fundamental de cada categoría de objeto
//
// Define las reglas inmutables (función, partes obligatorias, física)
// y los parámetros mutables (estética, proporciones, detalles).

use serde::{Deserialize, Serialize};

/// ADN de diseño: define las reglas fundamentales de una categoría
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignDNA {
    pub category: DesignCategory,
    pub core_constraints: CoreConstraints,
    pub functional_requirements: FunctionalRequirements,
    pub aesthetic_parameters: AestheticParameters,
    pub mutation_rules: MutationRules,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DesignCategory {
    Furniture(FurnitureType),
    Architecture(ArchitectureType),
    Vehicle(VehicleType),
    Nature(NatureType),
    Abstract(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FurnitureType {
    Table,
    Chair,
    Shelf,
    Bed,
    Lamp,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ArchitectureType {
    Wall,
    Column,
    Arch,
    Window,
    Door,
    Building,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum VehicleType {
    Car,
    Motorcycle,
    Boat,
    Aircraft,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum NatureType {
    Tree,
    Rock,
    Plant,
    Terrain,
}

// ============================================================
// RESTRICCIONES Y REQUISITOS
// ============================================================

/// Restricciones fundamentales (lo que NO puede cambiar)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoreConstraints {
    pub primary_function: String,
    pub required_parts: Vec<PartDefinition>,
    pub physics_rules: Vec<PhysicsRule>,
    /// Escala aproximada (min, max en metros) [x, y, z]
    pub size_bounds: ([f32; 3], [f32; 3]),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PartDefinition {
    pub name: String,
    /// "support", "surface", "decoration", "connection", "protection"
    pub role: String,
    pub quantity_range: (u32, u32),
    pub required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PhysicsRule {
    MustSupportWeight { min_kg: f32 },
    CenterOfGravityStable,
    NoFloatingParts,
    MinimumContactArea { min_area_m2: f32 },
    SymmetryRequired { axis: Axis },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum Axis {
    X,
    Y,
    Z,
}

/// Requisitos funcionales (lo que DEBE hacer)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionalRequirements {
    pub use_cases: Vec<UseCase>,
    pub ergonomics: Option<ErgonomicConstraints>,
    pub capacity: Option<Capacity>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UseCase {
    pub description: String,
    pub priority: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErgonomicConstraints {
    pub human_height_range: (f32, f32),
    pub reach_distance: f32,
    pub comfort_angle_range: Option<(f32, f32)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Capacity {
    pub max_weight_kg: f32,
    pub max_items: Option<u32>,
}

// ============================================================
// ESTÉTICA Y MUTACIÓN
// ============================================================

/// Parámetros estéticos (lo que PUEDE variar creativamente)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AestheticParameters {
    pub style_influences: Vec<StyleInfluence>,
    pub material_preferences: Vec<MaterialType>,
    /// (min, max) 0.0 = minimalista, 1.0 = ornamentado
    pub decoration_density: (f32, f32),
    /// 0.0 = asimétrico, 1.0 = perfectamente simétrico
    pub symmetry_preference: f32,
    pub complexity_range: (f32, f32),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StyleInfluence {
    pub name: String,
    pub weight: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MaterialType {
    Wood,
    Metal,
    Stone,
    Glass,
    Fabric,
    Plastic,
    Composite,
}

/// Reglas de mutación (cómo puede evolucionar)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MutationRules {
    pub allowed_mutations: Vec<MutationType>,
    pub mutation_rate: f32,
    pub max_deviation_from_parent: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MutationType {
    ScaleVariation { axis: Option<Axis>, factor_range: (f32, f32) },
    PartCountChange { part_name: String, delta_range: (i32, i32) },
    ShapeSubstitution { from: PrimitiveShape, to: Vec<PrimitiveShape> },
    ProportionShift { aspect_ratio_delta: f32 },
    DetailAddition { detail_type: DetailType, probability: f32 },
    MaterialBlend,
    SymmetryBreak { controlled: bool },
    FractalSubdivision { depth_range: (u32, u32) },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum PrimitiveShape {
    Cube,
    Cylinder,
    Sphere,
    Cone,
    Torus,
    Prism { sides: u32 },
    Custom { name: String },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum DetailType {
    Carving,
    Extrusion,
    Perforation,
    Molding,
    Inlay,
}

// ============================================================
// GRAMÁTICAS PREDEFINIDAS (ADN Base)
// ============================================================

impl DesignDNA {
    /// ADN para una mesa (base universal)
    pub fn table_base() -> Self {
        Self {
            category: DesignCategory::Furniture(FurnitureType::Table),
            core_constraints: CoreConstraints {
                primary_function: "Sostener objetos sobre una superficie horizontal elevada".into(),
                required_parts: vec![
                    PartDefinition {
                        name: "TableTop".into(),
                        role: "surface".into(),
                        quantity_range: (1, 1),
                        required: true,
                    },
                    PartDefinition {
                        name: "Leg".into(),
                        role: "support".into(),
                        quantity_range: (1, 8),
                        required: true,
                    },
                ],
                physics_rules: vec![
                    PhysicsRule::MustSupportWeight { min_kg: 10.0 },
                    PhysicsRule::CenterOfGravityStable,
                    PhysicsRule::NoFloatingParts,
                ],
                size_bounds: ([0.3, 0.5, 0.3], [3.0, 1.2, 2.0]),
            },
            functional_requirements: FunctionalRequirements {
                use_cases: vec![
                    UseCase { description: "Comer".into(), priority: 0.8 },
                    UseCase { description: "Trabajar".into(), priority: 0.6 },
                    UseCase { description: "Decorar".into(), priority: 0.3 },
                ],
                ergonomics: Some(ErgonomicConstraints {
                    human_height_range: (1.5, 1.9),
                    reach_distance: 0.8,
                    comfort_angle_range: None,
                }),
                capacity: Some(Capacity { max_weight_kg: 100.0, max_items: None }),
            },
            aesthetic_parameters: AestheticParameters {
                style_influences: vec![StyleInfluence { name: "Moderno".into(), weight: 0.5 }],
                material_preferences: vec![MaterialType::Wood, MaterialType::Metal],
                decoration_density: (0.0, 0.7),
                symmetry_preference: 0.8,
                complexity_range: (0.2, 0.8),
            },
            mutation_rules: MutationRules {
                allowed_mutations: vec![
                    MutationType::ScaleVariation { axis: Some(Axis::X), factor_range: (0.5, 2.0) },
                    MutationType::PartCountChange { part_name: "Leg".into(), delta_range: (-2, 4) },
                    MutationType::ShapeSubstitution {
                        from: PrimitiveShape::Cube,
                        to: vec![PrimitiveShape::Cylinder, PrimitiveShape::Prism { sides: 6 }],
                    },
                    MutationType::DetailAddition { detail_type: DetailType::Carving, probability: 0.3 },
                ],
                mutation_rate: 0.15,
                max_deviation_from_parent: 0.4,
            },
        }
    }

    /// ADN para una silla
    pub fn chair_base() -> Self {
        Self {
            category: DesignCategory::Furniture(FurnitureType::Chair),
            core_constraints: CoreConstraints {
                primary_function: "Sentar una persona de forma ergonómica".into(),
                required_parts: vec![
                    PartDefinition { name: "Seat".into(), role: "surface".into(), quantity_range: (1, 1), required: true },
                    PartDefinition { name: "Leg".into(), role: "support".into(), quantity_range: (1, 5), required: true },
                    PartDefinition { name: "Backrest".into(), role: "support".into(), quantity_range: (0, 1), required: false },
                    PartDefinition { name: "Armrest".into(), role: "support".into(), quantity_range: (0, 2), required: false },
                ],
                physics_rules: vec![
                    PhysicsRule::MustSupportWeight { min_kg: 120.0 },
                    PhysicsRule::CenterOfGravityStable,
                    PhysicsRule::NoFloatingParts,
                ],
                size_bounds: ([0.4, 0.4, 0.4], [0.8, 1.2, 0.7]),
            },
            functional_requirements: FunctionalRequirements {
                use_cases: vec![
                    UseCase { description: "Sentarse".into(), priority: 1.0 },
                    UseCase { description: "Descansar".into(), priority: 0.5 },
                ],
                ergonomics: Some(ErgonomicConstraints {
                    human_height_range: (1.5, 1.9),
                    reach_distance: 0.5,
                    comfort_angle_range: Some((95.0, 115.0)),
                }),
                capacity: Some(Capacity { max_weight_kg: 150.0, max_items: Some(1) }),
            },
            aesthetic_parameters: AestheticParameters {
                style_influences: vec![StyleInfluence { name: "Ergonómico".into(), weight: 0.6 }],
                material_preferences: vec![MaterialType::Wood, MaterialType::Metal, MaterialType::Fabric],
                decoration_density: (0.0, 0.5),
                symmetry_preference: 0.9,
                complexity_range: (0.2, 0.7),
            },
            mutation_rules: MutationRules {
                allowed_mutations: vec![
                    MutationType::ScaleVariation { axis: Some(Axis::Y), factor_range: (0.8, 1.3) },
                    MutationType::PartCountChange { part_name: "Leg".into(), delta_range: (-1, 1) },
                    MutationType::ShapeSubstitution {
                        from: PrimitiveShape::Cube,
                        to: vec![PrimitiveShape::Cylinder, PrimitiveShape::Sphere],
                    },
                    MutationType::DetailAddition { detail_type: DetailType::Molding, probability: 0.2 },
                    MutationType::ProportionShift { aspect_ratio_delta: 0.2 },
                ],
                mutation_rate: 0.12,
                max_deviation_from_parent: 0.35,
            },
        }
    }

    /// ADN para un vehículo (auto base)
    pub fn car_base() -> Self {
        Self {
            category: DesignCategory::Vehicle(VehicleType::Car),
            core_constraints: CoreConstraints {
                primary_function: "Transportar personas/carga sobre ruedas".into(),
                required_parts: vec![
                    PartDefinition { name: "Wheel".into(), role: "support".into(), quantity_range: (3, 6), required: true },
                    PartDefinition { name: "Chassis".into(), role: "surface".into(), quantity_range: (1, 1), required: true },
                    PartDefinition { name: "Cabin".into(), role: "protection".into(), quantity_range: (1, 1), required: true },
                ],
                physics_rules: vec![
                    PhysicsRule::CenterOfGravityStable,
                    PhysicsRule::SymmetryRequired { axis: Axis::X },
                ],
                size_bounds: ([2.0, 1.0, 1.5], [6.0, 3.0, 2.5]),
            },
            functional_requirements: FunctionalRequirements {
                use_cases: vec![
                    UseCase { description: "Transporte urbano".into(), priority: 1.0 },
                    UseCase { description: "Carreras".into(), priority: 0.3 },
                ],
                ergonomics: Some(ErgonomicConstraints {
                    human_height_range: (1.5, 1.9),
                    reach_distance: 0.6,
                    comfort_angle_range: Some((90.0, 120.0)),
                }),
                capacity: Some(Capacity { max_weight_kg: 500.0, max_items: Some(5) }),
            },
            aesthetic_parameters: AestheticParameters {
                style_influences: vec![
                    StyleInfluence { name: "Deportivo".into(), weight: 0.4 },
                    StyleInfluence { name: "Aerodinámico".into(), weight: 0.7 },
                ],
                material_preferences: vec![MaterialType::Metal, MaterialType::Glass],
                decoration_density: (0.1, 0.5),
                symmetry_preference: 0.95,
                complexity_range: (0.4, 0.9),
            },
            mutation_rules: MutationRules {
                allowed_mutations: vec![
                    MutationType::ProportionShift { aspect_ratio_delta: 0.3 },
                    MutationType::ScaleVariation { axis: Some(Axis::Y), factor_range: (0.7, 1.5) },
                    MutationType::DetailAddition { detail_type: DetailType::Extrusion, probability: 0.5 },
                ],
                mutation_rate: 0.2,
                max_deviation_from_parent: 0.5,
            },
        }
    }

    /// ADN para arquitectura (muro base)
    pub fn wall_base() -> Self {
        Self {
            category: DesignCategory::Architecture(ArchitectureType::Wall),
            core_constraints: CoreConstraints {
                primary_function: "Separar espacios y sostener estructura".into(),
                required_parts: vec![
                    PartDefinition { name: "VerticalSurface".into(), role: "surface".into(), quantity_range: (1, 1), required: true },
                ],
                physics_rules: vec![
                    PhysicsRule::MustSupportWeight { min_kg: 1000.0 },
                    PhysicsRule::NoFloatingParts,
                ],
                size_bounds: ([1.0, 2.0, 0.1], [50.0, 10.0, 1.0]),
            },
            functional_requirements: FunctionalRequirements {
                use_cases: vec![
                    UseCase { description: "División de espacios".into(), priority: 1.0 },
                    UseCase { description: "Soporte estructural".into(), priority: 0.9 },
                    UseCase { description: "Decoración".into(), priority: 0.4 },
                ],
                ergonomics: None,
                capacity: None,
            },
            aesthetic_parameters: AestheticParameters {
                style_influences: vec![
                    StyleInfluence { name: "Gótico".into(), weight: 0.3 },
                    StyleInfluence { name: "Moderno".into(), weight: 0.3 },
                    StyleInfluence { name: "Rústico".into(), weight: 0.3 },
                ],
                material_preferences: vec![MaterialType::Stone, MaterialType::Wood, MaterialType::Composite],
                decoration_density: (0.0, 1.0),
                symmetry_preference: 0.5,
                complexity_range: (0.1, 1.0),
            },
            mutation_rules: MutationRules {
                allowed_mutations: vec![
                    MutationType::DetailAddition { detail_type: DetailType::Carving, probability: 0.4 },
                    MutationType::DetailAddition { detail_type: DetailType::Perforation, probability: 0.3 },
                    MutationType::FractalSubdivision { depth_range: (1, 3) },
                    MutationType::ScaleVariation { axis: None, factor_range: (0.8, 1.5) },
                ],
                mutation_rate: 0.25,
                max_deviation_from_parent: 0.6,
            },
        }
    }

    /// ADN para edificios completos
    pub fn building_base() -> Self {
        Self {
            category: DesignCategory::Architecture(ArchitectureType::Building),
            core_constraints: CoreConstraints {
                primary_function: "Albergar personas y actividades en múltiples niveles".into(),
                required_parts: vec![
                    PartDefinition { name: "Foundation".into(), role: "support".into(), quantity_range: (1, 1), required: true },
                    PartDefinition { name: "Wall".into(), role: "surface".into(), quantity_range: (4, 20), required: true },
                    PartDefinition { name: "Floor".into(), role: "surface".into(), quantity_range: (1, 10), required: true },
                    PartDefinition { name: "Roof".into(), role: "protection".into(), quantity_range: (1, 1), required: true },
                    PartDefinition { name: "Window".into(), role: "decoration".into(), quantity_range: (0, 50), required: false },
                    PartDefinition { name: "Door".into(), role: "connection".into(), quantity_range: (1, 10), required: true },
                ],
                physics_rules: vec![
                    PhysicsRule::MustSupportWeight { min_kg: 10000.0 },
                    PhysicsRule::CenterOfGravityStable,
                    PhysicsRule::NoFloatingParts,
                ],
                size_bounds: ([5.0, 3.0, 5.0], [100.0, 50.0, 100.0]),
            },
            functional_requirements: FunctionalRequirements {
                use_cases: vec![
                    UseCase { description: "Habitar".into(), priority: 1.0 },
                    UseCase { description: "Trabajar".into(), priority: 0.7 },
                    UseCase { description: "Almacenar".into(), priority: 0.4 },
                ],
                ergonomics: Some(ErgonomicConstraints {
                    human_height_range: (1.5, 1.9),
                    reach_distance: 3.0,
                    comfort_angle_range: None,
                }),
                capacity: Some(Capacity { max_weight_kg: 50000.0, max_items: None }),
            },
            aesthetic_parameters: AestheticParameters {
                style_influences: vec![
                    StyleInfluence { name: "Moderno".into(), weight: 0.4 },
                    StyleInfluence { name: "Funcional".into(), weight: 0.4 },
                ],
                material_preferences: vec![MaterialType::Stone, MaterialType::Glass, MaterialType::Metal],
                decoration_density: (0.1, 0.8),
                symmetry_preference: 0.7,
                complexity_range: (0.3, 0.9),
            },
            mutation_rules: MutationRules {
                allowed_mutations: vec![
                    MutationType::PartCountChange { part_name: "Floor".into(), delta_range: (-1, 3) },
                    MutationType::PartCountChange { part_name: "Window".into(), delta_range: (-5, 10) },
                    MutationType::ScaleVariation { axis: Some(Axis::Y), factor_range: (0.7, 2.0) },
                    MutationType::DetailAddition { detail_type: DetailType::Extrusion, probability: 0.4 },
                    MutationType::DetailAddition { detail_type: DetailType::Perforation, probability: 0.3 },
                    MutationType::ProportionShift { aspect_ratio_delta: 0.3 },
                ],
                mutation_rate: 0.2,
                max_deviation_from_parent: 0.5,
            },
        }
    }

    /// ADN para árboles
    pub fn tree_base() -> Self {
        Self {
            category: DesignCategory::Nature(NatureType::Tree),
            core_constraints: CoreConstraints {
                primary_function: "Estructura orgánica vertical con ramificaciones".into(),
                required_parts: vec![
                    PartDefinition { name: "Trunk".into(), role: "support".into(), quantity_range: (1, 1), required: true },
                    PartDefinition { name: "Branch".into(), role: "connection".into(), quantity_range: (3, 30), required: true },
                    PartDefinition { name: "Canopy".into(), role: "surface".into(), quantity_range: (1, 5), required: false },
                ],
                physics_rules: vec![
                    PhysicsRule::CenterOfGravityStable,
                    PhysicsRule::NoFloatingParts,
                ],
                size_bounds: ([0.5, 1.0, 0.5], [5.0, 20.0, 5.0]),
            },
            functional_requirements: FunctionalRequirements {
                use_cases: vec![
                    UseCase { description: "Sombra".into(), priority: 0.7 },
                    UseCase { description: "Decoración ambiental".into(), priority: 1.0 },
                ],
                ergonomics: None,
                capacity: None,
            },
            aesthetic_parameters: AestheticParameters {
                style_influences: vec![StyleInfluence { name: "Orgánico".into(), weight: 1.0 }],
                material_preferences: vec![MaterialType::Wood],
                decoration_density: (0.3, 1.0),
                symmetry_preference: 0.3,
                complexity_range: (0.4, 1.0),
            },
            mutation_rules: MutationRules {
                allowed_mutations: vec![
                    MutationType::PartCountChange { part_name: "Branch".into(), delta_range: (-3, 5) },
                    MutationType::ScaleVariation { axis: Some(Axis::Y), factor_range: (0.5, 2.0) },
                    MutationType::FractalSubdivision { depth_range: (1, 4) },
                    MutationType::SymmetryBreak { controlled: true },
                    MutationType::ProportionShift { aspect_ratio_delta: 0.4 },
                ],
                mutation_rate: 0.3,
                max_deviation_from_parent: 0.6,
            },
        }
    }

    /// Obtener ADN base por nombre de categoría (para el Gym)
    pub fn from_category_name(name: &str) -> Option<Self> {
        match name.to_lowercase().as_str() {
            "table" | "mesa" => Some(Self::table_base()),
            "chair" | "silla" => Some(Self::chair_base()),
            "car" | "auto" | "carro" => Some(Self::car_base()),
            "wall" | "muro" | "pared" => Some(Self::wall_base()),
            "building" | "edificio" | "casa" => Some(Self::building_base()),
            "tree" | "arbol" | "árbol" => Some(Self::tree_base()),
            _ => None,
        }
    }

    /// Lista todas las categorías disponibles
    pub fn available_categories() -> Vec<&'static str> {
        vec!["table", "chair", "car", "wall", "building", "tree"]
    }
}
