use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PCGCommand {
    pub command_type: String,  // "create_forest", "spawn_rocks", "generate_road", etc
    pub biome: String,         // "forest", "desert", "urban", "mountain"
    pub density: f32,          // 0.0 - 1.0
    pub bounds: PCGBounds,
    pub asset_rules: Vec<AssetRule>,
    pub distribution: DistributionPattern,
    pub physics: bool,
    pub collision: bool,
    pub lod_settings: LODSettings,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PCGBounds {
    pub center: [f32; 3],
    pub size: [f32; 3],      // [X, Y, Z] en cm
    pub shape: String,        // "box", "sphere", "spline", "custom"
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AssetRule {
    pub asset_path: String,
    pub weight: f32,          // Probabilidad relativa
    pub scale_min: f32,
    pub scale_max: f32,
    pub rotation_random: bool,
    pub height_offset: f32,
    pub slope_range: Option<[f32; 2]>,  // ángulo min/max en grados
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DistributionPattern {
    pub pattern_type: String,  // "random", "grid", "poisson", "cluster", "path_following"
    pub spacing: f32,
    pub jitter: f32,           // 0.0 = perfecto, 1.0 = totalmente aleatorio
    pub seed: i32,
    pub avoid_overlap: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LODSettings {
    pub use_lods: bool,
    pub distance_scale: f32,
    pub cull_distance: f32,
}

// ============================================================
// ANALIZADOR DE INTENCIÓN PCG
// ============================================================

pub fn analyze_pcg_intent(message: &str, _context: &str) -> Option<PCGCommand> {
    let msg_lower = message.to_lowercase();
    
    // Detectar tipo de generación
    let command_type = detect_generation_type(&msg_lower);
    let biome = detect_biome(&msg_lower);
    let density = extract_density(&msg_lower);
    let bounds = parse_bounds(_context);
    let assets = detect_assets(&msg_lower, &biome);
    let distribution = determine_distribution(&msg_lower, &command_type);

    if command_type == "unknown" {
        return None;
    }

    Some(PCGCommand {
        command_type,
        biome,
        density,
        bounds,
        asset_rules: assets,
        distribution,
        physics: msg_lower.contains("física") || msg_lower.contains("physics"),
        collision: !msg_lower.contains("sin colisión") && !msg_lower.contains("no collision"),
        lod_settings: LODSettings {
            use_lods: true,
            distance_scale: 1.0,
            cull_distance: 10000.0,
        },
    })
}

fn detect_generation_type(msg: &str) -> String {
    if msg.contains("bosque") || msg.contains("forest") || msg.contains("árboles") {
        "create_forest".to_string()
    } else if msg.contains("rocas") || msg.contains("rocks") || msg.contains("piedras") {
        "spawn_rocks".to_string()
    } else if msg.contains("camino") || msg.contains("road") || msg.contains("path") {
        "generate_road".to_string()
    } else if msg.contains("ciudad") || msg.contains("urban") || msg.contains("edificios") {
        "generate_city".to_string()
    } else if msg.contains("pasto") || msg.contains("grass") || msg.contains("césped") {
        "scatter_grass".to_string()
    } else if msg.contains("flores") || msg.contains("flowers") || msg.contains("plantas") {
        "scatter_foliage".to_string()
    } else if msg.contains("muros") || msg.contains("walls") || msg.contains("cerca") {
        "generate_walls".to_string()
    } else {
        "unknown".to_string()
    }
}

fn detect_biome(msg: &str) -> String {
    if msg.contains("tropical") || msg.contains("selva") || msg.contains("jungle") {
        "tropical".to_string()
    } else if msg.contains("desierto") || msg.contains("desert") {
        "desert".to_string()
    } else if msg.contains("nieve") || msg.contains("snow") || msg.contains("ártico") {
        "arctic".to_string()
    } else if msg.contains("montaña") || msg.contains("mountain") {
        "mountain".to_string()
    } else if msg.contains("ciudad") || msg.contains("urban") {
        "urban".to_string()
    } else if msg.contains("bosque") || msg.contains("forest") {
        "temperate_forest".to_string()
    } else {
        "generic".to_string()
    }
}

fn extract_density(msg: &str) -> f32 {
    if msg.contains("muy denso") || msg.contains("very dense") || msg.contains("tupido") {
        0.9
    } else if msg.contains("denso") || msg.contains("dense") {
        0.7
    } else if msg.contains("medio") || msg.contains("medium") || msg.contains("normal") {
        0.5
    } else if msg.contains("disperso") || msg.contains("sparse") || msg.contains("poco") {
        0.3
    } else if msg.contains("muy disperso") || msg.contains("very sparse") {
        0.15
    } else {
        0.5  // default
    }
}

fn parse_bounds(_context: &str) -> PCGBounds {
    // Parsear contexto para extraer bounds
    // Por ahora, usar defaults razonables
    PCGBounds {
        center: [0.0, 0.0, 0.0],
        size: [10000.0, 10000.0, 1000.0],  // 100m x 100m
        shape: "box".to_string(),
    }
}

fn detect_assets(msg: &str, biome: &str) -> Vec<AssetRule> {
    let mut assets = Vec::new();

    // Detectar tipos específicos de árboles/plantas
    if msg.contains("pino") || msg.contains("pine") {
        assets.push(AssetRule {
            asset_path: "/Game/Environment/Trees/SM_Pine".to_string(),
            weight: 1.0,
            scale_min: 0.8,
            scale_max: 1.5,
            rotation_random: true,
            height_offset: 0.0,
            slope_range: Some([0.0, 45.0]),
            tags: vec!["tree".to_string(), "pine".to_string()],
        });
    }

    if msg.contains("roble") || msg.contains("oak") {
        assets.push(AssetRule {
            asset_path: "/Game/Environment/Trees/SM_Oak".to_string(),
            weight: 1.0,
            scale_min: 0.9,
            scale_max: 1.3,
            rotation_random: true,
            height_offset: 0.0,
            slope_range: Some([0.0, 30.0]),
            tags: vec!["tree".to_string(), "oak".to_string()],
        });
    }

    if msg.contains("arbusto") || msg.contains("bush") || msg.contains("matorral") {
        assets.push(AssetRule {
            asset_path: "/Game/Environment/Foliage/SM_Bush".to_string(),
            weight: 2.0,
            scale_min: 0.5,
            scale_max: 1.2,
            rotation_random: true,
            height_offset: -10.0,
            slope_range: Some([0.0, 60.0]),
            tags: vec!["foliage".to_string(), "bush".to_string()],
        });
    }

    if msg.contains("roca") || msg.contains("rock") || msg.contains("piedra") {
        assets.push(AssetRule {
            asset_path: "/Game/Environment/Rocks/SM_Rock".to_string(),
            weight: 1.0,
            scale_min: 0.5,
            scale_max: 2.0,
            rotation_random: true,
            height_offset: 0.0,
            slope_range: Some([0.0, 80.0]),
            tags: vec!["rock".to_string()],
        });
    }

    // Si no hay assets específicos, usar defaults del bioma
    if assets.is_empty() {
        assets = get_biome_default_assets(biome);
    }

    assets
}

fn get_biome_default_assets(biome: &str) -> Vec<AssetRule> {
    match biome {
        "temperate_forest" => vec![
            AssetRule {
                asset_path: "/Game/Environment/Trees/SM_Tree_Generic".to_string(),
                weight: 1.0,
                scale_min: 0.8,
                scale_max: 1.5,
                rotation_random: true,
                height_offset: 0.0,
                slope_range: Some([0.0, 45.0]),
                tags: vec!["tree".to_string()],
            },
        ],
        "desert" => vec![
            AssetRule {
                asset_path: "/Game/Environment/Desert/SM_Cactus".to_string(),
                weight: 1.0,
                scale_min: 0.7,
                scale_max: 1.3,
                rotation_random: true,
                height_offset: 0.0,
                slope_range: Some([0.0, 30.0]),
                tags: vec!["cactus".to_string()],
            },
        ],
        "arctic" => vec![
            AssetRule {
                asset_path: "/Game/Environment/Arctic/SM_Pine_Snow".to_string(),
                weight: 1.0,
                scale_min: 0.6,
                scale_max: 1.2,
                rotation_random: true,
                height_offset: 0.0,
                slope_range: Some([0.0, 50.0]),
                tags: vec!["tree".to_string(), "snow".to_string()],
            },
        ],
        _ => vec![
            AssetRule {
                asset_path: "/Engine/BasicShapes/Cube".to_string(),
                weight: 1.0,
                scale_min: 1.0,
                scale_max: 1.0,
                rotation_random: false,
                height_offset: 0.0,
                slope_range: None,
                tags: vec!["placeholder".to_string()],
            },
        ],
    }
}

fn determine_distribution(msg: &str, command_type: &str) -> DistributionPattern {
    let pattern = if msg.contains("grid") || msg.contains("cuadrícula") {
        "grid"
    } else if msg.contains("orgánico") || msg.contains("natural") || msg.contains("poisson") {
        "poisson"
    } else if msg.contains("grupo") || msg.contains("cluster") || msg.contains("manchas") {
        "cluster"
    } else if msg.contains("camino") || msg.contains("path") || msg.contains("línea") {
        "path_following"
    } else {
        "random"
    };

    let spacing = if command_type.contains("forest") {
        500.0  // 5 metros entre árboles
    } else if command_type.contains("grass") {
        50.0   // 50cm entre pastos
    } else {
        300.0
    };

    let seed = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_micros() as i32;

    DistributionPattern {
        pattern_type: pattern.to_string(),
        spacing,
        jitter: 0.3,
        seed,
        avoid_overlap: true,
    }
}

// ============================================================
// GENERADOR DE RESPUESTA CON EXPLICACIÓN
// ============================================================

pub fn generate_pcg_explanation(cmd: &PCGCommand) -> String {
    format!(
        r#"Kukuku... ¡Excelente petición, colega! Voy a generar tu {} con precisión científica.

📊 **ANÁLISIS DEL SISTEMA:**
- Bioma: {} 
- Densidad: {:.0}% (óptimo para rendimiento/estética)
- Área: {:.1}m x {:.1}m
- Patrón de distribución: {}
- Assets detectados: {} tipos

🔬 **CONFIGURACIÓN PCG:**
- Spacing: {:.2}m (evita solapamiento)
- Jitter: {:.0}% (naturalidad)
- Seed: {} (reproducible)
- Física: {}
- Colisión: {}

⚙️ **PROCESO DE GENERACIÓN:**
1. Creando PCG Graph programáticamente...
2. Configurando Surface Sampler (landscape/spline)
3. Aplicando filtros de pendiente y altura
4. Distribuyendo {} assets con patrón {}
5. Aplicando variación de escala y rotación
6. Optimizando con LODs y culling

La generación tardará aproximadamente {:.1}s para {} instancias.
¡La ciencia del PCG está en marcha!"#,
        cmd.command_type,
        cmd.biome,
        cmd.density * 100.0,
        cmd.bounds.size[0] / 100.0,
        cmd.bounds.size[1] / 100.0,
        cmd.distribution.pattern_type,
        cmd.asset_rules.len(),
        cmd.distribution.spacing / 100.0,
        cmd.distribution.jitter * 100.0,
        cmd.distribution.seed,
        if cmd.physics { "Activada" } else { "Desactivada" },
        if cmd.collision { "Activada" } else { "Desactivada" },
        cmd.asset_rules.len(),
        cmd.distribution.pattern_type,
        estimate_generation_time(cmd),
        estimate_instance_count(cmd)
    )
}

fn estimate_generation_time(cmd: &PCGCommand) -> f32 {
    let instances = estimate_instance_count(cmd);
    (instances as f32 * 0.0001) + 0.5  // ~0.1ms por instancia + overhead
}

fn estimate_instance_count(cmd: &PCGCommand) -> i32 {
    let area = (cmd.bounds.size[0] / 100.0) * (cmd.bounds.size[1] / 100.0);  // m²
    let spacing_m = cmd.distribution.spacing / 100.0;
    let density_factor = cmd.density;
    
    ((area / (spacing_m * spacing_m)) * density_factor) as i32
}
