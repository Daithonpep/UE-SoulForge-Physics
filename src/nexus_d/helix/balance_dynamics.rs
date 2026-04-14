use nalgebra::{Point3, Vector3, Point2};

/// Sistema de evaluación de equilibrio dinámico.
/// No importa qué tan "loca" sea la forma — si el CoG proyectado
/// cae dentro del polígono de soporte, es válida.
pub struct HelixBalanceSystem {
    /// Factor de seguridad mínimo (1.0 = justo en el borde)
    stability_margin: f64,
    /// Gravedad (m/s²)
    gravity: Vector3<f64>,
    /// Cargas externas aplicadas (peso de objetos sobre la mesa, etc.)
    external_loads: Vec<ExternalLoad>,
    /// Configuración de análisis dinámico
    dynamic_config: DynamicConfig,
}

#[derive(Clone, Debug)]
pub struct ExternalLoad {
    pub position: Point3<f64>,
    pub force: Vector3<f64>,
    pub is_distributed: bool,
    pub distribution_radius: f64,
}

#[derive(Clone, Debug)]
pub struct DynamicConfig {
    /// Simular perturbaciones (alguien empuja la mesa)
    pub perturbation_test: bool,
    /// Magnitud máxima de perturbación lateral (Newtons)
    pub max_lateral_force: f64,
    /// Considerar momento de inercia para volcaduras
    pub check_tipping: bool,
    /// Ángulo máximo de inclinación del suelo (grados)
    pub max_floor_slope: f64,
}

/// Resultado completo del análisis de equilibrio
#[derive(Clone, Debug)]
pub struct BalanceAnalysis {
    /// Centro de gravedad del sistema completo
    pub center_of_gravity: Point3<f64>,
    /// Proyección del CoG sobre el plano del suelo
    pub cog_projection: Point2<f64>,
    /// Polígono de soporte (convex hull de contactos con suelo)
    pub support_polygon: Vec<Point2<f64>>,
    /// ¿Está en equilibrio estático?
    pub is_stable: bool,
    /// Distancia mínima del CoG proyectado al borde del polígono
    /// (mayor = más estable)
    pub stability_margin_distance: f64,
    /// Factor de estabilidad normalizado [0.0 - 1.0]
    pub stability_factor: f64,
    /// Dirección más vulnerable a volcadura
    pub weakest_tipping_direction: Vector3<f64>,
    /// Fuerza lateral necesaria para volcar
    pub tipping_force_required: f64,
    /// Resultado del test de perturbación
    pub perturbation_safe: bool,
    /// Presión en cada punto de contacto (para verificar que 
    /// no rompe el suelo)
    pub contact_pressures: Vec<f64>,
    /// Score compuesto para MUSE/AURA
    pub balance_score: f64,
}

impl HelixBalanceSystem {
    pub fn new(stability_margin: f64) -> Self {
        Self {
            stability_margin,
            gravity: Vector3::new(0.0, 0.0, -9.81),
            external_loads: Vec::new(),
            dynamic_config: DynamicConfig {
                perturbation_test: true,
                max_lateral_force: 50.0,
                check_tipping: true,
                max_floor_slope: 2.0,
            },
        }
    }

    pub fn add_load(&mut self, load: ExternalLoad) {
        self.external_loads.push(load);
    }

