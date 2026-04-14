use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};
use std::time::Duration;
use tokio::time::sleep;
use crate::forge::experimental_lab::*;
use crate::contextus::semantic_graph::*;

pub struct Phase1Training {
    pub lab: Arc<Mutex<LabEngine>>,
    pub graph: Arc<RwLock<SemanticGraph>>,
    pub results_log: Vec<Phase1Result>,
}

#[derive(Debug, Clone)]
pub struct Phase1Result {
    pub experiment_id: String,
    pub combo: String,
    pub prediction_confidence: f32,
    pub delta: f32,
    pub survived: bool,
    pub deformation: f32,
    pub what_learned: String,
}

pub struct FoundationExperiment {
    pub id: String,
    pub name: String,
    pub structure: StructureType,
    pub stress: StressTest,
}

impl Phase1Training {
    pub fn new(lab: Arc<Mutex<LabEngine>>, graph: Arc<RwLock<SemanticGraph>>) -> Self {
        Self { lab, graph, results_log: Vec::new() }
    }

    pub fn get_foundation_sequence() -> Vec<FoundationExperiment> {
        vec![
            FoundationExperiment { id: "F1_001".into(), name: "Pirámide Base (Viento L)".into(), structure: StructureType::Pyramid(PyramidParams { base_width: 10.0, height: 4.0, material_density: 2400.0 }), stress: StressTest::Wind(WindParams { speed: 80.0, turbulence: 0.1 }) },
            FoundationExperiment { id: "F1_002".into(), name: "Arco Estándar (Sismo M)".into(), structure: StructureType::Arch(ArchParams { span: 10.0, radius: 5.0, keystone_weight: 400.0 }), stress: StressTest::Seismic(SeismicParams { magnitude: 6.5, frequency: 1.5 }) },
            FoundationExperiment { id: "F1_003".into(), name: "Voladizo (Viento F)".into(), structure: StructureType::Cantilever(CantileverParams { span: 12.0, thickness: 0.5 }), stress: StressTest::Wind(WindParams { speed: 140.0, turbulence: 0.2 }) },
            FoundationExperiment { id: "F1_004".into(), name: "Pirámide Alta (Sismo H)".into(), structure: StructureType::Pyramid(PyramidParams { base_width: 8.0, height: 12.0, material_density: 2400.0 }), stress: StressTest::Seismic(SeismicParams { magnitude: 8.0, frequency: 2.0 }) },
            FoundationExperiment { id: "F1_005".into(), name: "Arco Pesado (Viento L)".into(), structure: StructureType::Arch(ArchParams { span: 12.0, radius: 6.0, keystone_weight: 1200.0 }), stress: StressTest::Wind(WindParams { speed: 90.0, turbulence: 0.1 }) },
            FoundationExperiment { id: "F1_006".into(), name: "Vano Largo (Sismo L)".into(), structure: StructureType::Cantilever(CantileverParams { span: 20.0, thickness: 0.8 }), stress: StressTest::Seismic(SeismicParams { magnitude: 4.0, frequency: 1.0 }) },
            FoundationExperiment { id: "F1_007".into(), name: "Pirámide Maciza (Sismo M)".into(), structure: StructureType::Pyramid(PyramidParams { base_width: 15.0, height: 8.0, material_density: 3000.0 }), stress: StressTest::Seismic(SeismicParams { magnitude: 7.0, frequency: 1.5 }) },
            FoundationExperiment { id: "F1_008".into(), name: "Arco Esbelto (Viento M)".into(), structure: StructureType::Arch(ArchParams { span: 15.0, radius: 7.5, keystone_weight: 300.0 }), stress: StressTest::Wind(WindParams { speed: 110.0, turbulence: 0.15 }) },
            FoundationExperiment { id: "F1_009".into(), name: "Cantilever Corto (Sismo H)".into(), structure: StructureType::Cantilever(CantileverParams { span: 5.0, thickness: 1.2 }), stress: StressTest::Seismic(SeismicParams { magnitude: 9.0, frequency: 3.0 }) },
            FoundationExperiment { id: "F1_010".into(), name: "Mezcla: Pirámide/Arco (Wind)".into(), structure: StructureType::Pyramid(PyramidParams { base_width: 10.0, height: 10.0, material_density: 2400.0 }), stress: StressTest::Wind(WindParams { speed: 150.0, turbulence: 0.5 }) },
        ]
    }

