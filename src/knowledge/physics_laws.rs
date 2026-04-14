// src/knowledge/physics_laws.rs
use std::collections::HashMap;

// ═══════════════════════════════════════════
// ESTRUCTURAS BASE
// ═══════════════════════════════════════════

#[derive(Debug, Clone)]
pub struct PhysicsKnowledgeBase {
    pub mechanics: Vec<PhysicsLaw>,
    pub thermodynamics: Vec<PhysicsLaw>,
    pub fluid_dynamics: Vec<PhysicsLaw>,
    pub material_science: Vec<PhysicsLaw>,
    pub structural: Vec<PhysicsLaw>,
    pub chemistry: Vec<PhysicsLaw>,
    pub atomic: Vec<PhysicsLaw>,
    pub aerodynamics: Vec<PhysicsLaw>,
    
    // Constantes universales que Daithon conoce
    pub constants: HashMap<String, PhysicalConstant>,
    
    // Propiedades de materiales
    pub materials: HashMap<String, MaterialProperties>,
}

#[derive(Debug, Clone)]
pub struct PhysicsLaw {
    pub name: String,
    pub domain: PhysicsDomain,
    pub formula: String,
    pub compute: fn(&HashMap<String, f64>) -> f64,
    pub variables: Vec<Variable>,
    pub constraints: Vec<Constraint>,
    pub related_laws: Vec<String>,
    pub trust: f64,  // Confianza de Daithon en esta ley
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub symbol: String,
    pub unit: String,
    pub typical_range: (f64, f64),
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct Constraint {
    pub condition: String,
    pub consequence: String,
}

#[derive(Debug, Clone)]
pub struct PhysicalConstant {
    pub name: String,
    pub symbol: String,
    pub value: f64,
    pub unit: String,
    pub description: String,
}

#[derive(Debug, Clone)]
pub struct MaterialProperties {
    pub name: String,
    pub density: f64,           // kg/m³
    pub yield_strength: f64,    // Pa (límite elástico)
    pub ultimate_strength: f64, // Pa (límite de rotura)
    pub elastic_modulus: f64,   // Pa (módulo de Young)
    pub poisson_ratio: f64,    // adimensional
    pub thermal_conductivity: f64, // W/(m·K)
    pub specific_heat: f64,    // J/(kg·K)
    pub melting_point: f64,    // K
    pub friction_coefficient: f64, // adimensional
}

#[derive(Debug, Clone, PartialEq)]
pub enum PhysicsDomain {
    Mechanics,
    Thermodynamics,
    FluidDynamics,
    MaterialScience,
    Structural,
    Chemistry,
    Atomic,
    Aerodynamics,
    Electromagnetism,
}

// ═══════════════════════════════════════════
// INICIALIZACIÓN COMPLETA
// ═══════════════════════════════════════════

impl PhysicsKnowledgeBase {
    pub fn initialize() -> Self {
        let mut kb = Self {
            mechanics: Vec::new(),
            thermodynamics: Vec::new(),
            fluid_dynamics: Vec::new(),
            material_science: Vec::new(),
            structural: Vec::new(),
            chemistry: Vec::new(),
            atomic: Vec::new(),
            aerodynamics: Vec::new(),
            constants: HashMap::new(),
            materials: HashMap::new(),
        };

        kb.load_constants();
        kb.load_materials();
        kb.load_mechanics();
        kb.load_thermodynamics();
        kb.load_fluid_dynamics();
        kb.load_aerodynamics();
        kb.load_structural();
        kb.load_material_science();
        kb.load_chemistry();
        kb.load_atomic();
        
        kb
    }

    // ═══════════════════════════════════════
    // CONSTANTES UNIVERSALES
    // ═══════════════════════════════════════
    
    fn load_constants(&mut self) {
        let constants = vec![
            ("g", PhysicalConstant {
                name: "Aceleración gravitatoria".into(),
                symbol: "g".into(),
                value: 9.80665,
                unit: "m/s²".into(),
                description: "Aceleración de la gravedad en la superficie terrestre".into(),
            }),
            ("c", PhysicalConstant {
                name: "Velocidad de la luz".into(),
                symbol: "c".into(),
                value: 299_792_458.0,
                unit: "m/s".into(),
                description: "Velocidad de la luz en el vacío".into(),
            }),
            ("R", PhysicalConstant {
                name: "Constante de los gases ideales".into(),
                symbol: "R".into(),
                value: 8.314,
                unit: "J/(mol·K)".into(),
                description: "Constante universal de los gases".into(),
            }),
            ("k_B", PhysicalConstant {
                name: "Constante de Boltzmann".into(),
                symbol: "k_B".into(),
                value: 1.380649e-23,
                unit: "J/K".into(),
                description: "Relación entre energía y temperatura a nivel molecular".into(),
            }),
            ("N_A", PhysicalConstant {
                name: "Número de Avogadro".into(),
                symbol: "N_A".into(),
                value: 6.02214076e23,
                unit: "1/mol".into(),
                description: "Número de partículas en un mol".into(),
            }),
            ("h", PhysicalConstant {
                name: "Constante de Planck".into(),
                symbol: "h".into(),
                value: 6.62607015e-34,
                unit: "J·s".into(),
                description: "Cuanto de acción. Relaciona energía con frecuencia".into(),
            }),
            ("sigma", PhysicalConstant {
                name: "Constante de Stefan-Boltzmann".into(),
                symbol: "σ".into(),
                value: 5.670374419e-8,
                unit: "W/(m²·K⁴)".into(),
                description: "Radiación de cuerpo negro".into(),
            }),
            ("rho_air", PhysicalConstant {
                name: "Densidad del aire a nivel del mar".into(),
                symbol: "ρ_air".into(),
                value: 1.225,
                unit: "kg/m³".into(),
                description: "Densidad del aire seco a 15°C, 1 atm".into(),
            }),
            ("rho_water", PhysicalConstant {
                name: "Densidad del agua".into(),
                symbol: "ρ_water".into(),
                value: 1000.0,
                unit: "kg/m³".into(),
                description: "Densidad del agua pura a 4°C".into(),
            }),
            ("atm", PhysicalConstant {
                name: "Presión atmosférica estándar".into(),
                symbol: "P_atm".into(),
                value: 101325.0,
                unit: "Pa".into(),
                description: "Presión atmosférica al nivel del mar".into(),
            }),
            ("mu_air", PhysicalConstant {
                name: "Viscosidad dinámica del aire".into(),
                symbol: "μ_air".into(),
                value: 1.81e-5,
                unit: "Pa·s".into(),
                description: "Viscosidad del aire a 20°C".into(),
            }),
            ("e", PhysicalConstant {
                name: "Carga elemental".into(),
                symbol: "e".into(),
                value: 1.602176634e-19,
                unit: "C".into(),
                description: "Carga eléctrica de un protón".into(),
            }),
            ("epsilon_0", PhysicalConstant {
                name: "Permitividad del vacío".into(),
                symbol: "ε₀".into(),
                value: 8.854187817e-12,
                unit: "F/m".into(),
                description: "Permitividad eléctrica del espacio libre".into(),
            }),
        ];
        
        for (key, constant) in constants {
            self.constants.insert(key.to_string(), constant);
        }
    }

    // ═══════════════════════════════════════
    // MATERIALES
    // ═══════════════════════════════════════
    
