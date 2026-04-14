// gym/data_librarian.rs
// Agente Bibliotecario - Fetch, Voxelize, Compare
//
// Carga meshes de referencia (local / API), las voxeliza para simplificación,
// y compara punto a punto usando Chamfer + Hausdorff + IoU volumétrico.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

// ============================================================
// TIPOS GEOMÉTRICOS
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PointCloud {
    pub points: Vec<[f32; 3]>,
    pub normals: Vec<[f32; 3]>,
    pub bounds: BoundingBox,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BoundingBox {
    pub min: [f32; 3],
    pub max: [f32; 3],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoxelGrid {
    pub resolution: u32,
    pub voxels: Vec<bool>,
    pub world_size: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComparisonResult {
    pub similarity_score: f32,
    pub point_distance_avg: f32,
    pub hausdorff_distance: f32,
    pub volumetric_overlap: f32,
    pub heatmap_data: Vec<f32>,
}

// ============================================================
// DATA LIBRARIAN
// ============================================================

pub struct DataLibrarian {
    reference_cache: HashMap<String, PointCloud>,
    voxel_resolution: u32,
    pub asset_catalog: serde_json::Value,
}

impl DataLibrarian {
    pub fn new(voxel_resolution: u32) -> Self {
        let catalog_path = "config/assets_catalog.json";
        let asset_catalog = if let Ok(content) = std::fs::read_to_string(catalog_path) {
            serde_json::from_str(&content).unwrap_or(serde_json::json!({"categories": {}}))
        } else {
            log::warn!("⚠️ No se encontró config/assets_catalog.json, se usará catálogo vacío");
            serde_json::json!({"categories": {}})
        };

        Self {
            reference_cache: HashMap::new(),
            voxel_resolution,
            asset_catalog,
        }
    }

    pub fn get_asset_by_id(&self, id: &str) -> Option<serde_json::Value> {
        let categories = self.asset_catalog.get("categories")?.as_object()?;
        for cat in categories.values() {
            if let Some(assets) = cat.as_array() {
                for asset in assets {
                    if asset.get("id")? == id {
                        return Some(asset.clone());
                    }
                }
            }
        }
        None
    }

    pub fn get_random_asset(&self, category: &str) -> Option<serde_json::Value> {
        let assets = self.asset_catalog.get("categories")?.get(category)?.as_array()?;
        if assets.is_empty() { return None; }
        let idx = fastrand::usize(..assets.len());
        Some(assets[idx].clone())
    }

    // ============================================================
    // FETCH: Carga de referencias
    // ============================================================

    /// Carga un modelo 3D de referencia (cache-first)
    pub async fn fetch_reference(
        &mut self,
        source: &str,
    ) -> Result<PointCloud, Box<dyn std::error::Error>> {
        // Verificar cache primero
        if let Some(cached) = self.reference_cache.get(source) {
            log::info!("📚 Recuperado del caché: {}", source);
            return Ok(cached.clone());
        }

        log::info!("🔍 Cargando referencia: {}", source);

        let point_cloud = if source.starts_with("http") {
            self.fetch_from_api(source).await?
        } else {
            self.load_from_file(source)?
        };

        // Guardar en caché
        self.reference_cache
            .insert(source.to_string(), point_cloud.clone());

        Ok(point_cloud)
    }

    /// Carga desde archivo local (.obj, .ply, .gltf)
    fn load_from_file(&self, path: &str) -> Result<PointCloud, Box<dyn std::error::Error>> {
        let extension = Path::new(path)
            .extension()
            .and_then(|e| e.to_str())
            .ok_or(format!("Extensión de archivo inválida para el archivo: {}", path))?;

        match extension {
            "obj" => self.parse_obj(path),
            "ply" => self.parse_ply(path),
            "gltf" | "glb" => {
                log::warn!("⚠️ GLTF parsing requiere gltf crate — retornando placeholder");
                Ok(PointCloud {
                    points: vec![],
                    normals: vec![],
                    bounds: BoundingBox {
                        min: [0.0, 0.0, 0.0],
                        max: [1.0, 1.0, 1.0],
                    },
                })
            }
            _ => Err(format!("Formato no soportado: {}", extension).into()),
        }
    }

    /// Parser de OBJ (vértices y normales)
    fn parse_obj(&self, path: &str) -> Result<PointCloud, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let mut points = Vec::new();
        let mut normals = Vec::new();

        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.is_empty() {
                continue;
            }

            match parts[0] {
                "v" if parts.len() >= 4 => {
                    let x: f32 = parts[1].parse()?;
                    let y: f32 = parts[2].parse()?;
                    let z: f32 = parts[3].parse()?;
                    points.push([x, y, z]);
                }
                "vn" if parts.len() >= 4 => {
                    let x: f32 = parts[1].parse()?;
                    let y: f32 = parts[2].parse()?;
                    let z: f32 = parts[3].parse()?;
                    normals.push([x, y, z]);
                }
                _ => {}
            }
        }

        let bounds = Self::calculate_bounds(&points);

        Ok(PointCloud {
            points,
            normals,
            bounds,
        })
    }

    /// Parser de PLY
    fn parse_ply(&self, path: &str) -> Result<PointCloud, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let mut points = Vec::new();
        let mut in_data = false;

        for line in content.lines() {
            if line == "end_header" {
                in_data = true;
                continue;
            }
            if in_data {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 3 {
                    let x: f32 = parts[0].parse()?;
                    let y: f32 = parts[1].parse()?;
                    let z: f32 = parts[2].parse()?;
                    points.push([x, y, z]);
                }
            }
        }

        let bounds = Self::calculate_bounds(&points);

        Ok(PointCloud {
            points,
            normals: vec![],
            bounds,
        })
    }

    /// Descarga desde API (Poly Haven, Google Scanned Objects, etc.)
    async fn fetch_from_api(
        &self,
        url: &str,
    ) -> Result<PointCloud, Box<dyn std::error::Error>> {
        log::info!("🌐 Descargando desde: {}", url);

        // Implementación genérica con reqwest
        let response = reqwest::get(url).await?;
        let bytes = response.bytes().await?;

        // Intentar parsear como OBJ inline
        let text = String::from_utf8_lossy(&bytes);
        let mut points = Vec::new();

        for line in text.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 4 && parts[0] == "v" {
                if let (Ok(x), Ok(y), Ok(z)) = (
                    parts[1].parse::<f32>(),
                    parts[2].parse::<f32>(),
                    parts[3].parse::<f32>(),
                ) {
                    points.push([x, y, z]);
                }
            }
        }

        if points.is_empty() {
            return Err(format!("No se pudieron extraer vértices de: {}", url).into());
        }

        let bounds = Self::calculate_bounds(&points);

        Ok(PointCloud {
            points,
            normals: vec![],
            bounds,
        })
    }

    // ============================================================
    // VOXELIZE: Conversión a grid volumétrico
    // ============================================================

    /// Convierte PointCloud a matriz voxel simplificada
    pub fn voxelize(&self, point_cloud: &PointCloud) -> VoxelGrid {
        let bounds = &point_cloud.bounds;
        let world_size = Self::calculate_world_size(bounds);

        if world_size < f32::EPSILON {
            return VoxelGrid {
                resolution: self.voxel_resolution,
                voxels: vec![false; (self.voxel_resolution.pow(3)) as usize],
                world_size: 0.0,
            };
        }

        let voxel_size = world_size / self.voxel_resolution as f32;
        let total_voxels = (self.voxel_resolution.pow(3)) as usize;
        let mut voxels = vec![false; total_voxels];

        for point in &point_cloud.points {
            let x = ((point[0] - bounds.min[0]) / voxel_size).floor() as u32;
            let y = ((point[1] - bounds.min[1]) / voxel_size).floor() as u32;
            let z = ((point[2] - bounds.min[2]) / voxel_size).floor() as u32;

            if x < self.voxel_resolution
                && y < self.voxel_resolution
                && z < self.voxel_resolution
            {
                let index = Self::voxel_index(self.voxel_resolution, x, y, z);
                voxels[index] = true;
            }
        }

        let active = voxels.iter().filter(|&&v| v).count();
        log::info!(
            "🧊 Voxelizado: {} puntos → {} vóxeles activos",
            point_cloud.points.len(),
            active
        );

        VoxelGrid {
            resolution: self.voxel_resolution,
            voxels,
            world_size,
        }
    }

    fn voxel_index(resolution: u32, x: u32, y: u32, z: u32) -> usize {
        (x + y * resolution + z * resolution.pow(2)) as usize
    }

    // ============================================================
    // COMPARE: Comparación tensorial
    // ============================================================

    /// Comparación completa entre referencia y construcción de Daithon
    pub fn compare(
        &self,
        reference: &PointCloud,
        daithon_output: &PointCloud,
    ) -> ComparisonResult {
        log::info!("⚖️ Comparando construcciones...");

        // Protección ante nubes vacías
        if reference.points.is_empty() || daithon_output.points.is_empty() {
            return ComparisonResult {
                similarity_score: 0.0,
                point_distance_avg: f32::INFINITY,
                hausdorff_distance: f32::INFINITY,
                volumetric_overlap: 0.0,
                heatmap_data: vec![],
            };
        }

        let point_distance_avg = self.chamfer_distance(reference, daithon_output);
        let hausdorff_distance = self.hausdorff_distance(reference, daithon_output);
        let volumetric_overlap = self.volumetric_overlap(reference, daithon_output);

        let similarity_score = Self::calculate_similarity_score(
            point_distance_avg,
            hausdorff_distance,
            volumetric_overlap,
        );

        let heatmap_data = self.generate_error_heatmap(reference, daithon_output);

        ComparisonResult {
            similarity_score,
            point_distance_avg,
            hausdorff_distance,
            volumetric_overlap,
            heatmap_data,
        }
    }

    /// Distancia Chamfer (promedio de distancias mínimas bidireccional)
    fn chamfer_distance(&self, pc1: &PointCloud, pc2: &PointCloud) -> f32 {
        let dist_1_to_2: f32 = pc1
            .points
            .iter()
            .map(|p1| {
                pc2.points
                    .iter()
                    .map(|p2| Self::point_distance(p1, p2))
                    .fold(f32::INFINITY, f32::min)
            })
            .sum();

        let dist_2_to_1: f32 = pc2
            .points
            .iter()
            .map(|p2| {
                pc1.points
                    .iter()
                    .map(|p1| Self::point_distance(p1, p2))
                    .fold(f32::INFINITY, f32::min)
            })
            .sum();

        (dist_1_to_2 / pc1.points.len() as f32 + dist_2_to_1 / pc2.points.len() as f32)
            / 2.0
    }

    /// Distancia Hausdorff (peor caso)
    fn hausdorff_distance(&self, pc1: &PointCloud, pc2: &PointCloud) -> f32 {
        let max_1 = pc1
            .points
            .iter()
            .map(|p1| {
                pc2.points
                    .iter()
                    .map(|p2| Self::point_distance(p1, p2))
                    .fold(f32::INFINITY, f32::min)
            })
            .fold(f32::NEG_INFINITY, f32::max);

        let max_2 = pc2
            .points
            .iter()
            .map(|p2| {
                pc1.points
                    .iter()
                    .map(|p1| Self::point_distance(p1, p2))
                    .fold(f32::INFINITY, f32::min)
            })
            .fold(f32::NEG_INFINITY, f32::max);

        max_1.max(max_2)
    }

    /// Overlap volumétrico (IoU sobre voxels)
    fn volumetric_overlap(&self, pc1: &PointCloud, pc2: &PointCloud) -> f32 {
        let voxels1 = self.voxelize(pc1);
        let voxels2 = self.voxelize(pc2);

        let mut intersection = 0u32;
        let mut union = 0u32;

        for i in 0..voxels1.voxels.len() {
            if voxels1.voxels[i] && voxels2.voxels[i] {
                intersection += 1;
            }
            if voxels1.voxels[i] || voxels2.voxels[i] {
                union += 1;
            }
        }

        if union == 0 {
            return 0.0;
        }

        intersection as f32 / union as f32
    }

    /// Score de similitud combinado (0.0 – 1.0)
    fn calculate_similarity_score(
        chamfer: f32,
        hausdorff: f32,
        volumetric: f32,
    ) -> f32 {
        let normalized_chamfer = (1.0 - (chamfer / 10.0).min(1.0)).max(0.0);
        let normalized_hausdorff = (1.0 - (hausdorff / 10.0).min(1.0)).max(0.0);

        // 40% Chamfer, 30% Volumetric, 30% Hausdorff
        0.4 * normalized_chamfer + 0.3 * volumetric + 0.3 * normalized_hausdorff
    }

    /// Genera heatmap de errores para visualización
    fn generate_error_heatmap(
        &self,
        reference: &PointCloud,
        output: &PointCloud,
    ) -> Vec<f32> {
        reference
            .points
            .iter()
            .map(|p_ref| {
                output
                    .points
                    .iter()
                    .map(|p_out| Self::point_distance(p_ref, p_out))
                    .fold(f32::INFINITY, f32::min)
            })
            .collect()
    }

    // ============================================================
    // REPORTES
    // ============================================================

    /// Genera reporte de comparación
    pub fn generate_comparison_report(&self, result: &ComparisonResult) -> serde_json::Value {
        serde_json::json!({
            "similarity_percentage": (result.similarity_score * 100.0),
            "metrics": {
                "chamfer_distance": result.point_distance_avg,
                "hausdorff_distance": result.hausdorff_distance,
                "volumetric_iou": result.volumetric_overlap,
            },
            "quality_rating": Self::quality_rating(result.similarity_score),
            "heatmap_available": !result.heatmap_data.is_empty(),
        })
    }

    fn quality_rating(score: f32) -> &'static str {
        match score {
            s if s > 0.95 => "PERFECTO",
            s if s > 0.85 => "EXCELENTE",
            s if s > 0.70 => "BUENO",
            s if s > 0.50 => "ACEPTABLE",
            _ => "REQUIERE MEJORA",
        }
    }

    // ============================================================
    // UTILIDADES GEOMÉTRICAS
    // ============================================================

    fn point_distance(p1: &[f32; 3], p2: &[f32; 3]) -> f32 {
        ((p1[0] - p2[0]).powi(2) + (p1[1] - p2[1]).powi(2) + (p1[2] - p2[2]).powi(2))
            .sqrt()
    }

    pub fn calculate_bounds(points: &[[f32; 3]]) -> BoundingBox {
        let mut min = [f32::INFINITY; 3];
        let mut max = [f32::NEG_INFINITY; 3];

        for point in points {
            for i in 0..3 {
                min[i] = min[i].min(point[i]);
                max[i] = max[i].max(point[i]);
            }
        }

        BoundingBox { min, max }
    }

    fn calculate_world_size(bounds: &BoundingBox) -> f32 {
        let dx = bounds.max[0] - bounds.min[0];
        let dy = bounds.max[1] - bounds.min[1];
        let dz = bounds.max[2] - bounds.min[2];
        dx.max(dy).max(dz)
    }
}

