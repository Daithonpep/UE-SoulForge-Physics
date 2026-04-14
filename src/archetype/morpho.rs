use crate::archetype::taxonomy::*;
use crate::archetype::essence::*;
use rand::Rng;

/// Configuración del generador
pub struct MorphoConfig {
    /// Número de variaciones a generar por concepto
    pub variations_per_concept: usize,
    /// Nivel de mutación (0.0 = conservador, 1.0 = radical)
    pub mutation_strength: f64,
    /// Habilitar cruces entre tipos relacionados
    pub enable_cross_breeding: bool,
    /// Número de mejores diseños a retener
    pub elite_count: usize,
}

/// Generador de variaciones morfológicas
pub struct MorphoGenerator {
    config: MorphoConfig,
    taxonomy: TaxonomyTree,
}

impl MorphoGenerator {
    pub fn new(config: MorphoConfig, taxonomy: TaxonomyTree) -> Self {
        Self {
            config,
            taxonomy,
        }
    }

    /// Genera variaciones masivas para un concepto
    pub fn generate_variations(
        &mut self,
        concept_id: &str,
        principles: &ExtractedPrinciples,
    ) -> Vec<DesignGenome> {
        let mut genomes = Vec::with_capacity(self.config.variations_per_concept);

        // Obtener el concepto
        let concept = match self.taxonomy.get_node(concept_id) {
            Some(c) => c.clone(),
            None => return genomes,
        };

        // Generar variaciones
        for _ in 0..self.config.variations_per_concept {
            let genome = self.create_genome_from_principles(&concept, principles);
            genomes.push(genome);
        }

        // Evaluar rápidamente (fitness aproximado)
        let evaluated: Vec<_> = genomes.into_iter()
            .map(|mut g| {
                g.estimated_fitness = self.quick_fitness_estimate(&g, &concept.essence);
                g
            })
            .collect();

        // Ordenar por fitness y retornar elite
        let mut sorted = evaluated;
        // Orden inverso: mayor fitness primero
        sorted.sort_by(|a, b| b.estimated_fitness.partial_cmp(&a.estimated_fitness).unwrap_or(std::cmp::Ordering::Equal));

        sorted.into_iter()
            .take(self.config.elite_count)
            .collect()
    }

    fn create_genome_from_principles(
        &mut self,
        concept: &ConceptNode,
        principles: &ExtractedPrinciples,
    ) -> DesignGenome {
        let dims = &principles.typical_dimensions;
        let mut rng = rand::thread_rng();

        // Dimensiones con variación
        let width = self.sample_range(dims.width.0, dims.width.1, &mut rng);
        let depth = self.sample_range(dims.depth.0, dims.depth.1, &mut rng);
        let height = self.sample_range(dims.height.0, dims.height.1, &mut rng);

        // Número de soportes
        let support_count = self.sample_support_count(
            principles.typical_contact_points,
            &concept.essence.structural_principles,
            &mut rng
        );

        // Topología de soportes
        let support_topology = self.generate_support_topology(
            support_count,
            width,
            depth,
            &principles.typical_symmetry,
            &mut rng
        );

        // Forma de la superficie
        let surface_shape = self.generate_surface_shape(
            width,
            depth,
            height,
            &principles.typical_symmetry,
            &mut rng
        );

        DesignGenome {
            id: uuid::Uuid::new_v4().to_string(),
            concept_id: concept.id.clone(),
            dimensions: [width, depth, height],
            support_topology,
            surface_shape,
            material_distribution: self.generate_material_distribution(&mut rng),
            aesthetic_params: self.generate_aesthetic_params(&mut rng),
            estimated_fitness: 0.0,
            generation: 0,
            parent_ids: vec![],
        }
    }

    fn sample_range(&mut self, min: f64, max: f64, rng: &mut rand::rngs::ThreadRng) -> f64 {
        let mut value = rng.gen_range(min..=max);
        
        // Aplicar mutación
        let mutation = rng.gen_range(-self.config.mutation_strength..=self.config.mutation_strength);
        value *= 1.0 + mutation * 0.1;

        value.clamp(min * 0.5, max * 1.5)
    }

