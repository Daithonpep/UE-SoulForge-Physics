// harmonia/context.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Motor de contexto que determina pesos de evaluación
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MUSE {
    active_context: DesignContext,
    context_library: HashMap<String, DesignContext>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignContext {
    pub name: String,
    pub description: String,
    pub reality_profile: String, // "EARTH_REALITY", "FANTASY_WORLD", etc.
    pub priority_weights: PriorityWeights,
    pub constraints: ContextConstraints,
    pub aesthetic_rules: AestheticRules,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PriorityWeights {
    /// Integridad estructural (0.0 - 1.0)
    pub structural_integrity: f32,
    
    /// Eficiencia económica
    pub economic_efficiency: f32,
    
    /// Rendimiento aerodinámico/hidrodinámico
    pub aerodynamic_performance: f32,
    
    /// Estética y armonía visual
    pub aesthetic_harmony: f32,
    
    /// Innovación y originalidad
    pub innovation_factor: f32,
    
    /// Manufacturabilidad
    pub manufacturability: f32,
    
    /// Sostenibilidad ambiental
    pub sustainability: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextConstraints {
    /// Restricciones físicas obligatorias (safety first)
    pub hard_physics: bool,
    
    /// Presupuesto máximo (USD)
    pub budget_limit: Option<f32>,
    
    /// Tiempo de construcción máximo (días)
    pub time_limit: Option<u32>,
    
    /// Requisitos de certificación
    pub certifications: Vec<String>,
    
    /// Tolerancia a fallos (0.0 = cero tolerancia, 1.0 = permisivo)
    pub failure_tolerance: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AestheticRules {
    /// Nivel de simetría requerido (0.0 - 1.0)
    pub symmetry_requirement: f32,
    
    /// Tipo de simetría
    pub symmetry_type: SymmetryType,
    
    /// Uso de proporción áurea
    pub golden_ratio_enforcement: f32,
    
    /// Complejidad visual permitida
    pub visual_complexity_range: (f32, f32),
    
    /// Suavidad de superficies (0.0 = angular, 1.0 = orgánico)
    pub surface_smoothness: f32,
    
    /// Patrones rítmicos (Fibonacci, modulares)
    pub rhythmic_patterns: Vec<RhythmicPattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SymmetryType {
    None,
    Bilateral { axis: String },
    Radial { sectors: u32 },
    Translational { pattern: String },
    Fractal { depth: u32 },
    Combined(Vec<SymmetryType>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RhythmicPattern {
    Fibonacci { sequence_depth: u32 },
    Modular { module_size: f32, repetitions: u32 },
    Gradient { progression: String },
    Random { controlled_chaos: f32 },
}

impl MUSE {
    pub fn new() -> Self {
        let mut engine = Self {
            active_context: Self::default_context(),
            context_library: HashMap::new(),
        };

        engine.initialize_contexts();
        engine.active_context = engine.context_library["ENGINEERING_REALISTIC"].clone();
        engine
    }

    fn initialize_contexts(&mut self) {
        // ============================================
        // CONTEXTO 1: INGENIERÍA REALISTA
        // ============================================
        self.context_library.insert(
            "ENGINEERING_REALISTIC".to_string(),
            DesignContext {
                name: "Ingeniería Realista".to_string(),
                description: "Diseño industrial/civil con máxima prioridad en seguridad".to_string(),
                reality_profile: "EARTH_REALITY".to_string(),
                
                priority_weights: PriorityWeights {
                    structural_integrity: 0.45,      // ⚠️ MÁXIMA PRIORIDAD
                    economic_efficiency: 0.20,
                    aerodynamic_performance: 0.05,
                    aesthetic_harmony: 0.10,
                    innovation_factor: 0.05,
                    manufacturability: 0.10,
                    sustainability: 0.05,
                },

                constraints: ContextConstraints {
                    hard_physics: true,              // ⚠️ FÍSICA ESTRICTA
                    budget_limit: Some(1_000_000.0),
                    time_limit: Some(365),
                    certifications: vec![
                        "ISO 9001".to_string(),
                        "Building Code".to_string(),
                    ],
                    failure_tolerance: 0.0,          // ⚠️ CERO TOLERANCIA
                },

                aesthetic_rules: AestheticRules {
                    symmetry_requirement: 0.7,
                    symmetry_type: SymmetryType::Bilateral { axis: "Y".to_string() },
                    golden_ratio_enforcement: 0.3,
                    visual_complexity_range: (0.2, 0.6),
                    surface_smoothness: 0.4,
                    rhythmic_patterns: vec![
                        RhythmicPattern::Modular { 
                            module_size: 1.0, 
                            repetitions: 5 
                        },
                    ],
                },
            },
        );

        // ============================================
        // CONTEXTO 2: ARQUITECTURA DE ALTO DISEÑO
        // ============================================
        self.context_library.insert(
            "HIGH_DESIGN_ARCHITECTURE".to_string(),
            DesignContext {
                name: "Arquitectura de Alto Diseño".to_string(),
                description: "Balance entre función y forma estética excepcional".to_string(),
                reality_profile: "EARTH_REALITY".to_string(),
                
                priority_weights: PriorityWeights {
                    structural_integrity: 0.30,      // ⚖️ BALANCE
                    economic_efficiency: 0.15,
                    aerodynamic_performance: 0.05,
                    aesthetic_harmony: 0.35,         // 🎨 ALTA PRIORIDAD
                    innovation_factor: 0.10,
                    manufacturability: 0.03,
                    sustainability: 0.02,
                },

                constraints: ContextConstraints {
                    hard_physics: true,
                    budget_limit: Some(10_000_000.0), // Mayor presupuesto
                    time_limit: Some(730),
                    certifications: vec!["Architectural Award Eligible".to_string()],
                    failure_tolerance: 0.1,
                },

                aesthetic_rules: AestheticRules {
                    symmetry_requirement: 0.85,      // 🎨 ALTA SIMETRÍA
                    symmetry_type: SymmetryType::Combined(vec![
                        SymmetryType::Bilateral { axis: "Y".to_string() },
                        SymmetryType::Radial { sectors: 8 },
                    ]),
                    golden_ratio_enforcement: 0.8,   // 🎨 FIBONACCI FUERTE
                    visual_complexity_range: (0.5, 0.9),
                    surface_smoothness: 0.7,
                    rhythmic_patterns: vec![
                        RhythmicPattern::Fibonacci { sequence_depth: 8 },
                        RhythmicPattern::Gradient { progression: "harmonic".to_string() },
                    ],
                },
            },
        );

        // ============================================
        // CONTEXTO 3: AEROESPACIAL EXTREMO
        // ============================================
        self.context_library.insert(
            "AEROSPACE_EXTREME".to_string(),
            DesignContext {
                name: "Aeroespacial Extremo".to_string(),
                description: "Optimización aerodinámica y peso mínimo absoluto".to_string(),
                reality_profile: "SPACE_STATION".to_string(),
                
                priority_weights: PriorityWeights {
                    structural_integrity: 0.35,
                    economic_efficiency: 0.25,       // 💰 PESO = $$$
                    aerodynamic_performance: 0.30,   // ✈️ CRÍTICO
                    aesthetic_harmony: 0.02,
                    innovation_factor: 0.05,
                    manufacturability: 0.02,
                    sustainability: 0.01,
                },

                constraints: ContextConstraints {
                    hard_physics: true,
                    budget_limit: Some(100_000_000.0),
                    time_limit: Some(1095),
                    certifications: vec!["NASA/ESA Standards".to_string()],
                    failure_tolerance: 0.0,
                },

                aesthetic_rules: AestheticRules {
                    symmetry_requirement: 0.95,      // ✈️ BALANCE CRÍTICO
                    symmetry_type: SymmetryType::Radial { sectors: 6 },
                    golden_ratio_enforcement: 0.2,
                    visual_complexity_range: (0.1, 0.4),
                    surface_smoothness: 0.95,        // ✈️ FLUJO LAMINAR
                    rhythmic_patterns: vec![],
                },
            },
        );

        // ============================================
        // CONTEXTO 4: FANTASÍA CREATIVA
        // ============================================
        self.context_library.insert(
            "FANTASY_CREATIVE".to_string(),
            DesignContext {
                name: "Fantasía Creativa".to_string(),
                description: "Máxima libertad creativa, física relajada".to_string(),
                reality_profile: "FANTASY_WORLD".to_string(),
                
                priority_weights: PriorityWeights {
                    structural_integrity: 0.05,      // 🎭 MÍNIMO
                    economic_efficiency: 0.02,
                    aerodynamic_performance: 0.01,
                    aesthetic_harmony: 0.70,         // 🎨 MÁXIMO
                    innovation_factor: 0.20,
                    manufacturability: 0.01,
                    sustainability: 0.01,
                },

                constraints: ContextConstraints {
                    hard_physics: false,             // 🎭 FÍSICA RELAJADA
                    budget_limit: None,
                    time_limit: None,
                    certifications: vec![],
                    failure_tolerance: 1.0,          // 🎭 TOLERANTE
                },

                aesthetic_rules: AestheticRules {
                    symmetry_requirement: 0.5,
                    symmetry_type: SymmetryType::Fractal { depth: 4 },
                    golden_ratio_enforcement: 0.9,   // 🎨 BELLEZA PURA
                    visual_complexity_range: (0.7, 1.0),
                    surface_smoothness: 0.8,
                    rhythmic_patterns: vec![
                        RhythmicPattern::Fibonacci { sequence_depth: 12 },
                        RhythmicPattern::Random { controlled_chaos: 0.3 },
                    ],
                },
            },
        );

        // ============================================
        // CONTEXTO 5: COMPETICIÓN DEPORTIVA
        // ============================================
        self.context_library.insert(
            "RACING_PERFORMANCE".to_string(),
            DesignContext {
                name: "Competición Deportiva".to_string(),
                description: "Velocidad y rendimiento máximo".to_string(),
                reality_profile: "EARTH_REALITY".to_string(),
                
                priority_weights: PriorityWeights {
                    structural_integrity: 0.25,
                    economic_efficiency: 0.10,
                    aerodynamic_performance: 0.45,   // 🏎️ MÁXIMO
                    aesthetic_harmony: 0.10,
                    innovation_factor: 0.08,
                    manufacturability: 0.01,
                    sustainability: 0.01,
                },

                constraints: ContextConstraints {
                    hard_physics: true,
                    budget_limit: Some(5_000_000.0),
                    time_limit: Some(180),
                    certifications: vec!["FIA Regulations".to_string()],
                    failure_tolerance: 0.15,
                },

                aesthetic_rules: AestheticRules {
                    symmetry_requirement: 0.95,
                    symmetry_type: SymmetryType::Bilateral { axis: "X".to_string() },
                    golden_ratio_enforcement: 0.4,
                    visual_complexity_range: (0.3, 0.7),
                    surface_smoothness: 1.0,         // 🏎️ SUPERFICIE PERFECTA
                    rhythmic_patterns: vec![],
                },
            },
        );

        // ============================================
        // CONTEXTO 6: ARTE CONCEPTUAL
        // ============================================
        self.context_library.insert(
            "CONCEPTUAL_ART".to_string(),
            DesignContext {
                name: "Arte Conceptual".to_string(),
                description: "Innovación visual absoluta, sin restricciones".to_string(),
                reality_profile: "SCIFI_FUTURE".to_string(),
                
                priority_weights: PriorityWeights {
                    structural_integrity: 0.03,
                    economic_efficiency: 0.01,
                    aerodynamic_performance: 0.01,
                    aesthetic_harmony: 0.50,
                    innovation_factor: 0.40,         // 💡 NOVEDAD MÁXIMA
                    manufacturability: 0.03,
                    sustainability: 0.02,
                },

                constraints: ContextConstraints {
                    hard_physics: false,
                    budget_limit: None,
                    time_limit: None,
                    certifications: vec![],
                    failure_tolerance: 1.0,
                },

                aesthetic_rules: AestheticRules {
                    symmetry_requirement: 0.3,       // 🎨 ASIMETRÍA CREATIVA
                    symmetry_type: SymmetryType::None,
                    golden_ratio_enforcement: 0.6,
                    visual_complexity_range: (0.8, 1.0),
                    surface_smoothness: 0.5,
                    rhythmic_patterns: vec![
                        RhythmicPattern::Fibonacci { sequence_depth: 15 },
                        RhythmicPattern::Random { controlled_chaos: 0.7 },
                    ],
                },
            },
        );

        // Contexto por defecto
        self.active_context = self.context_library["ENGINEERING_REALISTIC"].clone();
    }

    /// Cambia el contexto activo
    pub fn set_context(&mut self, context_name: &str) -> Result<(), String> {
        if let Some(context) = self.context_library.get(context_name) {
            self.active_context = context.clone();
            
            println!("\n🎭 MUSE CONTEXT SWITCHED");
            println!("════════════════════════════════════════");
            println!("   Contexto: {}", self.active_context.name);
            println!("   {}", self.active_context.description);
            println!("\n   PESOS DE PRIORIDAD:");
            println!("   ├─ Integridad:      {:.0}%", self.active_context.priority_weights.structural_integrity * 100.0);
            println!("   ├─ Economía:        {:.0}%", self.active_context.priority_weights.economic_efficiency * 100.0);
            println!("   ├─ Aerodinámica:    {:.0}%", self.active_context.priority_weights.aerodynamic_performance * 100.0);
            println!("   ├─ Estética:        {:.0}%", self.active_context.priority_weights.aesthetic_harmony * 100.0);
            println!("   ├─ Innovación:      {:.0}%", self.active_context.priority_weights.innovation_factor * 100.0);
            println!("   ├─ Manufactura:     {:.0}%", self.active_context.priority_weights.manufacturability * 100.0);
            println!("   └─ Sostenibilidad:  {:.0}%", self.active_context.priority_weights.sustainability * 100.0);
            println!("\n   ESTÉTICA:");
            println!("   ├─ Simetría:        {:.0}%", self.active_context.aesthetic_rules.symmetry_requirement * 100.0);
            println!("   ├─ Fibonacci:       {:.0}%", self.active_context.aesthetic_rules.golden_ratio_enforcement * 100.0);
            println!("   └─ Suavidad:        {:.0}%", self.active_context.aesthetic_rules.surface_smoothness * 100.0);
            println!("════════════════════════════════════════\n");

            Ok(())
        } else {
            Err(format!("Contexto '{}' no encontrado", context_name))
        }
    }

    /// Infiere contexto desde un prompt natural
    pub fn infer_context_from_prompt(&mut self, prompt: &str) -> String {
        let prompt_lower = prompt.to_lowercase();

        // Palabras clave para cada contexto
        let keywords = vec![
            ("FANTASY_CREATIVE", vec!["fantasy", "magic", "creative", "artistic", "imaginative"]),
            ("CONCEPTUAL_ART", vec!["concept", "art", "experimental", "abstract", "avant-garde"]),
            ("RACING_PERFORMANCE", vec!["race", "speed", "fast", "aerodynamic", "performance", "racing"]),
            ("AEROSPACE_EXTREME", vec!["space", "rocket", "satellite", "aerospace", "orbital"]),
            ("HIGH_DESIGN_ARCHITECTURE", vec!["beautiful", "elegant", "stunning", "aesthetic", "masterpiece"]),
            ("ENGINEERING_REALISTIC", vec!["build", "construct", "engineer", "structural", "safe", "real"]),
        ];

        for (context_name, keys) in keywords {
            if keys.iter().any(|&keyword| prompt_lower.contains(keyword)) {
                let _ = self.set_context(context_name);
                return context_name.to_string();
            }
        }

        // Default
        let _ = self.set_context("ENGINEERING_REALISTIC");
        "ENGINEERING_REALISTIC".to_string()
    }

    pub fn get_active_context(&self) -> &DesignContext {
        &self.active_context
    }

    pub fn list_contexts(&self) -> Vec<String> {
        self.context_library.keys().cloned().collect()
    }

    fn default_context() -> DesignContext {
        DesignContext {
            name: "Default".to_string(),
            description: "Default context".to_string(),
            reality_profile: "EARTH_REALITY".to_string(),
            priority_weights: PriorityWeights {
                structural_integrity: 1.0,
                economic_efficiency: 0.0,
                aerodynamic_performance: 0.0,
                aesthetic_harmony: 0.0,
                innovation_factor: 0.0,
                manufacturability: 0.0,
                sustainability: 0.0,
            },
            constraints: ContextConstraints {
                hard_physics: true,
                budget_limit: None,
                time_limit: None,
                certifications: vec![],
                failure_tolerance: 0.0,
            },
            aesthetic_rules: AestheticRules {
                symmetry_requirement: 0.0,
                symmetry_type: SymmetryType::None,
                golden_ratio_enforcement: 0.0,
                visual_complexity_range: (0.0, 1.0),
                surface_smoothness: 0.0,
                rhythmic_patterns: vec![],
            },
        }
    }
}