// ============================================================
// GENERADOR DE GEOMETRÍA SINTÉTICA
// ============================================================

pub struct SyntheticGeometryGenerator;

impl SyntheticGeometryGenerator {
    pub fn generate(formula: &str, parameters: &[f32]) -> PointCloud {
        let f = formula.split('(').next().unwrap_or(formula);
        let pc = match f {
            "pyramid_truncated" => Self::pyramid_truncated(parameters),
            "hollow_cylinder" => Self::hollow_cylinder(parameters),
            "star_extrusion" => Self::star_extrusion(parameters),
            "bridge_arched" => Self::bridge_arched(parameters),
            "biome_necrotic" => Self::biome_necrotic(parameters),
            "house" => Self::house(parameters),
            "cabin" => Self::cabin(parameters),
            "mountain" => Self::mountain(parameters),
            "mountain_cascade" => Self::mountain_cascade(parameters),
            "car" => Self::car(parameters),
            "airplane" => Self::airplane(parameters),
            "horse" => Self::horse(parameters),
            "bird" => Self::bird(parameters),
            "fish" => Self::fish(parameters),
            "tree_pine" => Self::tree_pine(parameters),
            "tree_oak" => Self::tree_oak(parameters),
            "sphere" => Self::sphere(parameters),
            "torus" => Self::torus(parameters),
            "tower" => Self::tower(parameters),
            "wall_fortress" => Self::wall_fortress(parameters),
            "boat" => Self::boat(parameters),
            "table" => Self::table(parameters),
            "chair" => Self::chair(parameters),
            "terrain_hills" => Self::terrain_hills(parameters),
            "village" => Self::village(parameters),
            _ => {
                log::warn!("⚠️ Fórmula desconocida: {}, usando cubo", formula);
                Self::default_cube()
            }
        };
        pc
    }

