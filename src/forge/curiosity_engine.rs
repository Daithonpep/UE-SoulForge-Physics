use std::collections::{HashMap, HashSet};
use crate::contextus::semantic_graph::SemanticGraph;
use crate::forge::experimental_lab::{
    UnrealExperiment, StructureType, StressTest, PyramidParams, ArchParams, 
    WallParams, FoundationParams, SeismicParams, WindParams, Placement, SurfaceType, ForceDirection
};

pub struct CuriosityEngine {
    pub exploration_threshold: f32,
    pub novelty_drive: f32,
    pub explored_space: ExploredSpace,
}

pub struct ExploredSpace {
    pub keys_visited: HashSet<String>,
    pub results_cache: HashMap<String, Vec<bool>>,
}

impl CuriosityEngine {
    pub fn new() -> Self {
        Self {
            exploration_threshold: 0.7,
            novelty_drive: 0.5,
            explored_space: ExploredSpace {
                keys_visited: HashSet::new(),
                results_cache: HashMap::new(),
            },
        }
    }

    pub fn observe_result(&mut self, experiment: &UnrealExperiment, survived: bool) {
        let key = self.experiment_to_key(experiment);
        self.explored_space.keys_visited.insert(key.clone());
        self.explored_space.results_cache.entry(key).or_insert_with(Vec::new).push(survived);
    }

    fn experiment_to_key(&self, experiment: &UnrealExperiment) -> String {
        format!("{:?}_{:?}", experiment.structure_type, experiment.stress_test)
    }

    pub fn generate_curious_experiment(&self, graph: &SemanticGraph) -> Option<(UnrealExperiment, String)> {
        // Lógica de curiosidad: Ver áreas del grafo con pocos experimentos
        for (key, anchor) in &graph.empirical_anchors {
            if anchor.reproduction_count < 3 {
                println!("[CURIOSITY] Poca confianza en {}. Sugiriendo reproducción.", key);
                return Some((self.reproduce_experiment(key), format!("Repetición de {} (confianza baja)", key)));
            }
        }

        if self.novelty_drive > 0.4 {
            return Some((self.generate_random_experiment(), "Exploración de nuevo espacio de diseño".to_string()));
        }

        None
    }

    fn generate_random_experiment(&self) -> UnrealExperiment {
        let structure = match fastrand::usize(0..4) {
            0 => StructureType::Arch(ArchParams { 
                span: 5.0 + fastrand::f32() * 15.0, 
                radius: 3.0 + fastrand::f32() * 10.0,
                keystone_weight: 100.0 + fastrand::f32() * 900.0 
            }),
            1 => StructureType::Wall(WallParams { 
                height: 1.0 + fastrand::f32() * 4.0, 
                thickness: 0.1 + fastrand::f32() * 0.4, 
                length: 5.0 + fastrand::f32() * 10.0 
            }),
            2 => StructureType::Foundation(FoundationParams { 
                depth: 0.5 + fastrand::f32() * 2.5, 
                area: 10.0 + fastrand::f32() * 40.0, 
                soil_load_bearing: 150.0 
            }),
            _ => StructureType::Pyramid(PyramidParams { 
                base_width: 5.0 + fastrand::f32() * 15.0, 
                height: 5.0 + fastrand::f32() * 20.0, 
                material_density: 2400.0 
            }),
        };

        let stress = match fastrand::usize(0..2) {
            0 => StressTest::Seismic(SeismicParams { magnitude: 4.0 + fastrand::f32() * 5.5, frequency: 0.5 + fastrand::f32() * 2.5 }),
            _ => StressTest::Wind(WindParams { speed: 50.0 + fastrand::f32() * 150.0, turbulence: 0.1 + fastrand::f32() * 0.4 }),
        };

        UnrealExperiment { 
            structure_type: structure, 
            placement: Placement {
                position: [0.0, 0.0, 0.0],
                rotation: [0.0, 0.0, 0.0],
                on_surface: SurfaceType::Flat,
            },
            material: "concrete".to_string(),
            stress_test: stress, 
            force_direction: ForceDirection::Lateral,
            parameters: HashMap::new(), 
            duration_seconds: 15 
        }
    }

    fn reproduce_experiment(&self, key: &str) -> UnrealExperiment {
        let mut exp = if key.contains("arch") {
            UnrealExperiment {
                structure_type: StructureType::Arch(ArchParams { span: 10.0, radius: 5.0, keystone_weight: 400.0 }),
                placement: Placement { position: [0.0, 0.0, 0.0], rotation: [0.0, 0.0, 0.0], on_surface: SurfaceType::Flat },
                material: "concrete".to_string(),
                stress_test: StressTest::Seismic(SeismicParams { magnitude: 7.0, frequency: 1.5 }),
                force_direction: ForceDirection::Lateral,
                parameters: HashMap::new(), duration_seconds: 20,
            }
        } else if key.contains("wall") {
            UnrealExperiment {
                structure_type: StructureType::Wall(WallParams { height: 3.0, thickness: 0.2, length: 8.0 }),
                placement: Placement { position: [0.0, 0.0, 0.0], rotation: [0.0, 0.0, 0.0], on_surface: SurfaceType::Flat },
                material: "concrete".to_string(),
                stress_test: StressTest::Wind(WindParams { speed: 80.0, turbulence: 0.2 }),
                force_direction: ForceDirection::Lateral,
                parameters: HashMap::new(), duration_seconds: 20,
            }
        } else {
            UnrealExperiment {
                structure_type: StructureType::Pyramid(PyramidParams { base_width: 10.0, height: 10.0, material_density: 2400.0 }),
                placement: Placement { position: [0.0, 0.0, 0.0], rotation: [0.0, 0.0, 0.0], on_surface: SurfaceType::Flat },
                material: "concrete".to_string(),
                stress_test: StressTest::Seismic(SeismicParams { magnitude: 6.0, frequency: 1.0 }),
                force_direction: ForceDirection::Lateral,
                parameters: HashMap::new(), duration_seconds: 20,
            }
        };

        // Añadir pequeña variación para no ser idénticos
        self.apply_slight_variation(&mut exp);
        exp
    }

    fn apply_slight_variation(&self, experiment: &mut UnrealExperiment) {
        let stress_var = 0.9 + fastrand::f32() * 0.2; // +/- 10%
        match &mut experiment.stress_test {
            StressTest::Seismic(s) => {
                s.magnitude *= stress_var;
                s.magnitude = s.magnitude.clamp(1.0, 10.0);
            },
            StressTest::Wind(w) => {
                w.speed *= stress_var;
                w.speed = w.speed.clamp(50.0, 300.0);
            },
        }
    }
}
