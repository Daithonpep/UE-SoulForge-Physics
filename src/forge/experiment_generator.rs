use crate::contextus::semantic_graph::SemanticGraph;
use crate::forge::experimental_lab::{
    UnrealExperiment, StructureType, StressTest, Placement, SurfaceType, 
    ForceDirection, PyramidParams, ArchParams, CantileverParams, WallParams, 
    FoundationParams, SeismicParams, WindParams
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub struct ExperimentGenerator {
    pub rng: fastrand::Rng,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExperimentIntent {
    ExploreNewShape,
    PushLimits(String),
    TestOrientation(StructureType),
    CompareMaterials(StructureType),
    FindBreakingPoint(StructureType),
    TestSurfaceEffect,
    TestForceDirection(StructureType),
}

impl ExperimentGenerator {
    pub fn new() -> Self {
        Self {
            rng: fastrand::Rng::new(),
        }
    }

    pub fn generate(&mut self, graph: &SemanticGraph) -> (UnrealExperiment, ExperimentIntent) {
        let intent = self.decide_experiment_type(graph);

        let experiment = match &intent {
            ExperimentIntent::ExploreNewShape => self.generate_new_shape(),
            ExperimentIntent::PushLimits(_) => self.generate_new_shape(), // Placeholder for specialized limit pushing
            ExperimentIntent::TestOrientation(shape) => self.test_orientation(shape),
            ExperimentIntent::CompareMaterials(shape) => self.compare_materials(shape),
            ExperimentIntent::FindBreakingPoint(shape) => self.find_breaking_point(shape, graph),
            ExperimentIntent::TestSurfaceEffect => self.test_surface_effect(),
            ExperimentIntent::TestForceDirection(shape) => self.test_force_direction(shape),
        };

        (experiment, intent)
    }

    fn decide_experiment_type(&mut self, graph: &SemanticGraph) -> ExperimentIntent {
        let pyramid_count = graph.count_experiments_for("pyramid");
        let arch_count = graph.count_experiments_for("arch");
        let cantilever_count = graph.count_experiments_for("cantilever");
        let wall_count = graph.count_experiments_for("wall");
        let foundation_count = graph.count_experiments_for("foundation");

        let has_collapse_data = graph.has_any_collapse();
        let has_orientation_data = graph.has_orientation_experiments();

        if !has_collapse_data && (pyramid_count + arch_count) > 0 {
            let most_tested = self.get_random_tested_shape(graph);
            return ExperimentIntent::FindBreakingPoint(most_tested);
        }

        let total = pyramid_count + arch_count + cantilever_count + wall_count + foundation_count;
        let counts = [pyramid_count, arch_count, cantilever_count, wall_count, foundation_count];
        let max_single = counts.iter().max().unwrap_or(&0);

        if total > 0 && (*max_single as f32 / total as f32) > 0.6 {
            return ExperimentIntent::ExploreNewShape;
        }

        if !has_orientation_data && total > 0 {
            let shape = self.get_random_tested_shape(graph);
            return ExperimentIntent::TestOrientation(shape);
        }

        let options = vec![
            (30, ExperimentIntent::FindBreakingPoint(self.get_random_tested_shape(graph))),
            (20, ExperimentIntent::TestOrientation(self.get_random_tested_shape(graph))),
            (20, ExperimentIntent::TestForceDirection(self.get_random_tested_shape(graph))),
            (15, ExperimentIntent::TestSurfaceEffect),
            (10, ExperimentIntent::CompareMaterials(self.get_random_tested_shape(graph))),
            (5, ExperimentIntent::ExploreNewShape),
        ];

        self.weighted_choice(&options)
    }

    fn weighted_choice(&mut self, options: &[(u32, ExperimentIntent)]) -> ExperimentIntent {
        let total_weight: u32 = options.iter().map(|(w, _)| w).sum();
        let mut r = self.rng.u32(0..total_weight);
        for (w, intent) in options {
            if r < *w {
                return intent.clone();
            }
            r -= w;
        }
        options[0].1.clone()
    }

    fn get_random_tested_shape(&mut self, _graph: &SemanticGraph) -> StructureType {
        match self.rng.u8(0..4) {
            0 => StructureType::Arch(ArchParams { span: 10.0, radius: 5.0, keystone_weight: 500.0 }),
            1 => StructureType::Wall(WallParams { height: 3.0, thickness: 0.2, length: 10.0 }),
            2 => StructureType::Foundation(FoundationParams { depth: 1.0, area: 20.0, soil_load_bearing: 200.0 }),
            _ => StructureType::Pyramid(PyramidParams { base_width: 10.0, height: 10.0, material_density: 2400.0 }),
        }
    }

    fn generate_new_shape(&mut self) -> UnrealExperiment {
        let structure = match self.rng.u8(0..6) {
            0 => self.random_pyramid(),
            1 => self.random_arch(),
            2 => self.random_cantilever(),
            3 => self.random_wall(),
            4 => self.random_foundation(),
            _ => self.random_tower(),
        };

        UnrealExperiment {
            structure_type: structure,
            placement: Placement {
                position: [0.0, 0.0, 0.0],
                rotation: [0.0, 0.0, 0.0],
                on_surface: SurfaceType::Flat,
            },
            material: self.random_material(),
            stress_test: self.random_stress_moderate(),
            force_direction: ForceDirection::Lateral,
            parameters: HashMap::new(),
            duration_seconds: 15,
        }
    }

    fn random_pyramid(&mut self) -> StructureType {
        let base = self.rng.f32() * 20.0 + 5.0;
        let height = self.rng.f32() * 30.0 + 3.0;
        StructureType::Pyramid(PyramidParams { base_width: base, height, material_density: self.random_density() })
    }

    fn random_arch(&mut self) -> StructureType {
        StructureType::Arch(ArchParams {
            span: (self.rng.f32() * 10.0 + 2.0),
            radius: (self.rng.f32() * 15.0 + 3.0),
            keystone_weight: (self.rng.f32() * 1000.0 + 50.0),
        })
    }

    fn random_cantilever(&mut self) -> StructureType {
        StructureType::Cantilever(CantileverParams {
            span: (self.rng.f32() * 20.0 + 2.0),
            thickness: (self.rng.f32() * 2.0 + 0.1),
        })
    }

    fn random_wall(&mut self) -> StructureType {
        StructureType::Wall(WallParams {
            height: (self.rng.f32() * 15.0 + 2.0),
            thickness: (self.rng.f32() * 2.0 + 0.1),
            length: (self.rng.f32() * 20.0 + 3.0),
        })
    }

    fn random_foundation(&mut self) -> StructureType {
        StructureType::Foundation(FoundationParams {
            depth: (self.rng.f32() * 5.0 + 0.5),
            area: (self.rng.f32() * 50.0 + 5.0),
            soil_load_bearing: (self.rng.f32() * 300.0 + 50.0),
        })
    }

    fn random_tower(&mut self) -> StructureType {
        StructureType::Pyramid(PyramidParams {
            base_width: (self.rng.f32() * 5.0 + 2.0),
            height: (self.rng.f32() * 50.0 + 20.0),
            material_density: self.random_density(),
        })
    }

    fn find_breaking_point(&mut self, shape: &StructureType, graph: &SemanticGraph) -> UnrealExperiment {
        let max_survived = graph.get_max_survived_stress(shape);
        let magnitude = match max_survived {
            Some(m) => (m * (1.3 + self.rng.f32() * 0.2)).min(10.0),
            None => 8.0 + self.rng.f32() * 1.5,
        };

        UnrealExperiment {
            structure_type: shape.clone(),
            placement: Placement { position: [0.0, 0.0, 0.0], rotation: [0.0, 0.0, 0.0], on_surface: SurfaceType::Flat },
            material: self.random_material(),
            stress_test: StressTest::Seismic(SeismicParams { magnitude, frequency: self.rng.f32() * 3.0 + 0.5 }),
            force_direction: ForceDirection::Lateral,
            parameters: HashMap::new(),
            duration_seconds: 20,
        }
    }

    fn test_orientation(&mut self, shape: &StructureType) -> UnrealExperiment {
        let rotation = match self.rng.u8(0..6) {
            0 => [0.0, 0.0, 0.0],
            1 => [45.0, 0.0, 0.0],
            2 => [0.0, 45.0, 0.0],
            3 => [90.0, 0.0, 0.0],
            4 => [180.0, 0.0, 0.0],
            _ => [self.rng.f64() * 360.0, self.rng.f64() * 360.0, self.rng.f64() * 360.0],
        };

        let surface = match self.rng.u8(0..4) {
            0 => SurfaceType::Flat,
            1 => SurfaceType::Inclined(self.rng.f64() * 30.0 + 5.0),
            2 => SurfaceType::Uneven(self.rng.f64() * 0.5),
            _ => SurfaceType::Elevated(self.rng.f64() * 10.0 + 2.0),
        };

        UnrealExperiment {
            structure_type: shape.clone(),
            placement: Placement { position: [0.0, 0.0, 0.0], rotation, on_surface: surface },
            material: self.random_material(),
            stress_test: self.random_stress_moderate(),
            force_direction: ForceDirection::Lateral,
            parameters: HashMap::new(),
            duration_seconds: 15,
        }
    }

    fn test_force_direction(&mut self, shape: &StructureType) -> UnrealExperiment {
        let direction = match self.rng.u8(0..6) {
            0 => ForceDirection::Lateral,
            1 => ForceDirection::Frontal,
            2 => ForceDirection::Diagonal(self.rng.f64() * 360.0),
            3 => ForceDirection::FromBelow,
            4 => ForceDirection::Downward(self.rng.f64() * 50000.0 + 1000.0),
            _ => ForceDirection::Rotational(self.rng.f64() * 180.0),
        };

        UnrealExperiment {
            structure_type: shape.clone(),
            placement: Placement { position: [0.0, 0.0, 0.0], rotation: [0.0, 0.0, 0.0], on_surface: SurfaceType::Flat },
            material: self.random_material(),
            stress_test: self.random_stress_moderate(),
            force_direction: direction,
            parameters: HashMap::new(),
            duration_seconds: 15,
        }
    }

    fn test_surface_effect(&mut self) -> UnrealExperiment {
        let structure = self.random_pyramid();
        let surface = match self.rng.u8(0..5) {
            0 => SurfaceType::Flat,
            1 => SurfaceType::Inclined(10.0),
            2 => SurfaceType::Inclined(25.0),
            3 => SurfaceType::Inclined(45.0),
            _ => SurfaceType::Uneven(0.7),
        };

        UnrealExperiment {
            structure_type: structure,
            placement: Placement { position: [0.0, 0.0, 0.0], rotation: [0.0, 0.0, 0.0], on_surface: surface },
            material: self.random_material(),
            stress_test: self.random_stress_moderate(),
            force_direction: ForceDirection::Lateral,
            parameters: HashMap::new(),
            duration_seconds: 15,
        }
    }

    fn compare_materials(&mut self, shape: &StructureType) -> UnrealExperiment {
        let material = self.random_material();
        UnrealExperiment {
            structure_type: shape.clone(),
            placement: Placement { position: [0.0, 0.0, 0.0], rotation: [0.0, 0.0, 0.0], on_surface: SurfaceType::Flat },
            material,
            stress_test: self.random_stress_moderate(),
            force_direction: ForceDirection::Lateral,
            parameters: HashMap::new(),
            duration_seconds: 15,
        }
    }

    fn random_material(&mut self) -> String {
        let materials = vec!["concrete", "steel", "aluminum", "wood_oak", "granite", "titanium"];
        materials[self.rng.usize(0..materials.len())].to_string()
    }

    fn random_density(&mut self) -> f32 {
        let densities = vec![2400.0, 7850.0, 2700.0, 750.0, 2750.0];
        densities[self.rng.usize(0..densities.len())]
    }

    fn random_stress_moderate(&mut self) -> StressTest {
        if self.rng.bool() {
            StressTest::Seismic(SeismicParams {
                magnitude: self.rng.f32() * 4.0 + 5.0,
                frequency: self.rng.f32() * 3.0 + 0.5,
            })
        } else {
            StressTest::Wind(WindParams {
                speed: self.rng.f32() * 200.0 + 50.0,
                turbulence: self.rng.f32() * 0.8,
            })
        }
    }
}
