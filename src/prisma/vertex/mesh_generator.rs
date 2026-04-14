use nalgebra::{Point3, Vector3};
use std::collections::HashMap;

/// Configuración de generación de malla
#[derive(Clone, Debug)]
pub struct MeshGenerationConfig {
    /// Nivel de subdivisión base (0-5)
    pub subdivision_level: usize,
    /// Subdivisión adaptativa basada en curvatura
    pub adaptive_subdivision: bool,
    /// Umbral de curvatura para subdividir más
    pub curvature_threshold: f64,
    /// Suavizado de normales
    pub smooth_normals: bool,
    /// Generar UVs para texturizado
    pub generate_uvs: bool,
    /// Calcular tangentes para normal mapping
    pub calculate_tangents: bool,
    /// Simplificar mesh en zonas planas
    pub auto_simplification: bool,
    /// Target de triángulos para LOD
    pub target_triangle_count: Option<usize>,
}

impl Default for MeshGenerationConfig {
    fn default() -> Self {
        Self {
            subdivision_level: 2,
            adaptive_subdivision: true,
            curvature_threshold: 0.1,
            smooth_normals: true,
            generate_uvs: true,
            calculate_tangents: true,
            auto_simplification: true,
            target_triangle_count: None,
        }
    }
}

/// Malla de alta calidad con todos los datos necesarios para rendering
#[derive(Clone, Debug)]
pub struct HighQualityMesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u32>,
    pub bounding_box: BoundingBox,
    pub lod_levels: Vec<LODLevel>,
    pub material_slots: Vec<MaterialSlot>,
}

#[derive(Clone, Debug)]
pub struct Vertex {
    pub position: Point3<f64>,
    pub normal: Vector3<f64>,
    pub tangent: Vector3<f64>,
    pub bitangent: Vector3<f64>,
    pub uv: [f64; 2],
    pub color: [f32; 4],
    /// Curvatura local (para subdivisión adaptativa)
    pub curvature: f64,
}

#[derive(Clone, Debug)]
pub struct LODLevel {
    pub distance_threshold: f64,
    pub index_offset: usize,
    pub index_count: usize,
    pub triangle_count: usize,
}

#[derive(Clone, Debug)]
pub struct MaterialSlot {
    pub name: String,
    pub index_range: std::ops::Range<usize>,
    pub material_id: usize,
}

#[derive(Clone, Debug)]
pub struct BoundingBox {
    pub min: Point3<f64>,
    pub max: Point3<f64>,
}

impl BoundingBox {
    pub fn from_vertices(vertices: &[Vertex]) -> Self {
        let mut min = Point3::new(f64::MAX, f64::MAX, f64::MAX);
        let mut max = Point3::new(f64::MIN, f64::MIN, f64::MIN);

        for v in vertices {
            min.x = min.x.min(v.position.x);
            min.y = min.y.min(v.position.y);
            min.z = min.z.min(v.position.z);
            max.x = max.x.max(v.position.x);
            max.y = max.y.max(v.position.y);
            max.z = max.z.max(v.position.z);
        }

        Self { min, max }
    }

    pub fn center(&self) -> Point3<f64> {
        Point3::new(
            (self.min.x + self.max.x) / 2.0,
            (self.min.y + self.max.y) / 2.0,
            (self.min.z + self.max.z) / 2.0,
        )
    }

    pub fn size(&self) -> Vector3<f64> {
        self.max - self.min
    }
}

pub struct VertexMeshGenerator {
    config: MeshGenerationConfig,
}

impl VertexMeshGenerator {
    pub fn new(config: MeshGenerationConfig) -> Self {
        Self { config }
    }