    /// Análisis completo de equilibrio para una geometría solidificada
    pub fn analyze(
        &self,
        geometry_com: &Point3<f64>,
        geometry_mass: f64,
        ground_contacts: &[Point3<f64>],
        support_polygon: &[[f64; 2]],
    ) -> BalanceAnalysis {
        // 1. Calcular CoG combinado (geometría + cargas externas)
        let mut total_mass = geometry_mass;
        let mut weighted_com = geometry_com.coords * geometry_mass;

        for load in &self.external_loads {
            let load_mass = load.force.norm() / self.gravity.norm();
            weighted_com += load.position.coords * load_mass;
            total_mass += load_mass;
        }

        let combined_com = Point3::from(weighted_com / total_mass.max(1e-10)); // previene /0

        // 2. Proyectar CoG al plano del suelo (z = 0)
        let cog_proj = Point2::new(combined_com.x, combined_com.y);

        // 3. Convertir polígono de soporte
        let poly: Vec<Point2<f64>> = support_polygon
            .iter()
            .map(|p| Point2::new(p[0], p[1]))
            .collect();

        // 4. Verificar si la proyección está dentro del polígono
        let is_inside = point_in_polygon(&cog_proj, &poly);

        // 5. Calcular distancia al borde más cercano
        let margin_dist = if poly.len() >= 3 {
            min_distance_to_polygon_edge(&cog_proj, &poly)
        } else if poly.len() == 2 {
            // Solo dos puntos de contacto: distancia a la línea
            distance_to_line_segment(&cog_proj, &poly[0], &poly[1])
        } else if poly.len() == 1 {
            // Un solo punto: distancia directa
            nalgebra::distance(&cog_proj, &poly[0])
        } else {
            0.0
        };

        // Factor de estabilidad normalizado
        let poly_diameter = polygon_diameter(&poly);
        let stability_factor = if poly_diameter > 0.0 {
            (margin_dist / poly_diameter).min(1.0)
        } else {
            0.0
        };

        // 6. Encontrar dirección más vulnerable
        let (weakest_dir, tipping_force) = self.find_weakest_tipping(
            &combined_com,
            total_mass,
            &poly,
        );

        // 7. Test de perturbación
        let perturbation_safe = if self.dynamic_config.perturbation_test {
            tipping_force > self.dynamic_config.max_lateral_force
        } else {
            true
        };

        // 8. Calcular presiones de contacto
        let contact_pressures = self.calculate_contact_pressures(
            &combined_com,
            total_mass,
            ground_contacts,
        );

        // 9. Score compuesto
        let balance_score = self.compute_balance_score(
            is_inside,
            stability_factor,
            perturbation_safe,
            ground_contacts.len(),
        );

        BalanceAnalysis {
            center_of_gravity: combined_com,
            cog_projection: cog_proj,
            support_polygon: poly,
            is_stable: is_inside,
            stability_margin_distance: if is_inside { margin_dist } else { -margin_dist },
            stability_factor,
            weakest_tipping_direction: weakest_dir,
            tipping_force_required: tipping_force,
            perturbation_safe,
            contact_pressures,
            balance_score,
        }
    }

    fn find_weakest_tipping(
        &self,
        com: &Point3<f64>,
        mass: f64,
        polygon: &[Point2<f64>],
    ) -> (Vector3<f64>, f64) {
        if polygon.len() < 2 {
            return (Vector3::new(1.0, 0.0, 0.0), 0.0);
        }

        let mut min_force = f64::MAX;
        let mut weakest_dir = Vector3::new(1.0, 0.0, 0.0);

        // Revisar cada arista del polígono como posible eje de volcadura
        for i in 0..polygon.len() {
            let j = (i + 1) % polygon.len();
            let edge_start = &polygon[i];
            let edge_end = &polygon[j];

            // Vector de la arista
            let edge = Point2::new(
                edge_end.x - edge_start.x,
                edge_end.y - edge_start.y,
            );
            let edge_len = (edge.x * edge.x + edge.y * edge.y).sqrt();

            if edge_len < 1e-10 { continue; }

            // Normal hacia afuera de la arista
            let normal = Vector3::new(-edge.y / edge_len, edge.x / edge_len, 0.0);

            // Distancia del CoG proyectado a esta arista
            let cog_2d = Point2::new(com.x, com.y);
            let dist = distance_to_line_segment(&cog_2d, edge_start, edge_end);

            // Fuerza necesaria para volcar sobre esta arista
            // F_tip = m * g * d_horizontal / h_com
            let h = com.z.max(0.01);
            let tipping_force = mass * self.gravity.norm() * dist / h;

            if tipping_force < min_force {
                min_force = tipping_force;
                weakest_dir = normal;
            }
        }

        (weakest_dir, min_force)
    }