    fn pyramid_truncated(params: &[f32]) -> PointCloud {
        let base = params.get(0).copied().unwrap_or(4.0);
        let top = params.get(1).copied().unwrap_or(2.0);
        let height = params.get(2).copied().unwrap_or(5.0);

        let mut points = Vec::new();

        // Base inferior
        points.push([-base / 2.0, 0.0, -base / 2.0]);
        points.push([base / 2.0, 0.0, -base / 2.0]);
        points.push([base / 2.0, 0.0, base / 2.0]);
        points.push([-base / 2.0, 0.0, base / 2.0]);

        // Base superior
        points.push([-top / 2.0, height, -top / 2.0]);
        points.push([top / 2.0, height, -top / 2.0]);
        points.push([top / 2.0, height, top / 2.0]);
        points.push([-top / 2.0, height, top / 2.0]);

        // Puntos intermedios
        for t in 1..10 {
            let ratio = t as f32 / 10.0;
            let current_size = base + (top - base) * ratio;
            let y = height * ratio;

            points.push([-current_size / 2.0, y, -current_size / 2.0]);
            points.push([current_size / 2.0, y, -current_size / 2.0]);
            points.push([current_size / 2.0, y, current_size / 2.0]);
            points.push([-current_size / 2.0, y, current_size / 2.0]);
        }

        let bounds = DataLibrarian::calculate_bounds(&points);
        PointCloud { points, normals: vec![], bounds }
    }