    fn load_materials(&mut self) {
        let materials = vec![
            ("concrete", MaterialProperties {
                name: "Concreto".into(),
                density: 2400.0,
                yield_strength: 30e6,
                ultimate_strength: 40e6,
                elastic_modulus: 30e9,
                poisson_ratio: 0.2,
                thermal_conductivity: 1.7,
                specific_heat: 880.0,
                melting_point: 1500.0,
                friction_coefficient: 0.6,
            }),
            ("steel", MaterialProperties {
                name: "Acero estructural".into(),
                density: 7850.0,
                yield_strength: 250e6,
                ultimate_strength: 400e6,
                elastic_modulus: 200e9,
                poisson_ratio: 0.3,
                thermal_conductivity: 50.0,
                specific_heat: 500.0,
                melting_point: 1800.0,
                friction_coefficient: 0.74,
            }),
            ("aluminum", MaterialProperties {
                name: "Aluminio".into(),
                density: 2700.0,
                yield_strength: 270e6,
                ultimate_strength: 310e6,
                elastic_modulus: 69e9,
                poisson_ratio: 0.33,
                thermal_conductivity: 237.0,
                specific_heat: 897.0,
                melting_point: 933.0,
                friction_coefficient: 0.61,
            }),
            ("wood_oak", MaterialProperties {
                name: "Madera de roble".into(),
                density: 750.0,
                yield_strength: 40e6,
                ultimate_strength: 50e6,
                elastic_modulus: 12e9,
                poisson_ratio: 0.35,
                thermal_conductivity: 0.17,
                specific_heat: 2380.0,
                melting_point: 573.0, // Punto de ignición
                friction_coefficient: 0.5,
            }),
            ("glass", MaterialProperties {
                name: "Vidrio".into(),
                density: 2500.0,
                yield_strength: 33e6,
                ultimate_strength: 33e6,
                elastic_modulus: 70e9,
                poisson_ratio: 0.22,
                thermal_conductivity: 1.0,
                specific_heat: 840.0,
                melting_point: 1700.0,
                friction_coefficient: 0.4,
            }),
            ("titanium", MaterialProperties {
                name: "Titanio".into(),
                density: 4507.0,
                yield_strength: 880e6,
                ultimate_strength: 950e6,
                elastic_modulus: 116e9,
                poisson_ratio: 0.34,
                thermal_conductivity: 21.9,
                specific_heat: 523.0,
                melting_point: 1941.0,
                friction_coefficient: 0.36,
            }),
            ("carbon_fiber", MaterialProperties {
                name: "Fibra de carbono".into(),
                density: 1600.0,
                yield_strength: 3500e6,
                ultimate_strength: 4000e6,
                elastic_modulus: 230e9,
                poisson_ratio: 0.27,
                thermal_conductivity: 7.0,
                specific_heat: 710.0,
                melting_point: 3800.0,
                friction_coefficient: 0.2,
            }),
            ("rubber", MaterialProperties {
                name: "Caucho".into(),
                density: 1100.0,
                yield_strength: 15e6,
                ultimate_strength: 25e6,
                elastic_modulus: 0.05e9,
                poisson_ratio: 0.49,
                thermal_conductivity: 0.13,
                specific_heat: 2010.0,
                melting_point: 453.0,
                friction_coefficient: 0.9,
            }),
            ("granite", MaterialProperties {
                name: "Granito".into(),
                density: 2750.0,
                yield_strength: 14e6,
                ultimate_strength: 200e6,
                elastic_modulus: 70e9,
                poisson_ratio: 0.25,
                thermal_conductivity: 3.0,
                specific_heat: 790.0,
                melting_point: 1473.0,
                friction_coefficient: 0.65,
            }),
            ("ice", MaterialProperties {
                name: "Hielo".into(),
                density: 917.0,
                yield_strength: 1e6,
                ultimate_strength: 3e6,
                elastic_modulus: 9.3e9,
                poisson_ratio: 0.33,
                thermal_conductivity: 2.22,
                specific_heat: 2090.0,
                melting_point: 273.15,
                friction_coefficient: 0.03,
            }),
        ];
        
        for (key, material) in materials {
            self.materials.insert(key.to_string(), material);
        }
    }

    // ═══════════════════════════════════════
    // MECÁNICA CLÁSICA
    // ═══════════════════════════════════════
    
