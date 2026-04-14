// nexus/crucible.rs
// CRUCIBLE — Motor de ambientes hostiles para entrenamiento extremo
//
// Simula condiciones extremas: gravedad lunar, huracanes cat-5, terremotos 8.0,
// presión de fosa oceánica, impacto vehicular, etc.

use crate::sofia::primitives::*;
use crate::sofia::universal_validator::*;
use serde::{Deserialize, Serialize};

pub struct CRUCIBLE {
    scenarios: Vec<HostileScenario>,
    pub current_difficulty: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HostileScenario {
    pub name: String,
    pub description: String,
    pub environment: EnvironmentConfig,
    pub stress_tests: Vec<StressTest>,
    pub success_criteria: SuccessCriteria,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvironmentConfig {
    pub gravity: [f32; 3],
    pub atmosphere_pressure: f32,
    pub wind_speed: f32,
    pub wind_direction: [f32; 3],
    pub temperature: f32,
    pub seismic_activity: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StressTest {
    pub test_type: TestType,
    pub duration_seconds: f32,
    pub intensity: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TestType {
    StaticLoad { kg: f32, position: [f32; 3] },
    DynamicLoad { kg_per_second: f32, oscillation_hz: f32 },
    ImpactTest { energy_joules: f32, impact_point: [f32; 3] },
    TorsionStress { torque_nm: f32 },
    ThermalCycle { min_temp: f32, max_temp: f32, cycles: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessCriteria {
    pub min_survival_time: f32,
    pub max_deformation: f32,
    pub max_stress_points: f32,
    pub must_remain_functional: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub test_type: String,
    pub survival_time: f32,
    pub max_deformation: f32,
    pub max_stress: f32,
    pub passed: bool,
    pub failure_mode: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StressTestResult {
    pub scenario_name: String,
    pub passed: bool,
    pub total_score: f32,
    pub individual_results: Vec<TestResult>,
    pub lessons_learned: Vec<String>,
}

impl CRUCIBLE {
    pub fn new() -> Self {
        let mut crucible = Self {
            scenarios: Vec::new(),
            current_difficulty: 1.0,
        };
        crucible.initialize_scenarios();
        crucible
    }

    fn initialize_scenarios(&mut self) {
        // Luna (baja gravedad)
        self.scenarios.push(HostileScenario {
            name: "Lunar Base Furniture".into(),
            description: "Muebles para estación lunar - gravedad 1/6".into(),
            environment: EnvironmentConfig {
                gravity: [0.0, -1.62, 0.0],
                atmosphere_pressure: 0.0,
                wind_speed: 0.0,
                wind_direction: [0.0; 3],
                temperature: -50.0,
                seismic_activity: 0.0,
            },
            stress_tests: vec![
                StressTest { test_type: TestType::StaticLoad { kg: 100.0, position: [0.0, 1.0, 0.0] }, duration_seconds: 60.0, intensity: 0.7 },
                StressTest { test_type: TestType::ImpactTest { energy_joules: 50.0, impact_point: [0.0, 0.5, 0.0] }, duration_seconds: 1.0, intensity: 0.5 },
            ],
            success_criteria: SuccessCriteria { min_survival_time: 60.0, max_deformation: 0.05, max_stress_points: 100.0, must_remain_functional: true },
        });

        // Júpiter (alta gravedad 2.5x)
        self.scenarios.push(HostileScenario {
            name: "Jupiter Station Structure".into(),
            description: "Estructura orbital Júpiter - gravedad 2.5x".into(),
            environment: EnvironmentConfig {
                gravity: [0.0, -24.79, 0.0],
                atmosphere_pressure: 0.0,
                wind_speed: 0.0,
                wind_direction: [0.0; 3],
                temperature: -150.0,
                seismic_activity: 0.0,
            },
            stress_tests: vec![
                StressTest { test_type: TestType::StaticLoad { kg: 500.0, position: [0.0, 2.0, 0.0] }, duration_seconds: 120.0, intensity: 1.0 },
            ],
            success_criteria: SuccessCriteria { min_survival_time: 120.0, max_deformation: 0.02, max_stress_points: 500.0, must_remain_functional: true },
        });

        // Huracán Cat-5
        self.scenarios.push(HostileScenario {
            name: "Hurricane-Proof Architecture".into(),
            description: "Resistente a huracanes categoría 5 (305 km/h)".into(),
            environment: EnvironmentConfig {
                gravity: [0.0, -9.81, 0.0],
                atmosphere_pressure: 101325.0,
                wind_speed: 85.0,
                wind_direction: [1.0, 0.0, 0.0],
                temperature: 25.0,
                seismic_activity: 0.0,
            },
            stress_tests: vec![
                StressTest { test_type: TestType::DynamicLoad { kg_per_second: 1000.0, oscillation_hz: 2.0 }, duration_seconds: 300.0, intensity: 0.9 },
                StressTest { test_type: TestType::ImpactTest { energy_joules: 5000.0, impact_point: [5.0, 10.0, 0.0] }, duration_seconds: 1.0, intensity: 0.8 },
            ],
            success_criteria: SuccessCriteria { min_survival_time: 300.0, max_deformation: 0.5, max_stress_points: 1000.0, must_remain_functional: true },
        });

        // Terremoto 8.0
        self.scenarios.push(HostileScenario {
            name: "Earthquake Resilience".into(),
            description: "Resistente a terremoto magnitud 8.0".into(),
            environment: EnvironmentConfig {
                gravity: [0.0, -9.81, 0.0],
                atmosphere_pressure: 101325.0,
                wind_speed: 0.0,
                wind_direction: [0.0; 3],
                temperature: 20.0,
                seismic_activity: 8.0,
            },
            stress_tests: vec![
                StressTest { test_type: TestType::DynamicLoad { kg_per_second: 5000.0, oscillation_hz: 5.0 }, duration_seconds: 60.0, intensity: 1.0 },
            ],
            success_criteria: SuccessCriteria { min_survival_time: 60.0, max_deformation: 0.3, max_stress_points: 800.0, must_remain_functional: false },
        });

        // Fosa de las Marianas
        self.scenarios.push(HostileScenario {
            name: "Deep Sea Pressure".into(),
            description: "Cápsula para Fosa Marianas (11,000m)".into(),
            environment: EnvironmentConfig {
                gravity: [0.0, -9.81, 0.0],
                atmosphere_pressure: 110_000_000.0,
                wind_speed: 0.0,
                wind_direction: [0.0; 3],
                temperature: 2.0,
                seismic_activity: 0.0,
            },
            stress_tests: vec![
                StressTest { test_type: TestType::StaticLoad { kg: 0.0, position: [0.0; 3] }, duration_seconds: 3600.0, intensity: 1.0 },
            ],
            success_criteria: SuccessCriteria { min_survival_time: 3600.0, max_deformation: 0.01, max_stress_points: 2000.0, must_remain_functional: true },
        });

        // Crash test vehicular
        self.scenarios.push(HostileScenario {
            name: "Vehicle Crash Test".into(),
            description: "Impacto frontal a 50 km/h".into(),
            environment: EnvironmentConfig {
                gravity: [0.0, -9.81, 0.0],
                atmosphere_pressure: 101325.0,
                wind_speed: 0.0,
                wind_direction: [0.0; 3],
                temperature: 20.0,
                seismic_activity: 0.0,
            },
            stress_tests: vec![
                StressTest { test_type: TestType::ImpactTest { energy_joules: 150_000.0, impact_point: [0.0, 0.5, 2.0] }, duration_seconds: 0.5, intensity: 1.0 },
            ],
            success_criteria: SuccessCriteria { min_survival_time: 1.0, max_deformation: 0.8, max_stress_points: 1500.0, must_remain_functional: false },
        });
    }

    /// Ejecuta un stress test contra un diseño
    pub fn run_stress_test(&self, design: &UniversalDesign, scenario_name: &str) -> StressTestResult {
        let scenario = match self.scenarios.iter().find(|s| s.name == scenario_name) {
            Some(s) => s,
            None => {
                return StressTestResult {
                    scenario_name: scenario_name.into(),
                    passed: false,
                    total_score: 0.0,
                    individual_results: vec![],
                    lessons_learned: vec![format!("❌ Escenario '{}' no encontrado", scenario_name)],
                };
            }
        };

        log::info!("🔥 CRUCIBLE: {} — {}", scenario.name, scenario.description);

        let mut results = Vec::new();
        let mut total_score = 1.0_f32;

        for test in &scenario.stress_tests {
            let result = self.simulate_test(design, test, &scenario.environment);

            if !result.passed {
                total_score *= 0.5;
            }
            results.push(result);
        }

        let passed_all = results.iter().all(|r| r.passed);

        log::info!("🔥 CRUCIBLE resultado: {} (score: {:.2})",
            if passed_all { "✅ APROBADO" } else { "❌ FALLIDO" },
            total_score
        );

        StressTestResult {
            scenario_name: scenario.name.clone(),
            passed: passed_all,
            total_score,
            individual_results: results.clone(),
            lessons_learned: self.extract_lessons(&results),
        }
    }

    /// Ejecuta stress test "rápido" que devuelve un multiplicador de fitness (0.0 - 1.5)
    pub fn quick_stress_modifier(&self, design: &UniversalDesign) -> f32 {
        let base_strength = self.calculate_design_strength(design);

        // Escenario rápido: terremoto leve + viento moderado
        let env_stress = 0.5; // Estrés ambiental medio
        let resilience = (base_strength / env_stress).clamp(0.3, 1.5);

        resilience
    }

    fn simulate_test(&self, design: &UniversalDesign, test: &StressTest, environment: &EnvironmentConfig) -> TestResult {
        let base_strength = self.calculate_design_strength(design);
        let env_modifier = self.calculate_environmental_impact(environment);
        let effective_strength = base_strength * env_modifier;

        let survival_time = if effective_strength > test.intensity {
            test.duration_seconds
        } else {
            test.duration_seconds * (effective_strength / test.intensity)
        };

        let max_deformation = match &test.test_type {
            TestType::StaticLoad { kg, .. } => kg / (base_strength * 1000.0),
            TestType::DynamicLoad { kg_per_second, .. } => kg_per_second * test.intensity / (base_strength * 500.0),
            TestType::ImpactTest { energy_joules, .. } => energy_joules / (base_strength * 100_000.0),
            _ => 0.01,
        };

        let passed = survival_time >= test.duration_seconds && max_deformation <= 0.5;

        TestResult {
            test_type: format!("{:?}", test.test_type),
            survival_time,
            max_deformation,
            max_stress: base_strength * test.intensity * 100.0,
            passed,
            failure_mode: if !passed { Some("Deformación excesiva".into()) } else { None },
        }
    }

    fn calculate_design_strength(&self, design: &UniversalDesign) -> f32 {
        let support_count = design.primitives.iter()
            .filter(|p| matches!(p.primitive_type, FunctionalPrimitive::Support | FunctionalPrimitive::Span))
            .count();

        let base = (support_count as f32 * 0.2).min(1.0);
        let symmetry_bonus = if support_count >= 4 { 0.2 } else { 0.0 };
        let size_bonus = (design.bounding_box.width * design.bounding_box.depth * 0.05).min(0.3);

        (base + symmetry_bonus + size_bonus).max(0.1)
    }

    fn calculate_environmental_impact(&self, env: &EnvironmentConfig) -> f32 {
        let gravity_factor = (env.gravity[1].abs() / 9.81).clamp(0.5, 2.0);
        let pressure_factor = (env.atmosphere_pressure / 101325.0).clamp(0.1, 2.0);
        let wind_factor = 1.0 - (env.wind_speed / 100.0).min(0.5);
        (gravity_factor * pressure_factor * wind_factor).max(0.1)
    }

    fn extract_lessons(&self, results: &[TestResult]) -> Vec<String> {
        let mut lessons = Vec::new();
        for r in results {
            if !r.passed {
                if r.max_deformation > 0.3 { lessons.push("⚠️ Demasiado flexible - añadir refuerzos".into()); }
                if r.max_stress > 1000.0 { lessons.push("⚠️ Estrés excesivo - redistribuir carga".into()); }
            }
        }
        if lessons.is_empty() { lessons.push("✅ Diseño robusto".into()); }
        lessons
    }

    pub fn list_scenarios(&self) -> Vec<String> {
        self.scenarios.iter().map(|s| format!("{}: {}", s.name, s.description)).collect()
    }
}
