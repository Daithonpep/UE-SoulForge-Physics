use crate::archetype::taxonomy::*;
use crate::archetype::essence::*;
use crate::archetype::morpho::*;

/// Genesis Loop mejorado con ARCHETYPE
pub struct ArchetypeGenesisLoop {
    taxonomy: TaxonomyTree,
    extractor: EssenceExtractor,
    generator: MorphoGenerator,
    current_concept: String,
}

impl ArchetypeGenesisLoop {
    pub fn new() -> Self {
        let taxonomy = TaxonomyTree::new();
        let extractor = EssenceExtractor::new(taxonomy.clone());
        
        let generator = MorphoGenerator::new(
            MorphoConfig {
                variations_per_concept: 100_000, // 100k variaciones internas
                mutation_strength: 0.3,
                enable_cross_breeding: true,
                elite_count: 100, // Solo 100 mejores pasan a simulación real
            },
            taxonomy.clone(),
        );

        Self {
            taxonomy,
            extractor,
            generator,
            current_concept: "dining_table".into(),
        }
    }

    /// Cambia el objetivo de aprendizaje
    pub fn set_target_concept(&mut self, concept_id: &str) -> Result<(), String> {
        if self.taxonomy.get_node(concept_id).is_some() {
            self.current_concept = concept_id.to_string();
            Ok(())
        } else {
            Err(format!("Concept '{}' not found in taxonomy", concept_id))
        }
    }

    /// Ejecuta una era de entrenamiento basada en arquetipos
    pub fn run_archetype_era(&mut self) -> ArchetypeEraResult {
        // 1. Obtener concepto objetivo
        let concept = self.taxonomy.get_node(&self.current_concept)
            .expect("Current concept must exist")
            .clone();

        // 2. Analizar seed examples
        let seed_analyses: Vec<SeedAnalysis> = concept.seed_examples.iter()
            .map(|seed| self.extractor.analyze_seed(seed))
            .collect();

        // 3. Extraer principios comunes
        let principles = self.extractor.extract_common_principles(&seed_analyses);

        // 4. Generar 100,000 variaciones internas
        log::info!("[ARCHETYPE] Generating 100,000 internal variations for '{}'...", concept.name);
        let elite_genomes = self.generator.generate_variations(
            &self.current_concept,
            &principles,
        );

        log::info!("[ARCHETYPE] Elite count after internal selection: {}", elite_genomes.len());

        // 5. Convertir elite a formato Genesis para simulación real (Placeholder/Mock up for now)
        let genesis_designs: Vec<GenesisDesign> = elite_genomes.iter()
            .take(10) // Solo simular los top 10 en física completa
            .map(|genome| self.genome_to_genesis(genome))
            .collect();

        // 6. Simular en Phoenix/SEISMO (física real)
        let mut validated_designs = Vec::new();
        for design in genesis_designs {
            let sim_result = self.simulate_physics(&design);
            if sim_result.is_stable {
                validated_designs.push((design, sim_result));
            }
        }

        log::info!("[ARCHETYPE] Validated designs after physics: {}", validated_designs.len());

        ArchetypeEraResult {
            concept_id: self.current_concept.clone(),
            internal_variations_generated: 100_000,
            elite_selected: elite_genomes.len(),
            physics_validated: validated_designs.len(),
            best_designs: validated_designs.into_iter()
                .take(3)
                .map(|(d, _)| d)
                .collect(),
        }
    }

    fn genome_to_genesis(&self, genome: &DesignGenome) -> GenesisDesign {
        // Convertir DesignGenome a formato que Genesis/Canvas entiende
        GenesisDesign {
            id: genome.id.clone(),
            tabletop: genesis_design::Tabletop {
                width: genome.dimensions[0],
                depth: genome.dimensions[1],
                height: genome.dimensions[2],
                shape: match genome.surface_shape.shape_type {
                    ShapeType::Rectangular => "rectangular".into(),
                    ShapeType::Rounded => "rounded".into(),
                    ShapeType::Elliptical => "elliptical".into(),
                    ShapeType::Organic => "organic".into(),
                },
            },
            legs: genome.support_topology.positions.iter()
                .map(|pos| genesis_design::Leg {
                    x: pos[0],
                    y: pos[1],
                    radius: 0.03,
                    taper: 0.8,
                })
                .collect(),
            fitness: genome.estimated_fitness,
        }
    }

    fn simulate_physics(&self, _design: &GenesisDesign) -> PhysicsResult {
        // Llamar a Phoenix/SEISMO para simulación real
        // (Por ahora, placeholder)
        PhysicsResult {
            is_stable: true,
            has_floating: false,
            collisions: 0,
            stress_max: 1.5e6,
        }
    }
}

#[derive(Debug)]
pub struct ArchetypeEraResult {
    pub concept_id: String,
    pub internal_variations_generated: usize,
    pub elite_selected: usize,
    pub physics_validated: usize,
    pub best_designs: Vec<GenesisDesign>,
}

pub mod genesis_design {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Tabletop {
        pub width: f64,
        pub depth: f64,
        pub height: f64,
        pub shape: String,
    }

    #[derive(Debug, Clone, Serialize, Deserialize)]
    pub struct Leg {
        pub x: f64,
        pub y: f64,
        pub radius: f64,
        pub taper: f64,
    }
}

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenesisDesign {
    pub id: String,
    pub tabletop: genesis_design::Tabletop,
    pub legs: Vec<genesis_design::Leg>,
    pub fitness: f64,
}

#[derive(Debug)]
pub struct PhysicsResult {
    pub is_stable: bool,
    pub has_floating: bool,
    pub collisions: usize,
    pub stress_max: f64,
}