    fn load_mechanics(&mut self) {
        self.mechanics = vec![
            PhysicsLaw {
                name: "Segunda Ley de Newton".into(),
                domain: PhysicsDomain::Mechanics,
                formula: "F = m × a".into(),
                compute: |v| v["mass"] * v["acceleration"],
                variables: vec![
                    Variable {
                        name: "Fuerza".into(),
                        symbol: "F".into(),
                        unit: "N".into(),
                        typical_range: (0.0, 1e12),
                        description: "Fuerza neta aplicada al cuerpo".into(),
                    },
                    Variable {
                        name: "Masa".into(),
                        symbol: "m".into(),
                        unit: "kg".into(),
                        typical_range: (0.001, 1e9),
                        description: "Cantidad de materia del cuerpo".into(),
                    },
                    Variable {
                        name: "Aceleración".into(),
                        symbol: "a".into(),
                        unit: "m/s²".into(),
                        typical_range: (0.0, 1000.0),
                        description: "Tasa de cambio de velocidad".into(),
                    },
                ],
                constraints: vec![
                    Constraint {
                        condition: "v << c".into(),
                        consequence: "No relativista. Para v > 0.1c usar relatividad especial".into(),
                    },
                ],
                related_laws: vec!["Tercera Ley de Newton".into(), "Peso".into()],
                trust: 1.0,
            },
            
            PhysicsLaw {
                name: "Peso".into(),
                domain: PhysicsDomain::Mechanics,
                formula: "W = m × g".into(),
                compute: |v| v["mass"] * v.get("g").copied().unwrap_or(9.81),
                variables: vec![
                    Variable {
                        name: "Peso".into(),
                        symbol: "W".into(),
                        unit: "N".into(),
                        typical_range: (0.0, 1e10),
                        description: "Fuerza gravitatoria sobre un cuerpo".into(),
                    },
                ],
                constraints: vec![
                    Constraint {
                        condition: "Superficie terrestre".into(),
                        consequence: "g varía con altitud y latitud".into(),
                    },
                ],
                related_laws: vec!["Gravitación Universal".into()],
                trust: 1.0,
            },
            
            PhysicsLaw {
                name: "Energía Cinética".into(),
                domain: PhysicsDomain::Mechanics,
                formula: "Ek = ½ × m × v²".into(),
                compute: |v| 0.5 * v["mass"] * v["velocity"].powi(2),
                variables: vec![
                    Variable {
                        name: "Energía cinética".into(),
                        symbol: "Ek".into(),
                        unit: "J".into(),
                        typical_range: (0.0, 1e15),
                        description: "Energía asociada al movimiento".into(),
                    },
                    Variable {
                        name: "Velocidad".into(),
                        symbol: "v".into(),
                        unit: "m/s".into(),
                        typical_range: (0.0, 3e8),
                        description: "Rapidez del cuerpo".into(),
                    },
                ],
                constraints: vec![],
                related_laws: vec!["Conservación de Energía".into()],
                trust: 1.0,
            },
            
            PhysicsLaw {
                name: "Energía Potencial Gravitatoria".into(),
                domain: PhysicsDomain::Mechanics,
                formula: "Ep = m × g × h".into(),
                compute: |v| v["mass"] * v.get("g").copied().unwrap_or(9.81) * v["height"],
                variables: vec![
                    Variable {
                        name: "Altura".into(),
                        symbol: "h".into(),
                        unit: "m".into(),
                        typical_range: (0.0, 1e6),
                        description: "Altura respecto al punto de referencia".into(),
                    },
                ],
                constraints: vec![
                    Constraint {
                        condition: "h << Radio terrestre".into(),
                        consequence: "Para grandes alturas usar gravitación universal".into(),
                    },
                ],
                related_laws: vec!["Energía Cinética".into()],
                trust: 1.0,
            },
            
            PhysicsLaw {
                name: "Energía Sísmica Richter".into(),
                domain: PhysicsDomain::Mechanics,
                formula: "E = 10^(1.5M + 4.8)".into(),
                compute: |v| 10.0_f64.powf(1.5 * v["magnitude"] + 4.8),
                variables: vec![
                    Variable {
                        name: "Magnitud".into(),
                        symbol: "M".into(),
                        unit: "Richter".into(),
                        typical_range: (1.0, 10.0),
                        description: "Magnitud en escala Richter. Logarítmica. \
                                     Cada unidad = 31.6x más energía".into(),
                    },
                ],
                constraints: vec![
                    Constraint {
                        condition: "M > 9.5".into(),
                        consequence: "Nunca registrado. Teórico máximo ~10".into(),
                    },
                ],
                related_laws: vec!["Segunda Ley de Newton".into()],
                trust: 1.0,
            },
            
            PhysicsLaw {
                name: "Torque".into(),
                domain: PhysicsDomain::Mechanics,
                formula: "τ = F × d × sin(θ)".into(),
                compute: |v| {
                    v["force"] * v["distance"] 
                    * v.get("angle_rad").copied().unwrap_or(std::f64::consts::FRAC_PI_2).sin()
                },
                variables: vec![
                    Variable {
                        name: "Torque".into(),
                        symbol: "τ".into(),
                        unit: "N·m".into(),
                        typical_range: (0.0, 1e8),
                        description: "Momento de fuerza. Tendencia a rotar".into(),
                    },
                    Variable {
                        name: "Distancia al pivote".into(),
                        symbol: "d".into(),
                        unit: "m".into(),
                        typical_range: (0.0, 1000.0),
                        description: "Brazo de palanca".into(),
                    },
                ],
                constraints: vec![],
                related_laws: vec!["Momento de Volcamiento".into()],
                trust: 1.0,
            },
            
            PhysicsLaw {
                name: "Fricción".into(),
                domain: PhysicsDomain::Mechanics,
                formula: "f = μ × N".into(),
                compute: |v| v["friction_coefficient"] * v["normal_force"],
                variables: vec![
                    Variable {
                        name: "Fuerza de fricción".into(),
                        symbol: "f".into(),
                        unit: "N".into(),
                        typical_range: (0.0, 1e8),
                        description: "Fuerza que se opone al deslizamiento".into(),
                    },
                    Variable {
                        name: "Coeficiente de fricción".into(),
                        symbol: "μ".into(),
                        unit: "adimensional".into(),
                        typical_range: (0.01, 1.0),
                        description: "Depende de los materiales en contacto".into(),
                    },
                    Variable {
                        name: "Fuerza normal".into(),
                        symbol: "N".into(),
                        unit: "N".into(),
                        typical_range: (0.0, 1e10),
                        description: "Fuerza perpendicular a la superficie".into(),
                    },
                ],
                constraints: vec![
                    Constraint {
                        condition: "Fricción estática vs cinética".into(),
                        consequence: "μ_estático > μ_cinético siempre".into(),
                    },
                ],
                related_laws: vec!["Peso".into()],
                trust: 1.0,
            },
            
            PhysicsLaw {
                name: "Gravitación Universal".into(),
                domain: PhysicsDomain::Mechanics,
                formula: "F = G × m1 × m2 / r²".into(),
                compute: |v| {
                    6.674e-11 * v["mass_1"] * v["mass_2"] / v["distance"].powi(2)
                },
                variables: vec![
                    Variable {
                        name: "Masa 1".into(),
                        symbol: "m₁".into(),
                        unit: "kg".into(),
                        typical_range: (0.001, 1.989e30),
                        description: "Masa del primer cuerpo".into(),
                    },
                    Variable {
                        name: "Masa 2".into(),
                        symbol: "m₂".into(),
                        unit: "kg".into(),
                        typical_range: (0.001, 1.989e30),
                        description: "Masa del segundo cuerpo".into(),
                    },
                    Variable {
                        name: "Distancia".into(),
                        symbol: "r".into(),
                        unit: "m".into(),
                        typical_range: (0.01, 1e12),
                        description: "Distancia entre centros de masa".into(),
                    },
                ],
                constraints: vec![
                    Constraint {
                        condition: "r > 0".into(),
                        consequence: "La fuerza tiende a infinito cuando r→0".into(),
                    },
                ],
                related_laws: vec!["Peso".into()],
                trust: 1.0,
            },
            
            PhysicsLaw {
                name: "Impulso".into(),
                domain: PhysicsDomain::Mechanics,
                formula: "J = F × Δt = Δp".into(),
                compute: |v| v["force"] * v["delta_time"],
                variables: vec![
                    Variable {
                        name: "Impulso".into(),
                        symbol: "J".into(),
                        unit: "N·s".into(),
                        typical_range: (0.0, 1e8),
                        description: "Cambio en la cantidad de movimiento".into(),
                    },
                    Variable {
                        name: "Intervalo de tiempo".into(),
                        symbol: "Δt".into(),
                        unit: "s".into(),
                        typical_range: (0.001, 1000.0),
                        description: "Duración de la aplicación de fuerza".into(),
                    },
                ],
                constraints: vec![],
                related_laws: vec!["Segunda Ley de Newton".into()],
                trust: 1.0,
            },
            
            PhysicsLaw {
                name: "Presión".into(),
                domain: PhysicsDomain::Mechanics,
                formula: "P = F / A".into(),
                compute: |v| v["force"] / v["area"].max(0.0001),
                variables: vec![
                    Variable {
                        name: "Presión".into(),
                        symbol: "P".into(),
                        unit: "Pa".into(),
                        typical_range: (0.0, 1e12),
                        description: "Fuerza por unidad de área".into(),
                    },
                    Variable {
                        name: "Área".into(),
                        symbol: "A".into(),
                        unit: "m²".into(),
                        typical_range: (1e-6, 1e6),
                        description: "Superficie sobre la que se aplica la fuerza".into(),
                    },
                ],
                constraints: vec![],
                related_laws: vec!["Presión Hidrostática".into()],
                trust: 1.0,
            },
        ];
    }

    // ═══════════════════════════════════════
    // TERMODINÁMICA
    // ═══════════════════════════════════════
    
