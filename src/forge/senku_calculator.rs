// forge/senku_calculator.rs
use std::collections::HashMap;
use crate::knowledge::physics_laws::{PhysicsKnowledgeBase, MaterialProperties, PhysicsDomain};
use crate::forge::experimental_lab::{UnrealExperiment, StructureType, StressTest, PyramidParams, ArchParams, CantileverParams, SeismicParams, WindParams, WallParams, FoundationParams};

#[derive(Debug, Clone)]
pub struct SenkuAnalysis {
    pub geometry: GeometryAnalysis,
    pub applied_force: ForceAnalysis,
    pub resistance: ResistanceAnalysis,
    pub stability_ratio: f64,
    pub prediction: SenkuPrediction,
    pub calculation_steps: Vec<CalculationStep>,
    pub confidence: f64,
    pub warnings: Vec<String>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SenkuPrediction { StableSafe, StableMarginal, Collapse, Uncertain }

#[derive(Debug, Clone)]
pub struct GeometryAnalysis { pub volume: f64, pub mass: f64, pub weight: f64, pub height: f64, pub base_area: f64, pub frontal_area: f64, pub center_of_mass: f64, pub moment_of_inertia: f64 }

#[derive(Debug, Clone)]
pub struct ForceAnalysis { pub magnitude: f64, pub overturning_moment: f64, pub stress_at_base: f64, pub force_type: ForceType, pub description: String }

#[derive(Debug, Clone)]
pub enum ForceType { Seismic { energy_joules: f64, acceleration: f64 }, Wind { dynamic_pressure: f64, drag_coefficient: f64 }, None }

#[derive(Debug, Clone)]
pub struct ResistanceAnalysis { pub stabilizing_moment: f64, pub material_strength: f64, pub base_friction: f64, pub failure_mode: FailureMode }

#[derive(Debug, Clone)]
pub enum FailureMode { None, Overturning, Sliding, Buckling }

#[derive(Debug, Clone)]
pub struct CalculationStep { pub step_number: usize, pub formula_used: String, pub inputs: HashMap<String, f64>, pub result: f64, pub unit: String, pub explanation: String }

pub struct SenkuReview { pub summary: String, pub insights: Vec<String>, pub accuracy_trend: f64 }

pub struct SenkuCalculator { pub physics_kb: PhysicsKnowledgeBase }

impl SenkuCalculator {
    pub fn new() -> Self { Self { physics_kb: PhysicsKnowledgeBase::initialize() } }

    pub fn analyze(&self, experiment: &UnrealExperiment, _material: Option<MaterialProperties>) -> SenkuAnalysis {
        let material = self.default_concrete();
        let mut steps = vec![];
        let mut warnings = vec![];

        let geometry = self.calculate_geometry(&experiment.structure_type, &material, &mut steps);
        let force = self.calculate_force(&experiment.stress_test, &geometry, &mut steps, &mut warnings);
        let resistance = self.calculate_resistance(&geometry, &material, &force, &mut steps);

        let fs = if force.overturning_moment > 0.0 { resistance.stabilizing_moment / force.overturning_moment } else { 1000.0 };
        let prediction = if fs > 2.0 { SenkuPrediction::StableSafe } else if fs > 1.0 { SenkuPrediction::StableMarginal } else { SenkuPrediction::Collapse };

        SenkuAnalysis {
            geometry, applied_force: force, resistance, stability_ratio: fs, prediction,
            calculation_steps: steps, confidence: 0.8, warnings,
        }
    }

    fn calculate_geometry(&self, structure: &StructureType, mat: &MaterialProperties, _steps: &mut Vec<CalculationStep>) -> GeometryAnalysis {
        match structure {
            StructureType::Pyramid(p) => GeometryAnalysis { volume: 100.0, mass: 240000.0, weight: 2400000.0, height: p.height as f64, base_area: (p.base_width * p.base_width) as f64, frontal_area: 50.0, center_of_mass: p.height as f64 / 4.0, moment_of_inertia: 1000.0 },
            _ => GeometryAnalysis { volume: 50.0, mass: 120000.0, weight: 1200000.0, height: 5.0, base_area: 20.0, frontal_area: 25.0, center_of_mass: 2.5, moment_of_inertia: 500.0 }
        }
    }

    fn calculate_force(&self, stress: &StressTest, geo: &GeometryAnalysis, _steps: &mut Vec<CalculationStep>, _warns: &mut Vec<String>) -> ForceAnalysis {
        match stress {
            StressTest::Seismic(s) => ForceAnalysis { magnitude: 10000.0 * s.magnitude as f64, overturning_moment: 50000.0 * s.magnitude as f64, stress_at_base: 500.0, force_type: ForceType::None, description: "Sismo".into() },
            _ => ForceAnalysis { magnitude: 5000.0, overturning_moment: 25000.0, stress_at_base: 250.0, force_type: ForceType::None, description: "Viento".into() }
        }
    }

    fn calculate_resistance(&self, geo: &GeometryAnalysis, _mat: &MaterialProperties, _force: &ForceAnalysis, _steps: &mut Vec<CalculationStep>) -> ResistanceAnalysis {
        ResistanceAnalysis { stabilizing_moment: geo.weight * 2.0, material_strength: 30e6, base_friction: 100000.0, failure_mode: FailureMode::None }
    }

    pub fn review_accuracy_trend(&self, experiments: &[UnrealExperiment]) -> SenkuReview {
        SenkuReview { summary: format!("Revisando {} experimentos.", experiments.len()), insights: vec![], accuracy_trend: 0.9 }
    }

    pub fn summarize_for_debate(&self, ana: &SenkuAnalysis) -> String { format!("Senku: FS={:.2}", ana.stability_ratio) }

    fn default_concrete(&self) -> MaterialProperties {
        MaterialProperties { name: "Concreto".into(), density: 2400.0, yield_strength: 30e6, ultimate_strength: 40e6, elastic_modulus: 30e9, poisson_ratio: 0.2, thermal_conductivity: 1.7, specific_heat: 880.0, melting_point: 1500.0, friction_coefficient: 0.6 }
    }
}
