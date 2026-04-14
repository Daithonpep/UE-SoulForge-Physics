use crate::archetype::taxonomy::*;
use nalgebra::Point3;

/// Analiza geometría de ejemplos semilla y extrae principios
pub struct EssenceExtractor {
    taxonomy: TaxonomyTree,
}

impl EssenceExtractor {
    pub fn new(taxonomy: TaxonomyTree) -> Self {
        Self { taxonomy }
    }

    /// Analiza un ejemplo semilla y extrae sus características
    pub fn analyze_seed(&self, seed: &SeedExample) -> SeedAnalysis {
        match &seed.reference_geometry {
            GeometryReference::GLB { path } => {
                self.analyze_glb(path)
            }
            GeometryReference::Simplified { point_cloud } => {
                self.analyze_point_cloud(point_cloud)
            }
            GeometryReference::Parametric { params } => {
                self.analyze_parametric(params)
            }
        }
    }

    fn analyze_glb(&self, _path: &str) -> SeedAnalysis {
        // Cargar GLB y convertir a point cloud
        // (Por ahora, simplificado)
        SeedAnalysis::default()
    }

    fn analyze_point_cloud(&self, points: &[[f64; 3]]) -> SeedAnalysis {
        if points.is_empty() {
            return SeedAnalysis::default();
        }

        // Bounding box
        let mut min = Point3::new(f64::MAX, f64::MAX, f64::MAX);
        let mut max = Point3::new(f64::MIN, f64::MIN, f64::MIN);

        for p in points {
            min.x = min.x.min(p[0]);
            min.y = min.y.min(p[1]);
            min.z = min.z.min(p[2]);
            max.x = max.x.max(p[0]);
            max.y = max.y.max(p[1]);
            max.z = max.z.max(p[2]);
        }

        let dimensions = max - min;

        // Centro de masa
        let com = Point3::new(
            points.iter().map(|p| p[0]).sum::<f64>() / points.len() as f64,
            points.iter().map(|p| p[1]).sum::<f64>() / points.len() as f64,
            points.iter().map(|p| p[2]).sum::<f64>() / points.len() as f64,
        );

        // Detectar puntos de contacto con el suelo
        let ground_threshold = min.z + 0.02;
        let contact_points: Vec<Point3<f64>> = points.iter()
            .filter(|p| p[2] < ground_threshold)
            .map(|p| Point3::new(p[0], p[1], p[2]))
            .collect();

        // Simetría (simplificado: verificar bilateral en eje X)
        let symmetry_score = self.calculate_bilateral_symmetry(points, com.x);

        // Detección de componentes (clustering simple)
        let components = self.detect_components(points);

        SeedAnalysis {
            bounding_box: BoundingBoxRange {
                width: (dimensions.x - 0.1, dimensions.x + 0.1),
                depth: (dimensions.y - 0.1, dimensions.y + 0.1),
                height: (dimensions.z - 0.1, dimensions.z + 0.1),
            },
            center_of_mass: com,
            contact_points: contact_points.len(),
            symmetry_score,
            component_count: components,
            aspect_ratio: dimensions.x / dimensions.y.max(0.001),
            volume_estimate: dimensions.x * dimensions.y * dimensions.z,
        }
    }

    fn calculate_bilateral_symmetry(&self, points: &[[f64; 3]], axis_x: f64) -> f64 {
        let mut symmetry_errors = Vec::new();

        for point in points {
            let reflected_x = 2.0 * axis_x - point[0];
            
            // Buscar punto más cercano al reflejado
            let min_dist = points.iter()
                .map(|other| {
                    let dx = other[0] - reflected_x;
                    let dy = other[1] - point[1];
                    let dz = other[2] - point[2];
                    (dx * dx + dy * dy + dz * dz).sqrt()
                })
                .fold(f64::MAX, f64::min);

            symmetry_errors.push(min_dist);
        }

        let avg_error = symmetry_errors.iter().sum::<f64>() / symmetry_errors.len().max(1) as f64;
        
        // Score 0-1 (mayor = más simétrico)
        (-avg_error * 10.0).exp()
    }