    fn load_thermodynamics(&mut self) {
        self.thermodynamics = vec![
            PhysicsLaw {
                name: "Primera Ley de la Termodinámica".into(),
                domain: PhysicsDomain::Thermodynamics,
                formula: "ΔU = Q - W".into(),
                compute: |v| v["heat"] - v["work"],
                variables: vec![
                    Variable {
                        name: "Cambio de energía interna".into(),
                        symbol: "ΔU".into(),
                        unit: "J".into(),
                        typical_range: (-1e12, 1e12),
                        description: "Cambio en la energía térmica del sistema".into(),
                    },
                    Variable {
                        name: "Calor".into(),
                        symbol: "Q".into(),
                        unit: "J".into(),
                        typical_range: (-1e12, 1e12),
                        description: "Energía transferida por diferencia de temperatura. \
                                     Positivo si entra al sistema".into(),
                    },
                    Variable {
                        name: "Trabajo".into(),
                        symbol: "W".into(),
                        unit: "J".into(),
                        typical_range: (-1e12, 1e12),
                        description: "Energía transferida por fuerza mecánica. \
                                     Positivo si lo realiza el sistema".into(),
                    },
                ],
                constraints: vec![
                    Constraint {
                        condition: "Sistema cerrado".into(),
                        consequence: "No hay intercambio de masa con el entorno".into(),
                    },
                ],
                related_laws: vec!["Conservación de Energía".into()],
                trust: 1.0,
            },
            
            PhysicsLaw {
                name: "Transferencia de Calor".into(),
                domain: PhysicsDomain::Thermodynamics,
                formula: "Q = m × c × ΔT".into(),
                compute: |v| v["mass"] * v["specific_heat"] * v["delta_temp"],
                variables: vec![
                    Variable {
                        name: "Calor específico".into(),
                        symbol: "c".into(),
                        unit: "J/(kg·K)".into(),
                        typical_range: (100.0, 5000.0),
                        description: "Energía necesaria para elevar 1K un kg de material. \
                                     Agua=4186, Acero=500, Aire=1005".into(),
                    },
                    Variable {
                        name: "Cambio de temperatura".into(),
                        symbol: "ΔT".into(),
                        unit: "K".into(),
                        typical_range: (-3000.0, 3000.0),
                        description: "Diferencia de temperatura".into(),
                    },
                ],
                constraints: vec![
                    Constraint {
                        condition: "Sin cambio de fase".into(),
                        consequence: "Durante fusión o ebullición, T no cambia. Usar calor latente".into(),
                    },
                ],
                related_laws: vec!["Calor Latente".into()],
                trust: 1.0,
            },
            
            PhysicsLaw {
                name: "Calor Latente".into(),
                domain: PhysicsDomain::Thermodynamics,
                formula: "Q = m × L".into(),
                compute: |v| v["mass"] * v["latent_heat"],
                variables: vec![
                    Variable {
                        name: "Calor latente".into(),
                        symbol: "L".into(),
                        unit: "J/kg".into(),
                        typical_range: (1e4, 1e7),
                        description: "Energía para cambio de fase sin cambio de temperatura. \
                                     Fusión agua=334000, Vaporización agua=2260000".into(),
                    },
                ],
                constraints: vec![
                    Constraint {
                        condition: "Temperatura constante durante el cambio".into(),
                        consequence: "El sistema absorbe energía sin cambiar temperatura".into(),
                    },
                ],
                related_laws: vec!["Transferencia de Calor".into()],
                trust: 1.0,
            },
            
            PhysicsLaw {
                name: "Ley de Gases Ideales".into(),
                domain: PhysicsDomain::Thermodynamics,
                formula: "PV = nRT".into(),
                compute: |v| {
                    v["moles"] * 8.314 * v["temperature"]  // Resultado = PV
                },
                variables: vec![
                    Variable {
                        name: "Presión".into(), symbol: "P".into(),
                        unit: "Pa".into(), typical_range: (0.0, 1e9),
                        description: "Presión del gas".into(),
                    },
                    Variable {
                        name: "Volumen".into(), symbol: "V".into(),
                        unit: "m³".into(), typical_range: (1e-6, 1e6),
                        description: "Volumen del contenedor".into(),
                    },
                    Variable {
                        name: "Moles".into(), symbol: "n".into(),
                        unit: "mol".into(), typical_range: (0.001, 1e6),
                        description: "Cantidad de sustancia".into(),
                    },
                    Variable {
                        name: "Temperatura".into(), symbol: "T".into(),
                        unit: "K".into(), typical_range: (0.0, 1e6),
                        description: "Temperatura absoluta. 0K = -273.15°C".into(),
                    },
                ],
                constraints: vec![
                    Constraint {
                        condition: "Gas ideal".into(),
                        consequence: "Falla a alta presión o baja temperatura. \
                                     Usar Van der Waals para gases reales".into(),
                    },
                ],
                related_laws: vec!["Ley de Boyle".into(), "Ley de Charles".into()],
                trust: 1.0,
            },
            
            PhysicsLaw {
                name: "Conducción Térmica (Fourier)".into(),
                domain: PhysicsDomain::Thermodynamics,
                formula: "q = -k × A × (dT/dx)".into(),
                compute: |v| {
                    v["thermal_conductivity"] * v["area"] 
                    * (v["temp_hot"] - v["temp_cold"]) / v["thickness"]
                },
                variables: vec![
                    Variable {
                        name: "Conductividad térmica".into(),
                        symbol: "k".into(),
                        unit: "W/(m·K)".into(),
                        typical_range: (0.01, 500.0),
                        description: "Capacidad del material de conducir calor. \
                                     Cobre=401, Acero=50, Madera=0.17, Aire=0.025".into(),
                    },
                ],
                constraints: vec![
                    Constraint {
                        condition: "Estado estacionario".into(),
                        consequence: "El flujo de calor no cambia con el tiempo".into(),
                    },
                ],
                related_laws: vec![],
                trust: 1.0,
            },
            
            PhysicsLaw {
                name: "Radiación de Stefan-Boltzmann".into(),
                domain: PhysicsDomain::Thermodynamics,
                formula: "P = ε × σ × A × T⁴".into(),
                compute: |v| {
                    v.get("emissivity").copied().unwrap_or(1.0) 
                    * 5.670374419e-8 
                    * v["area"] 
                    * v["temperature"].powi(4)
                },
                variables: vec![
                    Variable {
                        name: "Emisividad".into(),
                        symbol: "ε".into(),
                        unit: "adimensional".into(),
                        typical_range: (0.0, 1.0),
                        description: "1.0 = cuerpo negro perfecto. \
                                     Metal pulido ≈ 0.05, Concreto ≈ 0.94".into(),
                    },
                ],
                constraints: vec![],
                related_laws: vec![],
                trust: 1.0,
            },
            
            PhysicsLaw {
                name: "Entropía".into(),
                domain: PhysicsDomain::Thermodynamics,
                formula: "ΔS = Q / T".into(),
                compute: |v| v["heat"] / v["temperature"],
                variables: vec![
                    Variable {
                        name: "Cambio de entropía".into(),
                        symbol: "ΔS".into(),
                        unit: "J/K".into(),
                        typical_range: (-1e6, 1e6),
                        description: "Medida del desorden. Siempre aumenta \
                                     en un sistema aislado (Segunda Ley)".into(),
                    },
                ],
                constraints: vec![
                    Constraint {
                        condition: "Proceso reversible".into(),
                        consequence: "Para procesos irreversibles ΔS > Q/T".into(),
                    },
                ],
                related_laws: vec!["Primera Ley de la Termodinámica".into()],
                trust: 1.0,
            },
        ];
    }

    // ═══════════════════════════════════════
    // DINÁMICA DE FLUIDOS
    // ═══════════════════════════════════════
    