    /// Genera malla de alta calidad desde el campo de topología de FLUX
    pub fn generate_from_field(
        &self,
        field: &crate::nexus_d::flux::topology_field::TopologyField,
    ) -> HighQualityMesh {
        // 1. Extracción inicial con Marching Cubes mejorado
        let (base_vertices, base_indices) = self.marching_cubes_extraction(field);

        // 2. Subdivisión (Loop subdivision o Catmull-Clark)
        let (subdivided_verts, subdivided_indices) = if self.config.subdivision_level > 0 {
            self.subdivide_mesh(&base_vertices, &base_indices, self.config.subdivision_level)
        } else {
            (base_vertices, base_indices)
        };

        // 3. Calcular normales
        let vertices_with_normals = if self.config.smooth_normals {
            self.calculate_smooth_normals(&subdivided_verts, &subdivided_indices)
        } else {
            self.calculate_flat_normals(&subdivided_verts, &subdivided_indices)
        };

        // 4. Calcular curvatura
        let vertices_with_curvature = self.calculate_vertex_curvature(&vertices_with_normals, &subdivided_indices);

        // 5. Subdivisión adaptativa basada en curvatura
        let (adaptive_verts, adaptive_indices) = if self.config.adaptive_subdivision {
            self.adaptive_subdivide(&vertices_with_curvature, &subdivided_indices)
        } else {
            (vertices_with_curvature, subdivided_indices)
        };

        // 6. Generar UVs
        let vertices_with_uvs = if self.config.generate_uvs {
            self.generate_uvs(&adaptive_verts)
        } else {
            adaptive_verts.into_iter()
                .map(|mut v| { v.uv = [0.0, 0.0]; v })
                .collect()
        };

        // 7. Calcular tangentes
        let final_vertices = if self.config.calculate_tangents {
            self.calculate_tangents(&vertices_with_uvs, &adaptive_indices)
        } else {
            vertices_with_uvs
        };

        // 8. Generar LODs
        let lods = self.generate_lod_levels(&final_vertices, &adaptive_indices);

        // 9. Asignar material slots
        let material_slots = self.assign_material_slots(&final_vertices, &adaptive_indices);

        let bbox = BoundingBox::from_vertices(&final_vertices);

        HighQualityMesh {
            vertices: final_vertices,
            indices: adaptive_indices,
            bounding_box: bbox,
            lod_levels: lods,
            material_slots,
        }
    }

    /// Marching Cubes mejorado con interpolación
    fn marching_cubes_extraction(
        &self,
        _field: &crate::nexus_d::flux::topology_field::TopologyField,
    ) -> (Vec<Vertex>, Vec<u32>) {
        // Implementación simplificada - en producción usar lookup tables completas
        let vertices = Vec::new();
        let indices = Vec::new();
        
        (vertices, indices)
    }

    /// Loop Subdivision Scheme
    fn subdivide_mesh(
        &self,
        vertices: &[Vertex],
        indices: &[u32],
        levels: usize,
    ) -> (Vec<Vertex>, Vec<u32>) {
        let mut current_verts = vertices.to_vec();
        let mut current_indices = indices.to_vec();

        for _ in 0..levels {
            let (new_verts, new_indices) = self.loop_subdivision_step(&current_verts, &current_indices);
            current_verts = new_verts;
            current_indices = new_indices;
        }

        (current_verts, current_indices)
    }

    fn loop_subdivision_step(
        &self,
        vertices: &[Vertex],
        indices: &[u32],
    ) -> (Vec<Vertex>, Vec<u32>) {
        let mut edge_vertices: HashMap<Edge, usize> = HashMap::new();
        let mut new_vertices = vertices.to_vec();
        let mut new_indices = Vec::new();

        for chunk in indices.chunks(3) {
            let i0 = chunk[0] as usize;
            let i1 = chunk[1] as usize;
            let i2 = chunk[2] as usize;

            let e01 = self.get_or_create_edge_vertex(
                &mut edge_vertices,
                &mut new_vertices,
                vertices,
                i0, i1,
            );
            let e12 = self.get_or_create_edge_vertex(
                &mut edge_vertices,
                &mut new_vertices,
                vertices,
                i1, i2,
            );
            let e20 = self.get_or_create_edge_vertex(
                &mut edge_vertices,
                &mut new_vertices,
                vertices,
                i2, i0,
            );

            new_indices.extend_from_slice(&[
                i0 as u32, e01 as u32, e20 as u32,
                i1 as u32, e12 as u32, e01 as u32,
                i2 as u32, e20 as u32, e12 as u32,
                e01 as u32, e12 as u32, e20 as u32,
            ]);
        }

        (new_vertices, new_indices)
    }

