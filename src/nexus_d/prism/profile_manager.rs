use crate::nexus_d::aura::aesthetic_reward::{AestheticProfile, AuraRewardSystem};
use crate::nexus_d::helix::balance_dynamics::DynamicConfig;
use std::collections::HashMap;

/// Configuración completa de un perfil de diseño
#[derive(Clone, Debug)]
pub struct DesignProfile {
    pub name: String,
    pub description: String,
    /// Perfil estético AURA
    pub aesthetic: AestheticProfile,
    /// Configuración de estabilidad HELIX
    pub stability_margin: f64,
    pub dynamic_config: DynamicConfig,
    /// Configuración de topología FLUX
    pub flux_resolution: usize,
    pub solidification_threshold: f64,
    pub fusion_proximity: f64,
    pub diffusion_rate: f64,
    pub propagation_iterations: usize,
    /// Pesos del fitness compuesto para GENESIS
    pub fitness_weights: FitnessWeights,
}

#[derive(Clone, Debug)]
pub struct FitnessWeights {
    /// Peso del score estructural (Phoenix + Seismo)
    pub structural: f64,
    /// Peso del score de equilibrio (Helix)
    pub balance: f64,
    /// Peso del score estético (AURA)
    pub aesthetic: f64,
    /// Peso del score funcional (SOFIA)
    pub functional: f64,
    /// Peso del score de manufacturabilidad
    pub manufacturability: f64,
}

pub struct PrismProfileManager {
    profiles: HashMap<String, DesignProfile>,
    active_profile: String,
}

impl PrismProfileManager {
    pub fn new() -> Self {
        let mut manager = Self {
            profiles: HashMap::new(),
            active_profile: String::new(),
        };

        // Registrar perfiles predefinidos
        manager.register(Self::profile_avant_garde());
        manager.register(Self::profile_futuristic());
        manager.register(Self::profile_sculptural());
        manager.register(Self::profile_industrial());
        manager.register(Self::profile_biomimetic());

        manager.active_profile = "futuristic".into();
        manager
    }

    pub fn register(&mut self, profile: DesignProfile) {
        self.profiles.insert(profile.name.clone(), profile);
    }

    pub fn activate(&mut self, name: &str) -> Result<(), String> {
        if self.profiles.contains_key(name) {
            self.active_profile = name.to_string();
            Ok(())
        } else {
            Err(format!(
                "Profile '{}' not found. Available: {:?}",
                name,
                self.profiles.keys().collect::<Vec<_>>()
            ))
        }
    }

    pub fn active(&self) -> &DesignProfile {
        self.profiles.get(&self.active_profile)
            .expect("Active profile must exist")
    }

    pub fn get_available_profiles(&self) -> Vec<String> {
        self.profiles.keys().cloned().collect()
    }

    pub fn compute_composite_fitness(
        &self,
        structural_score: f64,
        balance_score: f64,
        aesthetic_score: f64,
        functional_score: f64,
        manufacturability_score: f64,
    ) -> f64 {
        let w = &self.active().fitness_weights;

        // Hard constraints: si la estructura falla o no está en equilibrio,
        // el fitness es 0 sin importar qué tan bonito sea
        if structural_score < 0.3 || balance_score < 0.1 {
            return structural_score * 0.5 + balance_score * 0.5;
        }

        let weighted = structural_score * w.structural
            + balance_score * w.balance
            + aesthetic_score * w.aesthetic
            + functional_score * w.functional
            + manufacturability_score * w.manufacturability;

        // Normalizar
        let total_weight = w.structural + w.balance + w.aesthetic 
            + w.functional + w.manufacturability;

        if total_weight > 0.0 {
            weighted / total_weight
        } else {
            0.0
        }
    }

    // ─── Perfiles predefinidos ───

    fn profile_avant_garde() -> DesignProfile {
        DesignProfile {
            name: "avant_garde".into(),
            description: "Diseños arriesgados, mínimos puntos de contacto, formas orgánicas".into(),
            aesthetic: AuraRewardSystem::avant_garde_profile(),
            stability_margin: 0.8, // Menor margen aceptable
            dynamic_config: DynamicConfig {
                perturbation_test: true,
                max_lateral_force: 30.0,
                check_tipping: true,
                max_floor_slope: 1.0,
            },
            flux_resolution: 64,
            solidification_threshold: 0.30,
            fusion_proximity: 0.08,
            diffusion_rate: 0.4,
            propagation_iterations: 50,
            fitness_weights: FitnessWeights {
                structural: 0.20,
                balance: 0.15,
                aesthetic: 0.35,
                functional: 0.20,
                manufacturability: 0.10,
            },
        }
    }

