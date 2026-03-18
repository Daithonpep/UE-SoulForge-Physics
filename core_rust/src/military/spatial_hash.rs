use std::collections::HashMap;
use super::Vec3;

/// Grid adaptativo multinivel
pub struct HierarchicalSpatialHash {
    pub levels: Vec<SpatialLevel>,
}

pub struct SpatialLevel {
    pub cell_size: f64,
    pub cells: HashMap<CellKey, Vec<NodeIndex>>,
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub struct CellKey(pub i32, pub i32, pub i32);

pub type NodeIndex = u32;

impl HierarchicalSpatialHash {
    pub fn new(min_cell: f64, max_cell: f64, num_levels: usize) -> Self {
        let ratio = if num_levels > 1 {
            (max_cell / min_cell).powf(1.0 / (num_levels - 1) as f64)
        } else {
            1.0
        };
        
        let levels = (0..num_levels)
            .map(|i| SpatialLevel {
                cell_size: min_cell * ratio.powi(i as i32),
                cells: HashMap::new(),
            })
            .collect();

        Self { levels }
    }

    /// Cada nodo va al nivel apropiado según su radio de influencia
    pub fn insert(&mut self, index: NodeIndex, pos: Vec3, influence_radius: f64) {
        let level_idx = self.levels
            .iter()
            .position(|l| l.cell_size >= influence_radius)
            .unwrap_or(self.levels.len() - 1);

        let level = &mut self.levels[level_idx];
        let key = CellKey(
            (pos.x / level.cell_size).floor() as i32,
            (pos.y / level.cell_size).floor() as i32,
            (pos.z / level.cell_size).floor() as i32,
        );

        level.cells.entry(key).or_default().push(index);
    }

    /// Consulta eficiente de vecinos
    pub fn query_neighbors(&self, pos: Vec3, radius: f64) -> Vec<NodeIndex> {
        let mut results = Vec::new();

        for level in &self.levels {
            if level.cell_size > radius * 10.0 {
                continue; // Nivel demasiado grueso
            }

            let cells_per_axis = (radius / level.cell_size).ceil() as i32 + 1;
            let center = CellKey(
                (pos.x / level.cell_size).floor() as i32,
                (pos.y / level.cell_size).floor() as i32,
                (pos.z / level.cell_size).floor() as i32,
            );

            for dx in -cells_per_axis..=cells_per_axis {
                for dy in -cells_per_axis..=cells_per_axis {
                    for dz in -cells_per_axis..=cells_per_axis {
                        let key = CellKey(center.0 + dx, center.1 + dy, center.2 + dz);
                        if let Some(nodes) = level.cells.get(&key) {
                            results.extend_from_slice(nodes);
                        }
                    }
                }
            }
        }

        results
    }
    
    pub fn clear(&mut self) {
        for level in &mut self.levels {
            level.cells.clear();
        }
    }
}
