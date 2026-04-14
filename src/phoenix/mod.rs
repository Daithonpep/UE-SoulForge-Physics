// PHOENIX - PHysics Engine for Optimal Nexus Intelligence eXecution
//
// Motor de física multi-universo con:
// - REALITY Engine: Perfiles físicos configurables (Tierra, Luna, Marte, Espacio, etc.)
// - CFD Engine: Dinámica de fluidos computacional simplificada
// - Material Library: Propiedades reales de ingeniería
//
// Se integra con SOFIA (validación funcional) y NEXUS (entrenamiento híbrido)

pub mod reality_profiles;
pub mod cfd_engine;

use reality_profiles::REALITYEngine;
use cfd_engine::CFDEngine;

pub struct PHOENIX {
    pub reality: REALITYEngine,
    pub cfd: CFDEngine,
}

impl PHOENIX {
    pub fn new() -> Self {
        log::info!("🔥 PHOENIX ENGINE INITIALIZED");
        log::info!("  🌍 REALITY: Multi-universe physics profiles");
        log::info!("  💨 CFD: Computational Fluid Dynamics");

        Self {
            reality: REALITYEngine::new(),
            cfd: CFDEngine::new(32),
        }
    }

    /// Análisis completo de un diseño en el perfil activo
    pub fn full_analysis(
        &self,
        design: &crate::sofia::universal_validator::UniversalDesign,
    ) -> PhoenixAnalysisReport {
        let profile = self.reality.get_active_profile();

        // 1. Análisis aerodinámico (si hay viento/atmósfera)
        let aero = if profile.physics_laws.air_density > 0.001 {
            Some(self.cfd.simulate_aerodynamics(
                design,
                30.0, // velocidad de prueba estándar
                profile.physics_laws.air_density,
                profile.physics_laws.air_viscosity,
            ))
        } else {
            None
        };

        // 2. Análisis estructural basado en materiales
        let structural = self.analyze_structural_integrity(design, profile);

        // 3. Fitness modifier basado en física real
        let physics_fitness_modifier = self.calculate_physics_modifier(&aero, &structural, profile);

        PhoenixAnalysisReport {
            profile_name: profile.name.clone(),
            aerodynamic: aero,
            structural,
            physics_fitness_modifier,
        }
    }

    /// Modificador rápido de fitness (para usar dentro del loop evolutivo)
    pub fn quick_physics_modifier(
        &self,
        design: &crate::sofia::universal_validator::UniversalDesign,
    ) -> f32 {
        let profile = self.reality.get_active_profile();
        let gravity_factor = profile.physics_laws.gravity[1].abs() / 9.81;

        // Más soportes = más resistente a mayor gravedad
        let support_count = design.primitives.iter()
            .filter(|p| matches!(p.primitive_type,
                crate::sofia::primitives::FunctionalPrimitive::Support |
                crate::sofia::primitives::FunctionalPrimitive::Span
            ))
            .count() as f32;

        let structural_adequacy = (support_count * 0.25) / gravity_factor.max(0.1);

        // Penalizar exceso de material (eficiencia)
        let total_primitives = design.primitives.len() as f32;
        let efficiency = if total_primitives > 0.0 {
            (support_count / total_primitives).min(0.8)
        } else {
            0.0
        };

        (structural_adequacy * 0.7 + efficiency * 0.3).clamp(0.1, 1.5)
    }

    fn analyze_structural_integrity(
        &self,
        design: &crate::sofia::universal_validator::UniversalDesign,
        profile: &reality_profiles::PhysicsProfile,
    ) -> StructuralAnalysis {
        let gravity_mag = profile.physics_laws.gravity[1].abs();

        // Estimar peso del diseño
        let volume = design.bounding_box.width * design.bounding_box.height * design.bounding_box.depth;
        let default_density = 800.0; // Madera por defecto
        let estimated_weight = volume * default_density * gravity_mag;

        // Contar soportes y calcular carga por soporte
        let support_count = design.primitives.iter()
            .filter(|p| matches!(p.primitive_type,
                crate::sofia::primitives::FunctionalPrimitive::Support |
                crate::sofia::primitives::FunctionalPrimitive::Span
            ))
            .count() as f32;

        let load_per_support = if support_count > 0.0 {
            estimated_weight / support_count
        } else {
            estimated_weight
        };

        // Factor de seguridad (objetivo: >= 2.0)
        let material_strength = 40.0 * 1_000_000.0; // 40 MPa (madera)
        let support_area = 0.05 * 0.05; // 5cm × 5cm por soporte
        let stress = load_per_support / support_area;
        let safety_factor = material_strength / stress.max(0.001);

        let is_safe = safety_factor >= 2.0;

        StructuralAnalysis {
            estimated_weight_kg: estimated_weight / gravity_mag.max(0.001),
            load_per_support_n: load_per_support,
            safety_factor,
            is_structurally_safe: is_safe,
            failure_mode: if !is_safe {
                Some(format!("Factor seguridad {:.2} < 2.0 — riesgo de colapso", safety_factor))
            } else {
                None
            },
        }
    }

    fn calculate_physics_modifier(
        &self,
        aero: &Option<cfd_engine::AerodynamicAnalysis>,
        structural: &StructuralAnalysis,
        _profile: &reality_profiles::PhysicsProfile,
    ) -> f32 {
        let mut modifier = 1.0_f32;

        // Penalización estructural
        if !structural.is_structurally_safe {
            modifier *= 0.3;
        } else if structural.safety_factor > 4.0 {
            modifier *= 1.1; // Bonus por sobre-ingeniería
        }

        // Bonus/penalización aerodinámica
        if let Some(aero) = aero {
            if aero.drag_coefficient < 0.3 {
                modifier *= 1.15; // Muy aerodinámico
            } else if aero.drag_coefficient > 1.0 {
                modifier *= 0.8; // Muy poco aerodinámico
            }
        }

        modifier.clamp(0.1, 1.5)
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PhoenixAnalysisReport {
    pub profile_name: String,
    pub aerodynamic: Option<cfd_engine::AerodynamicAnalysis>,
    pub structural: StructuralAnalysis,
    pub physics_fitness_modifier: f32,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StructuralAnalysis {
    pub estimated_weight_kg: f32,
    pub load_per_support_n: f32,
    pub safety_factor: f32,
    pub is_structurally_safe: bool,
    pub failure_mode: Option<String>,
}