    fn profile_futuristic() -> DesignProfile {
        DesignProfile {
            name: "futuristic".into(),
            description: "Balance entre innovación y funcionalidad, proporciones áureas".into(),
            aesthetic: AuraRewardSystem::futuristic_profile(),
            stability_margin: 1.2,
            dynamic_config: DynamicConfig {
                perturbation_test: true,
                max_lateral_force: 50.0,
                check_tipping: true,
                max_floor_slope: 2.0,
            },
            flux_resolution: 48,
            solidification_threshold: 0.35,
            fusion_proximity: 0.10,
            diffusion_rate: 0.3,
            propagation_iterations: 40,
            fitness_weights: FitnessWeights {
                structural: 0.25,
                balance: 0.20,
                aesthetic: 0.25,
                functional: 0.20,
                manufacturability: 0.10,
            },
        }
    }

    fn profile_sculptural() -> DesignProfile {
        DesignProfile {
            name: "sculptural".into(),
            description: "Formas escultóricas, superficies mínimas, arte funcional".into(),
            aesthetic: AuraRewardSystem::sculptural_profile(),
            stability_margin: 1.0,
            dynamic_config: DynamicConfig {
                perturbation_test: true,
                max_lateral_force: 40.0,
                check_tipping: true,
                max_floor_slope: 1.5,
            },
            flux_resolution: 72,
            solidification_threshold: 0.28,
            fusion_proximity: 0.06,
            diffusion_rate: 0.45,
            propagation_iterations: 60,
            fitness_weights: FitnessWeights {
                structural: 0.20,
                balance: 0.15,
                aesthetic: 0.40,
                functional: 0.15,
                manufacturability: 0.10,
            },
        }
    }

    fn profile_industrial() -> DesignProfile {
        DesignProfile {
            name: "industrial".into(),
            description: "Robusto, manufactura sencilla, eficiente".into(),
            aesthetic: AestheticProfile {
                name: "Industrial".into(),
                weights: crate::nexus_d::aura::aesthetic_reward::AestheticWeights {
                    golden_ratio: 0.10,
                    symmetry: 0.20,
                    curvature_flow: 0.05,
                    minimalism: 0.10,
                    visual_contrast: 0.10,
                    formal_coherence: 0.20,
                    novelty: 0.05,
                },
                preferences: crate::nexus_d::aura::aesthetic_reward::StylePreferences {
                    symmetry_type: crate::nexus_d::aura::aesthetic_reward::SymmetryPreference::Bilateral,
                    curvature_preference: crate::nexus_d::aura::aesthetic_reward::CurvaturePreference::Geometric,
                    min_contact_points: 4,
                    max_contact_points: 6,
                    preferred_aspect_ratio: 1.5,
                    fibonacci_bonus: false,
                    rewarded_motifs: Vec::new(),
                },
            },
            stability_margin: 2.0,
            dynamic_config: DynamicConfig {
                perturbation_test: true,
                max_lateral_force: 100.0,
                check_tipping: true,
                max_floor_slope: 5.0,
            },
            flux_resolution: 32,
            solidification_threshold: 0.45,
            fusion_proximity: 0.15,
            diffusion_rate: 0.2,
            propagation_iterations: 25,
            fitness_weights: FitnessWeights {
                structural: 0.35,
                balance: 0.25,
                aesthetic: 0.05,
                functional: 0.20,
                manufacturability: 0.15,
            },
        }
    }

    const PHI: f64 = 1.618033988749895;

    fn profile_biomimetic() -> DesignProfile {
        DesignProfile {
            name: "biomimetic".into(),
            description: "Inspirado en la naturaleza, estructuras tipo hueso/coral".into(),
            aesthetic: AestheticProfile {
                name: "Biomimetic".into(),
                weights: crate::nexus_d::aura::aesthetic_reward::AestheticWeights {
                    golden_ratio: 0.20,
                    symmetry: 0.10,
                    curvature_flow: 0.25,
                    minimalism: 0.15,
                    visual_contrast: 0.10,
                    formal_coherence: 0.10,
                    novelty: 0.10,
                },
                preferences: crate::nexus_d::aura::aesthetic_reward::StylePreferences {
                    symmetry_type: crate::nexus_d::aura::aesthetic_reward::SymmetryPreference::Radial { order: 5 },
                    curvature_preference: crate::nexus_d::aura::aesthetic_reward::CurvaturePreference::Organic,
                    min_contact_points: 3,
                    max_contact_points: 5,
                    preferred_aspect_ratio: Self::PHI,
                    fibonacci_bonus: true,
                    rewarded_motifs: vec![
                        crate::nexus_d::aura::aesthetic_reward::GeometricMotif::SpiralFibonacci,
                        crate::nexus_d::aura::aesthetic_reward::GeometricMotif::VoronoiPattern,
                    ],
                },
            },
            stability_margin: 1.5,
            dynamic_config: DynamicConfig {
                perturbation_test: true,
                max_lateral_force: 60.0,
                check_tipping: true,
                max_floor_slope: 3.0,
            },
            flux_resolution: 64,
            solidification_threshold: 0.30,
            fusion_proximity: 0.07,
            diffusion_rate: 0.5,
            propagation_iterations: 55,
            fitness_weights: FitnessWeights {
                structural: 0.20,
                balance: 0.15,
                aesthetic: 0.30,
                functional: 0.20,
                manufacturability: 0.15,
            },
        }
    }
}
