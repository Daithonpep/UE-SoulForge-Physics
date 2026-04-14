// src/agents/geometer.rs
use serde::{Deserialize, Serialize};
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GeometricInstruction {
    pub phase_id: u32,
    pub tasks: Vec<PlacedObject>,
    pub pcg_graph_config: PCGGraphConfig,
    pub total_objects: u32,
    pub estimated_draw_calls: u32,
    pub memory_estimate_mb: f32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PlacedObject {
    pub object_id: String,
    pub mesh_path: String,
    pub transform: Transform3D,
    pub scale: f32,
    pub rotation_variance: f32,
    pub pcg_attributes: PCGAttributes,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transform3D {
    pub location: [f32; 3],
    pub rotation: [f32; 3],
    pub scale: [f32; 3],
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PCGAttributes {
    pub density: f32,
    pub seed: u32,
    pub jitter: f32,
    pub slope_max: f32,
    pub height_min: f32,
    pub height_max: f32,
    pub use_nanite: bool,
    pub use_lod: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PCGGraphConfig {
    pub use_surface_sampler: bool,
    pub use_density_filter: bool,
    pub use_self_pruning: bool,
    pub poisson_disk_radius: f32,
    pub global_seed: u32,
}

pub struct Geometer {
    spatial_grid: SpatialHashGrid,
    mesh_database: HashMap<String, MeshInfo>,
}

#[derive(Clone)]
struct MeshInfo {
    default_path: String,
    avg_poly_count: u32,
    recommended_lod: bool,
    recommended_nanite: bool,
}

struct SpatialHashGrid {
    cell_size: f32,
    objects: HashMap<(i32, i32), Vec<PlacedObject>>,
}

impl Geometer {
    pub fn new() -> Self {
        let mut geo = Geometer {
            spatial_grid: SpatialHashGrid { cell_size: 500.0, objects: HashMap::new() },
            mesh_database: HashMap::new(),
        };
        geo.initialize_mesh_database();
        geo
    }

    fn initialize_mesh_database(&mut self) {
        if let Ok(content) = std::fs::read_to_string("config/assets_catalog.json") {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(categories) = json.get("categories").and_then(|c| c.as_object()) {
                    for (_, items) in categories {
                        if let Some(assets) = items.as_array() {
                            for asset in assets {
                                if let (Some(id), Some(path)) = (asset.get("id").and_then(|i| i.as_str()), asset.get("path").and_then(|p| p.as_str())) {
                                    self.mesh_database.insert(id.to_string(), MeshInfo {
                                        default_path: path.to_string(),
                                        avg_poly_count: 5000,
                                        recommended_lod: true,
                                        recommended_nanite: false,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        } else {
            // Fallback values if file is missing
            self.mesh_database.insert("gravestone".to_string(), MeshInfo {
                default_path: "/Game/Environment/Cemetery/SM_Gravestone_01".to_string(),
                avg_poly_count: 4800,
                recommended_lod: true,
                recommended_nanite: false,
            });
            self.mesh_database.insert("dead_tree".to_string(), MeshInfo {
                default_path: "/Game/Environment/Trees/SM_DeadTree_01".to_string(),
                avg_poly_count: 18500,
                recommended_lod: true,
                recommended_nanite: true,
            });
        }
    }

    pub fn execute_plan(&mut self, task_graph: crate::agents::architect::TaskGraph) -> GeometricInstruction {
        log::info!("📐 GEÓMETRA: Ejecutando plan de construcción...");

        let mut all_placed = Vec::new();
        let mut total_objects = 0;
        let global_seed = rand::thread_rng().gen_range(100000..999999);

        for phase in &task_graph.phases {
            for task in &phase.tasks {
                let placed = self.generate_objects_for_task(task, global_seed, &task_graph.style);
                all_placed.extend(placed);
                total_objects += task.quantity_range.1;
            }
        }

        let pcg_config = PCGGraphConfig {
            use_surface_sampler: true,
            use_density_filter: true,
            use_self_pruning: true,
            poisson_disk_radius: 350.0,
            global_seed,
        };

        GeometricInstruction {
            phase_id: 0,
            tasks: all_placed,
            pcg_graph_config: pcg_config,
            total_objects: total_objects as u32,
            estimated_draw_calls: (total_objects as f32 * 1.8) as u32,
            memory_estimate_mb: (total_objects as f32 * 0.12),
        }
    }

    fn generate_objects_for_task(
        &mut self,
        task: &crate::agents::architect::Task,
        global_seed: u32,
        style: &str,
    ) -> Vec<PlacedObject> {
        let mut objects = Vec::new();
        let mut rng = rand::thread_rng();

        let mesh_info = self.mesh_database.get(&task.element_type)
            .cloned()
            .unwrap_or(MeshInfo {
                default_path: "/Engine/BasicShapes/Cube".to_string(),
                avg_poly_count: 1000,
                recommended_lod: true,
                recommended_nanite: false,
            });

        let count = rng.gen_range(task.quantity_range.0..=task.quantity_range.1);

        for i in 0..count {
            let x = rng.gen_range(-5000.0..5000.0);
            let y = rng.gen_range(-5000.0..5000.0);
            let z = 0.0;

            let rotation = match task.element_type.as_str() {
                "gravestone" => [0.0, rng.gen_range(-15.0..15.0), 0.0],
                "dead_tree" => [0.0, rng.gen_range(0.0..360.0), 0.0],
                _ => [0.0, rng.gen_range(0.0..360.0), 0.0],
            };

            let scale = match task.element_type.as_str() {
                "dead_tree" => rng.gen_range(0.7..1.6),
                "gravestone" => rng.gen_range(0.85..1.25),
                _ => 1.0,
            };

            objects.push(PlacedObject {
                object_id: format!("{}_{}", task.element_type, i),
                mesh_path: mesh_info.default_path.clone(),
                transform: Transform3D {
                    location: [x, y, z],
                    rotation,
                    scale: [scale, scale, scale],
                },
                scale,
                rotation_variance: task.aesthetic_requirements.rotation_chaos,
                pcg_attributes: PCGAttributes {
                    density: 0.65,
                    seed: global_seed + i as u32,
                    jitter: 0.4,
                    slope_max: 35.0,
                    height_min: -50.0,
                    height_max: 200.0,
                    use_nanite: mesh_info.recommended_nanite,
                    use_lod: mesh_info.recommended_lod,
                },
            });
        }

        objects
    }
}