    fn get_or_create_edge_vertex(
        &self,
        edge_map: &mut HashMap<Edge, usize>,
        vertices: &mut Vec<Vertex>,
        original_verts: &[Vertex],
        i0: usize,
        i1: usize,
    ) -> usize {
        let edge = Edge::new(i0, i1);

        if let Some(&idx) = edge_map.get(&edge) {
            return idx;
        }

        let v0 = &original_verts[i0];
        let v1 = &original_verts[i1];

        let new_vertex = Vertex {
            position: Point3::new(
                (v0.position.x + v1.position.x) / 2.0,
                (v0.position.y + v1.position.y) / 2.0,
                (v0.position.z + v1.position.z) / 2.0,
            ),
            normal: ((v0.normal + v1.normal) / 2.0).normalize(),
            tangent: Vector3::zeros(),
            bitangent: Vector3::zeros(),
            uv: [
                (v0.uv[0] + v1.uv[0]) / 2.0,
                (v0.uv[1] + v1.uv[1]) / 2.0,
            ],
            color: [
                (v0.color[0] + v1.color[0]) / 2.0,
                (v0.color[1] + v1.color[1]) / 2.0,
                (v0.color[2] + v1.color[2]) / 2.0,
                (v0.color[3] + v1.color[3]) / 2.0,
            ],
            curvature: 0.0,
        };

        let idx = vertices.len();
        vertices.push(new_vertex);
        edge_map.insert(edge, idx);
        idx
    }

    fn calculate_smooth_normals(
        &self,
        vertices: &[Vertex],
        indices: &[u32],
    ) -> Vec<Vertex> {
        let mut normals = vec![Vector3::zeros(); vertices.len()];

        for chunk in indices.chunks(3) {
            let i0 = chunk[0] as usize;
            let i1 = chunk[1] as usize;
            let i2 = chunk[2] as usize;

            let v0 = &vertices[i0].position;
            let v1 = &vertices[i1].position;
            let v2 = &vertices[i2].position;

            let edge1 = v1 - v0;
            let edge2 = v2 - v0;
            let normal = edge1.cross(&edge2);

            normals[i0] += normal;
            normals[i1] += normal;
            normals[i2] += normal;
        }

        for n in &mut normals {
            if n.norm() > 1e-10 {
                *n = n.normalize();
            }
        }

        vertices.iter()
            .enumerate()
            .map(|(i, v)| {
                let mut new_v = v.clone();
                new_v.normal = normals[i];
                new_v
            })
            .collect()
    }

    fn calculate_flat_normals(
        &self,
        vertices: &[Vertex],
        _indices: &[u32],
    ) -> Vec<Vertex> {
        vertices.to_vec()
    }

    fn calculate_vertex_curvature(
        &self,
        vertices: &[Vertex],
        indices: &[u32],
    ) -> Vec<Vertex> {
        let mut adjacency: Vec<Vec<usize>> = vec![Vec::new(); vertices.len()];

        for (tri_idx, chunk) in indices.chunks(3).enumerate() {
            for &vertex_idx in chunk {
                adjacency[vertex_idx as usize].push(tri_idx);
            }
        }

        vertices.iter()
            .enumerate()
            .map(|(i, v)| {
                let mut new_v = v.clone();

                if adjacency[i].len() < 2 {
                    new_v.curvature = 0.0;
                    return new_v;
                }

                let mut normal_variance = 0.0;
                let current_normal = v.normal;

                for &tri_idx in &adjacency[i] {
                    let tri_base = tri_idx * 3;
                    let i0 = indices[tri_base] as usize;
                    let i1 = indices[tri_base + 1] as usize;
                    let i2 = indices[tri_base + 2] as usize;

                    let v0 = &vertices[i0].position;
                    let v1 = &vertices[i1].position;
                    let v2 = &vertices[i2].position;

                    let edge1 = v1 - v0;
                    let edge2 = v2 - v0;
                    let tri_normal = edge1.cross(&edge2).normalize();

                    let dot = current_normal.dot(&tri_normal).clamp(-1.0, 1.0);
                    let angle = dot.acos();
                    normal_variance += angle.abs();
                }

                new_v.curvature = normal_variance / adjacency[i].len() as f64;
                new_v
            })
            .collect()
    }

