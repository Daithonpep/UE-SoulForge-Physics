use nalgebra::{Point3, Vector3};
use std::f64::consts::PI;

/// El número áureo
const PHI: f64 = 1.618033988749895;
/// Inverso del número áureo
const PHI_INV: f64 = 0.618033988749895;

/// Perfil de estilo que configura las preferencias estéticas
#[derive(Clone, Debug)]
pub struct AestheticProfile {
    pub name: String,
    /// Peso de cada métrica en el score final [0.0 - 1.0]
    pub weights: AestheticWeights,
    /// Preferencias específicas del estilo
    pub preferences: StylePreferences,
}

#[derive(Clone, Debug)]
pub struct AestheticWeights {
    /// Proporción áurea y Fibonacci
    pub golden_ratio: f64,
    /// Simetría (bilateral, radial, etc.)
    pub symmetry: f64,
    /// Curvatura suave vs ángulos duros
    pub curvature_flow: f64,
    /// Minimalismo (lograr más con menos material)
    pub minimalism: f64,
    /// Contraste visual (variación de proporciones)
    pub visual_contrast: f64,
    /// Coherencia de lenguaje formal
    pub formal_coherence: f64,
    /// Innovación (qué tan diferente es de diseños anteriores)
    pub novelty: f64,
}

#[derive(Clone, Debug)]
pub struct StylePreferences {
    /// Tipo de simetría preferida
    pub symmetry_type: SymmetryPreference,
    /// ¿Preferir curvas o ángulos?
    pub curvature_preference: CurvaturePreference,
    /// Número mínimo de puntos de contacto deseado
    pub min_contact_points: usize,
    /// Número máximo de puntos de contacto deseado
    pub max_contact_points: usize,
    /// Relación altura/ancho preferida
    pub preferred_aspect_ratio: f64,
    /// ¿Premiar si usa Fibonacci en las proporciones?
    pub fibonacci_bonus: bool,
    /// Lista de "motivos" geométricos premiados
    pub rewarded_motifs: Vec<GeometricMotif>,
}

#[derive(Clone, Debug)]
pub enum SymmetryPreference {
    Bilateral,
    Radial { order: usize },
    Asymmetric, // Premia la asimetría controlada
    Any,
}

#[derive(Clone, Debug)]
pub enum CurvaturePreference {
    Organic,       // Curvas suaves tipo naturaleza
    Geometric,     // Ángulos definidos
    Mixed,         // Combinación armoniosa
    MinimalSurface, // Superficies tipo jabón
}

#[derive(Clone, Debug, PartialEq)]
pub enum GeometricMotif {
    SpiralFibonacci,
    VoronoiPattern,
    CatenaryArch,
    HyperbolicParaboloid,
    MoebiusTwist,
    CrossedStructure, // Tipo Xena con patas cruzadas
    CentralColumn,    // Base central única
    CantileverExtension,
}

/// Resultado del análisis estético
#[derive(Clone, Debug)]
pub struct AestheticAnalysis {
    pub total_score: f64,
    pub golden_ratio_score: f64,
    pub symmetry_score: f64,
    pub curvature_score: f64,
    pub minimalism_score: f64,
    pub contrast_score: f64,
    pub coherence_score: f64,
    pub novelty_score: f64,
    pub detected_motifs: Vec<GeometricMotif>,
    pub suggestions: Vec<String>,
}

pub struct AuraRewardSystem {
    profile: AestheticProfile,
    /// Historial de diseños previos para calcular novedad
    design_history: Vec<DesignFingerprint>,
}

#[derive(Clone, Debug)]
pub struct DesignFingerprint {
    pub proportions: Vec<f64>,
    pub symmetry_order: usize,
    pub contact_count: usize,
    pub curvature_distribution: Vec<f64>,
}

impl AuraRewardSystem {
    pub fn new(profile: AestheticProfile) -> Self {
        Self {
            profile,
            design_history: Vec::new(),
        }
    }