    fn load_fluid_dynamics(&mut self) {
        self.fluid_dynamics = vec![
            PhysicsLaw {
                name: "Presión Hidrostática".into(),
                domain: PhysicsDomain::FluidDynamics,
                formula: "P = ρ × g × h".into(),
                compute: |v| {
                    v["density"] * v.get("g").copied().unwrap_or(9.81) * v["depth"]
                },
                variables: vec![
                    Variable {
                        name: "Densidad del fluido".into(),
                        symbol: "ρ".into(),
                        unit: "kg/m³".into(),
                        typical_range: (0.1, 13600.0),
                        description: "Agua=1000, Mercurio=13600, Aire=1.225".into(),
                    },
                    Variable {
                        name: "Profundidad".into(),
                        symbol: "h".into(),
                        unit: "m".into(),
                        typical_range: (0.0, 11000.0),
                        description: "Profundidad bajo la superficie. \
                                     Fosa de las Marianas = 10994m".into(),
                    },
                ],
                constraints: vec![
                    Constraint {
                        condition: "Fluido incompresible".into(),
                        consequence: "ρ constante. Válido para líquidos, no gases a gran profundidad".into(),
                    },
                ],
                related_laws: vec!["Principio de Pascal".into()],
                trust: 1.0,
            },
            
            PhysicsLaw {
                name: "Ecuación de Bernoulli".into(),
                domain: PhysicsDomain::FluidDynamics,
                formula: "P + ½ρv² + ρgh = constante".into(),
                compute: |v| {
                    v["pressure"] 
                    + 0.5 * v["density"] * v["velocity"].powi(2) 
                    + v["density"] * 9.81 * v.get("height").copied().unwrap_or(0.0)
                },
                variables: vec![
                    Variable {
                        name: "Presión estática".into(), symbol: "P".into(),
                        unit: "Pa".into(), typical_range: (0.0, 1e8),
                        description: "Presión del fluido en reposo".into(),
                    },
                    Variable {
                        name: "Velocidad del fluido".into(), symbol: "v".into(),
                        unit: "m/s".into(), typical_range: (0.0, 343.0),
                        description: "Velocidad del flujo".into(),
                    },
                ],
                constraints: vec![
                    Constraint {
                        condition: "Flujo laminar, incompresible, no viscoso".into(),
                        consequence: "No aplica en turbulencia o fluidos viscosos".into(),
                    },
                ],
                related_laws: vec!["Presión Dinámica".into(), "Efecto Venturi".into()],
                trust: 1.0,
            },
            
            PhysicsLaw {
                name: "Número de Reynolds".into(),
                domain: PhysicsDomain::FluidDynamics,
                formula: "Re = ρ × v × L / μ".into(),
                compute: |v| {
                    v["density"] * v["velocity"] * v["characteristic_length"] / v["viscosity"]
                },
                variables: vec![
                    Variable {
                        name: "Número de Reynolds".into(),
                        symbol: "Re".into(),
                        unit: "adimensional".into(),
                        typical_range: (0.0, 1e8),
                        description: "Re < 2300 laminar, Re > 4000 turbulento. \
                                     Define el régimen del flujo".into(),
                    },
                    Variable {
                        name: "Longitud característica".into(),
                        symbol: "L".into(),
                        unit: "m".into(),
                        typical_range: (0.001, 1000.0),
                        description: "Dimensión típica del objeto en el flujo".into(),
                    },
                    Variable {
                        name: "Viscosidad dinámica".into(),
                        symbol: "μ".into(),
                        unit: "Pa·s".into(),
                        typical_range: (1e-6, 1e3),
                        description: "Resistencia interna al flujo. \
                                     Agua=0.001, Miel=2.0, Aire=1.81e-5".into(),
                    },
                ],
                constraints: vec![],
                related_laws: vec!["Ecuación de Bernoulli".into()],
                trust: 1.0,
            },
            
            PhysicsLaw {
                name: "Ecuación de Continuidad".into(),
                domain: PhysicsDomain::FluidDynamics,
                formula: "A₁v₁ = A₂v₂".into(),
                compute: |v| v["area_1"] * v["velocity_1"], // = area_2 * velocity_2
                variables: vec![
                    Variable {
                        name: "Caudal volumétrico".into(),
                        symbol: "Q".into(),
                        unit: "m³/s".into(),
                        typical_range: (0.0, 1e6),
                        description: "Volumen de fluido por unidad de tiempo. \
                                     Se conserva en flujo incompresible".into(),
                    },
                ],
                constraints: vec![
                    Constraint {
                        condition: "Flujo estacionario e incompresible".into(),
                        consequence: "Si el área se reduce, la velocidad aumenta".into(),
                    },
                ],
                related_laws: vec!["Ecuación de Bernoulli".into()],
                trust: 1.0,
            },
            
            PhysicsLaw {
                name: "Ley de Stokes (Arrastre viscoso)".into(),
                domain: PhysicsDomain::FluidDynamics,
                formula: "F_d = 6π × μ × r × v".into(),
                compute: |v| {
                    6.0 * std::f64::consts::PI * v["viscosity"] * v["radius"] * v["velocity"]
                },
                variables: vec![
                    Variable {
                        name: "Radio de la esfera".into(),
                        symbol: "r".into(),
                        unit: "m".into(),
                        typical_range: (1e-9, 0.1),
                        description: "Solo para esferas pequeñas en flujo lento".into(),
                    },
                ],
                constraints: vec![
                    Constraint {
                        condition: "Re << 1".into(),
                        consequence: "Solo válido para Reynolds muy bajo (flujo reptante)".into(),
                    },
                ],
                related_laws: vec!["Número de Reynolds".into()],
                trust: 1.0,
            },
        ];
    }

    // ═══════════════════════════════════════
    // AERODINÁMICA
    // ═══════════════════════════════════════
    
    fn load_aerodynamics(&mut self) {
        self.aerodynamics = vec![
            PhysicsLaw {
                name: "Presión Dinámica".into(),
                domain: PhysicsDomain::Aerodynamics,
                formula: "q = ½ × ρ × v²".into(),
                compute: |v| {
                    0.5 * v.get("rho").copied().unwrap_or(1.225) * v["velocity"].powi(2)
                },
                variables: vec![
                    Variable {
                        name: "Presión dinámica".into(),
                        symbol: "q".into(),
                        unit: "Pa".into(),
                        typical_range: (0.0, 1e6),
                        description: "Presión debida al movimiento del fluido. \
                                     A 100 km/h en aire: ~480 Pa".into(),
                    },
                ],
                constraints: vec![
                    Constraint {
                        condition: "Mach < 0.3".into(),
                        consequence: "Para flujo compresible (Mach>0.3) usar correcciones".into(),
                    },
                ],
                related_laws: vec!["Fuerza de Arrastre".into(), "Fuerza de Sustentación".into()],
                trust: 1.0,
            },
            
            PhysicsLaw {
                name: "Fuerza de Arrastre".into(),
                domain: PhysicsDomain::Aerodynamics,
                formula: "F_d = Cd × q × A".into(),
                compute: |v| {
                    v["drag_coefficient"] 
                    * (0.5 * v.get("rho").copied().unwrap_or(1.225) * v["velocity"].powi(2))
                    * v["frontal_area"]
                },
                variables: vec![
                    Variable {
                        name: "Coeficiente de arrastre".into(),
                        symbol: "Cd".into(),
                        unit: "adimensional".into(),
                        typical_range: (0.01, 2.5),
                        description: "Esfera=0.47, Cubo=1.05, Cilindro=0.82, \
                                     Cono=0.50, Ala perfil=0.04, Auto deportivo=0.25, \
                                     Placa plana=1.98, Gota=0.04".into(),
                    },
                    Variable {
                        name: "Área frontal".into(),
                        symbol: "A".into(),
                        unit: "m²".into(),
                        typical_range: (0.001, 1000.0),
                        description: "Área proyectada perpendicular al flujo".into(),
                    },
                ],
                constraints: vec![],
                related_laws: vec!["Presión Dinámica".into(), "Número de Reynolds".into()],
                trust: 1.0,
            },
            
            PhysicsLaw {
                name: "Fuerza de Sustentación".into(),
                domain: PhysicsDomain::Aerodynamics,
                formula: "F_L = Cl × q × A".into(),
                compute: |v| {
                    v["lift_coefficient"] 
                    * (0.5 * v.get("rho").copied().unwrap_or(1.225) * v["velocity"].powi(2))
                    * v["wing_area"]
                },
                variables: vec![
                    Variable {
                        name: "Coeficiente de sustentación".into(),
                        symbol: "Cl".into(),
                        unit: "adimensional".into(),
                        typical_range: (-0.5, 2.5),
                        description: "Depende del perfil alar y ángulo de ataque. \
                                     Perfil NACA típico max Cl ≈ 1.5. \
                                     Stall cuando Cl cae abruptamente".into(),
                    },
                    Variable {
                        name: "Área alar".into(),
                        symbol: "A".into(),
                        unit: "m²".into(),
                        typical_range: (0.1, 1000.0),
                        description: "Área de la superficie sustentadora".into(),
                    },
                ],
                constraints: vec![
                    Constraint {
                        condition: "Ángulo de ataque < ángulo de stall".into(),
                        consequence: "Más allá del stall, Cl cae y el ala pierde sustentación".into(),
                    },
                ],
                related_laws: vec!["Fuerza de Arrastre".into()],
                trust: 1.0,
            },
            
            PhysicsLaw {
                name: "Número de Mach".into(),
                domain: PhysicsDomain::Aerodynamics,
                formula: "Ma = v / c_sonido".into(),
                compute: |v| {
                    v["velocity"] / v.get("speed_of_sound").copied().unwrap_or(343.0)
                },
                variables: vec![
                    Variable {
                        name: "Número de Mach".into(),
                        symbol: "Ma".into(),
                        unit: "adimensional".into(),
                        typical_range: (0.0, 25.0),
                        description: "Ma<0.3 incompresible, 0.3-1.0 subsónico, \
                                     1.0 sónico, 1.0-5.0 supersónico, >5.0 hipersónico".into(),
                    },
                    Variable {
                        name: "Velocidad del sonido".into(),
                        symbol: "c".into(),
                        unit: "m/s".into(),
                        typical_range: (200.0, 1500.0),
                        description: "En aire a 20°C: 343 m/s. En agua: 1482 m/s. \
                                     En acero: 5120 m/s".into(),
                    },
                ],
                constraints: vec![],
                related_laws: vec!["Presión Dinámica".into()],
                trust: 1.0,
            },
            
            PhysicsLaw {
                name: "Relación Sustentación/Arrastre".into(),
                domain: PhysicsDomain::Aerodynamics,
                formula: "L/D = Cl / Cd".into(),
                compute: |v| v["lift_coefficient"] / v["drag_coefficient"].max(0.0001),
                variables: vec![
                    Variable {
                        name: "Eficiencia aerodinámica".into(),
                        symbol: "L/D".into(),
                        unit: "adimensional".into(),
                        typical_range: (1.0, 70.0),
                        description: "Planeador=30-70, Avión comercial=15-20, \
                                     Auto deportivo no aplica (sin sustentación)".into(),
                    },
                ],
                constraints: vec![],
                related_laws: vec!["Fuerza de Sustentación".into(), "Fuerza de Arrastre".into()],
                trust: 1.0,
            },
        ];
    }