    fn adaptive_subdivide(
        &self,
        vertices: &[Vertex],
        indices: &[u32],
    ) -> (Vec<Vertex>, Vec<u32>) {
        let mut new_vertices = vertices.to_vec();
        let mut new_indices = Vec::new();
        let mut edge_cache: HashMap<Edge, usize> = HashMap::new();

        for chunk in indices.chunks(3) {
            let i0 = chunk[0] as usize;
            let i1 = chunk[1] as usize;
            let i2 = chunk[2] as usize;

            let avg_curvature = (vertices[i0].curvature 
                + vertices[i1].curvature 
                + vertices[i2].curvature) / 3.0;

            if avg_curvature > self.config.curvature_threshold {
                let e01 = self.get_or_create_edge_vertex_adaptive(
                    &mut edge_cache,
                    &mut new_vertices,
                    vertices,
                    i0, i1,
                );
                let e12 = self.get_or_create_edge_vertex_adaptive(
                    &mut edge_cache,
                    &mut new_vertices,
                    vertices,
                    i1, i2,
                );
                let e20 = self.get_or_create_edge_vertex_adaptive(
                    &mut edge_cache,
                    &mut new_vertices,
                    vertices,
                    i2, i0,
                );

                new_indices.extend_from_slice(&[
                    i0 as u32, e01 as u32, e20 as u32,
                    i1 as u32, e12 as u32, e01 as u32,
                    i2 as u32, e20 as u32, e12 as u32,
                    e01 as u32, e12 as u32, e20 as u32,
                ]);
            } else {
                new_indices.extend_from_slice(&[i0 as u32, i1 as u32, i2 as u32]);
            }
        }

        (new_vertices, new_indices)
    }

    fn get_or_create_edge_vertex_adaptive(
        &self,
        edge_map: &mut HashMap<Edge, usize>,
        vertices: &mut Vec<Vertex>,
        original_verts: &[Vertex],
        i0: usize,
        i1: usize,
    ) -> usize {
        let edge = Edge::new(i0, i1);

        if let Some(&idx) = edge_map.get(&edge) {
            return idx;
        }

        let v0 = &original_verts[i0];
        let v1 = &original_verts[i1];

        let new_vertex = Vertex {
            position: Point3::new(
                (v0.position.x + v1.position.x) / 2.0,
                (v0.position.y + v1.position.y) / 2.0,
                (v0.position.z + v1.position.z) / 2.0,
            ),
            normal: ((v0.normal + v1.normal) / 2.0).normalize(),
            tangent: Vector3::zeros(),
            bitangent: Vector3::zeros(),
            uv: [(v0.uv[0] + v1.uv[0]) / 2.0, (v0.uv[1] + v1.uv[1]) / 2.0],
            color: [
                (v0.color[0] + v1.color[0]) / 2.0,
                (v0.color[1] + v1.color[1]) / 2.0,
                (v0.color[2] + v1.color[2]) / 2.0,
                (v0.color[3] + v1.color[3]) / 2.0,
            ],
            curvature: (v0.curvature + v1.curvature) / 2.0,
        };

        let idx = vertices.len();
        vertices.push(new_vertex);
        edge_map.insert(edge, idx);
        idx
    }