    fn detect_components(&self, points: &[[f64; 3]]) -> usize {
        // Clustering simple basado en distancia
        
        if points.len() < 10 {
            return 1;
        }

        // Heurística: contar "islas" de densidad
        let mut visited = vec![false; points.len()];
        let mut component_count = 0;
        let threshold = 0.1; // 10cm de distancia

        for i in 0..points.len() {
            if visited[i] {
                continue;
            }

            // Nuevo componente
            component_count += 1;
            let mut stack = vec![i];

            while let Some(current) = stack.pop() {
                if visited[current] {
                    continue;
                }
                visited[current] = true;

                let p_current = &points[current];

                for (j, p_other) in points.iter().enumerate() {
                    if visited[j] {
                        continue;
                    }

                    let dist = (
                        (p_current[0] - p_other[0]).powi(2) +
                        (p_current[1] - p_other[1]).powi(2) +
                        (p_current[2] - p_other[2]).powi(2)
                    ).sqrt();

                    if dist < threshold {
                        stack.push(j);
                    }
                }
            }
        }

        component_count
    }

    fn analyze_parametric(&self, _params: &std::collections::HashMap<String, f64>) -> SeedAnalysis {
        SeedAnalysis::default()
    }

    /// Extrae los principios comunes de múltiples seeds
    pub fn extract_common_principles(&self, analyses: &[SeedAnalysis]) -> ExtractedPrinciples {
        if analyses.is_empty() {
            return ExtractedPrinciples::default();
        }

        // Dimensiones promedio
        let avg_width = analyses.iter()
            .map(|a| (a.bounding_box.width.0 + a.bounding_box.width.1) / 2.0)
            .sum::<f64>() / analyses.len() as f64;

        let avg_depth = analyses.iter()
            .map(|a| (a.bounding_box.depth.0 + a.bounding_box.depth.1) / 2.0)
            .sum::<f64>() / analyses.len() as f64;

        let avg_height = analyses.iter()
            .map(|a| (a.bounding_box.height.0 + a.bounding_box.height.1) / 2.0)
            .sum::<f64>() / analyses.len() as f64;

        // Varianza (para determinar rangos)
        let width_variance = analyses.iter()
            .map(|a| {
                let w = (a.bounding_box.width.0 + a.bounding_box.width.1) / 2.0;
                (w - avg_width).powi(2)
            })
            .sum::<f64>() / analyses.len() as f64;

        let width_std = width_variance.sqrt();

        // Número típico de puntos de contacto
        let mut contact_counts: Vec<usize> = analyses.iter()
            .map(|a| a.contact_points)
            .collect();
        contact_counts.sort_unstable();
        let median_contacts = contact_counts[contact_counts.len() / 2];

        // Simetría promedio
        let avg_symmetry = analyses.iter()
            .map(|a| a.symmetry_score)
            .sum::<f64>() / analyses.len() as f64;

        ExtractedPrinciples {
            typical_dimensions: BoundingBoxRange {
                width: (avg_width - width_std, avg_width + width_std),
                depth: (avg_depth - width_std, avg_depth + width_std),
                height: (avg_height - width_std, avg_height + width_std),
            },
            typical_contact_points: median_contacts,
            typical_symmetry: if avg_symmetry > 0.7 {
                SymmetryType::Bilateral
            } else if avg_symmetry > 0.3 {
                SymmetryType::Mixed
            } else {
                SymmetryType::Asymmetric
            },
            component_count_range: (
                analyses.iter().map(|a| a.component_count).min().unwrap_or(1),
                analyses.iter().map(|a| a.component_count).max().unwrap_or(1),
            ),
        }
    }
}

#[derive(Debug, Clone)]
pub struct SeedAnalysis {
    pub bounding_box: BoundingBoxRange,
    pub center_of_mass: Point3<f64>,
    pub contact_points: usize,
    pub symmetry_score: f64,
    pub component_count: usize,
    pub aspect_ratio: f64,
    pub volume_estimate: f64,
}

impl Default for SeedAnalysis {
    fn default() -> Self {
        Self {
            bounding_box: BoundingBoxRange {
                width: (0.0, 0.0),
                depth: (0.0, 0.0),
                height: (0.0, 0.0),
            },
            center_of_mass: Point3::new(0.0, 0.0, 0.0),
            contact_points: 0,
            symmetry_score: 0.0,
            component_count: 0,
            aspect_ratio: 0.0,
            volume_estimate: 0.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExtractedPrinciples {
    pub typical_dimensions: BoundingBoxRange,
    pub typical_contact_points: usize,
    pub typical_symmetry: SymmetryType,
    pub component_count_range: (usize, usize),
}

impl Default for ExtractedPrinciples {
    fn default() -> Self {
        Self {
            typical_dimensions: BoundingBoxRange {
                width: (0.5, 1.5),
                depth: (0.5, 1.5),
                height: (0.5, 1.5),
            },
            typical_contact_points: 4,
            typical_symmetry: SymmetryType::Bilateral,
            component_count_range: (1, 5),
        }
    }
}
