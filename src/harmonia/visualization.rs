// harmonia/visualization.rs
use super::genetic_algorithm::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionVisualization {
    pub timeline: Vec<GenerationFrame>,
    pub pareto_front: Vec<ParetoPoint>,
    pub lineage_tree: LineageTree,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationFrame {
    pub generation: u32,
    pub timestamp: String,
    pub population_heatmap: Vec<PopulationMember>,
    pub best_design_snapshot: String, // JSON del diseño
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PopulationMember {
    pub id: String,
    pub fitness: f32,
    pub age: u32,
    pub position_2d: [f32; 2], // Proyección PCA
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParetoPoint {
    pub structural_score: f32,
    pub aesthetic_score: f32,
    pub economic_score: f32,
    pub design_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageTree {
    pub nodes: Vec<LineageNode>,
    pub edges: Vec<LineageEdge>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageNode {
    pub id: String,
    pub generation: u32,
    pub fitness: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageEdge {
    pub from: String,
    pub to: String,
    pub mutation_type: String,
}

impl EvolutionResult {
    pub fn to_visualization(&self) -> EvolutionVisualization {
        // Obtenemos el timestamp actual usando chrono
        // Requiere: "chrono" en Cargo.toml
        let timestamp = chrono::Utc::now().to_rfc3339();

        let timeline = self.generations.iter()
            .map(|gen| GenerationFrame {
                generation: gen.generation,
                timestamp: timestamp.clone(),
                population_heatmap: vec![],
                best_design_snapshot: "{}".to_string(), // Serialize design
            })
            .collect();

        EvolutionVisualization {
            timeline,
            pareto_front: vec![],
            lineage_tree: LineageTree {
                nodes: vec![],
                edges: vec![],
            },
        }
    }
}