    fn sample_support_count(
        &mut self,
        typical: usize,
        principles: &[StructuralPrinciple],
        rng: &mut rand::rngs::ThreadRng
    ) -> usize {
        // Obtener rangos de los principios
        let (min_supports, max_supports) = principles.iter()
            .find_map(|p| match p {
                StructuralPrinciple::VerticalSupport { min_points, max_points } => {
                    Some((*min_points, *max_points))
                }
                _ => None,
            })
            .unwrap_or((1, 8));

        // Muestra con bias hacia el típico
        let options = (min_supports..=max_supports).collect::<Vec<_>>();
        let weights: Vec<f64> = options.iter()
            .map(|&n| {
                let diff = (n as i32 - typical as i32).abs();
                (-diff as f64 * 0.5).exp()
            })
            .collect();

        let total_weight: f64 = weights.iter().sum();
        let mut rnd = rng.gen::<f64>() * total_weight;

        for (i, &w) in weights.iter().enumerate() {
            rnd -= w;
            if rnd <= 0.0 {
                return options[i];
            }
        }

        typical.max(1)
    }

    fn generate_support_topology(
        &mut self,
        count: usize,
        width: f64,
        depth: f64,
        symmetry: &SymmetryType,
        rng: &mut rand::rngs::ThreadRng
    ) -> SupportTopology {
        let mut positions = Vec::new();

        match symmetry {
            SymmetryType::Bilateral => {
                // Distribución simétrica
                for i in 0..count {
                    let angle = (i as f64 / count.max(1) as f64) * std::f64::consts::TAU;
                    let radius = (width.min(depth) / 2.0) * 0.8;
                    let x = angle.cos() * radius;
                    let y = angle.sin() * radius * (depth / width.max(0.001));
                    
                    positions.push([x, y, 0.0]);
                }
            }
            SymmetryType::Radial { order } => {
                let actual_order = (*order).max(1);
                let angle_step = std::f64::consts::TAU / actual_order as f64;
                for i in 0..actual_order {
                    let angle = i as f64 * angle_step;
                    let radius = (width.min(depth) / 2.0) * 0.8;
                    let x = angle.cos() * radius;
                    let y = angle.sin() * radius;
                    positions.push([x, y, 0.0]);
                }
            }
            SymmetryType::Asymmetric | SymmetryType::Mixed => {
                // Distribución aleatoria pero estable
                for _ in 0..count {
                    let x = rng.gen_range(-width/2.0..=width/2.0);
                    let y = rng.gen_range(-depth/2.0..=depth/2.0);
                    positions.push([x, y, 0.0]);
                }
            }
        }

        SupportTopology {
            support_type: if count == 1 {
                SupportType::CentralPedestal
            } else if count <= 4 {
                SupportType::CornerLegs
            } else {
                SupportType::Distributed
            },
            positions,
            cross_bracing: rng.gen_bool(0.3),
        }
    }

    fn generate_surface_shape(
        &mut self,
        width: f64,
        depth: f64,
        height: f64,
        _symmetry: &SymmetryType,
        rng: &mut rand::rngs::ThreadRng
    ) -> SurfaceShape {
        let shape_type = match rng.gen_range(0..4) {
            0 => ShapeType::Rectangular,
            1 => ShapeType::Rounded,
            2 => ShapeType::Elliptical,
            _ => ShapeType::Organic,
        };

        SurfaceShape {
            shape_type,
            width,
            depth,
            height,
            edge_radius: rng.gen_range(0.0..=0.05),
            thickness: rng.gen_range(0.02..=0.08),
        }
    }

    fn generate_material_distribution(&mut self, rng: &mut rand::rngs::ThreadRng) -> MaterialDistribution {
        MaterialDistribution {
            uniform: rng.gen_bool(0.7),
            hollow_regions: if rng.gen_bool(0.3) {
                vec![HollowRegion {
                    center: [0.0, 0.0, 0.0],
                    radius: 0.1,
                }]
            } else {
                vec![]
            },
        }
    }

    fn generate_aesthetic_params(&mut self, rng: &mut rand::rngs::ThreadRng) -> AestheticParams {
        AestheticParams {
            style: self.sample_style(rng),
            curvature_preference: rng.gen_range(0.0..=1.0),
            minimalism_level: rng.gen_range(0.0..=1.0),
            golden_ratio_adherence: rng.gen_range(0.0..=1.0),
        }
    }

