// sofia/template_library.rs
// Biblioteca Universal de Templates — Auto-Expandible
use super::primitives::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
pub struct UniversalTemplateLibrary {
    templates: HashMap<String, ObjectTemplate>,
    categories: HashMap<ObjectCategory, Vec<String>>,
}

impl UniversalTemplateLibrary {
    pub fn new() -> Self {
        let mut library = Self {
            templates: HashMap::new(),
            categories: HashMap::new(),
        };
        library.load_base_templates();
        library
    }

    fn load_base_templates(&mut self) {
        // MUEBLES
        self.add_template(Self::create_table_template());
        self.add_template(Self::create_chair_template());
        self.add_template(Self::create_bed_template());
        self.add_template(Self::create_sofa_template());
        // VEHÍCULOS
        self.add_template(Self::create_car_template());
        self.add_template(Self::create_airplane_template());
        self.add_template(Self::create_boat_template());
        // ARQUITECTURA
        self.add_template(Self::create_building_template());
        self.add_template(Self::create_bridge_template());
        self.add_template(Self::create_door_template());
        // CONTENEDORES
        self.add_template(Self::create_cup_template());
        // INFRAESTRUCTURA
        self.add_template(Self::create_space_station_template());
        self.add_template(Self::create_tower_template());
    }

    pub fn add_template(&mut self, template: ObjectTemplate) {
        let category = template.category.clone();
        let name = template.name.clone();
        self.templates.insert(name.clone(), template);
        self.categories.entry(category).or_insert_with(Vec::new).push(name);
    }

    // ============ MUEBLES ============

