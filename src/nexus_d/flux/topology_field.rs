use nalgebra::{Vector3, Point3, Matrix3};
use std::collections::HashMap;
use rayon::prelude::*;

/// Representa un campo volumétrico donde el material puede existir
/// con densidad variable. Esto permite formas orgánicas y fusiones.
#[derive(Clone, Debug)]
pub struct TopologyField {
    /// Resolución de la grilla voxel (celdas por eje)
    pub resolution: usize,
    /// Valores de densidad para cada voxel [0.0 = vacío, 1.0 = sólido]
    pub density: Vec<f64>,
    /// Dimensiones del espacio de trabajo en metros
    pub workspace_size: Vector3<f64>,
    /// Umbral para considerar material sólido
    pub solidification_threshold: f64,
    /// Campos de fuerza que influencian la distribución
    pub force_attractors: Vec<ForceAttractor>,
    /// Historial de fusiones realizadas
    pub fusion_log: Vec<FusionEvent>,
}

/// Un atractor de fuerza que guía dónde se acumula material
#[derive(Clone, Debug)]
pub struct ForceAttractor {
    pub position: Point3<f64>,
    pub strength: f64,
    pub radius: f64,
    pub attractor_type: AttractorType,
    /// Tensor de dirección preferida (para formas anisotrópicas)
    pub direction_bias: Option<Matrix3<f64>>,
}

#[derive(Clone, Debug)]
pub enum AttractorType {
    /// Atrae material (crea masa)
    Structural,
    /// Repele material (crea huecos/negativos)
    Void,
    /// Crea flujo direccional (para curvas)
    Flow { direction: Vector3<f64> },
    /// Atractor espiral (Fibonacci, formas orgánicas)
    Spiral { 
        axis: Vector3<f64>, 
        golden_ratio: bool,
        turns: f64,
    },
    /// Atractor de superficie mínima (formas tipo jabón)
    MinimalSurface,
}

#[derive(Clone, Debug)]
pub struct FusionEvent {
    pub timestamp: u64,
    pub components_merged: Vec<usize>,
    pub resulting_volume: f64,
    pub structural_improvement: f64,
    pub aesthetic_score_delta: f64,
}

/// Resultado de la solidificación del campo
#[derive(Clone, Debug)]
pub struct SolidifiedGeometry {
    pub vertices: Vec<Point3<f64>>,
    pub triangles: Vec<[usize; 3]>,
    pub volume: f64,
    pub surface_area: f64,
    pub center_of_mass: Point3<f64>,
    pub moment_of_inertia: Matrix3<f64>,
    /// Puntos donde la geometría toca el plano del suelo
    pub ground_contact_points: Vec<Point3<f64>>,
    /// Polígono de soporte proyectado
    pub support_polygon: Vec<[f64; 2]>,
}

impl TopologyField {
    pub fn new(resolution: usize, workspace_size: Vector3<f64>) -> Self {
        let total_voxels = resolution.pow(3);
        Self {
            resolution,
            density: vec![0.0; total_voxels],
            workspace_size,
            solidification_threshold: 0.35,
            force_attractors: Vec::new(),
            fusion_log: Vec::new(),
        }
    }

    /// Convierte coordenadas 3D a índice lineal en el campo
    pub fn coord_to_index(&self, x: usize, y: usize, z: usize) -> usize {
        z * self.resolution * self.resolution + y * self.resolution + x
    }

    /// Convierte índice lineal a posición en el mundo
    pub fn index_to_world(&self, idx: usize) -> Point3<f64> {
        let r = self.resolution;
        let z = idx / (r * r);
        let y = (idx % (r * r)) / r;
        let x = idx % r;

        Point3::new(
            (x as f64 / r as f64) * self.workspace_size.x,
            (y as f64 / r as f64) * self.workspace_size.y,
            (z as f64 / r as f64) * self.workspace_size.z,
        )
    }