    fn sample_style(&mut self, rng: &mut rand::rngs::ThreadRng) -> String {
        let styles = vec![
            "modern", "minimalist", "industrial", "organic",
            "sculptural", "traditional", "futuristic", "artisan",
        ];
        styles[rng.gen_range(0..styles.len())].to_string()
    }

    /// Fitness rápido (sin simulación física completa)
    fn quick_fitness_estimate(&self, genome: &DesignGenome, essence: &Essence) -> f64 {
        let mut score = 0.5;

        // Verificar dimensiones dentro de rango
        let geo = &essence.geometric_properties;
        let w = genome.dimensions[0];
        let d = genome.dimensions[1];
        let h = genome.dimensions[2];

        let in_range = w >= geo.typical_dimensions.width.0 * 0.5
            && w <= geo.typical_dimensions.width.1 * 1.5
            && d >= geo.typical_dimensions.depth.0 * 0.5
            && d <= geo.typical_dimensions.depth.1 * 1.5
            && h >= geo.typical_dimensions.height.0 * 0.5
            && h <= geo.typical_dimensions.height.1 * 1.5;

        if in_range {
            score += 0.2;
        }

        // Verificar número de soportes
        let support_count = genome.support_topology.positions.len();
        if support_count >= 1 && support_count <= 8 {
            score += 0.2;
        }

        // Bonus por estética
        score += genome.aesthetic_params.minimalism_level * 0.1;

        score.min(1.0)
    }

    /// Cruce entre dos genomas
    pub fn crossover(&mut self, parent_a: &DesignGenome, parent_b: &DesignGenome) -> DesignGenome {
        let mut rng = rand::thread_rng();
        let mut child = parent_a.clone();
        child.id = uuid::Uuid::new_v4().to_string();
        child.generation = parent_a.generation.max(parent_b.generation) + 1;
        child.parent_ids = vec![parent_a.id.clone(), parent_b.id.clone()];

        // Mezclar dimensiones
        for i in 0..3 {
            child.dimensions[i] = if rng.gen_bool(0.5) {
                parent_a.dimensions[i]
            } else {
                parent_b.dimensions[i]
            };
        }

        // Mezclar topología (más complejo)
        if rng.gen_bool(0.5) {
            child.support_topology = parent_a.support_topology.clone();
        } else {
            child.support_topology = parent_b.support_topology.clone();
        }

        // Mezclar parámetros estéticos
        child.aesthetic_params.curvature_preference =
            (parent_a.aesthetic_params.curvature_preference + parent_b.aesthetic_params.curvature_preference) / 2.0;

        child
    }
}

/// Genoma de diseño
#[derive(Debug, Clone)]
pub struct DesignGenome {
    pub id: String,
    pub concept_id: String,
    pub dimensions: [f64; 3],
    pub support_topology: SupportTopology,
    pub surface_shape: SurfaceShape,
    pub material_distribution: MaterialDistribution,
    pub aesthetic_params: AestheticParams,
    pub estimated_fitness: f64,
    pub generation: usize,
    pub parent_ids: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct SupportTopology {
    pub support_type: SupportType,
    pub positions: Vec<[f64; 3]>,
    pub cross_bracing: bool,
}

#[derive(Debug, Clone)]
pub enum SupportType {
    CentralPedestal,
    CornerLegs,
    Distributed,
    Crossed,
}

#[derive(Debug, Clone)]
pub struct SurfaceShape {
    pub shape_type: ShapeType,
    pub width: f64,
    pub depth: f64,
    pub height: f64,
    pub edge_radius: f64,
    pub thickness: f64,
}

#[derive(Debug, Clone)]
pub enum ShapeType {
    Rectangular,
    Rounded,
    Elliptical,
    Organic,
}

#[derive(Debug, Clone)]
pub struct MaterialDistribution {
    pub uniform: bool,
    pub hollow_regions: Vec<HollowRegion>,
}

#[derive(Debug, Clone)]
pub struct HollowRegion {
    pub center: [f64; 3],
    pub radius: f64,
}

#[derive(Debug, Clone)]
pub struct AestheticParams {
    pub style: String,
    pub curvature_preference: f64,
    pub minimalism_level: f64,
    pub golden_ratio_adherence: f64,
}