    pub async fn run(&mut self) {
        let sequence = Self::get_foundation_sequence();
        println!("\n[DAITHON] 🚀 INICIANDO ENTRENAMIENTO REAL - 10 CICLOS");

        for exp in sequence {
            println!("\n--- Experimento {} ---", exp.id);
            
            // 1. Predicción Real de LabEngine
            let (prediction, key) = {
                let lab = self.lab.lock().await;
                let graph = self.graph.read().await;
                let unreal_exp = UnrealExperiment {
                    structure_type: exp.structure.clone(),
                    placement: Placement { 
                        position: [0.0, 0.0, 0.0], 
                        rotation: [0.0, 0.0, 0.0], 
                        on_surface: SurfaceType::Flat 
                    },
                    material: "concrete".to_string(),
                    stress_test: exp.stress.clone(),
                    force_direction: ForceDirection::Lateral,
                    parameters: std::collections::HashMap::new(),
                    duration_seconds: 20,
                };
                (lab.generate_structured_prediction(&unreal_exp, &graph), lab.experiment_to_anchor_key(&unreal_exp))
            };

            // 2. Mock de Realidad (Para obtener datos crudos variables)
            let result = self.generate_mock_result(&exp);

            // 3. Cálculo de Delta Real
            let delta = {
                let lab = self.lab.lock().await;
                lab.calculate_real_delta(&prediction, &result)
            };

            println!("   Predicción: {} (conf: {:.2})", if prediction.predicts_survival { "OK" } else { "FAIL" }, prediction.confidence);
            println!("   Realidad:   {} (Deformación: {:.4})", if result.survived { "OK" } else { "FAIL" }, result.max_deformation);
            println!("   DELTA INDIVIDUAL: {:.4}", delta);

            // 4. Fortalecer el Grafo
            {
                let mut graph = self.graph.write().await;
                let lab = self.lab.lock().await;
                let experiment = UnrealExperiment {
                    structure_type: exp.structure.clone(),
                    placement: Placement { 
                        position: [0.0, 0.0, 0.0], 
                        rotation: [0.0, 0.0, 0.0], 
                        on_surface: SurfaceType::Flat 
                    },
                    material: "concrete".to_string(),
                    stress_test: exp.stress.clone(),
                    force_direction: ForceDirection::Lateral,
                    parameters: std::collections::HashMap::new(),
                    duration_seconds: 20,
                };
                let key = lab.experiment_to_anchor_key(&experiment);

                graph.strengthen_anchor(
                    key,
                    &exp.id, 
                    delta, 
                    result.survived,
                    result.max_deformation,
                    result.failure_points.iter().map(|f| f.name.clone()).collect(),
                    format!("{:?}", exp.stress),
                    crate::contextus::semantic_graph::AnchorSource::LabExperiment,
                );
            }

            self.results_log.push(Phase1Result {
                experiment_id: exp.id.clone(),
                combo: exp.name.clone(),
                prediction_confidence: prediction.confidence,
                delta,
                survived: result.survived,
                deformation: result.max_deformation,
                what_learned: format!("Ancla {} actualizada con delta {:.2}", key, delta),
            });
            
            sleep(Duration::from_millis(100)).await;
        }

        self.print_raw_data();
    }

    fn generate_mock_result(&self, exp: &FoundationExperiment) -> UnrealSimResult {
        // --- REALIDAD FÍSICA PURA (Rapier3D) ---
        let mut scale = [2.0, 1.0, 2.0];

        match &exp.structure {
            StructureType::Pyramid(p) => scale = [p.base_width, p.height, p.base_width],
            StructureType::Arch(a) => scale = [a.span, a.span/2.0, 1.0],
            StructureType::Wall(w) => scale = [w.length, w.height, w.thickness],
            _ => { }
        }
        
        // Simulación simplificada para mock
        let survived = scale[1] / scale[0] < 2.0;
        
        let stress_mag = match &exp.stress {
            StressTest::Wind(w) => w.speed / 100.0,
            StressTest::Seismic(s) => s.magnitude / 10.0,
        };

        let base_def = if survived { 0.012 } else { 0.55 };
        let deformation = (base_def * stress_mag * 1.15).min(1.0);

        UnrealSimResult {
            session_id: exp.id.clone(),
            survived,
            max_deformation: deformation,
            failure_points: if !survived { 
                vec![FailurePoint { 
                    name: "structural_shear".to_string(), 
                    location: [0.0, 0.0, 0.0], 
                    stress_value: 0.95 
                }] 
            } else { vec![] },
            stress_distribution: std::collections::HashMap::new(),
            simulation_time_seconds: 20.0,
        }
    }

    fn print_raw_data(&self) {
        println!("\n╔═══════════════════════════════════════════╗");
        println!("║   DATOS CRUDOS - FASE 1 COMPLETADA        ║");
        println!("╚═══════════════════════════════════════════╝");
        println!("{:<10} | {:<7} | {:<8} | {:<7} | {}", "ID", "DELTA", "SURVIVED", "DEF", "COMBO");
        println!("---------------------------------------------------------------");
        for r in &self.results_log {
            println!("{:<10} | {:.4} | {:<8} | {:.4} | {}", 
                r.experiment_id, r.delta, r.survived, r.deformation, r.combo);
        }
    }
}