    /// Propaga los atractores de fuerza a través del campo de densidad.
    /// Esta es la función principal que "esculpe" la forma. (Paralelizado)
    pub fn propagate_forces(&mut self, iterations: usize, diffusion_rate: f64) {
        for _iter in 0..iterations {
            let current_density = self.density.clone();
            let mut new_density = vec![0.0; self.density.len()];
            let r = self.resolution;
            let force_attractors = &self.force_attractors; // para préstamo a la clausura

            new_density.par_iter_mut().enumerate().for_each(|(idx, d)| {
                let world_pos = {
                    let z = idx / (r * r);
                    let y = (idx % (r * r)) / r;
                    let x = idx % r;
                    Point3::new(
                        (x as f64 / r as f64) * self.workspace_size.x,
                        (y as f64 / r as f64) * self.workspace_size.y,
                        (z as f64 / r as f64) * self.workspace_size.z,
                    )
                };

                let mut accumulated_influence = 0.0_f64;

                // Calcular influencia de todos los atractores
                for attractor in force_attractors {
                    let dist = nalgebra::distance(&world_pos, &attractor.position);

                    if dist > attractor.radius * 3.0 {
                        continue;
                    }

                    let base_influence = match &attractor.attractor_type {
                        AttractorType::Structural => {
                            attractor.strength * gaussian_falloff(dist, attractor.radius)
                        }
                        AttractorType::Void => {
                            -attractor.strength * gaussian_falloff(dist, attractor.radius)
                        }
                        AttractorType::Flow { direction } => {
                            let to_point = (world_pos - attractor.position).normalize();
                            let alignment = to_point.dot(&direction).abs();
                            attractor.strength 
                                * gaussian_falloff(dist, attractor.radius) 
                                * alignment
                        }
                        AttractorType::Spiral { axis, golden_ratio, turns } => {
                            let phi: f64 = if *golden_ratio { 1.618033988749 } else { 1.0 };
                            let relative = world_pos - attractor.position;
                            let height = relative.dot(&axis);
                            let radial = (relative - axis * height).norm();
                            let angle = height * turns * std::f64::consts::TAU;
                            let spiral_radius = radial * phi.powf(angle / std::f64::consts::TAU);
                            let spiral_dist = (spiral_radius - radial).abs();
                            attractor.strength 
                                * gaussian_falloff(spiral_dist, attractor.radius * 0.3)
                                * gaussian_falloff(dist, attractor.radius)
                        }
                        AttractorType::MinimalSurface => {
                            // Favorece superficies con curvatura media = 0
                            attractor.strength 
                                * gaussian_falloff(dist, attractor.radius) 
                                * 0.5
                        }
                    };

                    // Aplicar bias direccional si existe
                    let final_influence = if let Some(bias) = &attractor.direction_bias {
                        let dir = (world_pos - attractor.position).normalize();
                        let biased = bias * dir;
                        base_influence * biased.norm()
                    } else {
                        base_influence
                    };

                    accumulated_influence += final_influence;
                }

                // Difusión: promediar con vecinos (suaviza la forma)
                let z = idx / (r * r);
                let y = (idx % (r * r)) / r;
                let x = idx % r;

                let mut neighbor_avg = 0.0;
                let mut neighbor_count = 0u32;

                for &(dx, dy, dz) in &[
                    (1i32, 0, 0), (-1, 0, 0),
                    (0, 1, 0), (0, -1, 0),
                    (0, 0, 1), (0, 0, -1),
                ] {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    let nz = z as i32 + dz;

                    if nx >= 0 && nx < r as i32 
                        && ny >= 0 && ny < r as i32 
                        && nz >= 0 && nz < r as i32 
                    {
                        let ni = nz as usize * r * r + ny as usize * r + nx as usize;
                        neighbor_avg += current_density[ni];
                        neighbor_count += 1;
                    }
                }

                if neighbor_count > 0 {
                    neighbor_avg /= neighbor_count as f64;
                }

                *d = current_density[idx] * (1.0 - diffusion_rate)
                    + neighbor_avg * diffusion_rate
                    + accumulated_influence * 0.1;

                // Clamp
                *d = d.clamp(0.0, 1.0);
            });

            self.density = new_density;
        }
    }