    fn hollow_cylinder(params: &[f32]) -> PointCloud {
        let outer_radius = params.get(0).copied().unwrap_or(2.0);
        let inner_radius = params.get(1).copied().unwrap_or(1.5);
        let height = params.get(2).copied().unwrap_or(4.0);

        let mut points = Vec::new();
        let segments = 32;

        for i in 0..segments {
            let angle = (i as f32 / segments as f32) * 2.0 * std::f32::consts::PI;
            let cos = angle.cos();
            let sin = angle.sin();

            for h in 0..10 {
                let y = (h as f32 / 10.0) * height;
                points.push([outer_radius * cos, y, outer_radius * sin]);
                points.push([inner_radius * cos, y, inner_radius * sin]);
            }
        }

        let bounds = DataLibrarian::calculate_bounds(&points);
        PointCloud { points, normals: vec![], bounds }
    }

    fn star_extrusion(params: &[f32]) -> PointCloud {
        let points_count = params.get(0).copied().unwrap_or(5.0) as u32;
        let inner_radius = params.get(1).copied().unwrap_or(1.0);
        let outer_radius = params.get(2).copied().unwrap_or(2.0);
        let depth = params.get(3).copied().unwrap_or(0.5);

        let mut points = Vec::new();

        for i in 0..(points_count * 2) {
            let angle =
                (i as f32 / (points_count * 2) as f32) * 2.0 * std::f32::consts::PI;
            let radius = if i % 2 == 0 { outer_radius } else { inner_radius };

            let x = radius * angle.cos();
            let z = radius * angle.sin();

            points.push([x, 0.0, z]);
            points.push([x, depth, z]);
        }

        let bounds = DataLibrarian::calculate_bounds(&points);
        PointCloud { points, normals: vec![], bounds }
    }

    fn bridge_arched(params: &[f32]) -> PointCloud {
        let arcs = params.get(0).copied().unwrap_or(3.0) as u32;
        let span = params.get(1).copied().unwrap_or(10.0);
        let arc_height = params.get(2).copied().unwrap_or(3.0);

        let mut points = Vec::new();

        for arc in 0..arcs {
            let offset_x = arc as f32 * span;

            for t in 0..20 {
                let ratio = t as f32 / 20.0;
                let x = offset_x + ratio * span;
                let y = arc_height * (1.0 - (2.0 * ratio - 1.0).powi(2));

                points.push([x, y, -0.5]);
                points.push([x, y, 0.5]);
            }
        }

        let bounds = DataLibrarian::calculate_bounds(&points);
        PointCloud { points, normals: vec![], bounds }
    }

    fn biome_necrotic(params: &[f32]) -> PointCloud {
        let trees = params.get(0).copied().unwrap_or(20.0) as u32;
        let _fog_density = params.get(1).copied().unwrap_or(0.7);
        let mushroom_clusters = params.get(2).copied().unwrap_or(8.0) as u32;

        let mut points = Vec::new();
        // NOTE: Uses fastrand::i32 or fastrand::f32
        
        // Árboles muertos
        for _ in 0..trees {
            let base_x = fastrand::f32() * 50.0 - 25.0;
            let base_z = fastrand::f32() * 50.0 - 25.0;
            let height = 3.0 + fastrand::f32() * 5.0;

            for h in 0..10 {
                let y = (h as f32 / 10.0) * height;
                points.push([base_x, y, base_z]);
            }

            // Ramas retorcidas
            for _ in 0..(3 + fastrand::u32(0..3)) {
                let branch_start = height * 0.5 + fastrand::f32() * height * 0.3;
                let angle = fastrand::f32() * 2.0 * std::f32::consts::PI;
                let branch_length = 1.0 + fastrand::f32() * 2.0;

                for b in 0..5 {
                    let ratio = b as f32 / 5.0;
                    points.push([
                        base_x + angle.cos() * ratio * branch_length,
                        branch_start + ratio * branch_length * 0.3,
                        base_z + angle.sin() * ratio * branch_length,
                    ]);
                }
            }
        }

        // Hongos bioluminiscentes
        for _ in 0..mushroom_clusters {
            let cx = fastrand::f32() * 40.0 - 20.0;
            let cz = fastrand::f32() * 40.0 - 20.0;

            for _ in 0..(3 + fastrand::u32(0..5)) {
                let ox = cx + (fastrand::f32() - 0.5) * 2.0;
                let oz = cz + (fastrand::f32() - 0.5) * 2.0;
                let mh = 0.2 + fastrand::f32() * 0.5;

                points.push([ox, 0.0, oz]);
                points.push([ox, mh * 0.7, oz]);

                for a in 0..8 {
                    let angle = (a as f32 / 8.0) * 2.0 * std::f32::consts::PI;
                    let cap_r = 0.3 + fastrand::f32() * 0.2;
                    points.push([
                        ox + angle.cos() * cap_r,
                        mh,
                        oz + angle.sin() * cap_r,
                    ]);
                }
            }
        }

        let bounds = DataLibrarian::calculate_bounds(&points);
        PointCloud { points, normals: vec![], bounds }
    }