    // ═══════════════════════════════════════
    // INGENIERÍA ESTRUCTURAL
    // ═══════════════════════════════════════
    
    fn load_structural(&mut self) {
        self.structural = vec![
            PhysicsLaw {
                name: "Esfuerzo Normal".into(),
                domain: PhysicsDomain::Structural,
                formula: "σ = F / A".into(),
                compute: |v| v["force"] / v["area"].max(0.0001),
                variables: vec![
                    Variable {
                        name: "Esfuerzo".into(),
                        symbol: "σ".into(),
                        unit: "Pa".into(),
                        typical_range: (0.0, 1e10),
                        description: "Si σ > σ_yield del material, hay deformación permanente. \
                                     Si σ > σ_ultimate, hay fractura".into(),
                    },
                ],
                constraints: vec![
                    Constraint {
                        condition: "Distribución uniforme".into(),
                        consequence: "En la práctica hay concentradores de esfuerzo \
                                     en esquinas y agujeros".into(),
                    },
                ],
                related_laws: vec!["Ley de Hooke".into()],
                trust: 1.0,
            },
            
            PhysicsLaw {
                name: "Ley de Hooke".into(),
                domain: PhysicsDomain::Structural,
                formula: "σ = E × ε".into(),
                compute: |v| v["elastic_modulus"] * v["strain"],
                variables: vec![
                    Variable {
                        name: "Módulo de Young".into(),
                        symbol: "E".into(),
                        unit: "Pa".into(),
                        typical_range: (1e6, 500e9),
                        description: "Rigidez del material. \
                                     Acero=200GPa, Concreto=30GPa, Madera=12GPa, \
                                     Caucho=0.05GPa".into(),
                    },
                    Variable {
                        name: "Deformación unitaria".into(),
                        symbol: "ε".into(),
                        unit: "adimensional".into(),
                        typical_range: (0.0, 0.5),
                        description: "ΔL/L. Cambio relativo de longitud. \
                                     ε=0.002 típico en límite elástico del acero".into(),
                    },
                ],
                constraints: vec![
                    Constraint {
                        condition: "Región elástica".into(),
                        consequence: "Solo válida antes del límite elástico (yield). \
                                     Después hay deformación plástica permanente".into(),
                    },
                ],
                related_laws: vec!["Esfuerzo Normal".into()],
                trust: 1.0,
            },
            
            PhysicsLaw {
                name: "Momento de Volcamiento".into(),
                domain: PhysicsDomain::Structural,
                formula: "M_volc = F_lateral × h_aplicación".into(),
                compute: |v| v["lateral_force"] * v["height_of_application"],
                variables: vec![
                    Variable {
                        name: "Momento volcante".into(),
                        symbol: "M_v".into(),
                        unit: "N·m".into(),
                        typical_range: (0.0, 1e12),
                        description: "Si M_volcante > M_estabilizante, la estructura vuelca".into(),
                    },
                ],
                constraints: vec![],
                related_laws: vec!["Momento Estabilizante".into()],
                trust: 1.0,
            },
            
            PhysicsLaw {
                name: "Momento Estabilizante".into(),
                domain: PhysicsDomain::Structural,
                formula: "M_est = W × d_base/2".into(),
                compute: |v| v["weight"] * v["base_width"] / 2.0,
                variables: vec![
                    Variable {
                        name: "Momento estabilizante".into(),
                        symbol: "M_e".into(),
                        unit: "N·m".into(),
                        typical_range: (0.0, 1e12),
                        description: "Resistencia al volcamiento por peso propio. \
                                     Base más ancha = más estable".into(),
                    },
                ],
                constraints: vec![
                    Constraint {
                        condition: "Factor de seguridad".into(),
                        consequence: "En ingeniería: M_est/M_volc > 1.5 para seguridad".into(),
                    },
                ],
                related_laws: vec!["Momento de Volcamiento".into(), "Peso".into()],
                trust: 1.0,
            },
            
            PhysicsLaw {
                name: "Pandeo de Euler (Columnas)".into(),
                domain: PhysicsDomain::Structural,
                formula: "P_cr = π² × E × I / L²".into(),
                compute: |v| {
                    std::f64::consts::PI.powi(2) 
                    * v["elastic_modulus"] 
                    * v["moment_of_inertia"] 
                    / v["length"].powi(2)
                },
                variables: vec![
                    Variable {
                        name: "Carga crítica de pandeo".into(),
                        symbol: "P_cr".into(),
                        unit: "N".into(),
                        typical_range: (1e3, 1e9),
                        description: "Carga axial máxima antes de que la columna flambe. \
                                     Columnas esbeltas fallan por pandeo antes que por compresión".into(),
                    },
                    Variable {
                        name: "Momento de inercia".into(),
                        symbol: "I".into(),
                        unit: "m⁴".into(),
                        typical_range: (1e-8, 1e2),
                        description: "Resistencia geométrica a la flexión. \
                                     Sección cuadrada: I = b⁴/12. Circular: I = πr⁴/4".into(),
                    },
                    Variable {
                        name: "Longitud de la columna".into(),
                        symbol: "L".into(),
                        unit: "m".into(),
                        typical_range: (0.1, 100.0),
                        description: "Longitud libre. Más larga = menos carga crítica".into(),
                    },
                ],
                constraints: vec![
                    Constraint {
                        condition: "Columna esbelta (L/r > 120)".into(),
                        consequence: "Columnas cortas fallan por aplastamiento, no pandeo".into(),
                    },
                ],
                related_laws: vec!["Ley de Hooke".into()],
                trust: 1.0,
            },
            
            PhysicsLaw {
                name: "Factor de Seguridad".into(),
                domain: PhysicsDomain::Structural,
                formula: "FS = Resistencia / Carga".into(),
                compute: |v| v["resistance"] / v["load"].max(0.001),
                variables: vec![
                    Variable {
                        name: "Factor de seguridad".into(),
                        symbol: "FS".into(),
                        unit: "adimensional".into(),
                        typical_range: (1.0, 10.0),
                        description: "FS=1.0 falla inminente. FS=1.5 edificios. \
                                     FS=2.0 puentes. FS=3.0 aeronáutica. \
                                     FS>4.0 nuclear".into(),
                    },
                ],
                constraints: vec![],
                related_laws: vec![],
                trust: 1.0,
            },
        ];
    }