    /// Perfiles predefinidos
    pub fn avant_garde_profile() -> AestheticProfile {
        AestheticProfile {
            name: "Avant-Garde".into(),
            weights: AestheticWeights {
                golden_ratio: 0.15,
                symmetry: 0.05,       // Baja: la asimetría es bienvenida
                curvature_flow: 0.20,
                minimalism: 0.25,     // Alta: menos es más
                visual_contrast: 0.15,
                formal_coherence: 0.10,
                novelty: 0.10,
            },
            preferences: StylePreferences {
                symmetry_type: SymmetryPreference::Asymmetric,
                curvature_preference: CurvaturePreference::Organic,
                min_contact_points: 1,
                max_contact_points: 3,
                preferred_aspect_ratio: PHI,
                fibonacci_bonus: true,
                rewarded_motifs: vec![
                    GeometricMotif::CentralColumn,
                    GeometricMotif::CantileverExtension,
                    GeometricMotif::SpiralFibonacci,
                ],
            },
        }
    }

    pub fn futuristic_profile() -> AestheticProfile {
        AestheticProfile {
            name: "Futuristic".into(),
            weights: AestheticWeights {
                golden_ratio: 0.20,
                symmetry: 0.15,
                curvature_flow: 0.15,
                minimalism: 0.15,
                visual_contrast: 0.10,
                formal_coherence: 0.15,
                novelty: 0.10,
            },
            preferences: StylePreferences {
                symmetry_type: SymmetryPreference::Radial { order: 3 },
                curvature_preference: CurvaturePreference::Mixed,
                min_contact_points: 2,
                max_contact_points: 4,
                preferred_aspect_ratio: PHI,
                fibonacci_bonus: true,
                rewarded_motifs: vec![
                    GeometricMotif::CrossedStructure,
                    GeometricMotif::HyperbolicParaboloid,
                    GeometricMotif::CatenaryArch,
                ],
            },
        }
    }

    pub fn sculptural_profile() -> AestheticProfile {
        AestheticProfile {
            name: "Sculptural".into(),
            weights: AestheticWeights {
                golden_ratio: 0.10,
                symmetry: 0.10,
                curvature_flow: 0.25,
                minimalism: 0.20,
                visual_contrast: 0.15,
                formal_coherence: 0.10,
                novelty: 0.10,
            },
            preferences: StylePreferences {
                symmetry_type: SymmetryPreference::Any,
                curvature_preference: CurvaturePreference::MinimalSurface,
                min_contact_points: 1,
                max_contact_points: 5,
                preferred_aspect_ratio: 1.414, // √2
                fibonacci_bonus: true,
                rewarded_motifs: vec![
                    GeometricMotif::MoebiusTwist,
                    GeometricMotif::VoronoiPattern,
                    GeometricMotif::SpiralFibonacci,
                ],
            },
        }
    }

    /// Análisis completo de un diseño
    pub fn evaluate(
        &self,
        vertices: &[Point3<f64>],
        bounding_box: &BoundingBox,
        contact_points: &[Point3<f64>],
        curvatures: &[f64],
    ) -> AestheticAnalysis {
        let golden = self.eval_golden_ratio(bounding_box);
        let symmetry = self.eval_symmetry(vertices);
        let curvature = self.eval_curvature_flow(curvatures);
        let minimalism = self.eval_minimalism(vertices, contact_points, bounding_box);
        let contrast = self.eval_visual_contrast(bounding_box, vertices);
        let coherence = self.eval_formal_coherence(curvatures, vertices);
        let novelty = self.eval_novelty(vertices, contact_points);
        let motifs = self.detect_motifs(vertices, contact_points, bounding_box);

        let w = &self.profile.weights;

        let total = golden * w.golden_ratio
            + symmetry * w.symmetry
            + curvature * w.curvature_flow
            + minimalism * w.minimalism
            + contrast * w.visual_contrast
            + coherence * w.formal_coherence
            + novelty * w.novelty;

        // Bonus por motivos detectados
        let motif_bonus = motifs.len() as f64 * 0.03;

        // Bonus por Fibonacci si está habilitado
        let fib_bonus = if self.profile.preferences.fibonacci_bonus {
            self.eval_fibonacci_proportions(bounding_box) * 0.05
        } else {
            0.0
        };

        // Bonus por número de contactos preferido
        let contact_bonus = self.eval_contact_elegance(contact_points.len());

        let total_with_bonus = (total + motif_bonus + fib_bonus + contact_bonus).min(1.0);

        let suggestions = self.generate_suggestions(
            golden, symmetry, curvature, minimalism, contact_points.len()
        );

        AestheticAnalysis {
            total_score: total_with_bonus,
            golden_ratio_score: golden,
            symmetry_score: symmetry,
            curvature_score: curvature,
            minimalism_score: minimalism,
            contrast_score: contrast,
            coherence_score: coherence,
            novelty_score: novelty,
            detected_motifs: motifs,
            suggestions,
        }
    }