    // ============= HOUSE =============
    fn house(params: &[f32]) -> PointCloud {
        let w = params.get(0).copied().unwrap_or(4.0);
        let h = params.get(1).copied().unwrap_or(3.0);
        let d = params.get(2).copied().unwrap_or(5.0);
        let roof_h = params.get(3).copied().unwrap_or(2.0);
        let mut pts = Vec::new();
        let s = 8;
        for i in 0..=s { for j in 0..=s { let u=i as f32/s as f32; let v=j as f32/s as f32;
            pts.push([-w/2.0+w*u, 0.0, -d/2.0+d*v]); // floor
            pts.push([-w/2.0+w*u, h, -d/2.0+d*v]); // ceiling
            pts.push([-w/2.0, h*v, -d/2.0+d*u]); // left wall
            pts.push([w/2.0, h*v, -d/2.0+d*u]); // right wall
            pts.push([-w/2.0+w*u, h*v, -d/2.0]); // front wall
            pts.push([-w/2.0+w*u, h*v, d/2.0]); // back wall
        }}
        // Roof (triangular)
        for i in 0..=s { for j in 0..=s { let u=i as f32/s as f32; let v=j as f32/s as f32;
            let rh = roof_h * (1.0-(2.0*u-1.0).abs());
            pts.push([-w/2.0+w*u, h+rh, -d/2.0+d*v]);
        }}
        // Door
        for i in 0..5 { let v=i as f32/5.0;
            pts.push([-0.4, h*v*0.7, -d/2.0-0.05]);
            pts.push([0.4, h*v*0.7, -d/2.0-0.05]);
        }
        let bounds = DataLibrarian::calculate_bounds(&pts);
        PointCloud { points: pts, normals: vec![], bounds }
    }

    fn cabin(params: &[f32]) -> PointCloud {
        let mut p = Self::house(&[3.0, 2.5, 4.0, 1.5]);
        // Add chimney
        for i in 0..6 { let y = 2.5+1.5+i as f32*0.3;
            p.points.push([1.0,y,-1.0]); p.points.push([1.4,y,-1.0]);
            p.points.push([1.0,y,-0.6]); p.points.push([1.4,y,-0.6]);
        }
        // Porch pillars
        for i in 0..8 { let y=i as f32*0.35;
            p.points.push([-1.5,y,-2.5]); p.points.push([1.5,y,-2.5]);
        }
        p.bounds = DataLibrarian::calculate_bounds(&p.points);
        p
    }

    // ============= MOUNTAIN =============
    fn mountain(params: &[f32]) -> PointCloud {
        let r = params.get(0).copied().unwrap_or(8.0);
        let h = params.get(1).copied().unwrap_or(10.0);
        let mut pts = Vec::new();
        let rings = 20; let segs = 24;
        for ri in 0..=rings { let t=ri as f32/rings as f32;
            let y = h*t;
            let cr = r*(1.0-t*t); // parabolic taper
            for si in 0..segs { let a=(si as f32/segs as f32)*std::f32::consts::TAU;
                let noise = (fastrand::f32()-0.5)*cr*0.15;
                pts.push([a.cos()*(cr+noise), y, a.sin()*(cr+noise)]);
            }
        }
        pts.push([0.0, h, 0.0]); // peak
        let bounds = DataLibrarian::calculate_bounds(&pts);
        PointCloud { points: pts, normals: vec![], bounds }
    }

    fn mountain_cascade(params: &[f32]) -> PointCloud {
        let mut p = Self::mountain(&[10.0, 12.0]);
        // Waterfall path down the side
        for i in 0..40 { let t=i as f32/40.0;
            let y=12.0*(1.0-t); let x=3.0+t*2.0+(fastrand::f32()-0.5)*0.3;
            let z=(fastrand::f32()-0.5)*0.5;
            p.points.push([x,y,z]);
            // Splash width
            p.points.push([x+0.2,y,z+0.2]); p.points.push([x-0.2,y,z-0.2]);
        }
        // Pool at base
        for i in 0..16 { let a=(i as f32/16.0)*std::f32::consts::TAU;
            for r in 1..4 { let rr=r as f32*0.6;
                p.points.push([5.0+a.cos()*rr, 0.1, a.sin()*rr]);
            }
        }
        p.bounds = DataLibrarian::calculate_bounds(&p.points);
        p
    }

    // ============= CAR =============
    fn car(params: &[f32]) -> PointCloud {
        let l = params.get(0).copied().unwrap_or(4.0);
        let w = params.get(1).copied().unwrap_or(1.8);
        let h = params.get(2).copied().unwrap_or(1.4);
        let mut pts = Vec::new();
        let s = 10;
        // Body box
        for i in 0..=s { for j in 0..=s { let u=i as f32/s as f32; let v=j as f32/s as f32;
            pts.push([-l/2.0+l*u, 0.3, -w/2.0+w*v]); // bottom
            pts.push([-l/2.0+l*u, h*0.5, -w/2.0+w*v]); // hood level
            pts.push([-l/2.0+l*u, 0.3+h*0.5*v, -w/2.0]); // left
            pts.push([-l/2.0+l*u, 0.3+h*0.5*v, w/2.0]); // right
        }}
        // Cabin (tapered top)
        for i in 0..=6 { for j in 0..=6 { let u=i as f32/6.0; let v=j as f32/6.0;
            let cw = w*0.85; let cl = l*0.45;
            pts.push([-cl/2.0+cl*u, h*0.5+h*0.5*v, -cw/2.0]);
            pts.push([-cl/2.0+cl*u, h*0.5+h*0.5*v, cw/2.0]);
            pts.push([-cl/2.0+cl*u, h, -cw/2.0+cw*v]);
        }}
        // Wheels (4 circles)
        for &(wx,wz) in &[(-l*0.3,-w/2.0),(l*0.3,-w/2.0),(-l*0.3,w/2.0),(l*0.3,w/2.0)] {
            for a in 0..12 { let ang=(a as f32/12.0)*std::f32::consts::TAU;
                pts.push([wx+ang.cos()*0.3, 0.3+ang.sin()*0.3, wz]);
            }
        }
        let bounds = DataLibrarian::calculate_bounds(&pts);
        PointCloud { points: pts, normals: vec![], bounds }
    }