    // ═══════════════════════════════════════
    // CIENCIA DE MATERIALES
    // ═══════════════════════════════════════
    
    fn load_material_science(&mut self) {
        self.material_science = vec![
            PhysicsLaw {
                name: "Dilatación Térmica Lineal".into(),
                domain: PhysicsDomain::MaterialScience,
                formula: "ΔL = α × L₀ × ΔT".into(),
                compute: |v| v["expansion_coefficient"] * v["original_length"] * v["delta_temp"],
                variables: vec![
                    Variable {
                        name: "Coeficiente de dilatación".into(),
                        symbol: "α".into(),
                        unit: "1/K".into(),
                        typical_range: (1e-7, 1e-4),
                        description: "Acero=12e-6, Aluminio=23e-6, Concreto=10e-6, \
                                     Vidrio=9e-6. Indica cuánto crece por grado".into(),
                    },
                ],
                constraints: vec![
                    Constraint {
                        condition: "Material isotrópico".into(),
                        consequence: "Materiales compuestos dilatan diferente en cada dirección".into(),
                    },
                ],
                related_laws: vec![],
                trust: 1.0,
            },
            
            PhysicsLaw {
                name: "Fatiga de Material".into(),
                domain: PhysicsDomain::MaterialScience,
                formula: "N_f = (σ_f / σ_a)^b".into(),
                compute: |v| {
                    (v["fatigue_strength"] / v["stress_amplitude"]).powf(v.get("basquin_exponent").copied().unwrap_or(8.0))
                },
                variables: vec![
                    Variable {
                        name: "Ciclos hasta falla".into(),
                        symbol: "N_f".into(),
                        unit: "ciclos".into(),
                        typical_range: (1e3, 1e10),
                        description: "Número de ciclos de carga antes de fractura. \
                                     El material falla con carga menor que la estática \
                                     si se aplica repetidamente".into(),
                    },
                    Variable {
                        name: "Amplitud de esfuerzo".into(),
                        symbol: "σ_a".into(),
                        unit: "Pa".into(),
                        typical_range: (1e6, 1e9),
                        description: "Esfuerzo alternante aplicado en cada ciclo".into(),
                    },
                ],
                constraints: vec![
                    Constraint {
                        condition: "Aceros tienen límite de fatiga".into(),
                        consequence: "Por debajo del límite, aguantan infinitos ciclos. \
                                     Aluminio NO tiene límite: siempre falla eventualmente".into(),
                    },
                ],
                related_laws: vec!["Esfuerzo Normal".into()],
                trust: 1.0,
            },
        ];
    }

    // ═══════════════════════════════════════
    // QUÍMICA
    // ═══════════════════════════════════════
    
    fn load_chemistry(&mut self) {
        self.chemistry = vec![
            PhysicsLaw {
                name: "Ley de Conservación de Masa".into(),
                domain: PhysicsDomain::Chemistry,
                formula: "Σ masa_reactivos = Σ masa_productos".into(),
                compute: |v| v["reactant_mass"], // Siempre igual a producto
                variables: vec![
                    Variable {
                        name: "Masa de reactivos".into(),
                        symbol: "m_r".into(),
                        unit: "kg".into(),
                        typical_range: (1e-6, 1e6),
                        description: "La masa total no cambia en una reacción química. \
                                     Solo se redistribuye entre productos".into(),
                    },
                ],
                constraints: vec![
                    Constraint {
                        condition: "Reacciones químicas ordinarias".into(),
                        consequence: "En reacciones nucleares, E=mc² aplica y la masa cambia".into(),
                    },
                ],
                related_laws: vec![],
                trust: 1.0,
            },
            
            PhysicsLaw {
                name: "Energía de Enlace".into(),
                domain: PhysicsDomain::Chemistry,
                formula: "ΔH = Σ(enlaces rotos) - Σ(enlaces formados)".into(),
                compute: |v| v["bonds_broken"] - v["bonds_formed"],
                variables: vec![
                    Variable {
                        name: "Entalpía de reacción".into(),
                        symbol: "ΔH".into(),
                        unit: "kJ/mol".into(),
                        typical_range: (-2000.0, 2000.0),
                        description: "ΔH < 0: exotérmica (libera calor). \
                                     ΔH > 0: endotérmica (absorbe calor). \
                                     C-H=413, O=O=498, C=O=799, O-H=463 kJ/mol".into(),
                    },
                ],
                constraints: vec![],
                related_laws: vec!["Primera Ley de la Termodinámica".into()],
                trust: 1.0,
            },
            
            PhysicsLaw {
                name: "Ley de Arrhenius (Velocidad de Reacción)".into(),
                domain: PhysicsDomain::Chemistry,
                formula: "k = A × e^(-Ea/RT)".into(),
                compute: |v| {
                    v["pre_exponential"] 
                    * (-v["activation_energy"] / (8.314 * v["temperature"])).exp()
                },
                variables: vec![
                    Variable {
                        name: "Constante de velocidad".into(),
                        symbol: "k".into(),
                        unit: "variable".into(),
                        typical_range: (1e-10, 1e10),
                        description: "Qué tan rápido ocurre la reacción. \
                                     Aumenta exponencialmente con temperatura".into(),
                    },
                    Variable {
                        name: "Energía de activación".into(),
                        symbol: "Ea".into(),
                        unit: "J/mol".into(),
                        typical_range: (1e3, 5e5),
                        description: "Barrera energética para que la reacción ocurra. \
                                     Catalizadores la reducen sin consumirse".into(),
                    },
                ],
                constraints: vec![],
                related_laws: vec![],
                trust: 1.0,
            },
            
            PhysicsLaw {
                name: "pH".into(),
                domain: PhysicsDomain::Chemistry,
                formula: "pH = -log₁₀[H⁺]".into(),
                compute: |v| -(v["h_concentration"].log10()),
                variables: vec![
                    Variable {
                        name: "pH".into(),
                        symbol: "pH".into(),
                        unit: "adimensional".into(),
                        typical_range: (0.0, 14.0),
                        description: "pH 7 = neutro. pH < 7 ácido. pH > 7 básico. \
                                     Estómago=1.5, Limón=2, Agua=7, Sangre=7.4, \
                                     Lejía=13".into(),
                    },
                    Variable {
                        name: "Concentración de H⁺".into(),
                        symbol: "[H⁺]".into(),
                        unit: "mol/L".into(),
                        typical_range: (1e-14, 1.0),
                        description: "Concentración de iones hidrógeno en solución".into(),
                    },
                ],
                constraints: vec![
                    Constraint {
                        condition: "Solución acuosa".into(),
                        consequence: "No tiene sentido hablar de pH en solventes no acuosos".into(),
                    },
                ],
                related_laws: vec![],
                trust: 1.0,
            },
            
            PhysicsLaw {
                name: "Oxidación-Reducción".into(),
                domain: PhysicsDomain::Chemistry,
                formula: "Oxidación pierde e⁻, Reducción gana e⁻".into(),
                compute: |v| v["electrons_transferred"] * 96485.0 * v["voltage"],
                variables: vec![
                    Variable {
                        name: "Energía libre de Gibbs".into(),
                        symbol: "ΔG".into(),
                        unit: "J".into(),
                        typical_range: (-1e6, 1e6),
                        description: "ΔG = -nFE. Si ΔG < 0, reacción espontánea. \
                                     n = electrones transferidos, F = constante de Faraday, \
                                     E = potencial de celda".into(),
                    },
                ],
                constraints: vec![],
                related_laws: vec!["Energía de Enlace".into()],
                trust: 1.0,
            },
        ];
    }

