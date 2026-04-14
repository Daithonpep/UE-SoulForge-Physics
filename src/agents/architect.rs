// src/agents/architect.rs
// El Arquitecto: Estratega Lógico que descompone intenciones en tareas

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskGraph {
    pub project_name: String,
    pub style: String,                  // "gothic", "scifi", "medieval", etc
    pub phases: Vec<ConstructionPhase>,
    pub global_constraints: Constraints,
    pub estimated_complexity: f32,      // 0-1
    pub requires_lod: bool,
    pub requires_nanite: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConstructionPhase {
    pub phase_id: u32,
    pub phase_name: String,
    pub priority: u8,                   // 1=highest
    pub dependencies: Vec<u32>,         // IDs de phases previas
    pub tasks: Vec<Task>,
    pub parallel_execution: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub task_id: String,
    pub task_type: TaskType,
    pub element_type: String,           // "grave", "fog", "tree", "fence"
    pub quantity_range: (u32, u32),     // (min, max)
    pub spatial_distribution: SpatialPattern,
    pub aesthetic_requirements: AestheticRequirements,
    pub technical_requirements: TechnicalRequirements,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TaskType {
    CreateTerrain,
    PlaceStructures,
    ScatterObjects,
    AddAtmosphere,
    ApplyMaterials,
    SetupLighting,
    AddPostProcess,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SpatialPattern {
    Grid { spacing: f32 },
    Random { density: f32 },
    Cluster { cluster_size: u32, spread: f32 },
    Path { curvature: f32, width: f32 },
    Radial { center: String, radius: f32 },
    Organic { poisson_radius: f32 },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AestheticRequirements {
    pub color_palette: Vec<String>,     // ["#2C3E50", "#7F8C8D"]
    pub material_style: String,         // "weathered", "pristine", "magical"
    pub scale_variation: f32,           // 0-1
    pub rotation_chaos: f32,            // 0-1
    pub must_align_to_surface: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TechnicalRequirements {
    pub max_poly_budget: u32,
    pub requires_collision: bool,
    pub requires_physics: bool,
    pub lod_levels: u8,
    pub use_instancing: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Constraints {
    pub max_area: f32,                  // En metros cuadrados
    pub performance_target_fps: u32,
    pub max_draw_calls: u32,
    pub memory_budget_mb: u32,
}

// ============================================================
// ARQUITECTO - ANALIZADOR SEMÁNTICO
// ============================================================

pub struct Architect {
    style_database: HashMap<String, StyleTemplate>,
    element_catalog: HashMap<String, ElementSpec>,
}

#[derive(Clone)]
struct StyleTemplate {
    color_schemes: Vec<Vec<String>>,
    typical_elements: Vec<String>,
    atmosphere_settings: AtmosphereSettings,
    material_preferences: Vec<String>,
}

#[derive(Clone)]
struct AtmosphereSettings {
    fog_density: f32,
    fog_color: String,
    ambient_light_color: String,
    directional_light_intensity: f32,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ElementSpec {
    pub mesh_path_hints: Vec<String>,
    pub typical_scale: f32,
    pub poly_count: u32,
    pub requires_lod: bool,
}

impl Architect {
    pub fn new() -> Self {
        let mut architect = Architect {
            style_database: HashMap::new(),
            element_catalog: HashMap::new(),
        };

        architect.initialize_knowledge_base();
        architect.load_asset_catalog();
        architect
    }

    fn load_asset_catalog(&mut self) {
        if let Ok(content) = std::fs::read_to_string("config/assets_catalog.json") {
            if let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) {
                if let Some(categories) = json.get("categories").and_then(|c| c.as_object()) {
                    for (_, items) in categories {
                        if let Some(assets) = items.as_array() {
                            for asset in assets {
                                if let (Some(id), Some(path)) = (asset.get("id").and_then(|i| i.as_str()), asset.get("path").and_then(|p| p.as_str())) {
                                    self.element_catalog.insert(id.to_string(), ElementSpec {
                                        mesh_path_hints: vec![path.to_string()],
                                        typical_scale: 1.0,
                                        poly_count: 5000,
                                        requires_lod: true,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn initialize_knowledge_base(&mut self) {
        // Base de conocimiento de estilos
        self.style_database.insert("gothic".to_string(), StyleTemplate {
            color_schemes: vec![
                vec!["#2C3E50".to_string(), "#34495E".to_string(), "#7F8C8D".to_string()],
                vec!["#1C1C1C".to_string(), "#3A3A3A".to_string(), "#8B4513".to_string()],
            ],
            typical_elements: vec![
                "gravestone".to_string(),
                "fence".to_string(),
                "dead_tree".to_string(),
                "fog".to_string(),
                "gate".to_string(),
                "crypt".to_string(),
                "statue".to_string(),
                "candles".to_string(),
            ],
            atmosphere_settings: AtmosphereSettings {
                fog_density: 0.05,
                fog_color: "#7F8C8D".to_string(),
                ambient_light_color: "#34495E".to_string(),
                directional_light_intensity: 0.3,
            },
            material_preferences: vec![
                "weathered_stone".to_string(),
                "rusted_iron".to_string(),
                "moss_covered".to_string(),
            ],
        });

        self.style_database.insert("scifi".to_string(), StyleTemplate {
            color_schemes: vec![
                vec!["#00FFFF".to_string(), "#0080FF".to_string(), "#FFFFFF".to_string()],
                vec!["#FF00FF".to_string(), "#8000FF".to_string(), "#000000".to_string()],
            ],
            typical_elements: vec![
                "hologram".to_string(),
                "terminal".to_string(),
                "neon_light".to_string(),
                "platform".to_string(),
                "energy_core".to_string(),
            ],
            atmosphere_settings: AtmosphereSettings {
                fog_density: 0.02,
                fog_color: "#00FFFF".to_string(),
                ambient_light_color: "#0080FF".to_string(),
                directional_light_intensity: 0.5,
            },
            material_preferences: vec![
                "metallic_chrome".to_string(),
                "emissive_neon".to_string(),
                "holographic".to_string(),
            ],
        });

        // Catálogo de elementos
        self.element_catalog.insert("gravestone".to_string(), ElementSpec {
            mesh_path_hints: vec![
                "/Game/Environment/Graveyard/SM_Gravestone".to_string(),
                "/Game/Props/Cemetery/SM_Tombstone".to_string(),
            ],
            typical_scale: 1.2,
            poly_count: 5000,
            requires_lod: true,
        });

        self.element_catalog.insert("dead_tree".to_string(), ElementSpec {
            mesh_path_hints: vec![
                "/Game/Environment/Trees/SM_DeadTree".to_string(),
            ],
            typical_scale: 3.5,
            poly_count: 15000,
            requires_lod: true,
        });

        self.element_catalog.insert("fog".to_string(), ElementSpec {
            mesh_path_hints: vec![
                "/Engine/VolumetricFog".to_string(),
            ],
            typical_scale: 1.0,
            poly_count: 0,
            requires_lod: false,
        });
    }

    pub fn analyze_and_plan(&self, user_prompt: &str) -> TaskGraph {
        log::info!("🎯 ARQUITECTO: Analizando prompt...");
        log::info!("   '{}'", user_prompt);

        // 1. Detectar estilo
        let style = self.detect_style(user_prompt);
        log::info!("   Estilo detectado: {}", style);

        // 2. Extraer elementos mencionados
        let elements = self.extract_elements(user_prompt);
        log::info!("   Elementos identificados: {:?}", elements);

        // 3. Inferir elementos implícitos basados en el estilo
        let mut all_elements = elements.clone();
        all_elements.extend(self.infer_implicit_elements(&style, &elements));

        // 4. Determinar jerarquía de construcción
        let phases = self.build_construction_phases(&style, &all_elements);

        // 5. Estimar complejidad
        let complexity = self.estimate_complexity(&phases);

        // 6. Determinar requerimientos técnicos
        let (requires_lod, requires_nanite) = self.determine_technical_needs(&phases);

        let task_graph = TaskGraph {
            project_name: self.generate_project_name(user_prompt),
            style: style.clone(),
            phases,
            global_constraints: Constraints {
                max_area: 100000.0,  // 1000m²
                performance_target_fps: 60,
                max_draw_calls: 2000,
                memory_budget_mb: 4096,
            },
            estimated_complexity: complexity,
            requires_lod,
            requires_nanite,
        };

        log::info!("✅ ARQUITECTO: Plan generado");
        log::info!("   Fases: {}", task_graph.phases.len());
        log::info!("   Complejidad: {:.2}", task_graph.estimated_complexity);
        log::info!("   LOD: {} | Nanite: {}", requires_lod, requires_nanite);

        task_graph
    }

    fn detect_style(&self, prompt: &str) -> String {
        let prompt_lower = prompt.to_lowercase();

        for (style_name, _) in &self.style_database {
            if prompt_lower.contains(style_name) {
                return style_name.clone();
            }
        }

        // Inferir por palabras clave
        if prompt_lower.contains("cementerio") || prompt_lower.contains("gótico") 
            || prompt_lower.contains("oscuro") || prompt_lower.contains("tumbas") {
            return "gothic".to_string();
        }

        if prompt_lower.contains("futuro") || prompt_lower.contains("neón") 
            || prompt_lower.contains("tecnológico") || prompt_lower.contains("cyberpunk") {
            return "scifi".to_string();
        }

        "generic".to_string()
    }

    fn extract_elements(&self, prompt: &str) -> Vec<String> {
        let mut elements = Vec::new();
        let prompt_lower = prompt.to_lowercase();

        // Diccionario de palabras → elementos
        let element_keywords = [
            (vec!["lápida", "tumba", "sepulcro", "gravestone"], "gravestone"),
            (vec!["niebla", "fog", "bruma"], "fog"),
            (vec!["verja", "reja", "fence", "cerca"], "fence"),
            (vec!["árbol", "tree"], "dead_tree"),
            (vec!["puerta", "gate", "portal"], "gate"),
            (vec!["cripta", "crypt", "mausoleo"], "crypt"),
            (vec!["estatua", "statue"], "statue"),
            (vec!["vela", "candle"], "candles"),
            (vec!["luz", "light", "iluminación"], "lighting"),
        ];

        for (keywords, element) in &element_keywords {
            for keyword in keywords {
                if prompt_lower.contains(keyword) {
                    if !elements.contains(&element.to_string()) {
                        elements.push(element.to_string());
                    }
                    break;
                }
            }
        }

        elements
    }

    fn infer_implicit_elements(&self, style: &str, explicit_elements: &[String]) -> Vec<String> {
        let mut implicit = Vec::new();

        if let Some(style_template) = self.style_database.get(style) {
            // Si es un cementerio y no se mencionó el suelo
            if explicit_elements.contains(&"gravestone".to_string()) 
                && !explicit_elements.contains(&"ground".to_string()) {
                implicit.push("ground".to_string());
            }

            // Si hay verjas pero no puerta
            if explicit_elements.contains(&"fence".to_string()) 
                && !explicit_elements.contains(&"gate".to_string()) {
                implicit.push("gate".to_string());
            }

            // Si no se mencionó iluminación en un estilo oscuro
            if style == "gothic" && !explicit_elements.contains(&"lighting".to_string()) {
                implicit.push("lighting".to_string());
            }
        }

        implicit
    }

    fn build_construction_phases(&self, style: &str, elements: &[String]) -> Vec<ConstructionPhase> {
        let mut phases = Vec::new();

        // FASE 0: Terreno base (siempre primero)
        if elements.contains(&"ground".to_string()) || elements.len() > 0 {
            phases.push(ConstructionPhase {
                phase_id: 0,
                phase_name: "Foundation".to_string(),
                priority: 1,
                dependencies: vec![],
                tasks: vec![
                    Task {
                        task_id: "terrain_base".to_string(),
                        task_type: TaskType::CreateTerrain,
                        element_type: "ground".to_string(),
                        quantity_range: (1, 1),
                        spatial_distribution: SpatialPattern::Grid { spacing: 1.0 },
                        aesthetic_requirements: self.get_aesthetic_for_element(style, "ground"),
                        technical_requirements: TechnicalRequirements {
                            max_poly_budget: 50000,
                            requires_collision: true,
                            requires_physics: false,
                            lod_levels: 3,
                            use_instancing: false,
                        },
                    }
                ],
                parallel_execution: false,
            });
        }

        // FASE 1: Estructuras grandes (criptas, árboles grandes)
        let large_structures: Vec<String> = elements.iter()
            .filter(|e| matches!(e.as_str(), "crypt" | "dead_tree" | "gate"))
            .cloned()
            .collect();

        if !large_structures.is_empty() {
            let mut tasks = Vec::new();

            for element in large_structures {
                tasks.push(self.create_task_for_element(&element, style));
            }

            phases.push(ConstructionPhase {
                phase_id: 1,
                phase_name: "Large Structures".to_string(),
                priority: 2,
                dependencies: vec![0],
                tasks,
                parallel_execution: true,
            });
        }

        // FASE 2: Objetos medios (lápidas, verjas)
        let medium_objects: Vec<String> = elements.iter()
            .filter(|e| matches!(e.as_str(), "gravestone" | "fence" | "statue"))
            .cloned()
            .collect();

        if !medium_objects.is_empty() {
            let mut tasks = Vec::new();

            for element in medium_objects {
                tasks.push(self.create_task_for_element(&element, style));
            }

            phases.push(ConstructionPhase {
                phase_id: 2,
                phase_name: "Medium Objects".to_string(),
                priority: 3,
                dependencies: vec![0, 1],
                tasks,
                parallel_execution: true,
            });
        }

        // FASE 3: Detalles pequeños (velas, flores)
        let small_details: Vec<String> = elements.iter()
            .filter(|e| matches!(e.as_str(), "candles" | "flowers" | "debris"))
            .cloned()
            .collect();

        if !small_details.is_empty() {
            let mut tasks = Vec::new();

            for element in small_details {
                tasks.push(self.create_task_for_element(&element, style));
            }

            phases.push(ConstructionPhase {
                phase_id: 3,
                phase_name: "Small Details".to_string(),
                priority: 4,
                dependencies: vec![0, 1, 2],
                tasks,
                parallel_execution: true,
            });
        }

        // FASE 4: Atmósfera (niebla, iluminación, postproceso)
        let atmosphere_elements: Vec<String> = elements.iter()
            .filter(|e| matches!(e.as_str(), "fog" | "lighting"))
            .cloned()
            .collect();

        if !atmosphere_elements.is_empty() || style == "gothic" {
            let mut tasks = Vec::new();

            if elements.contains(&"fog".to_string()) || style == "gothic" {
                tasks.push(Task {
                    task_id: "volumetric_fog".to_string(),
                    task_type: TaskType::AddAtmosphere,
                    element_type: "fog".to_string(),
                    quantity_range: (1, 1),
                    spatial_distribution: SpatialPattern::Grid { spacing: 1.0 },
                    aesthetic_requirements: self.get_aesthetic_for_element(style, "fog"),
                    technical_requirements: TechnicalRequirements {
                        max_poly_budget: 0,
                        requires_collision: false,
                        requires_physics: false,
                        lod_levels: 0,
                        use_instancing: false,
                    },
                });
            }

            tasks.push(Task {
                task_id: "lighting_setup".to_string(),
                task_type: TaskType::SetupLighting,
                element_type: "lighting".to_string(),
                quantity_range: (1, 1),
                spatial_distribution: SpatialPattern::Grid { spacing: 1.0 },
                aesthetic_requirements: self.get_aesthetic_for_element(style, "lighting"),
                technical_requirements: TechnicalRequirements {
                    max_poly_budget: 0,
                    requires_collision: false,
                    requires_physics: false,
                    lod_levels: 0,
                    use_instancing: false,
                },
            });

            phases.push(ConstructionPhase {
                phase_id: 4,
                phase_name: "Atmosphere".to_string(),
                priority: 5,
                dependencies: vec![0, 1, 2, 3],
                tasks,
                parallel_execution: true,
            });
        }

        phases
    }

    fn create_task_for_element(&self, element: &str, style: &str) -> Task {
        let (quantity_min, quantity_max, distribution) = match element {
            "gravestone" => (20, 50, SpatialPattern::Organic { poisson_radius: 300.0 }),
            "dead_tree" => (5, 15, SpatialPattern::Cluster { cluster_size: 3, spread: 500.0 }),
            "fence" => (1, 1, SpatialPattern::Path { curvature: 0.3, width: 200.0 }),
            "crypt" => (1, 3, SpatialPattern::Random { density: 0.001 }),
            "statue" => (2, 8, SpatialPattern::Grid { spacing: 1000.0 }),
            "candles" => (30, 100, SpatialPattern::Cluster { cluster_size: 5, spread: 100.0 }),
            "gate" => (1, 1, SpatialPattern::Grid { spacing: 1.0 }),
            _ => (1, 10, SpatialPattern::Random { density: 0.01 }),
        };

        let task_type = match element {
            "ground" => TaskType::CreateTerrain,
            "lighting" => TaskType::SetupLighting,
            "fog" => TaskType::AddAtmosphere,
            _ => TaskType::ScatterObjects,
        };

        Task {
            task_id: format!("place_{}", element),
            task_type,
            element_type: element.to_string(),
            quantity_range: (quantity_min, quantity_max),
            spatial_distribution: distribution,
            aesthetic_requirements: self.get_aesthetic_for_element(style, element),
            technical_requirements: self.get_technical_requirements_for_element(element),
        }
    }

    fn get_aesthetic_for_element(&self, style: &str, element: &str) -> AestheticRequirements {
        let style_template = self.style_database.get(style);

        let color_palette = if let Some(template) = style_template {
            template.color_schemes[0].clone()
        } else {
            vec!["#FFFFFF".to_string()]
        };

        AestheticRequirements {
            color_palette,
            material_style: match style {
                "gothic" => "weathered".to_string(),
                "scifi" => "pristine".to_string(),
                _ => "generic".to_string(),
            },
            scale_variation: match element {
                "gravestone" => 0.3,
                "dead_tree" => 0.4,
                _ => 0.2,
            },
            rotation_chaos: match element {
                "gravestone" => 0.15,
                "candles" => 0.0,
                _ => 1.0,
            },
            must_align_to_surface: matches!(element, "gravestone" | "statue" | "candles"),
        }
    }

    fn get_technical_requirements_for_element(&self, element: &str) -> TechnicalRequirements {
        if let Some(spec) = self.element_catalog.get(element) {
            TechnicalRequirements {
                max_poly_budget: spec.poly_count,
                requires_collision: matches!(element, "fence" | "crypt" | "gate"),
                requires_physics: false,
                lod_levels: if spec.requires_lod { 3 } else { 0 },
                use_instancing: true,
            }
        } else {
            TechnicalRequirements {
                max_poly_budget: 10000,
                requires_collision: true,
                requires_physics: false,
                lod_levels: 2,
                use_instancing: true,
            }
        }
    }

    fn estimate_complexity(&self, phases: &[ConstructionPhase]) -> f32 {
        let total_tasks: usize = phases.iter().map(|p| p.tasks.len()).sum();
        let total_objects: u32 = phases.iter()
            .flat_map(|p| &p.tasks)
            .map(|t| t.quantity_range.1)
            .sum();

        let poly_budget: u32 = phases.iter()
            .flat_map(|p| &p.tasks)
            .map(|t| t.technical_requirements.max_poly_budget * t.quantity_range.1)
            .sum();

        // Normalizar a 0-1
        let task_complexity = (total_tasks as f32 / 20.0).min(1.0);
        let object_complexity = (total_objects as f32 / 200.0).min(1.0);
        let poly_complexity = (poly_budget as f32 / 5000000.0).min(1.0);

        (task_complexity + object_complexity + poly_complexity) / 3.0
    }

    fn determine_technical_needs(&self, phases: &[ConstructionPhase]) -> (bool, bool) {
        let total_polys: u32 = phases.iter()
            .flat_map(|p| &p.tasks)
            .map(|t| t.technical_requirements.max_poly_budget * t.quantity_range.1)
            .sum();

        let requires_lod = total_polys > 1000000;
        let requires_nanite = total_polys > 5000000;

        (requires_lod, requires_nanite)
    }

    fn generate_project_name(&self, prompt: &str) -> String {
        let words: Vec<&str> = prompt.split_whitespace().take(3).collect();
        let base_name = words.join("_");
        
        format!("Daithon_{}_{}", base_name, chrono::Utc::now().timestamp() % 10000)
    }
}