    // ============= AIRPLANE =============
    fn airplane(params: &[f32]) -> PointCloud {
        let fuse_l = params.get(0).copied().unwrap_or(8.0);
        let wing_span = params.get(1).copied().unwrap_or(10.0);
        let mut pts = Vec::new();
        // Fuselage (elongated cylinder)
        for i in 0..30 { let t=i as f32/30.0;
            let x = -fuse_l/2.0+fuse_l*t;
            let r = 0.5*(1.0-(2.0*t-1.0).powi(2)).max(0.1); // tapered
            for s in 0..8 { let a=(s as f32/8.0)*std::f32::consts::TAU;
                pts.push([x, a.sin()*r+0.5, a.cos()*r]);
            }
        }
        // Wings
        for i in 0..20 { let t=i as f32/20.0;
            let z = -wing_span/2.0+wing_span*t;
            let chord = 1.2*(1.0-(2.0*t-1.0).abs()*0.3);
            pts.push([-chord/2.0, 0.5, z]);
            pts.push([chord/2.0, 0.5, z]);
            pts.push([0.0, 0.55, z]);
        }
        // Tail vertical
        for i in 0..6 { let t=i as f32/6.0;
            pts.push([fuse_l/2.0-0.5, 0.5+t*1.5, 0.0]);
            pts.push([fuse_l/2.0-0.5-t*0.5, 0.5+t*1.5, 0.0]);
        }
        // Tail horizontal
        for i in 0..8 { let t=i as f32/8.0;
            let z=-1.5+3.0*t;
            pts.push([fuse_l/2.0-0.8, 0.5, z]);
        }
        let bounds = DataLibrarian::calculate_bounds(&pts);
        PointCloud { points: pts, normals: vec![], bounds }
    }

    // ============= HORSE =============
    fn horse(params: &[f32]) -> PointCloud {
        let scale = params.get(0).copied().unwrap_or(1.0);
        let mut pts = Vec::new();
        let s = |x:f32,y:f32,z:f32| [x*scale,y*scale,z*scale];
        // Body (ellipsoid)
        for i in 0..16 { for j in 0..10 { let u=(i as f32/16.0)*std::f32::consts::TAU; let v=(j as f32/10.0)*std::f32::consts::PI;
            pts.push(s(u.cos()*1.5*v.sin(), 1.2+v.cos()*0.6, u.sin()*0.6*v.sin()));
        }}
        // Legs (4)
        for &(lx,lz) in &[(-0.8_f32,-0.3),(-0.8,0.3),(0.8,-0.3),(0.8,0.3)] {
            for i in 0..8 { let y=i as f32*0.15; pts.push(s(lx,y,lz)); }
        }
        // Neck + Head
        for i in 0..8 { let t=i as f32/8.0;
            pts.push(s(-1.5-t*0.8, 1.2+t*1.0, 0.0));
            pts.push(s(-1.5-t*0.8, 1.2+t*1.0, 0.15));
            pts.push(s(-1.5-t*0.8, 1.2+t*1.0, -0.15));
        }
        // Head box
        for i in 0..4 { let t=i as f32/4.0;
            pts.push(s(-2.3-t*0.6, 2.0+t*0.2, 0.0));
            pts.push(s(-2.3-t*0.6, 2.2, 0.1));
            pts.push(s(-2.3-t*0.6, 2.2, -0.1));
        }
        // Tail
        for i in 0..6 { let t=i as f32/6.0;
            pts.push(s(1.5+t*0.8, 1.2-t*0.5, (fastrand::f32()-0.5)*0.2));
        }
        let bounds = DataLibrarian::calculate_bounds(&pts);
        PointCloud { points: pts, normals: vec![], bounds }
    }

    // ============= BIRD =============
    fn bird(params: &[f32]) -> PointCloud {
        let scale = params.get(0).copied().unwrap_or(1.0);
        let mut pts = Vec::new();
        // Body
        for i in 0..12 { for j in 0..8 { let u=(i as f32/12.0)*std::f32::consts::TAU; let v=(j as f32/8.0)*std::f32::consts::PI;
            pts.push([u.cos()*0.4*v.sin()*scale, 0.3*v.cos()*scale, u.sin()*0.3*v.sin()*scale]);
        }}
        // Wings spread
        for &side in &[-1.0_f32, 1.0] { for i in 0..12 { let t=i as f32/12.0;
            let span = t*2.0; let droop = -t*t*0.3;
            pts.push([0.0, droop*scale, side*span*scale]);
            pts.push([0.1*scale, droop*scale, side*span*scale]);
            pts.push([-0.2*(1.0-t)*scale, droop*scale, side*span*scale]);
        }}
        // Head + beak
        pts.push([-0.5*scale, 0.1*scale, 0.0]);
        pts.push([-0.7*scale, 0.05*scale, 0.0]);
        // Tail
        for i in 0..4 { let t=i as f32/4.0;
            pts.push([0.4*scale+t*0.4*scale, t*0.15*scale, (fastrand::f32()-0.5)*0.2*scale]);
        }
        let bounds = DataLibrarian::calculate_bounds(&pts);
        PointCloud { points: pts, normals: vec![], bounds }
    }

