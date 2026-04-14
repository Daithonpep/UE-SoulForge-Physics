// sofia/primitives.rs
// Primitivas funcionales universales — los "átomos" del diseño
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Primitivas funcionales universales
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum FunctionalPrimitive {
    // SOPORTE Y ESTRUCTURA
    Support,
    Anchor,
    Span,
    
    // SUPERFICIES
    Platform,
    Wall,
    Ceiling,
    
    // CONTENCIÓN
    Container,
    Enclosure,
    
    // ACCESO Y MOVIMIENTO
    Opening,
    Passage,
    Hinge,
    Slider,
    
    // INTERACCIÓN HUMANA
    Grip,
    Seat,
    Rest,
    
    // UTILIDAD
    Illuminator,
    Shield,
    Display,
    
    // MOVILIDAD
    Wheel,
    Track,
    Propeller,
    Wing,
    
    // ENERGÍA Y FLUIDOS
    Conduit,
    Reservoir,
    Exchanger,
}

/// Relación funcional entre primitivas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionalRelation {
    pub relation_type: RelationType,
    pub from_primitive: String,
    pub to_primitive: String,
    pub constraints: Vec<RelationConstraint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationType {
    MustConnectTo,
    MustSupportBy,
    MustEnclose,
    MustBeAbove,
    MustBeBelow,
    MustAlign,
    CanContain,
    ShouldFace,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RelationConstraint {
    Distance { min: f32, max: f32 },
    Angle { required: f32, tolerance: f32 },
    Symmetry { axis: String },
    Quantity { min: u32, max: u32 },
    Proportion { ratio: f32, tolerance: f32 },
}

/// Template de objeto basado en primitivas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ObjectTemplate {
    pub name: String,
    pub category: ObjectCategory,
    pub required_primitives: Vec<PrimitiveRequirement>,
    pub relations: Vec<FunctionalRelation>,
    pub scale_reference: ScaleReference,
    pub function_description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrimitiveRequirement {
    pub primitive: FunctionalPrimitive,
    pub role_name: String,
    pub quantity: QuantitySpec,
    pub properties: HashMap<String, PropertyValue>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum QuantitySpec {
    Exact(u32),
    Range { min: u32, max: u32 },
    Variable { default: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyValue {
    Dimension { value: f32, unit: String },
    Area { value: f32 },
    Angle { degrees: f32 },
    Weight { kg: f32 },
    Boolean(bool),
    Text(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ObjectCategory {
    Furniture,
    Vehicle,
    Architecture,
    Container,
    Tool,
    Electronics,
    Nature,
    Infrastructure,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScaleReference {
    pub reference_entity: String,
    pub typical_size: [f32; 3],
    pub size_range: ([f32; 3], [f32; 3]),
}
