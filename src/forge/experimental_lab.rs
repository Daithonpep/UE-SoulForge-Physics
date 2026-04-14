use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::contextus::semantic_graph::SemanticGraph;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnrealSimResult {
    pub session_id: String,
    pub survived: bool,
    pub max_deformation: f32,
    pub failure_points: Vec<FailurePoint>,
    pub stress_distribution: HashMap<String, f32>,
    pub simulation_time_seconds: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FailurePoint {
    pub name: String,
    pub location: [f32; 3],
    pub stress_value: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuralPrediction {
    pub predicts_survival: bool,
    pub expected_deformation: f32,
    pub expected_failure_points: Vec<String>,
    pub confidence: f32,
    pub reasoning: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuralHypothesis {
    pub claim: String,
    pub target_variable: String,
    pub expected_outcome: f32,
    pub related_concept: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnrealExperiment {
    pub structure_type: StructureType,
    pub placement: Placement,
    pub material: String,
    pub stress_test: StressTest,
    pub force_direction: ForceDirection,
    pub parameters: HashMap<String, f32>, 
    pub duration_seconds: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StructureType {
    Pyramid(PyramidParams),
    Arch(ArchParams),
    Cantilever(CantileverParams),
    Wall(WallParams),
    Foundation(FoundationParams),
    House(HouseParams),
    FreeForm(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HouseParams {
    pub num_rooms: u32,
    pub floor_area: f32,
    pub has_foundation: bool,
    pub wall_thickness: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WallParams {
    pub height: f32,
    pub thickness: f32,
    pub length: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FoundationParams {
    pub depth: f32,
    pub area: f32,
    pub soil_load_bearing: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PyramidParams {
    pub base_width: f32,
    pub height: f32,
    pub material_density: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArchParams {
    pub span: f32,
    pub radius: f32,
    pub keystone_weight: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CantileverParams {
    pub span: f32,
    pub thickness: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StressTest {
    Seismic(SeismicParams),
    Wind(WindParams),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeismicParams {
    pub magnitude: f32,
    pub frequency: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WindParams {
    pub speed: f32,
    pub turbulence: f32,
}

pub struct LabEngine {
    pub active_sessions: Vec<LabSession>,
}

#[derive(Clone)]
pub struct LabSession {
    pub hypothesis_id: String,
    pub hypothesis: Option<StructuralHypothesis>,
    pub experiment: UnrealExperiment,
    pub prediction: StructuralPrediction,
    pub result: Option<UnrealSimResult>,
    pub accuracy_delta: Option<f32>,
}

impl LabEngine {
    pub fn new() -> Self {
        Self { active_sessions: Vec::new() }
    }

    pub fn experiment_to_anchor_key(&self, experiment: &UnrealExperiment) -> String {
        match (&experiment.structure_type, &experiment.stress_test) {
            (StructureType::Arch(_), StressTest::Seismic(_)) => "arch_seismic".into(),
            (StructureType::Arch(_), StressTest::Wind(_)) => "arch_wind".into(),
            (StructureType::Pyramid(_), StressTest::Seismic(_)) => "pyramid_seismic".into(),
            (StructureType::Cantilever(_), StressTest::Wind(_)) => "cantilever_wind".into(),
            (StructureType::Wall(_), _) => "wall_stability".into(),
            (StructureType::Foundation(_), _) => "foundation_load".into(),
            (StructureType::House(_), _) => "house_integrity".into(),
            _ => "generic_experiment".into(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Placement {
    pub position: [f64; 3],
    pub rotation: [f64; 3],    // Grados en X, Y, Z
    pub on_surface: SurfaceType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SurfaceType {
    Flat,
    Inclined(f64),     // Ángulo de la pendiente
    Uneven(f64),       // Factor de irregularidad 0-1
    Elevated(f64),     // Altura sobre el suelo
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ForceDirection {
    Lateral,           // Desde un lado
    Frontal,           // De frente
    Diagonal(f64),     // Ángulo en grados
    FromBelow,         // Desde abajo (sismo vertical)
    Downward(f64),     // Carga adicional desde arriba
    Rotational(f64),   // Torsión
}

impl LabEngine {

    pub fn generate_structured_prediction(
        &self, 
        experiment: &UnrealExperiment, 
        graph: &SemanticGraph
    ) -> StructuralPrediction {
        let key = self.experiment_to_anchor_key(experiment);
        
        if let Some(anchor) = graph.empirical_anchors.get(&key) {
            return StructuralPrediction {
                predicts_survival: anchor.survival_rate > 0.5,
                expected_deformation: anchor.avg_deformation,
                expected_failure_points: anchor.known_failure_points.clone(),
                confidence: anchor.confidence,
                reasoning: format!(
                    "{} experimentos previos. Tasa de supervivencia: {:.0}%.",
                    anchor.reproduction_count, anchor.survival_rate * 100.0
                ),
            };
        }

        match &experiment.structure_type {
            StructureType::Arch(p) => {
                let stress = match &experiment.stress_test {
                    StressTest::Seismic(s) => s.magnitude / 10.0,
                    StressTest::Wind(w) => w.speed / 200.0,
                };
                StructuralPrediction {
                    predicts_survival: p.keystone_weight * stress < 300.0,
                    expected_deformation: p.keystone_weight * 0.0002,
                    expected_failure_points: vec!["keystone".into()],
                    confidence: 0.35,
                    reasoning: "Modelo de carga crítica para arcos.".into(),
                }
            },
            StructureType::Pyramid(p) => {
                let ratio = p.height / p.base_width;
                StructuralPrediction {
                    predicts_survival: ratio < 1.2,
                    expected_deformation: ratio * 0.05,
                    expected_failure_points: vec![],
                    confidence: 0.3,
                    reasoning: "Estabilidad geométrica básica.".into(),
                }
            },
            StructureType::Cantilever(p) => {
                let stress = match &experiment.stress_test {
                    StressTest::Seismic(s) => s.magnitude / 10.0,
                    StressTest::Wind(w) => w.speed / 200.0,
                };
                let limit = p.span * p.thickness * 0.003;
                StructuralPrediction {
                    predicts_survival: stress < limit,
                    expected_deformation: stress * p.span * 0.01,
                    expected_failure_points: vec!["root".into()],
                    confidence: 0.4,
                    reasoning: "Carga crítica corregida post-F1.".into(),
                }
            },
            StructureType::Wall(p) => {
                let load = match &experiment.stress_test {
                    StressTest::Seismic(s) => s.magnitude * p.height * 0.5,
                    StressTest::Wind(w) => (w.speed / 10.0).powi(2) * p.height * 0.1,
                };
                let resistance = p.thickness * 100.0;
                StructuralPrediction {
                    predicts_survival: load < resistance,
                    expected_deformation: load / resistance * 0.05,
                    expected_failure_points: vec!["base_shear".into()],
                    confidence: 0.5,
                    reasoning: "Análisis de estabilidad de muro esbelto.".into(),
                }
            },
            StructureType::Foundation(p) => {
                let total_load = p.area * 500.0; // Carga simulada
                let limit = p.area * p.soil_load_bearing * (1.0 + p.depth * 0.2);
                StructuralPrediction {
                    predicts_survival: total_load < limit,
                    expected_deformation: total_load / limit * 0.01,
                    expected_failure_points: vec!["subsidence".into()],
                    confidence: 0.6,
                    reasoning: "Carga de asentamiento de cimentación.".into(),
                }
            },
            StructureType::House(p) => {
                let load = p.num_rooms as f32 * p.floor_area * 10.0;
                let resistance = if p.has_foundation { 1.2 } else { 0.5 } * p.wall_thickness * 1000.0;
                StructuralPrediction {
                    predicts_survival: load < resistance,
                    expected_deformation: load / resistance * 0.08,
                    expected_failure_points: if !p.has_foundation { vec!["foundation_failure".into()] } else { vec!["roof_collapse".into()] },
                    confidence: 0.45,
                    reasoning: "Integridad estructural de vivienda básica.".into(),
                }
            },
            _ => StructuralPrediction {
                predicts_survival: true,
                expected_deformation: 0.01,
                expected_failure_points: vec![],
                confidence: 0.1,
                reasoning: "Default".into(),
            }
        }
    }

    pub fn calculate_real_delta(&self, prediction: &StructuralPrediction, result: &UnrealSimResult) -> f32 {
        let mut score = 1.0;

        // 1. Acierto en supervivencia (Penalización mayor)
        if prediction.predicts_survival != result.survived {
            score -= 0.5;
        }

        // 2. Error en deformación (Normalizado)
        let def_error = (prediction.expected_deformation - result.max_deformation).abs();
        let def_penalty = (def_error * 10.0).min(0.5); // Tope de penalización por deformación
        score -= def_penalty;

        score.max(0.0)
    }

    pub fn process_result(
        &mut self,
        session_idx: usize,
        unreal_res: UnrealSimResult,
        graph: &mut SemanticGraph,
        source: crate::contextus::semantic_graph::AnchorSource,
    ) {
        let (prediction, hypothesis_id, experiment) = if let Some(session) = self.active_sessions.get_mut(session_idx) {
            let p = session.prediction.clone();
            let h = session.hypothesis_id.clone();
            let e = session.experiment.clone();
            session.result = Some(unreal_res.clone());
            (p, h, e)
        } else {
            return;
        };

        let accuracy = self.calculate_real_delta(&prediction, &unreal_res);
        let key = self.experiment_to_anchor_key(&experiment);

        println!("[LAB] Resultado recibido: session {}", hypothesis_id);
        println!("      Sobrevivió: {}, Deformación: {:.4}", unreal_res.survived, unreal_res.max_deformation);
        println!("      Delta calculado (Precisión): {:.2}", accuracy);

        if let Some(session) = self.active_sessions.get_mut(session_idx) {
            session.accuracy_delta = Some(accuracy);
        }

        graph.strengthen_anchor(
            key,
            &hypothesis_id,
            accuracy,
            unreal_res.survived,
            unreal_res.max_deformation,
            unreal_res.failure_points.iter().map(|f| f.name.clone()).collect(),
            format!("{:?}", experiment.stress_test),
            source,
        );
    }

    pub fn to_unreal_command(&self, session: &LabSession) -> serde_json::Value {
        serde_json::json!({
            "action": "run_physics_sim",
            "session_id": session.hypothesis_id,
            "experiment": session.experiment,
            "prediction": session.prediction,
            "hypothesis": session.hypothesis,
        })
    }
}