    fn eval_golden_ratio(&self, bb: &BoundingBox) -> f64 {
        let dims = bb.dimensions();
        let mut sorted_dims = vec![dims.x, dims.y, dims.z];
        sorted_dims.sort_by(|a, b| a.partial_cmp(b).unwrap());

        if sorted_dims[0] < 1e-10 { return 0.0; }

        // Verificar proporciones entre dimensiones
        let ratios = vec![
            sorted_dims[2] / sorted_dims[1],
            sorted_dims[1] / sorted_dims[0],
            sorted_dims[2] / sorted_dims[0],
        ];

        let mut best_score = 0.0_f64;
        for ratio in &ratios {
            // ¿Qué tan cerca está de PHI o sus potencias?
            let candidates = [PHI, PHI * PHI, PHI.sqrt(), 2.0 * PHI, PHI_INV];
            for &target in &candidates {
                let deviation = (ratio - target).abs() / target;
                let score = (-deviation * 5.0).exp(); // Gaussian around target
                best_score = best_score.max(score);
            }
        }

        best_score
    }

    fn eval_symmetry(&self, vertices: &[Point3<f64>]) -> f64 {
        if vertices.is_empty() { return 0.0; }

        // Calcular centroide
        let centroid = vertices.iter()
            .fold(Vector3::zeros(), |acc, v| acc + v.coords)
            / vertices.len() as f64;

        match &self.profile.preferences.symmetry_type {
            SymmetryPreference::Bilateral => {
                // Verificar simetría respecto al plano YZ (x = centroid.x)
                self.bilateral_symmetry_score(vertices, &centroid, 0)
            }
            SymmetryPreference::Radial { order } => {
                self.radial_symmetry_score(vertices, &centroid, *order)
            }
            SymmetryPreference::Asymmetric => {
                // Invertido: premiar FALTA de simetría, pero con balance
                let bilateral = self.bilateral_symmetry_score(vertices, &centroid, 0);
                // No completamente asimétrico (caótico), sino controlado
                if bilateral < 0.3 {
                    0.5 + bilateral // Algo de estructura
                } else if bilateral > 0.8 {
                    0.3 // Demasiado simétrico para este perfil
                } else {
                    0.7 // Sweet spot
                }
            }
            SymmetryPreference::Any => {
                let bilateral = self.bilateral_symmetry_score(vertices, &centroid, 0);
                let radial = self.radial_symmetry_score(vertices, &centroid, 4);
                bilateral.max(radial)
            }
        }
    }

    fn bilateral_symmetry_score(
        &self,
        vertices: &[Point3<f64>],
        centroid: &Vector3<f64>,
        axis: usize, // 0=X, 1=Y, 2=Z
    ) -> f64 {
        let mut total_error = 0.0;
        let mut matched = 0;

        for v in vertices {
            let mut reflected = v.coords - centroid;
            reflected[axis] = -reflected[axis];
            let reflected_pos = Point3::from(reflected + centroid);

            // Buscar el vértice más cercano al reflejado
            let min_dist = vertices.iter()
                .map(|other| nalgebra::distance(other, &reflected_pos))
                .fold(f64::MAX, f64::min);

            let tolerance = 0.05; // 5cm de tolerancia
            if min_dist < tolerance {
                matched += 1;
            }
            total_error += min_dist;
        }

        if vertices.is_empty() { return 0.0; }

        let match_ratio = matched as f64 / vertices.len() as f64;
        let avg_error = total_error / vertices.len() as f64;

        match_ratio * 0.7 + (-avg_error * 10.0).exp() * 0.3
    }

