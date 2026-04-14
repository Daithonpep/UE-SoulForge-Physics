// phoenix/reality_profiles.rs
// REALITY Engine — Motor de realidad configurable multi-universo
//
// 7 perfiles de física predefinidos:
//   EARTH_REALITY  — Física real, materiales reales, regulaciones IBC
//   MOON_BASE      — Gravedad 1/6, vacío, titanio + regolito
//   MARS_COLONY    — Gravedad 38%, atmósfera tenue CO2
//   SPACE_STATION  — Microgravedad, presurización interna
//   UNDERWATER     — Presión 500atm, agua salada, corrosión
//   FANTASY_WORLD  — Mithril, gravedad reducida, sin restricciones
//   SCIFI_FUTURE   — Grafeno, smart materials, anti-gravedad

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================
// REALITY ENGINE
// ============================================================

pub struct REALITYEngine {
    active_profile: PhysicsProfile,
    available_profiles: HashMap<String, PhysicsProfile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsProfile {
    pub name: String,
    pub description: String,
    pub physics_laws: PhysicsLaws,
    pub material_library: MaterialLibrary,
    pub evaluation_mode: EvaluationMode,
    pub constraints: DesignConstraints,
}

// ============================================================
// LEYES FÍSICAS
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsLaws {
    pub gravity: [f32; 3],          // m/s²
    pub air_density: f32,            // kg/m³
    pub air_viscosity: f32,          // Pa·s
    pub sound_speed: f32,            // m/s
    pub ambient_temperature: f32,    // °C
    pub atmospheric_pressure: f32,   // Pa
    pub drag_coefficient_multiplier: f32,
}

// ============================================================
// MATERIALES
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialLibrary {
    pub materials: HashMap<String, MaterialProperties>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialProperties {
    pub name: String,
    pub density: f32,                // kg/m³
    pub youngs_modulus: f32,          // GPa
    pub yield_strength: f32,         // MPa
    pub tensile_strength: f32,       // MPa
    pub poisson_ratio: f32,
    pub fatigue_limit: u64,          // ciclos
    pub cost_per_kg: f32,            // USD
    pub damping_coefficient: f32,
    pub friction_coefficient: f32,
    pub thermal_conductivity: f32,   // W/(m·K)
    pub is_isotropic: bool,          // true=igual en todas direcciones
}

// ============================================================
// MODO DE EVALUACIÓN
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvaluationMode {
    HyperRealistic {
        include_fatigue: bool,
        include_corrosion: bool,
        include_manufacturing_tolerances: bool,
    },
    Realistic {
        simplified_materials: bool,
    },
    SemiRealistic {
        gravity_scaling: f32,
        material_strength_bonus: f32,
    },
    Fantasy {
        ignore_structural_limits: bool,
        aesthetic_weight: f32,
    },
    SciFi {
        exotic_materials: bool,
        anti_gravity_available: bool,
    },
}

// ============================================================
// RESTRICCIONES DE DISEÑO
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignConstraints {
    pub max_budget: Option<f32>,
    pub max_construction_time: Option<u32>,
    pub max_weight: Option<f32>,
    pub required_lifespan: Option<u32>,
    pub required_certifications: Vec<String>,
    pub regulations: Vec<Regulation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Regulation {
    pub name: String,
    pub code: String,
    pub requirements: Vec<String>,
}

// ============================================================
// RESULTADO DE VALIDACIÓN
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstraintValidation {
    pub passed: bool,
    pub violations: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DesignStatistics {
    pub estimated_cost: f32,
    pub total_weight: f32,
    pub estimated_lifespan_years: u32,
    pub construction_time_days: u32,
}

// ============================================================
// IMPLEMENTACIÓN
// ============================================================

impl REALITYEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            active_profile: Self::earth_profile(),
            available_profiles: HashMap::new(),
        };
        engine.initialize_profiles();
        engine
    }

    fn initialize_profiles(&mut self) {
        let profiles = vec![
            Self::earth_profile(),
            Self::moon_profile(),
            Self::mars_profile(),
            Self::space_profile(),
            Self::underwater_profile(),
            Self::fantasy_profile(),
            Self::scifi_profile(),
        ];
        for p in profiles {
            self.available_profiles.insert(p.name.clone(), p);
        }
    }

    pub fn switch_profile(&mut self, name: &str) -> Result<(), String> {
        if let Some(p) = self.available_profiles.get(name) {
            self.active_profile = p.clone();
            log::info!("🌍 REALITY: Perfil → {} (g={:?})", p.name, p.physics_laws.gravity);
            Ok(())
        } else {
            Err(format!("Perfil '{}' no encontrado. Disponibles: {:?}", name, self.list_profiles()))
        }
    }

    pub fn get_active_profile(&self) -> &PhysicsProfile {
        &self.active_profile
    }

    pub fn list_profiles(&self) -> Vec<String> {
        self.available_profiles.keys().cloned().collect()
    }

    pub fn get_material(&self, name: &str) -> Option<&MaterialProperties> {
        self.active_profile.material_library.materials.get(name)
    }

    pub fn list_materials(&self) -> Vec<String> {
        self.active_profile.material_library.materials.keys().cloned().collect()
    }

    /// Valida restricciones del perfil activo contra estadísticas de un diseño
    pub fn validate_constraints(&self, stats: &DesignStatistics) -> ConstraintValidation {
        let c = &self.active_profile.constraints;
        let mut violations = Vec::new();
        let mut warnings = Vec::new();

        if let Some(max) = c.max_budget {
            if stats.estimated_cost > max {
                violations.push(format!("❌ Presupuesto: ${:.0} > ${:.0}", stats.estimated_cost, max));
            } else if stats.estimated_cost > max * 0.9 {
                warnings.push(format!("⚠️ Presupuesto al {:.0}%", (stats.estimated_cost / max) * 100.0));
            }
        }
        if let Some(max) = c.max_weight {
            if stats.total_weight > max {
                violations.push(format!("❌ Peso: {:.1}kg > {:.1}kg", stats.total_weight, max));
            }
        }
        if let Some(req) = c.required_lifespan {
            if stats.estimated_lifespan_years < req {
                violations.push(format!("❌ Vida útil: {} < {} años", stats.estimated_lifespan_years, req));
            }
        }
        if let Some(max) = c.max_construction_time {
            if stats.construction_time_days > max {
                violations.push(format!("❌ Construcción: {} > {} días", stats.construction_time_days, max));
            }
        }

        ConstraintValidation { passed: violations.is_empty(), violations, warnings }
    }

    /// Exporta el perfil activo como JSON
    pub fn export_profile_json(&self) -> serde_json::Value {
        let p = &self.active_profile;
        serde_json::json!({
            "name": p.name,
            "description": p.description,
            "gravity": p.physics_laws.gravity,
            "air_density": p.physics_laws.air_density,
            "temperature": p.physics_laws.ambient_temperature,
            "pressure_pa": p.physics_laws.atmospheric_pressure,
            "materials": p.material_library.materials.keys().collect::<Vec<_>>(),
            "evaluation_mode": format!("{:?}", p.evaluation_mode),
            "certifications": p.constraints.required_certifications,
        })
    }

    // ================================================================
    // PERFILES PREDEFINIDOS
    // ================================================================

    fn earth_profile() -> PhysicsProfile {
        let mut mats = HashMap::new();

        mats.insert("structural_steel".into(), MaterialProperties {
            name: "Structural Steel (A36)".into(),
            density: 7850.0, youngs_modulus: 200.0, yield_strength: 250.0,
            tensile_strength: 400.0, poisson_ratio: 0.30, fatigue_limit: 1_000_000,
            cost_per_kg: 0.80, damping_coefficient: 0.02, friction_coefficient: 0.74,
            thermal_conductivity: 50.0, is_isotropic: true,
        });
        mats.insert("aluminum_6061".into(), MaterialProperties {
            name: "Aluminum 6061-T6".into(),
            density: 2700.0, youngs_modulus: 68.9, yield_strength: 276.0,
            tensile_strength: 310.0, poisson_ratio: 0.33, fatigue_limit: 500_000,
            cost_per_kg: 2.50, damping_coefficient: 0.01, friction_coefficient: 0.61,
            thermal_conductivity: 167.0, is_isotropic: true,
        });
        mats.insert("reinforced_concrete".into(), MaterialProperties {
            name: "Reinforced Concrete".into(),
            density: 2400.0, youngs_modulus: 30.0, yield_strength: 40.0,
            tensile_strength: 4.0, poisson_ratio: 0.20, fatigue_limit: 10_000_000,
            cost_per_kg: 0.12, damping_coefficient: 0.05, friction_coefficient: 0.65,
            thermal_conductivity: 1.7, is_isotropic: true,
        });
        mats.insert("pine_wood".into(), MaterialProperties {
            name: "Pine Wood".into(),
            density: 550.0, youngs_modulus: 9.0, yield_strength: 40.0,
            tensile_strength: 70.0, poisson_ratio: 0.35, fatigue_limit: 100_000,
            cost_per_kg: 0.50, damping_coefficient: 0.08, friction_coefficient: 0.50,
            thermal_conductivity: 0.12, is_isotropic: false, // Madera es anisotrópica
        });
        mats.insert("carbon_fiber".into(), MaterialProperties {
            name: "Carbon Fiber Composite".into(),
            density: 1600.0, youngs_modulus: 230.0, yield_strength: 600.0,
            tensile_strength: 3500.0, poisson_ratio: 0.30, fatigue_limit: 5_000_000,
            cost_per_kg: 25.0, damping_coefficient: 0.015, friction_coefficient: 0.40,
            thermal_conductivity: 7.0, is_isotropic: false, // Depende de dirección de fibra
        });
        mats.insert("oak_wood".into(), MaterialProperties {
            name: "Oak Wood".into(),
            density: 750.0, youngs_modulus: 12.0, yield_strength: 50.0,
            tensile_strength: 90.0, poisson_ratio: 0.35, fatigue_limit: 150_000,
            cost_per_kg: 1.20, damping_coefficient: 0.06, friction_coefficient: 0.55,
            thermal_conductivity: 0.17, is_isotropic: false,
        });
        mats.insert("glass".into(), MaterialProperties {
            name: "Tempered Glass".into(),
            density: 2500.0, youngs_modulus: 70.0, yield_strength: 120.0,
            tensile_strength: 150.0, poisson_ratio: 0.22, fatigue_limit: 50_000,
            cost_per_kg: 3.0, damping_coefficient: 0.005, friction_coefficient: 0.40,
            thermal_conductivity: 1.0, is_isotropic: true,
        });

        PhysicsProfile {
            name: "EARTH_REALITY".into(),
            description: "Física del mundo real — Tierra al nivel del mar".into(),
            physics_laws: PhysicsLaws {
                gravity: [0.0, -9.81, 0.0],
                air_density: 1.225,
                air_viscosity: 0.0000181,
                sound_speed: 343.0,
                ambient_temperature: 20.0,
                atmospheric_pressure: 101325.0,
                drag_coefficient_multiplier: 1.0,
            },
            material_library: MaterialLibrary { materials: mats },
            evaluation_mode: EvaluationMode::HyperRealistic {
                include_fatigue: true,
                include_corrosion: true,
                include_manufacturing_tolerances: true,
            },
            constraints: DesignConstraints {
                max_budget: Some(1_000_000.0),
                max_construction_time: Some(365),
                max_weight: Some(100_000.0),
                required_lifespan: Some(50),
                required_certifications: vec!["ISO 9001".into(), "Building Code".into()],
                regulations: vec![Regulation {
                    name: "International Building Code".into(),
                    code: "IBC 2021".into(),
                    requirements: vec!["Seismic resistance".into(), "Fire safety".into(), "Wind load".into()],
                }],
            },
        }
    }

    fn moon_profile() -> PhysicsProfile {
        let mut mats = HashMap::new();
        mats.insert("titanium_alloy".into(), MaterialProperties {
            name: "Titanium Ti-6Al-4V".into(),
            density: 4430.0, youngs_modulus: 113.8, yield_strength: 880.0,
            tensile_strength: 950.0, poisson_ratio: 0.342, fatigue_limit: 10_000_000,
            cost_per_kg: 35.0, damping_coefficient: 0.005, friction_coefficient: 0.36,
            thermal_conductivity: 7.0, is_isotropic: true,
        });
        mats.insert("lunar_regolith".into(), MaterialProperties {
            name: "Compacted Lunar Regolith".into(),
            density: 1500.0, youngs_modulus: 15.0, yield_strength: 5.0,
            tensile_strength: 2.0, poisson_ratio: 0.25, fatigue_limit: 1_000_000,
            cost_per_kg: 0.0, damping_coefficient: 0.10, friction_coefficient: 0.80,
            thermal_conductivity: 0.001, is_isotropic: true,
        });

        PhysicsProfile {
            name: "MOON_BASE".into(),
            description: "Luna — gravedad 1/6, vacío, ±250°C".into(),
            physics_laws: PhysicsLaws {
                gravity: [0.0, -1.62, 0.0],
                air_density: 0.0, air_viscosity: 0.0, sound_speed: 0.0,
                ambient_temperature: -20.0, atmospheric_pressure: 0.0,
                drag_coefficient_multiplier: 0.0,
            },
            material_library: MaterialLibrary { materials: mats },
            evaluation_mode: EvaluationMode::HyperRealistic {
                include_fatigue: true, include_corrosion: false, include_manufacturing_tolerances: true,
            },
            constraints: DesignConstraints {
                max_budget: Some(100_000_000.0), max_construction_time: Some(1095),
                max_weight: Some(10_000.0), required_lifespan: Some(100),
                required_certifications: vec!["NASA Safety".into()],
                regulations: vec![],
            },
        }
    }

    fn mars_profile() -> PhysicsProfile {
        let mut mats = Self::earth_profile().material_library.materials;
        mats.insert("martian_concrete".into(), MaterialProperties {
            name: "Mars Regolith Concrete".into(),
            density: 2200.0, youngs_modulus: 20.0, yield_strength: 25.0,
            tensile_strength: 3.0, poisson_ratio: 0.22, fatigue_limit: 5_000_000,
            cost_per_kg: 0.05, damping_coefficient: 0.06, friction_coefficient: 0.70,
            thermal_conductivity: 1.2, is_isotropic: true,
        });

        PhysicsProfile {
            name: "MARS_COLONY".into(),
            description: "Marte — gravedad 38%, CO2 tenue, -63°C".into(),
            physics_laws: PhysicsLaws {
                gravity: [0.0, -3.71, 0.0],
                air_density: 0.020, air_viscosity: 0.00001, sound_speed: 240.0,
                ambient_temperature: -63.0, atmospheric_pressure: 600.0,
                drag_coefficient_multiplier: 0.02,
            },
            material_library: MaterialLibrary { materials: mats },
            evaluation_mode: EvaluationMode::HyperRealistic {
                include_fatigue: true, include_corrosion: true, include_manufacturing_tolerances: true,
            },
            constraints: DesignConstraints {
                max_budget: Some(50_000_000.0), max_construction_time: Some(730),
                max_weight: Some(50_000.0), required_lifespan: Some(75),
                required_certifications: vec!["Mars Settlement Standards".into()],
                regulations: vec![],
            },
        }
    }

    fn space_profile() -> PhysicsProfile {
        let mats = Self::moon_profile().material_library.materials;
        PhysicsProfile {
            name: "SPACE_STATION".into(),
            description: "Órbita terrestre — microgravedad".into(),
            physics_laws: PhysicsLaws {
                gravity: [0.0, -0.0001, 0.0],
                air_density: 0.0, air_viscosity: 0.0, sound_speed: 0.0,
                ambient_temperature: -100.0, atmospheric_pressure: 0.0,
                drag_coefficient_multiplier: 0.0,
            },
            material_library: MaterialLibrary { materials: mats },
            evaluation_mode: EvaluationMode::HyperRealistic {
                include_fatigue: true, include_corrosion: false, include_manufacturing_tolerances: true,
            },
            constraints: DesignConstraints {
                max_budget: Some(500_000_000.0), max_construction_time: Some(3650),
                max_weight: Some(100_000.0), required_lifespan: Some(30),
                required_certifications: vec!["ISS Standards".into()],
                regulations: vec![],
            },
        }
    }

    fn underwater_profile() -> PhysicsProfile {
        let mats = Self::earth_profile().material_library.materials;
        PhysicsProfile {
            name: "UNDERWATER".into(),
            description: "Océano profundo — 500atm, corrosión salina".into(),
            physics_laws: PhysicsLaws {
                gravity: [0.0, -9.81, 0.0],
                air_density: 1025.0, // Agua de mar
                air_viscosity: 0.001,
                sound_speed: 1500.0,
                ambient_temperature: 4.0,
                atmospheric_pressure: 50_000_000.0,
                drag_coefficient_multiplier: 3.0,
            },
            material_library: MaterialLibrary { materials: mats },
            evaluation_mode: EvaluationMode::HyperRealistic {
                include_fatigue: true, include_corrosion: true, include_manufacturing_tolerances: true,
            },
            constraints: DesignConstraints {
                max_budget: Some(10_000_000.0), max_construction_time: Some(730),
                max_weight: Some(50_000.0), required_lifespan: Some(25),
                required_certifications: vec!["Deep Sea Cert".into()],
                regulations: vec![],
            },
        }
    }

    fn fantasy_profile() -> PhysicsProfile {
        let mut mats = HashMap::new();
        mats.insert("mithril".into(), MaterialProperties {
            name: "Mithril (Fantasy Alloy)".into(),
            density: 1000.0, youngs_modulus: 500.0, yield_strength: 2000.0,
            tensile_strength: 5000.0, poisson_ratio: 0.25, fatigue_limit: u64::MAX,
            cost_per_kg: 1000.0, damping_coefficient: 0.001, friction_coefficient: 0.10,
            thermal_conductivity: 200.0, is_isotropic: true,
        });
        mats.insert("enchanted_ironwood".into(), MaterialProperties {
            name: "Enchanted Ironwood".into(),
            density: 800.0, youngs_modulus: 100.0, yield_strength: 500.0,
            tensile_strength: 800.0, poisson_ratio: 0.30, fatigue_limit: 100_000_000,
            cost_per_kg: 50.0, damping_coefficient: 0.02, friction_coefficient: 0.30,
            thermal_conductivity: 5.0, is_isotropic: false,
        });

        PhysicsProfile {
            name: "FANTASY_WORLD".into(),
            description: "Mundo fantástico — gravedad baja, sin restricciones".into(),
            physics_laws: PhysicsLaws {
                gravity: [0.0, -2.0, 0.0],
                air_density: 1.0, air_viscosity: 0.00001, sound_speed: 400.0,
                ambient_temperature: 25.0, atmospheric_pressure: 101325.0,
                drag_coefficient_multiplier: 0.5,
            },
            material_library: MaterialLibrary { materials: mats },
            evaluation_mode: EvaluationMode::Fantasy {
                ignore_structural_limits: true, aesthetic_weight: 0.8,
            },
            constraints: DesignConstraints {
                max_budget: None, max_construction_time: None,
                max_weight: None, required_lifespan: None,
                required_certifications: vec![], regulations: vec![],
            },
        }
    }

    fn scifi_profile() -> PhysicsProfile {
        let mut mats = HashMap::new();
        mats.insert("graphene_composite".into(), MaterialProperties {
            name: "Graphene Nanocomposite".into(),
            density: 1200.0, youngs_modulus: 1000.0, yield_strength: 5000.0,
            tensile_strength: 130_000.0, poisson_ratio: 0.15, fatigue_limit: u64::MAX,
            cost_per_kg: 500.0, damping_coefficient: 0.001, friction_coefficient: 0.05,
            thermal_conductivity: 5000.0, is_isotropic: false,
        });
        mats.insert("smart_material".into(), MaterialProperties {
            name: "Self-Healing Smart Material".into(),
            density: 2000.0, youngs_modulus: 150.0, yield_strength: 800.0,
            tensile_strength: 1200.0, poisson_ratio: 0.28, fatigue_limit: u64::MAX,
            cost_per_kg: 10_000.0, damping_coefficient: 0.005, friction_coefficient: 0.20,
            thermal_conductivity: 100.0, is_isotropic: true,
        });

        PhysicsProfile {
            name: "SCIFI_FUTURE".into(),
            description: "Futuro lejano — grafeno, anti-gravedad, auto-reparación".into(),
            physics_laws: PhysicsLaws {
                gravity: [0.0, -9.81, 0.0],
                air_density: 1.225, air_viscosity: 0.0000181, sound_speed: 343.0,
                ambient_temperature: 22.0, atmospheric_pressure: 101325.0,
                drag_coefficient_multiplier: 0.3,
            },
            material_library: MaterialLibrary { materials: mats },
            evaluation_mode: EvaluationMode::SciFi {
                exotic_materials: true, anti_gravity_available: true,
            },
            constraints: DesignConstraints {
                max_budget: Some(10_000_000_000.0), max_construction_time: Some(3650),
                max_weight: Some(1_000_000.0), required_lifespan: Some(200),
                required_certifications: vec![], regulations: vec![],
            },
        }
    }
}