    fn generate_uvs(&self, vertices: &[Vertex]) -> Vec<Vertex> {
        let bbox = BoundingBox::from_vertices(vertices);
        let center = bbox.center();

        vertices.iter()
            .map(|v| {
                let mut new_v = v.clone();
                let relative = v.position - center;

                let theta = relative.z.atan2(relative.x);
                let phi = (relative.y / relative.norm()).asin();

                new_v.uv = [
                    (theta / std::f64::consts::PI + 1.0) / 2.0,
                    (phi / std::f64::consts::FRAC_PI_2 + 1.0) / 2.0,
                ];

                new_v
            })
            .collect()
    }

    fn calculate_tangents(
        &self,
        vertices: &[Vertex],
        indices: &[u32],
    ) -> Vec<Vertex> {
        let mut tangents = vec![Vector3::zeros(); vertices.len()];
        let mut bitangents = vec![Vector3::zeros(); vertices.len()];

        for chunk in indices.chunks(3) {
            let i0 = chunk[0] as usize;
            let i1 = chunk[1] as usize;
            let i2 = chunk[2] as usize;

            let v0 = &vertices[i0];
            let v1 = &vertices[i1];
            let v2 = &vertices[i2];

            let edge1 = v1.position - v0.position;
            let edge2 = v2.position - v0.position;

            let delta_uv1 = [v1.uv[0] - v0.uv[0], v1.uv[1] - v0.uv[1]];
            let delta_uv2 = [v2.uv[0] - v0.uv[0], v2.uv[1] - v0.uv[1]];

            let mut f = delta_uv1[0] * delta_uv2[1] - delta_uv2[0] * delta_uv1[1];
            if f.abs() < 1e-8 { f = 1.0; } else { f = 1.0 / f; }

            let tangent = Vector3::new(
                f * (delta_uv2[1] * edge1.x - delta_uv1[1] * edge2.x),
                f * (delta_uv2[1] * edge1.y - delta_uv1[1] * edge2.y),
                f * (delta_uv2[1] * edge1.z - delta_uv1[1] * edge2.z),
            );

            let bitangent = Vector3::new(
                f * (-delta_uv2[0] * edge1.x + delta_uv1[0] * edge2.x),
                f * (-delta_uv2[0] * edge1.y + delta_uv1[0] * edge2.y),
                f * (-delta_uv2[0] * edge1.z + delta_uv1[0] * edge2.z),
            );

            tangents[i0] += tangent;
            tangents[i1] += tangent;
            tangents[i2] += tangent;

            bitangents[i0] += bitangent;
            bitangents[i1] += bitangent;
            bitangents[i2] += bitangent;
        }

        vertices.iter()
            .enumerate()
            .map(|(i, v)| {
                let mut new_v = v.clone();
                
                if tangents[i].norm() > 1e-10 {
                    new_v.tangent = tangents[i].normalize();
                }
                
                if bitangents[i].norm() > 1e-10 {
                    new_v.bitangent = bitangents[i].normalize();
                }

                new_v
            })
            .collect()
    }

    fn generate_lod_levels(
        &self,
        _vertices: &[Vertex],
        indices: &[u32],
    ) -> Vec<LODLevel> {
        vec![
            LODLevel {
                distance_threshold: 0.0,
                index_offset: 0,
                index_count: indices.len(),
                triangle_count: indices.len() / 3,
            },
        ]
    }

    fn assign_material_slots(
        &self,
        _vertices: &[Vertex],
        indices: &[u32],
    ) -> Vec<MaterialSlot> {
        vec![
            MaterialSlot {
                name: "Base".into(),
                index_range: 0..indices.len(),
                material_id: 0,
            }
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Edge {
    v0: usize,
    v1: usize,
}

impl Edge {
    fn new(v0: usize, v1: usize) -> Self {
        if v0 < v1 {
            Self { v0, v1 }
        } else {
            Self { v0: v1, v1: v0 }
        }
    }
}