    /// Detecta componentes separados y evalúa si deben fusionarse.
    pub fn detect_and_fuse_components(&mut self, proximity_threshold: f64) -> Vec<FusionEvent> {
        let components = self.find_connected_components();
        let mut fusions = Vec::new();

        // Encontrar pares de componentes que están cerca
        for i in 0..components.len() {
            for j in (i + 1)..components.len() {
                let min_dist = self.minimum_distance_between_components(
                    &components[i], 
                    &components[j]
                );

                if min_dist < proximity_threshold {
                    // Fusionar: llenar el espacio entre componentes
                    let bridge_voxels = self.create_bridge(
                        &components[i], 
                        &components[j],
                        min_dist,
                    );

                    for voxel_idx in &bridge_voxels {
                        self.density[*voxel_idx] = 0.7;
                    }

                    let event = FusionEvent {
                        timestamp: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap_or_default()
                            .as_secs(),
                        components_merged: vec![i, j],
                        resulting_volume: bridge_voxels.len() as f64,
                        structural_improvement: 0.0, // Se calcula después
                        aesthetic_score_delta: 0.0,   // Se calcula después
                    };

                    fusions.push(event);
                }
            }
        }

        // Suavizar las zonas de fusión
        if !fusions.is_empty() {
            self.propagate_forces(5, 0.3);
        }

        self.fusion_log.extend(fusions.clone());
        fusions
    }

    /// Flood-fill para encontrar componentes conectados
    fn find_connected_components(&self) -> Vec<Vec<usize>> {
        let mut visited = vec![false; self.density.len()];
        let mut components = Vec::new();
        let r = self.resolution;

        for idx in 0..self.density.len() {
            if visited[idx] || self.density[idx] < self.solidification_threshold {
                continue;
            }

            let mut component = Vec::new();
            let mut stack = vec![idx];

            while let Some(current) = stack.pop() {
                if visited[current] {
                    continue;
                }
                visited[current] = true;

                if self.density[current] >= self.solidification_threshold {
                    component.push(current);

                    let z = current / (r * r);
                    let y = (current % (r * r)) / r;
                    let x = current % r;

                    for &(dx, dy, dz) in &[
                        (1i32, 0, 0), (-1, 0, 0),
                        (0, 1, 0), (0, -1, 0),
                        (0, 0, 1), (0, 0, -1),
                    ] {
                        let nx = x as i32 + dx;
                        let ny = y as i32 + dy;
                        let nz = z as i32 + dz;

                        if nx >= 0 && nx < r as i32 
                            && ny >= 0 && ny < r as i32 
                            && nz >= 0 && nz < r as i32 
                        {
                            let ni = self.coord_to_index(
                                nx as usize, ny as usize, nz as usize
                            );
                            if !visited[ni] {
                                stack.push(ni);
                            }
                        }
                    }
                }
            }

            if !component.is_empty() {
                components.push(component);
            }
        }

        components
    }

    fn minimum_distance_between_components(
        &self, 
        comp_a: &[usize], 
        comp_b: &[usize]
    ) -> f64 {
        let mut min_dist = f64::MAX;

        // Optimización: solo revisar voxels de borde
        let border_a = self.get_border_voxels(comp_a);
        let border_b = self.get_border_voxels(comp_b);

        for &a in &border_a {
            let pos_a = self.index_to_world(a);
            for &b in &border_b {
                let pos_b = self.index_to_world(b);
                let dist = nalgebra::distance(&pos_a, &pos_b);
                if dist < min_dist {
                    min_dist = dist;
                }
            }
        }

        min_dist
    }