    fn radial_symmetry_score(
        &self,
        vertices: &[Point3<f64>],
        centroid: &Vector3<f64>,
        order: usize,
    ) -> f64 {
        if order < 2 { return 0.0; }

        let angle = 2.0 * PI / order as f64;
        let mut total_score = 0.0;

        for rot in 1..order {
            let theta = angle * rot as f64;
            let cos_t = theta.cos();
            let sin_t = theta.sin();

            let mut rotation_match = 0;

            for v in vertices {
                let relative = v.coords - centroid;
                // Rotar alrededor del eje Z
                let rotated = Vector3::new(
                    relative.x * cos_t - relative.y * sin_t,
                    relative.x * sin_t + relative.y * cos_t,
                    relative.z,
                );
                let rotated_pos = Point3::from(rotated + centroid);

                let min_dist = vertices.iter()
                    .map(|other| nalgebra::distance(other, &rotated_pos))
                    .fold(f64::MAX, f64::min);

                if min_dist < 0.05 {
                    rotation_match += 1;
                }
            }

            if !vertices.is_empty() {
                total_score += rotation_match as f64 / vertices.len() as f64;
            }
        }

        total_score / (order - 1).max(1) as f64
    }

    fn eval_curvature_flow(&self, curvatures: &[f64]) -> f64 {
        if curvatures.is_empty() { return 0.5; }

        match &self.profile.preferences.curvature_preference {
            CurvaturePreference::Organic => {
                // Premiar curvaturas suaves y continuas
                let smoothness = curvature_smoothness(curvatures);
                let has_curves = curvatures.iter()
                    .filter(|&&c| c.abs() > 0.01)
                    .count() as f64 / curvatures.len() as f64;
                smoothness * 0.6 + has_curves * 0.4
            }
            CurvaturePreference::Geometric => {
                // Premiar cambios bruscos (aristas definidas)
                let sharpness = curvature_sharpness(curvatures);
                sharpness
            }
            CurvaturePreference::Mixed => {
                let smoothness = curvature_smoothness(curvatures);
                let sharpness = curvature_sharpness(curvatures);
                // Premiar la coexistencia armoniosa
                (smoothness * sharpness).sqrt()
            }
            CurvaturePreference::MinimalSurface => {
                // Curvatura media cercana a 0 en todas partes
                let mean_curvature: f64 = curvatures.iter()
                    .map(|c| c.abs())
                    .sum::<f64>() / curvatures.len() as f64;
                (-mean_curvature * 5.0).exp()
            }
        }
    }

    fn eval_minimalism(
        &self,
        vertices: &[Point3<f64>],
        contacts: &[Point3<f64>],
        bb: &BoundingBox,
    ) -> f64 {
        let dims = bb.dimensions();
        let bounding_volume = dims.x * dims.y * dims.z;

        if bounding_volume < 1e-10 { return 0.0; }

        // Material vs espacio que ocupa
        let material_ratio = vertices.len() as f64 / bounding_volume;

        // Menos material con más función = más minimalista
        let efficiency = if contacts.len() > 0 {
            1.0 / (1.0 + material_ratio * 100.0)
        } else {
            0.0
        };

        // Pocos contactos = más minimalista
        let contact_factor = match contacts.len() {
            1 => 1.0,
            2 => 0.9,
            3 => 0.75,
            4 => 0.5,
            _ => 0.3,
        };

        efficiency * 0.5 + contact_factor * 0.5
    }

    fn eval_visual_contrast(
        &self,
        bb: &BoundingBox,
        _vertices: &[Point3<f64>],
    ) -> f64 {
        let dims = bb.dimensions();
        let sorted = {
            let mut d = vec![dims.x, dims.y, dims.z];
            d.sort_by(|a, b| a.partial_cmp(b).unwrap());
            d
        };

        if sorted[0] < 1e-10 { return 0.0; }

        // Contraste entre la dimensión más grande y la más pequeña
        let aspect_contrast = sorted[2] / sorted[0];

        // Un rango medio de contraste es más interesante visualmente
        let target = self.profile.preferences.preferred_aspect_ratio;
        let deviation = (aspect_contrast - target).abs() / target;
        (-deviation * 3.0).exp()
    }