    // ═══════════════════════════════════════
    // FÍSICA ATÓMICA Y NUCLEAR
    // ═══════════════════════════════════════
    
    fn load_atomic(&mut self) {
        self.atomic = vec![
            PhysicsLaw {
                name: "Equivalencia Masa-Energía".into(),
                domain: PhysicsDomain::Atomic,
                formula: "E = m × c²".into(),
                compute: |v| v["mass"] * (299_792_458.0_f64).powi(2),
                variables: vec![
                    Variable {
                        name: "Energía".into(),
                        symbol: "E".into(),
                        unit: "J".into(),
                        typical_range: (0.0, 1e20),
                        description: "1 kg de masa = 89.9 petajoules. \
                                     Bomba de Hiroshima ≈ 0.7 gramos convertidos".into(),
                    },
                ],
                constraints: vec![],
                related_laws: vec![],
                trust: 1.0,
            },
            
            PhysicsLaw {
                name: "Energía del Fotón".into(),
                domain: PhysicsDomain::Atomic,
                formula: "E = h × f".into(),
                compute: |v| 6.62607015e-34 * v["frequency"],
                variables: vec![
                    Variable {
                        name: "Frecuencia".into(),
                        symbol: "f".into(),
                        unit: "Hz".into(),
                        typical_range: (1e3, 1e20),
                        description: "Radio=1e6, Microondas=1e10, Luz visible=5e14, \
                                     Rayos X=1e18, Gamma=1e20".into(),
                    },
                ],
                constraints: vec![],
                related_laws: vec!["Equivalencia Masa-Energía".into()],
                trust: 1.0,
            },
            
            PhysicsLaw {
                name: "Ley de Decaimiento Radiactivo".into(),
                domain: PhysicsDomain::Atomic,
                formula: "N(t) = N₀ × e^(-λt)".into(),
                compute: |v| {
                    v["initial_atoms"] * (-v["decay_constant"] * v["time"]).exp()
                },
                variables: vec![
                    Variable {
                        name: "Constante de decaimiento".into(),
                        symbol: "λ".into(),
                        unit: "1/s".into(),
                        typical_range: (1e-20, 1e10),
                        description: "λ = ln(2)/t_½. Vida media del C-14: 5730 años. \
                                     U-238: 4.5e9 años. Po-214: 164 microsegundos".into(),
                    },
                    Variable {
                        name: "Vida media".into(),
                        symbol: "t½".into(),
                        unit: "s".into(),
                        typical_range: (1e-7, 1e17),
                        description: "Tiempo para que la mitad de los átomos decaigan".into(),
                    },
                ],
                constraints: vec![],
                related_laws: vec![],
                trust: 1.0,
            },
            
            PhysicsLaw {
                name: "Principio de Incertidumbre de Heisenberg".into(),
                domain: PhysicsDomain::Atomic,
                formula: "Δx × Δp ≥ ℏ/2".into(),
                compute: |v| {
                    // Mínima incertidumbre en posición dada incertidumbre en momento
                    (6.62607015e-34 / (2.0 * std::f64::consts::PI)) 
                    / (2.0 * v["momentum_uncertainty"])
                },
                variables: vec![
                    Variable {
                        name: "Incertidumbre en posición".into(),
                        symbol: "Δx".into(),
                        unit: "m".into(),
                        typical_range: (1e-15, 1e-6),
                        description: "No es limitación del instrumento. \
                                     Es propiedad fundamental de la naturaleza".into(),
                    },
                    Variable {
                        name: "Incertidumbre en momento".into(),
                        symbol: "Δp".into(),
                        unit: "kg·m/s".into(),
                        typical_range: (1e-30, 1e-20),
                        description: "Cuanto más precisa la posición, \
                                     menos preciso el momento y viceversa".into(),
                    },
                ],
                constraints: vec![
                    Constraint {
                        condition: "Escala cuántica".into(),
                        consequence: "Irrelevante para objetos macroscópicos".into(),
                    },
                ],
                related_laws: vec!["Energía del Fotón".into()],
                trust: 1.0,
            },
        ];
    }

    // ═══════════════════════════════════════
    // MÉTODOS DE CONSULTA
    // ═══════════════════════════════════════
    
    pub fn find_law(&self, name: &str) -> Option<&PhysicsLaw> {
        let all_laws = self.all_laws();
        all_laws.into_iter().find(|l| l.name.to_lowercase().contains(&name.to_lowercase()))
    }
    
    pub fn find_by_domain(&self, domain: &PhysicsDomain) -> Vec<&PhysicsLaw> {
        self.all_laws().into_iter().filter(|l| l.domain == *domain).collect()
    }
    
    pub fn find_material(&self, name: &str) -> Option<&MaterialProperties> {
        self.materials.get(name)
            .or_else(|| {
                self.materials.values().find(|m| 
                    m.name.to_lowercase().contains(&name.to_lowercase())
                )
            })
    }
    
    pub fn get_constant(&self, symbol: &str) -> Option<f64> {
        self.constants.get(symbol).map(|c| c.value)
    }
    
    pub fn all_laws(&self) -> Vec<&PhysicsLaw> {
        let mut all = Vec::new();
        all.extend(self.mechanics.iter());
        all.extend(self.thermodynamics.iter());
        all.extend(self.fluid_dynamics.iter());
        all.extend(self.aerodynamics.iter());
        all.extend(self.structural.iter());
        all.extend(self.material_science.iter());
        all.extend(self.chemistry.iter());
        all.extend(self.atomic.iter());
        all
    }
    
    pub fn related_laws(&self, law_name: &str) -> Vec<&PhysicsLaw> {
        if let Some(law) = self.find_law(law_name) {
            law.related_laws.iter()
                .filter_map(|name| self.find_law(name))
                .collect()
        } else {
            Vec::new()
        }
    }
    
    pub fn laws_count(&self) -> usize {
        self.all_laws().len()
    }
    
    pub fn print_summary(&self) {
        println!("\n╔══════════════════════════════════════════╗");
        println!("║  BASE DE CONOCIMIENTO FÍSICA DE DAITHON  ║");
        println!("╚══════════════════════════════════════════╝");
        println!("  Mecánica:           {} leyes", self.mechanics.len());
        println!("  Termodinámica:      {} leyes", self.thermodynamics.len());
        println!("  Fluidos:            {} leyes", self.fluid_dynamics.len());
        println!("  Aerodinámica:       {} leyes", self.aerodynamics.len());
        println!("  Estructural:        {} leyes", self.structural.len());
        println!("  Materiales:         {} leyes", self.material_science.len());
        println!("  Química:            {} leyes", self.chemistry.len());
        println!("  Atómica:            {} leyes", self.atomic.len());
        println!("  ─────────────────────────────");
        println!("  Total:              {} leyes", self.laws_count());
        println!("  Constantes:         {}", self.constants.len());
        println!("  Materiales:         {}", self.materials.len());
    }
}