    fn get_border_voxels(&self, component: &[usize]) -> Vec<usize> {
        let r = self.resolution;
        let comp_set: std::collections::HashSet<usize> = component.iter().copied().collect();
        let mut borders = Vec::new();

        for &idx in component {
            let z = idx / (r * r);
            let y = (idx % (r * r)) / r;
            let x = idx % r;

            let is_border = [(1i32,0,0),(-1,0,0),(0,1,0),(0,-1,0),(0,0,1),(0,0,-1)]
                .iter()
                .any(|&(dx, dy, dz)| {
                    let nx = x as i32 + dx;
                    let ny = y as i32 + dy;
                    let nz = z as i32 + dz;

                    if nx < 0 || nx >= r as i32 
                        || ny < 0 || ny >= r as i32 
                        || nz < 0 || nz >= r as i32 
                    {
                        return true;
                    }

                    let ni = self.coord_to_index(nx as usize, ny as usize, nz as usize);
                    !comp_set.contains(&ni)
                });

            if is_border {
                borders.push(idx);
            }
        }

        borders
    }

    /// Crea un puente de material entre dos componentes
    fn create_bridge(
        &self, 
        comp_a: &[usize], 
        comp_b: &[usize],
        _distance: f64,
    ) -> Vec<usize> {
        // Encontrar los puntos más cercanos entre componentes
        let border_a = self.get_border_voxels(comp_a);
        let border_b = self.get_border_voxels(comp_b);

        let mut closest_a = 0;
        let mut closest_b = 0;
        let mut min_dist = f64::MAX;

        for &a in &border_a {
            let pos_a = self.index_to_world(a);
            for &b in &border_b {
                let pos_b = self.index_to_world(b);
                let d = nalgebra::distance(&pos_a, &pos_b);
                if d < min_dist {
                    min_dist = d;
                    closest_a = a;
                    closest_b = b;
                }
            }
        }

        // Trazar línea entre puntos más cercanos y llenar voxels
        let pos_a = self.index_to_world(closest_a);
        let pos_b = self.index_to_world(closest_b);
        let r = self.resolution;

        let steps = (min_dist * r as f64 / self.workspace_size.x.max(1.0)) as usize + 1;
        let mut bridge = Vec::new();

        for step in 0..=steps {
            let t = step as f64 / steps.max(1) as f64;
            let interp = Point3::new(
                pos_a.x + (pos_b.x - pos_a.x) * t,
                pos_a.y + (pos_b.y - pos_a.y) * t,
                pos_a.z + (pos_b.z - pos_a.z) * t,
            );

            // Convertir posición del mundo a coordenada de grilla
            let gx = ((interp.x / self.workspace_size.x) * r as f64) as usize;
            let gy = ((interp.y / self.workspace_size.y) * r as f64) as usize;
            let gz = ((interp.z / self.workspace_size.z) * r as f64) as usize;

            if gx < r && gy < r && gz < r {
                let idx = self.coord_to_index(gx, gy, gz);
                bridge.push(idx);

                // También agregar vecinos para dar grosor al puente
                for &(dx, dy, dz) in &[
                    (1i32,0,0),(-1,0,0),(0,1,0),(0,-1,0),(0,0,1),(0,0,-1)
                ] {
                    let nx = gx as i32 + dx;
                    let ny = gy as i32 + dy;
                    let nz = gz as i32 + dz;
                    if nx >= 0 && nx < r as i32 
                        && ny >= 0 && ny < r as i32 
                        && nz >= 0 && nz < r as i32 
                    {
                        bridge.push(self.coord_to_index(
                            nx as usize, ny as usize, nz as usize
                        ));
                    }
                }
            }
        }

        bridge.sort_unstable();
        bridge.dedup();
        bridge
    }