    fn calculate_contact_pressures(
        &self,
        com: &Point3<f64>,
        total_mass: f64,
        contacts: &[Point3<f64>],
    ) -> Vec<f64> {
        if contacts.is_empty() {
            return Vec::new();
        }

        let total_weight = total_mass * self.gravity.norm();

        // Distribución inversamente proporcional a la distancia al CoG
        let distances: Vec<f64> = contacts
            .iter()
            .map(|c| {
                let dx = c.x - com.x;
                let dy = c.y - com.y;
                (dx * dx + dy * dy).sqrt().max(0.01)
            })
            .collect();

        let inv_sum: f64 = distances.iter().map(|d| 1.0 / d).sum();

        distances
            .iter()
            .map(|d| {
                let fraction = (1.0 / d) / inv_sum;
                fraction * total_weight
            })
            .collect()
    }

    fn compute_balance_score(
        &self,
        is_stable: bool,
        stability_factor: f64,
        perturbation_safe: bool,
        num_contacts: usize,
    ) -> f64 {
        if !is_stable {
            return 0.0;
        }

        let mut score = stability_factor * 0.6;

        if perturbation_safe {
            score += 0.2;
        }

        // Bonus por elegancia: menos contactos = más impresionante
        // (si es estable con pocos contactos, es un mejor diseño)
        let elegance_bonus = match num_contacts {
            1 => 0.20,     // Base central única — máximo bonus
            2 => 0.15,     // Dos puntos — muy elegante
            3 => 0.10,     // Trípode — clásico
            4 => 0.05,     // Cuatro patas — estándar
            _ => 0.0,      // Más de 4 — sin bonus
        };

        score += elegance_bonus;
        score.min(1.0)
    }
}

// ─── Utilidades geométricas 2D ───

fn point_in_polygon(point: &Point2<f64>, polygon: &[Point2<f64>]) -> bool {
    if polygon.len() < 3 {
        return false;
    }

    let mut inside = false;
    let n = polygon.len();

    let mut j = n - 1;
    for i in 0..n {
        let pi = &polygon[i];
        let pj = &polygon[j];

        if ((pi.y > point.y) != (pj.y > point.y))
            && (point.x < (pj.x - pi.x) * (point.y - pi.y) / (pj.y - pi.y) + pi.x)
        {
            inside = !inside;
        }
        j = i;
    }

    inside
}

fn distance_to_line_segment(
    point: &Point2<f64>,
    seg_start: &Point2<f64>,
    seg_end: &Point2<f64>,
) -> f64 {
    let dx = seg_end.x - seg_start.x;
    let dy = seg_end.y - seg_start.y;
    let len_sq = dx * dx + dy * dy;

    if len_sq < 1e-12 {
        return nalgebra::distance(point, seg_start);
    }

    let t = ((point.x - seg_start.x) * dx + (point.y - seg_start.y) * dy) / len_sq;
    let t = t.clamp(0.0, 1.0);

    let proj = Point2::new(seg_start.x + t * dx, seg_start.y + t * dy);
    nalgebra::distance(point, &proj)
}

fn min_distance_to_polygon_edge(
    point: &Point2<f64>,
    polygon: &[Point2<f64>],
) -> f64 {
    let mut min_dist = f64::MAX;

    for i in 0..polygon.len() {
        let j = (i + 1) % polygon.len();
        let d = distance_to_line_segment(point, &polygon[i], &polygon[j]);
        if d < min_dist {
            min_dist = d;
        }
    }

    min_dist
}

fn polygon_diameter(polygon: &[Point2<f64>]) -> f64 {
    let mut max_dist = 0.0_f64;
    for i in 0..polygon.len() {
        for j in (i + 1)..polygon.len() {
            let d = nalgebra::distance(&polygon[i], &polygon[j]);
            if d > max_dist {
                max_dist = d;
            }
        }
    }
    max_dist
}