    // ============= FISH =============
    fn fish(params: &[f32]) -> PointCloud {
        let scale = params.get(0).copied().unwrap_or(1.0);
        let mut pts = Vec::new();
        // Body profile (tapered ellipse)
        for i in 0..20 { let t=i as f32/20.0;
            let x = (-1.0+2.0*t)*scale;
            let r = 0.4*(1.0-(2.0*t-1.0).powi(2)).max(0.05)*scale;
            for s in 0..8 { let a=(s as f32/8.0)*std::f32::consts::TAU;
                pts.push([x, a.sin()*r, a.cos()*r*0.7]);
            }
        }
        // Tail fin
        for i in 0..5 { let t=i as f32/5.0;
            pts.push([scale+t*0.4*scale, t*0.4*scale, 0.0]);
            pts.push([scale+t*0.4*scale, -t*0.4*scale, 0.0]);
        }
        // Dorsal fin
        for i in 0..6 { let t=i as f32/6.0;
            pts.push([(-0.3+t*0.6)*scale, 0.3*scale+t*0.2*scale, 0.0]);
        }
        let bounds = DataLibrarian::calculate_bounds(&pts);
        PointCloud { points: pts, normals: vec![], bounds }
    }

    // ============= TREES =============
    fn tree_pine(params: &[f32]) -> PointCloud {
        let h = params.get(0).copied().unwrap_or(6.0);
        let mut pts = Vec::new();
        // Trunk
        for i in 0..10 { let y=i as f32*h*0.06;
            for s in 0..6 { let a=(s as f32/6.0)*std::f32::consts::TAU;
                pts.push([a.cos()*0.15, y, a.sin()*0.15]);
            }
        }
        // Cone layers
        for layer in 0..4 { let ly=h*0.3+layer as f32*h*0.17;
            let lr=h*0.3*(1.0-layer as f32*0.2);
            for s in 0..12 { let a=(s as f32/12.0)*std::f32::consts::TAU;
                for r in 0..3 { let rr=r as f32/3.0*lr;
                    pts.push([a.cos()*rr, ly+rr*0.3, a.sin()*rr]);
                }
            }
        }
        pts.push([0.0, h, 0.0]);
        let bounds = DataLibrarian::calculate_bounds(&pts);
        PointCloud { points: pts, normals: vec![], bounds }
    }

    fn tree_oak(params: &[f32]) -> PointCloud {
        let h = params.get(0).copied().unwrap_or(5.0);
        let mut pts = Vec::new();
        // Trunk
        for i in 0..8 { let y=i as f32*h*0.08;
            for s in 0..6 { let a=(s as f32/6.0)*std::f32::consts::TAU;
                pts.push([a.cos()*0.2, y, a.sin()*0.2]);
            }
        }
        // Sphere crown
        let cy = h*0.65; let cr = h*0.35;
        for i in 0..16 { for j in 0..12 { let u=(i as f32/16.0)*std::f32::consts::TAU; let v=(j as f32/12.0)*std::f32::consts::PI;
            let noise = 1.0+(fastrand::f32()-0.5)*0.3;
            pts.push([u.cos()*cr*v.sin()*noise, cy+v.cos()*cr*noise, u.sin()*cr*v.sin()*noise]);
        }}
        let bounds = DataLibrarian::calculate_bounds(&pts);
        PointCloud { points: pts, normals: vec![], bounds }
    }

    // ============= PRIMITIVES =============
    fn sphere(params: &[f32]) -> PointCloud {
        let r = params.get(0).copied().unwrap_or(2.0);
        let mut pts = Vec::new();
        for i in 0..20 { for j in 0..16 { let u=(i as f32/20.0)*std::f32::consts::TAU; let v=(j as f32/16.0)*std::f32::consts::PI;
            pts.push([r*u.cos()*v.sin(), r*v.cos(), r*u.sin()*v.sin()]);
        }}
        let bounds = DataLibrarian::calculate_bounds(&pts);
        PointCloud { points: pts, normals: vec![], bounds }
    }

    fn torus(params: &[f32]) -> PointCloud {
        let major_r = params.get(0).copied().unwrap_or(2.0);
        let minor_r = params.get(1).copied().unwrap_or(0.5);
        let mut pts = Vec::new();
        for i in 0..24 { for j in 0..12 { let u=(i as f32/24.0)*std::f32::consts::TAU; let v=(j as f32/12.0)*std::f32::consts::TAU;
            pts.push([(major_r + minor_r * v.cos()) * u.cos(), minor_r * v.sin(), (major_r + minor_r * v.cos()) * u.sin()]);
        }}
        let bounds = DataLibrarian::calculate_bounds(&pts);
        PointCloud { points: pts, normals: vec![], bounds }
    }

    // ============= STRUCTURES =============
    fn tower(params: &[f32]) -> PointCloud {
        let h = params.get(0).copied().unwrap_or(10.0);
        let r = params.get(1).copied().unwrap_or(1.5);
        let mut pts = Vec::new();
        for i in 0..20 { let y=i as f32*h/20.0;
            let cr = r*(1.0-i as f32*0.01);
            for s in 0..12 { let a=(s as f32/12.0)*std::f32::consts::TAU;
                pts.push([a.cos()*cr, y, a.sin()*cr]);
            }
        }
        // Battlement on top
        for s in 0..12 { let a=(s as f32/12.0)*std::f32::consts::TAU;
            let bh = if s%2==0 { 0.8 } else { 0.3 };
            pts.push([a.cos()*r*0.95, h+bh, a.sin()*r*0.95]);
        }
        let bounds = DataLibrarian::calculate_bounds(&pts);
        PointCloud { points: pts, normals: vec![], bounds }
    }

    fn wall_fortress(params: &[f32]) -> PointCloud {
        let length = params.get(0).copied().unwrap_or(12.0);
        let h = params.get(1).copied().unwrap_or(4.0);
        let mut pts = Vec::new();
        let s = 20;
        for i in 0..=s { let u=i as f32/s as f32;
            for j in 0..=8 { let v=j as f32/8.0;
                pts.push([-length/2.0+length*u, h*v, 0.0]);
                pts.push([-length/2.0+length*u, h*v, 0.6]);
            }
        }
        // Merlons
        for i in 0..=s { let u=i as f32/s as f32;
            if i%2==0 { pts.push([-length/2.0+length*u, h+0.6, 0.3]); }
        }
        let bounds = DataLibrarian::calculate_bounds(&pts);
        PointCloud { points: pts, normals: vec![], bounds }
    }