    /// Extrae la geometría sólida final del campo usando marching cubes simplificado
    pub fn solidify(&self) -> SolidifiedGeometry {
        let mut vertices = Vec::new();
        let mut triangles = Vec::new();
        let mut ground_contacts = Vec::new();
        let r = self.resolution;
        let ground_z_threshold = self.workspace_size.z * 0.02;

        // Marching cubes simplificado: extraer superficie
        for z in 0..r - 1 {
            for y in 0..r - 1 {
                for x in 0..r - 1 {
                    let corners = [
                        self.density[self.coord_to_index(x, y, z)],
                        self.density[self.coord_to_index(x + 1, y, z)],
                        self.density[self.coord_to_index(x + 1, y + 1, z)],
                        self.density[self.coord_to_index(x, y + 1, z)],
                        self.density[self.coord_to_index(x, y, z + 1)],
                        self.density[self.coord_to_index(x + 1, y, z + 1)],
                        self.density[self.coord_to_index(x + 1, y + 1, z + 1)],
                        self.density[self.coord_to_index(x, y + 1, z + 1)],
                    ];

                    let threshold = self.solidification_threshold;
                    let inside_count = corners.iter().filter(|&&c| c >= threshold).count();

                    // Si hay mezcla de dentro/fuera, hay superficie aquí
                    if inside_count > 0 && inside_count < 8 {
                        let center = Point3::new(
                            ((x as f64 + 0.5) / r as f64) * self.workspace_size.x,
                            ((y as f64 + 0.5) / r as f64) * self.workspace_size.y,
                            ((z as f64 + 0.5) / r as f64) * self.workspace_size.z,
                        );

                        //let vi = vertices.len();
                        vertices.push(center);

                        if center.z < ground_z_threshold {
                            ground_contacts.push(center);
                        }
                    }
                }
            }
        }

        // Calcular centro de masa (de todo el material sólido)
        let mut com = Vector3::new(0.0, 0.0, 0.0);
        let mut total_mass = 0.0;

        for idx in 0..self.density.len() {
            if self.density[idx] >= self.solidification_threshold {
                let pos = self.index_to_world(idx);
                com += pos.coords * self.density[idx];
                total_mass += self.density[idx];
            }
        }

        if total_mass > 0.0 {
            com /= total_mass;
        }

        // Polígono de soporte (convex hull 2D de puntos de contacto)
        let support_polygon: Vec<[f64; 2]> = if ground_contacts.len() >= 3 {
            convex_hull_2d(&ground_contacts)
        } else {
            ground_contacts.iter().map(|p| [p.x, p.y]).collect()
        };

        SolidifiedGeometry {
            vertices,
            triangles,
            volume: total_mass,
            surface_area: 0.0, // Se calcula con mesh real
            center_of_mass: Point3::from(com),
            moment_of_inertia: Matrix3::identity(), // Simplificado
            ground_contact_points: ground_contacts,
            support_polygon,
        }
    }
}

/// Caída gaussiana suave para influencia de atractores
fn gaussian_falloff(distance: f64, radius: f64) -> f64 {
    (-0.5 * (distance / radius).powi(2)).exp()
}

/// Convex hull 2D simplificado (gift wrapping)
fn convex_hull_2d(points: &[Point3<f64>]) -> Vec<[f64; 2]> {
    if points.len() < 3 {
        return points.iter().map(|p| [p.x, p.y]).collect();
    }

    let mut pts: Vec<[f64; 2]> = points.iter().map(|p| [p.x, p.y]).collect();

    // Encontrar punto más a la izquierda
    let start = pts.iter()
        .enumerate()
        .min_by(|a, b| a.1[0].partial_cmp(&b.1[0]).unwrap())
        .map(|(i, _)| i)
        .unwrap_or(0);

    let mut hull = Vec::new();
    let mut current = start;

    loop {
        hull.push(pts[current]);
        let mut next = 0;

        for i in 0..pts.len() {
            if i == current { continue; }

            if next == current {
                next = i;
                continue;
            }

            let cross = cross_2d(&pts[current], &pts[next], &pts[i]);
            if cross > 0.0 || (cross == 0.0 && dist_2d(&pts[current], &pts[i]) > dist_2d(&pts[current], &pts[next])) {
                next = i;
            }
        }

        current = next;
        if current == start { break; }
        if hull.len() > pts.len() { break; } // Safety
    }

    hull
}

fn cross_2d(o: &[f64; 2], a: &[f64; 2], b: &[f64; 2]) -> f64 {
    (a[0] - o[0]) * (b[1] - o[1]) - (a[1] - o[1]) * (b[0] - o[0])
}

fn dist_2d(a: &[f64; 2], b: &[f64; 2]) -> f64 {
    ((a[0] - b[0]).powi(2) + (a[1] - b[1]).powi(2)).sqrt()
}