    fn create_table_template() -> ObjectTemplate {
        let mut top_props = HashMap::new();
        top_props.insert("orientation".into(), PropertyValue::Text("horizontal".into()));
        top_props.insert("flatness_tolerance".into(), PropertyValue::Angle { degrees: 2.0 });
        top_props.insert("min_area".into(), PropertyValue::Area { value: 0.24 });
        top_props.insert("height_from_ground".into(), PropertyValue::Dimension { value: 0.75, unit: "meters".into() });
        // ERGONOMÍA CLAVE: espacio libre debajo para las piernas
        top_props.insert("min_knee_clearance".into(), PropertyValue::Dimension { value: 0.60, unit: "meters".into() });
        top_props.insert("min_leg_spread_clearance".into(), PropertyValue::Dimension { value: 0.50, unit: "meters".into() });

        let mut leg_props = HashMap::new();
        leg_props.insert("load_capacity".into(), PropertyValue::Weight { kg: 50.0 });
        leg_props.insert("must_touch_ground".into(), PropertyValue::Boolean(true));
        // Las patas NO deben invadir el espacio de las rodillas
        leg_props.insert("max_inset_from_edge".into(), PropertyValue::Dimension { value: 0.10, unit: "meters".into() });

        ObjectTemplate {
            name: "table".into(),
            category: ObjectCategory::Furniture,
            function_description: "Sostener objetos a altura cómoda. Debe dejar espacio libre debajo para las piernas de personas sentadas.".into(),
            required_primitives: vec![
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Platform,
                    role_name: "top_surface".into(),
                    quantity: QuantitySpec::Exact(1),
                    properties: top_props,
                },
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Support,
                    role_name: "legs".into(),
                    quantity: QuantitySpec::Range { min: 1, max: 6 },
                    properties: leg_props,
                },
            ],
            relations: vec![
                FunctionalRelation {
                    relation_type: RelationType::MustSupportBy,
                    from_primitive: "top_surface".into(),
                    to_primitive: "legs".into(),
                    constraints: vec![RelationConstraint::Distance { min: 0.0, max: 0.05 }],
                },
                FunctionalRelation {
                    relation_type: RelationType::MustBeAbove,
                    from_primitive: "top_surface".into(),
                    to_primitive: "legs".into(),
                    constraints: vec![],
                },
            ],
            scale_reference: ScaleReference {
                reference_entity: "adult_human_seated".into(),
                typical_size: [1.2, 0.75, 0.8],
                size_range: ([0.4, 0.5, 0.4], [3.0, 1.2, 2.0]),
            },
        }
    }

    fn create_chair_template() -> ObjectTemplate {
        let mut seat_props = HashMap::new();
        seat_props.insert("height".into(), PropertyValue::Dimension { value: 0.45, unit: "meters".into() });
        seat_props.insert("min_area".into(), PropertyValue::Area { value: 0.16 }); // 40x40cm mínimo
        seat_props.insert("max_height".into(), PropertyValue::Dimension { value: 0.50, unit: "meters".into() });

        let mut back_props = HashMap::new();
        back_props.insert("angle".into(), PropertyValue::Angle { degrees: 105.0 });
        back_props.insert("min_height".into(), PropertyValue::Dimension { value: 0.30, unit: "meters".into() });

        ObjectTemplate {
            name: "chair".into(),
            category: ObjectCategory::Furniture,
            function_description: "Soportar persona sentada con respaldo. Asiento a 45cm, respaldo a 100-110°.".into(),
            required_primitives: vec![
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Seat,
                    role_name: "seat_surface".into(),
                    quantity: QuantitySpec::Exact(1),
                    properties: seat_props,
                },
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Rest,
                    role_name: "backrest".into(),
                    quantity: QuantitySpec::Exact(1),
                    properties: back_props,
                },
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Support,
                    role_name: "legs".into(),
                    quantity: QuantitySpec::Range { min: 3, max: 5 },
                    properties: HashMap::new(),
                },
            ],
            relations: vec![
                FunctionalRelation {
                    relation_type: RelationType::MustSupportBy,
                    from_primitive: "seat_surface".into(),
                    to_primitive: "legs".into(),
                    constraints: vec![],
                },
                FunctionalRelation {
                    relation_type: RelationType::MustConnectTo,
                    from_primitive: "backrest".into(),
                    to_primitive: "seat_surface".into(),
                    constraints: vec![RelationConstraint::Angle { required: 105.0, tolerance: 15.0 }],
                },
            ],
            scale_reference: ScaleReference {
                reference_entity: "adult_human_seated".into(),
                typical_size: [0.5, 0.9, 0.5],
                size_range: ([0.4, 0.7, 0.4], [0.7, 1.3, 0.7]),
            },
        }
    }

    fn create_bed_template() -> ObjectTemplate {
        let mut mattress_props = HashMap::new();
        mattress_props.insert("min_length".into(), PropertyValue::Dimension { value: 1.9, unit: "meters".into() });
        mattress_props.insert("min_width".into(), PropertyValue::Dimension { value: 0.9, unit: "meters".into() });
        mattress_props.insert("height_from_ground".into(), PropertyValue::Dimension { value: 0.45, unit: "meters".into() });

        ObjectTemplate {
            name: "bed".into(),
            category: ObjectCategory::Furniture,
            function_description: "Superficie horizontal para dormir. Min 190x90cm, altura 45cm.".into(),
            required_primitives: vec![
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Platform,
                    role_name: "mattress_support".into(),
                    quantity: QuantitySpec::Exact(1),
                    properties: mattress_props,
                },
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Support,
                    role_name: "frame_legs".into(),
                    quantity: QuantitySpec::Range { min: 4, max: 6 },
                    properties: HashMap::new(),
                },
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Rest,
                    role_name: "headboard".into(),
                    quantity: QuantitySpec::Range { min: 0, max: 1 },
                    properties: HashMap::new(),
                },
            ],
            relations: vec![
                FunctionalRelation {
                    relation_type: RelationType::MustSupportBy,
                    from_primitive: "mattress_support".into(),
                    to_primitive: "frame_legs".into(),
                    constraints: vec![],
                },
            ],
            scale_reference: ScaleReference {
                reference_entity: "adult_human_lying".into(),
                typical_size: [1.4, 0.5, 2.0],
                size_range: ([0.9, 0.3, 1.9], [2.0, 0.8, 2.2]),
            },
        }
    }

    fn create_sofa_template() -> ObjectTemplate {
        let mut seat_props = HashMap::new();
        seat_props.insert("height".into(), PropertyValue::Dimension { value: 0.42, unit: "meters".into() });
        seat_props.insert("depth".into(), PropertyValue::Dimension { value: 0.55, unit: "meters".into() });
        seat_props.insert("min_seats".into(), PropertyValue::Dimension { value: 2.0, unit: "persons".into() });

        ObjectTemplate {
            name: "sofa".into(),
            category: ObjectCategory::Furniture,
            function_description: "Asiento blando para 2+ personas con respaldo y reposabrazos.".into(),
            required_primitives: vec![
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Seat,
                    role_name: "seat_cushion".into(),
                    quantity: QuantitySpec::Range { min: 1, max: 4 },
                    properties: seat_props,
                },
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Rest,
                    role_name: "backrest".into(),
                    quantity: QuantitySpec::Exact(1),
                    properties: HashMap::new(),
                },
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Rest,
                    role_name: "armrests".into(),
                    quantity: QuantitySpec::Range { min: 0, max: 2 },
                    properties: HashMap::new(),
                },
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Support,
                    role_name: "base".into(),
                    quantity: QuantitySpec::Range { min: 4, max: 6 },
                    properties: HashMap::new(),
                },
            ],
            relations: vec![],
            scale_reference: ScaleReference {
                reference_entity: "adult_human_seated".into(),
                typical_size: [2.0, 0.85, 0.9],
                size_range: ([1.2, 0.7, 0.7], [3.5, 1.0, 1.1]),
            },
        }
    }

    // ============ VEHÍCULOS ============

    fn create_car_template() -> ObjectTemplate {
        let mut wheel_props = HashMap::new();
        wheel_props.insert("diameter".into(), PropertyValue::Dimension { value: 0.6, unit: "meters".into() });
        wheel_props.insert("can_rotate".into(), PropertyValue::Boolean(true));

        let mut cabin_props = HashMap::new();
        cabin_props.insert("weatherproof".into(), PropertyValue::Boolean(true));
        cabin_props.insert("min_headroom".into(), PropertyValue::Dimension { value: 0.9, unit: "meters".into() });

        ObjectTemplate {
            name: "car".into(),
            category: ObjectCategory::Vehicle,
            function_description: "Transportar personas sobre ruedas en carreteras.".into(),
            required_primitives: vec![
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Wheel,
                    role_name: "wheels".into(),
                    quantity: QuantitySpec::Range { min: 3, max: 6 },
                    properties: wheel_props,
                },
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Enclosure,
                    role_name: "cabin".into(),
                    quantity: QuantitySpec::Exact(1),
                    properties: cabin_props,
                },
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Seat,
                    role_name: "seats".into(),
                    quantity: QuantitySpec::Range { min: 2, max: 8 },
                    properties: HashMap::new(),
                },
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Shield,
                    role_name: "windshield".into(),
                    quantity: QuantitySpec::Exact(1),
                    properties: HashMap::new(),
                },
            ],
            relations: vec![
                FunctionalRelation {
                    relation_type: RelationType::MustAlign,
                    from_primitive: "wheels".into(),
                    to_primitive: "wheels".into(),
                    constraints: vec![RelationConstraint::Symmetry { axis: "longitudinal".into() }],
                },
                FunctionalRelation {
                    relation_type: RelationType::MustEnclose,
                    from_primitive: "cabin".into(),
                    to_primitive: "seats".into(),
                    constraints: vec![],
                },
            ],
            scale_reference: ScaleReference {
                reference_entity: "adult_human_seated".into(),
                typical_size: [1.8, 1.5, 4.5],
                size_range: ([1.5, 1.2, 3.0], [2.5, 2.0, 6.0]),
            },
        }
    }

    fn create_airplane_template() -> ObjectTemplate {
        ObjectTemplate {
            name: "airplane".into(),
            category: ObjectCategory::Vehicle,
            function_description: "Vehículo aéreo con alas y fuselaje para transporte.".into(),
            required_primitives: vec![
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Enclosure,
                    role_name: "fuselage".into(),
                    quantity: QuantitySpec::Exact(1),
                    properties: HashMap::new(),
                },
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Wing,
                    role_name: "wings".into(),
                    quantity: QuantitySpec::Exact(2),
                    properties: HashMap::new(),
                },
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Propeller,
                    role_name: "engines".into(),
                    quantity: QuantitySpec::Range { min: 1, max: 4 },
                    properties: HashMap::new(),
                },
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Wing,
                    role_name: "tail_fin".into(),
                    quantity: QuantitySpec::Range { min: 1, max: 3 },
                    properties: HashMap::new(),
                },
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Wheel,
                    role_name: "landing_gear".into(),
                    quantity: QuantitySpec::Range { min: 3, max: 6 },
                    properties: HashMap::new(),
                },
            ],
            relations: vec![
                FunctionalRelation {
                    relation_type: RelationType::MustAlign,
                    from_primitive: "wings".into(),
                    to_primitive: "fuselage".into(),
                    constraints: vec![RelationConstraint::Symmetry { axis: "longitudinal".into() }],
                },
            ],
            scale_reference: ScaleReference {
                reference_entity: "commercial_jet".into(),
                typical_size: [35.0, 12.0, 40.0],
                size_range: ([5.0, 2.0, 6.0], [80.0, 20.0, 80.0]),
            },
        }
    }

    fn create_boat_template() -> ObjectTemplate {
        ObjectTemplate {
            name: "boat".into(),
            category: ObjectCategory::Vehicle,
            function_description: "Vehículo acuático con casco flotante.".into(),
            required_primitives: vec![
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Container,
                    role_name: "hull".into(),
                    quantity: QuantitySpec::Exact(1),
                    properties: {
                        let mut p = HashMap::new();
                        p.insert("watertight".into(), PropertyValue::Boolean(true));
                        p
                    },
                },
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Platform,
                    role_name: "deck".into(),
                    quantity: QuantitySpec::Range { min: 1, max: 5 },
                    properties: HashMap::new(),
                },
            ],
            relations: vec![
                FunctionalRelation {
                    relation_type: RelationType::MustBeAbove,
                    from_primitive: "deck".into(),
                    to_primitive: "hull".into(),
                    constraints: vec![],
                },
            ],
            scale_reference: ScaleReference {
                reference_entity: "fishing_boat".into(),
                typical_size: [3.0, 2.0, 8.0],
                size_range: ([1.0, 0.5, 2.0], [40.0, 15.0, 100.0]),
            },
        }
    }

    // ============ ARQUITECTURA ============

    fn create_building_template() -> ObjectTemplate {
        ObjectTemplate {
            name: "building".into(),
            category: ObjectCategory::Architecture,
            function_description: "Estructura habitable con múltiples espacios protegidos.".into(),
            required_primitives: vec![
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Wall,
                    role_name: "exterior_walls".into(),
                    quantity: QuantitySpec::Range { min: 4, max: 100 },
                    properties: HashMap::new(),
                },
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Platform,
                    role_name: "floors".into(),
                    quantity: QuantitySpec::Range { min: 1, max: 200 },
                    properties: HashMap::new(),
                },
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Ceiling,
                    role_name: "roof".into(),
                    quantity: QuantitySpec::Exact(1),
                    properties: HashMap::new(),
                },
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Opening,
                    role_name: "doors".into(),
                    quantity: QuantitySpec::Range { min: 1, max: 500 },
                    properties: HashMap::new(),
                },
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Opening,
                    role_name: "windows".into(),
                    quantity: QuantitySpec::Range { min: 0, max: 1000 },
                    properties: HashMap::new(),
                },
            ],
            relations: vec![
                FunctionalRelation {
                    relation_type: RelationType::MustBeAbove,
                    from_primitive: "roof".into(),
                    to_primitive: "floors".into(),
                    constraints: vec![],
                },
            ],
            scale_reference: ScaleReference {
                reference_entity: "building_floor".into(),
                typical_size: [20.0, 15.0, 30.0],
                size_range: ([5.0, 3.0, 5.0], [200.0, 300.0, 200.0]),
            },
        }
    }

    fn create_bridge_template() -> ObjectTemplate {
        ObjectTemplate {
            name: "bridge".into(),
            category: ObjectCategory::Architecture,
            function_description: "Estructura que cruza un obstáculo conectando dos puntos.".into(),
            required_primitives: vec![
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Span,
                    role_name: "deck_span".into(),
                    quantity: QuantitySpec::Exact(1),
                    properties: HashMap::new(),
                },
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Support,
                    role_name: "pillars".into(),
                    quantity: QuantitySpec::Range { min: 2, max: 50 },
                    properties: HashMap::new(),
                },
            ],
            relations: vec![
                FunctionalRelation {
                    relation_type: RelationType::MustSupportBy,
                    from_primitive: "deck_span".into(),
                    to_primitive: "pillars".into(),
                    constraints: vec![],
                },
            ],
            scale_reference: ScaleReference {
                reference_entity: "vehicle_bridge".into(),
                typical_size: [10.0, 5.0, 100.0],
                size_range: ([2.0, 1.0, 5.0], [50.0, 100.0, 5000.0]),
            },
        }
    }

    fn create_door_template() -> ObjectTemplate {
        let mut panel_props = HashMap::new();
        panel_props.insert("min_height".into(), PropertyValue::Dimension { value: 2.0, unit: "meters".into() });
        panel_props.insert("min_width".into(), PropertyValue::Dimension { value: 0.8, unit: "meters".into() });

        ObjectTemplate {
            name: "door".into(),
            category: ObjectCategory::Architecture,
            function_description: "Apertura controlable en una pared para permitir paso humano.".into(),
            required_primitives: vec![
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Opening,
                    role_name: "panel".into(),
                    quantity: QuantitySpec::Exact(1),
                    properties: panel_props,
                },
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Hinge,
                    role_name: "hinges".into(),
                    quantity: QuantitySpec::Range { min: 2, max: 3 },
                    properties: HashMap::new(),
                },
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Grip,
                    role_name: "handle".into(),
                    quantity: QuantitySpec::Range { min: 1, max: 2 },
                    properties: HashMap::new(),
                },
            ],
            relations: vec![
                FunctionalRelation {
                    relation_type: RelationType::MustConnectTo,
                    from_primitive: "hinges".into(),
                    to_primitive: "panel".into(),
                    constraints: vec![],
                },
            ],
            scale_reference: ScaleReference {
                reference_entity: "adult_human_standing".into(),
                typical_size: [0.9, 2.1, 0.05],
                size_range: ([0.6, 1.8, 0.03], [2.0, 3.0, 0.1]),
            },
        }
    }

    // ============ CONTENEDORES ============

    fn create_cup_template() -> ObjectTemplate {
        let mut body_props = HashMap::new();
        body_props.insert("watertight".into(), PropertyValue::Boolean(true));
        body_props.insert("open_top".into(), PropertyValue::Boolean(true));
        body_props.insert("capacity_ml".into(), PropertyValue::Dimension { value: 250.0, unit: "milliliters".into() });

        ObjectTemplate {
            name: "cup".into(),
            category: ObjectCategory::Container,
            function_description: "Contener líquidos para beber con una mano.".into(),
            required_primitives: vec![
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Container,
                    role_name: "main_body".into(),
                    quantity: QuantitySpec::Exact(1),
                    properties: body_props,
                },
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Grip,
                    role_name: "handle".into(),
                    quantity: QuantitySpec::Range { min: 0, max: 1 },
                    properties: HashMap::new(),
                },
            ],
            relations: vec![],
            scale_reference: ScaleReference {
                reference_entity: "human_hand".into(),
                typical_size: [0.08, 0.10, 0.08],
                size_range: ([0.05, 0.06, 0.05], [0.15, 0.20, 0.15]),
            },
        }
    }

    // ============ INFRAESTRUCTURA ============

    fn create_space_station_template() -> ObjectTemplate {
        ObjectTemplate {
            name: "space_station".into(),
            category: ObjectCategory::Infrastructure,
            function_description: "Estructura habitable orbital con soporte vital.".into(),
            required_primitives: vec![
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Enclosure,
                    role_name: "pressurized_modules".into(),
                    quantity: QuantitySpec::Range { min: 1, max: 50 },
                    properties: HashMap::new(),
                },
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Passage,
                    role_name: "connecting_tunnels".into(),
                    quantity: QuantitySpec::Range { min: 0, max: 20 },
                    properties: HashMap::new(),
                },
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Wing,
                    role_name: "solar_panels".into(),
                    quantity: QuantitySpec::Range { min: 2, max: 100 },
                    properties: HashMap::new(),
                },
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Opening,
                    role_name: "airlocks".into(),
                    quantity: QuantitySpec::Range { min: 1, max: 10 },
                    properties: HashMap::new(),
                },
            ],
            relations: vec![
                FunctionalRelation {
                    relation_type: RelationType::MustConnectTo,
                    from_primitive: "pressurized_modules".into(),
                    to_primitive: "connecting_tunnels".into(),
                    constraints: vec![],
                },
            ],
            scale_reference: ScaleReference {
                reference_entity: "ISS".into(),
                typical_size: [100.0, 50.0, 100.0],
                size_range: ([10.0, 5.0, 10.0], [500.0, 200.0, 500.0]),
            },
        }
    }

    fn create_tower_template() -> ObjectTemplate {
        ObjectTemplate {
            name: "tower".into(),
            category: ObjectCategory::Infrastructure,
            function_description: "Estructura vertical alta para observación o telecomunicaciones.".into(),
            required_primitives: vec![
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Support,
                    role_name: "base_structure".into(),
                    quantity: QuantitySpec::Exact(1),
                    properties: HashMap::new(),
                },
                PrimitiveRequirement {
                    primitive: FunctionalPrimitive::Platform,
                    role_name: "observation_deck".into(),
                    quantity: QuantitySpec::Range { min: 0, max: 5 },
                    properties: HashMap::new(),
                },
            ],
            relations: vec![],
            scale_reference: ScaleReference {
                reference_entity: "communications_tower".into(),
                typical_size: [5.0, 50.0, 5.0],
                size_range: ([2.0, 10.0, 2.0], [30.0, 600.0, 30.0]),
            },
        }
    }

    // ============ CONSULTAS ============

    pub fn get_template(&self, name: &str) -> Option<&ObjectTemplate> {
        self.templates.get(name)
    }

    pub fn get_category_templates(&self, category: &ObjectCategory) -> Vec<&ObjectTemplate> {
        self.categories.get(category)
            .map(|names| names.iter().filter_map(|n| self.templates.get(n)).collect())
            .unwrap_or_default()
    }

    pub fn search_by_function(&self, keywords: &[&str]) -> Vec<&ObjectTemplate> {
        self.templates.values()
            .filter(|t| keywords.iter().any(|k| t.function_description.to_lowercase().contains(&k.to_lowercase())))
            .collect()
    }

    pub fn all_template_names(&self) -> Vec<&str> {
        self.templates.keys().map(|s| s.as_str()).collect()
    }

    pub fn add_custom_template(&mut self, template: ObjectTemplate) {
        self.add_template(template);
    }

    pub fn export_to_json(&self, path: &str) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(&self.templates)?;
        std::fs::write(path, json)?;
        log::info!("📚 SOFIA: Biblioteca exportada a: {}", path);
        Ok(())
    }

    pub fn import_from_json(&mut self, path: &str) -> std::io::Result<()> {
        let json = std::fs::read_to_string(path)?;
        let templates: HashMap<String, ObjectTemplate> = serde_json::from_str(&json)?;
        for (_, template) in templates {
            self.add_template(template);
        }
        log::info!("📂 SOFIA: Templates importados desde: {}", path);
        Ok(())
    }
}