    fn eval_formal_coherence(
        &self,
        curvatures: &[f64],
        _vertices: &[Point3<f64>],
    ) -> f64 {
        if curvatures.is_empty() {
            return 0.5;
        }

        // ¿Las curvaturas siguen un patrón consistente?
        let curvature_variance = statistical_variance(curvatures);
        let mean_curvature = curvatures.iter().sum::<f64>() / curvatures.len() as f64;

        // Coeficiente de variación bajo = más coherente
        if mean_curvature.abs() < 1e-10 {
            return 0.5;
        }

        let cv = curvature_variance.sqrt() / mean_curvature.abs();
        (-cv * 2.0).exp()
    }

    fn eval_novelty(
        &self,
        _vertices: &[Point3<f64>],
        contacts: &[Point3<f64>],
    ) -> f64 {
        if self.design_history.is_empty() {
            return 1.0; // Primer diseño es siempre novedoso
        }

        let current = DesignFingerprint {
            proportions: Vec::new(), // Simplificado
            symmetry_order: 0,
            contact_count: contacts.len(),
            curvature_distribution: Vec::new(),
        };

        // Distancia promedio a todos los diseños anteriores
        let avg_distance: f64 = self.design_history.iter()
            .map(|prev| {
                let contact_diff = (current.contact_count as f64 
                    - prev.contact_count as f64).abs();
                contact_diff / 10.0
            })
            .sum::<f64>() / self.design_history.len() as f64;

        avg_distance.min(1.0)
    }

    fn eval_fibonacci_proportions(&self, bb: &BoundingBox) -> f64 {
        let dims = bb.dimensions();
        let fibonacci_ratios = [1.0, 1.0, 2.0, 3.0, 5.0, 8.0, 13.0, 21.0];
        let fib_ratios: Vec<f64> = fibonacci_ratios.windows(2)
            .map(|w| w[1] / w[0])
            .collect();

        let mut sorted = vec![dims.x, dims.y, dims.z];
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

        if sorted[0] < 1e-10 { return 0.0; }

        let actual_ratios = vec![
            sorted[1] / sorted[0],
            sorted[2] / sorted[1],
        ];

        let mut best_match = 0.0_f64;
        for actual in &actual_ratios {
            for fib in &fib_ratios {
                let error = (actual - fib).abs() / fib;
                let score = (-error * 5.0).exp();
                best_match = best_match.max(score);
            }
        }

        best_match
    }

    fn eval_contact_elegance(&self, num_contacts: usize) -> f64 {
        let prefs = &self.profile.preferences;
        if num_contacts >= prefs.min_contact_points 
            && num_contacts <= prefs.max_contact_points 
        {
            // Dentro del rango preferido
            // Bonus extra si está en el mínimo
            if num_contacts == prefs.min_contact_points {
                0.05
            } else {
                0.02
            }
        } else {
            -0.05 // Penalización leve fuera del rango
        }
    }

    fn detect_motifs(
        &self,
        vertices: &[Point3<f64>],
        contacts: &[Point3<f64>],
        bb: &BoundingBox,
    ) -> Vec<GeometricMotif> {
        let mut detected = Vec::new();

        // Central Column: un solo punto de contacto o contactos 
        // agrupados en el centro
        if contacts.len() <= 2 {
            let centroid_xy = bb.center();
            let contacts_near_center = contacts.iter()
                .filter(|c| {
                    let dx = c.x - centroid_xy.x;
                    let dy = c.y - centroid_xy.y;
                    (dx * dx + dy * dy).sqrt() < bb.dimensions().x * 0.2
                })
                .count();

            if contacts_near_center == contacts.len() && contacts.len() > 0 {
                detected.push(GeometricMotif::CentralColumn);
            }
        }

        // Crossed Structure: contactos en patrón X
        if contacts.len() >= 4 {
            // Verificar si los contactos forman un patrón cruzado
            let center = bb.center();
            let mut quadrant_counts = [0u32; 4];

            for c in contacts {
                let qx = if c.x > center.x { 1 } else { 0 };
                let qy = if c.y > center.y { 1 } else { 0 };
                quadrant_counts[qx + qy * 2] += 1;
            }

            if quadrant_counts.iter().all(|&q| q >= 1) {
                detected.push(GeometricMotif::CrossedStructure);
            }
        }

        // Cantilever: centro de masa significativamente 
        // desplazado del centro del bounding box
        let com = vertices.iter()
            .fold(Vector3::zeros(), |acc, v| acc + v.coords)
            / vertices.len().max(1) as f64;
        let center = bb.center();
        let offset = ((com.x - center.x).powi(2) + (com.y - center.y).powi(2)).sqrt();
        if offset > bb.dimensions().x * 0.15 {
            detected.push(GeometricMotif::CantileverExtension);
        }

        detected
    }