    fn boat(params: &[f32]) -> PointCloud {
        let l = params.get(0).copied().unwrap_or(5.0);
        let mut pts = Vec::new();
        for i in 0..20 { let t=i as f32/20.0;
            let x=-l/2.0+l*t;
            let hw=0.8*(1.0-(2.0*t-1.0).powi(2)).max(0.1);
            for j in 0..8 { let a=(j as f32/8.0)*std::f32::consts::PI;
                pts.push([x, -a.sin()*0.5, -hw+2.0*hw*(j as f32/8.0)]);
            }
            pts.push([x, 0.0, -hw]); pts.push([x, 0.0, hw]); // gunwale
        }
        // Mast
        for i in 0..8 { pts.push([0.0, i as f32*0.5, 0.0]); }
        let bounds = DataLibrarian::calculate_bounds(&pts);
        PointCloud { points: pts, normals: vec![], bounds }
    }

    // ============= FURNITURE =============
    fn table(params: &[f32]) -> PointCloud {
        let w = params.get(0).copied().unwrap_or(2.0);
        let h = params.get(1).copied().unwrap_or(1.0);
        let mut pts = Vec::new();
        // Top surface
        for i in 0..=8 { for j in 0..=8 { let u=i as f32/8.0; let v=j as f32/8.0;
            pts.push([-w/2.0+w*u, h, -w/2.0+w*v]);
        }}
        // 4 legs
        for &(lx,lz) in &[(-w/2.0+0.1,-w/2.0+0.1),(w/2.0-0.1,-w/2.0+0.1),(-w/2.0+0.1,w/2.0-0.1),(w/2.0-0.1,w/2.0-0.1)] {
            for i in 0..6 { pts.push([lx, i as f32*h/6.0, lz]); }
        }
        let bounds = DataLibrarian::calculate_bounds(&pts);
        PointCloud { points: pts, normals: vec![], bounds }
    }

    fn chair(params: &[f32]) -> PointCloud {
        let mut p = Self::table(&[0.5, 0.5]);
        // Backrest
        for i in 0..6 { for j in 0..4 { let u=i as f32/6.0; let v=j as f32/4.0;
            p.points.push([-0.25+0.5*u, 0.5+0.5*v, 0.25]);
        }}
        p.bounds = DataLibrarian::calculate_bounds(&p.points);
        p
    }

    // ============= TERRAIN =============
    fn terrain_hills(params: &[f32]) -> PointCloud {
        let size = params.get(0).copied().unwrap_or(20.0);
        let hills = params.get(1).copied().unwrap_or(3.0) as u32;
        let mut pts = Vec::new();
        let res = 30;
        // Hill centers
        let mut centers = Vec::new();
        for _ in 0..hills { centers.push(((fastrand::f32()-0.5)*size*0.6, (fastrand::f32()-0.5)*size*0.6, 1.0+fastrand::f32()*3.0)); }
        for i in 0..=res { for j in 0..=res { let u=i as f32/res as f32; let v=j as f32/res as f32;
            let x=-size/2.0+size*u; let z=-size/2.0+size*v;
            let mut y = 0.0_f32;
            for &(cx,cz,ch) in &centers {
                let d=((x-cx).powi(2)+(z-cz).powi(2)).sqrt();
                y += ch*(-d*d/(size*0.3)).exp();
            }
            pts.push([x, y, z]);
        }}
        let bounds = DataLibrarian::calculate_bounds(&pts);
        PointCloud { points: pts, normals: vec![], bounds }
    }

    // ============= VILLAGE (L3 composite) =============
    fn village(params: &[f32]) -> PointCloud {
        let houses = params.get(0).copied().unwrap_or(5.0) as u32;
        let mut pts = Vec::new();
        for h in 0..houses {
            let ox = (h as f32 - houses as f32/2.0)*6.0;
            let oz = (fastrand::f32()-0.5)*4.0;
            let hw = 2.0+fastrand::f32()*2.0;
            let hh = 2.0+fastrand::f32()*1.5;
            let house = Self::house(&[hw, hh, hw*1.2, hh*0.5]);
            for p in &house.points { pts.push([p[0]+ox, p[1], p[2]+oz]); }
        }
        // Road
        for i in 0..40 { let t=i as f32/40.0;
            let x = -(houses as f32)*3.0+(houses as f32)*6.0*t;
            for w in -2..=2 { pts.push([x, 0.01, w as f32*0.4]); }
        }
        // Trees scattered
        for _ in 0..8 {
            let tx = (fastrand::f32()-0.5)*houses as f32*6.0;
            let tz = 4.0+fastrand::f32()*3.0;
            let tree = Self::tree_pine(&[3.0+fastrand::f32()*2.0]);
            for p in &tree.points { pts.push([p[0]+tx, p[1], p[2]+tz]); }
        }
        let bounds = DataLibrarian::calculate_bounds(&pts);
        PointCloud { points: pts, normals: vec![], bounds }
    }

    pub fn default_cube() -> PointCloud {
        let points = vec![
            [-1.0, -1.0, -1.0], [1.0, -1.0, -1.0],
            [1.0, 1.0, -1.0], [-1.0, 1.0, -1.0],
            [-1.0, -1.0, 1.0], [1.0, -1.0, 1.0],
            [1.0, 1.0, 1.0], [-1.0, 1.0, 1.0],
        ];
        let bounds = DataLibrarian::calculate_bounds(&points);
        PointCloud { points, normals: vec![], bounds }
    }
}