    fn generate_suggestions(
        &self,
        golden: f64,
        _symmetry: f64,
        curvature: f64,
        minimalism: f64,
        contact_count: usize,
    ) -> Vec<String> {
        let mut suggestions = Vec::new();

        if golden < 0.5 {
            suggestions.push(
                "Ajustar proporciones hacia ratio áureo (1:1.618)".into()
            );
        }

        if curvature < 0.4 {
            match &self.profile.preferences.curvature_preference {
                CurvaturePreference::Organic => {
                    suggestions.push(
                        "Suavizar transiciones entre superficies".into()
                    );
                }
                CurvaturePreference::Geometric => {
                    suggestions.push(
                        "Definir aristas más marcadas".into()
                    );
                }
                _ => {}
            }
        }

        if minimalism < 0.4 && self.profile.weights.minimalism > 0.15 {
            suggestions.push(
                "Reducir material: explorar formas huecas o perforadas".into()
            );
        }

        if contact_count > self.profile.preferences.max_contact_points {
            suggestions.push(format!(
                "Reducir puntos de contacto de {} a máximo {}",
                contact_count,
                self.profile.preferences.max_contact_points
            ));
        }

        suggestions
    }

    /// Registrar un diseño en el historial para cálculos de novedad
    pub fn register_design(&mut self, fingerprint: DesignFingerprint) {
        self.design_history.push(fingerprint);
        // Mantener solo los últimos 100 para no saturar memoria
        if self.design_history.len() > 100 {
            self.design_history.remove(0);
        }
    }
}

/// Bounding Box auxiliar
#[derive(Clone, Debug)]
pub struct BoundingBox {
    pub min: Point3<f64>,
    pub max: Point3<f64>,
}

impl BoundingBox {
    pub fn from_points(points: &[Point3<f64>]) -> Self {
        let mut min = Point3::new(f64::MAX, f64::MAX, f64::MAX);
        let mut max = Point3::new(f64::MIN, f64::MIN, f64::MIN);

        for p in points {
            min.x = min.x.min(p.x);
            min.y = min.y.min(p.y);
            min.z = min.z.min(p.z);
            max.x = max.x.max(p.x);
            max.y = max.y.max(p.y);
            max.z = max.z.max(p.z);
        }

        Self { min, max }
    }

    pub fn dimensions(&self) -> Vector3<f64> {
        self.max - self.min
    }

    pub fn center(&self) -> Point3<f64> {
        Point3::new(
            (self.min.x + self.max.x) / 2.0,
            (self.min.y + self.max.y) / 2.0,
            (self.min.z + self.max.z) / 2.0,
        )
    }
}

// Utilidades estadísticas
fn curvature_smoothness(curvatures: &[f64]) -> f64 {
    if curvatures.len() < 2 { return 1.0; }

    let total_change: f64 = curvatures.windows(2)
        .map(|w| (w[1] - w[0]).abs())
        .sum();

    let avg_change = total_change / (curvatures.len() - 1) as f64;
    (-avg_change * 10.0).exp()
}

fn curvature_sharpness(curvatures: &[f64]) -> f64 {
    if curvatures.len() < 2 { return 0.0; }

    let total_change: f64 = curvatures.windows(2)
        .map(|w| (w[1] - w[0]).abs())
        .sum();

    let avg_change = total_change / (curvatures.len() - 1) as f64;
    (1.0 - (-avg_change * 5.0).exp()).max(0.0)
}

fn statistical_variance(data: &[f64]) -> f64 {
    if data.is_empty() { return 0.0; }
    let mean = data.iter().sum::<f64>() / data.len() as f64;
    data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / data.len() as f64
}
