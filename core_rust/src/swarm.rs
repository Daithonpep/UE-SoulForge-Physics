//! # Swarm – Cerebro de Enjambre (v3 – Optimización Masiva para 100k+ Nodos)
//!
//! Implementa el comportamiento colectivo usando Rayon para paralelismo 
//! y una Rejilla Espacial (Grid Hash) para reducir la complejidad de O(N²) a O(N).

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use rayon::prelude::*;

// ─── Tipos ────────────────────────────────────────────────────────────────────

pub type Vec3 = [f32; 3];

#[derive(Debug, Clone)]
pub struct SwarmAgent {
    pub id:           u32,
    pub position:     Vec3,
    pub velocity:     Vec3,
    pub acceleration: Vec3,
    pub mass:         f32,
    pub is_dormant:   bool,
    pub energy:       f32,
}

impl SwarmAgent {
    fn new(id: u32, position: Vec3) -> Self {
        Self {
            id,
            position,
            velocity: [0.0, 0.0, 0.0],
            acceleration: [0.0, 0.0, 0.0],
            mass: 1.0,
            is_dormant: false,
            energy: 0.0,
        }
    }
}

// ─── Rejilla Espacial (Grid Hash) ──────────────────────────────────────────────

const GRID_CELL_SIZE: f32 = 100.0; // 100cm (1 metro) por celda

#[derive(Clone)]
struct SpatialGrid {
    cells: HashMap<(i32, i32, i32), Vec<u32>>,
}

impl SpatialGrid {
    fn new(agents: &HashMap<u32, SwarmAgent>) -> Self {
        let mut cells = HashMap::new();
        for agent in agents.values() {
            if agent.is_dormant { continue; }
            let key = (
                (agent.position[0] / GRID_CELL_SIZE).floor() as i32,
                (agent.position[1] / GRID_CELL_SIZE).floor() as i32,
                (agent.position[2] / GRID_CELL_SIZE).floor() as i32,
            );
            cells.entry(key).or_insert_with(Vec::new).push(agent.id);
        }
        Self { cells }
    }

    fn get_neighbors(&self, pos: &Vec3) -> Vec<u32> {
        let mut neighbors = Vec::new();
        let gx = (pos[0] / GRID_CELL_SIZE).floor() as i32;
        let gy = (pos[1] / GRID_CELL_SIZE).floor() as i32;
        let gz = (pos[2] / GRID_CELL_SIZE).floor() as i32;

        for dx in -1..=1 {
            for dy in -1..=1 {
                for dz in -1..=1 {
                    if let Some(ids) = self.cells.get(&(gx + dx, gy + dy, gz + dz)) {
                        neighbors.extend(ids);
                    }
                }
            }
        }
        neighbors
    }
}

// ─── Estado Global ────────────────────────────────────────────────────────────

struct SwarmState {
    agents:     HashMap<u32, SwarmAgent>,
    next_id:    u32,
    pub sleep_epsilon: f32,
}

impl SwarmState {
    fn new() -> Self {
        Self {
            agents:  HashMap::new(),
            next_id: 0,
            sleep_epsilon: 0.01,
        }
    }
}

static SWARM: OnceLock<Mutex<SwarmState>> = OnceLock::new();
fn state() -> &'static Mutex<SwarmState> {
    SWARM.get_or_init(|| Mutex::new(SwarmState::new()))
}

// ─── API Interna ──────────────────────────────────────────────────────────────

pub fn register_agent(position: Vec3, dormant: bool) -> Result<u32, String> {
    let mut s = state().lock().map_err(|e| e.to_string())?;
    let id = s.next_id;
    let mut agent = SwarmAgent::new(id, position);
    agent.is_dormant = dormant;
    s.agents.insert(id, agent);
    s.next_id += 1;
    Ok(id)
}

pub fn clear() {
    if let Ok(mut s) = state().lock() {
        s.agents.clear();
        s.next_id = 0;
    }
}

pub fn wake_agent(id: u32) {
    if let Ok(mut s) = state().lock() {
        if let Some(a) = s.agents.get_mut(&id) {
            a.is_dormant = false;
            a.energy = 1.0;
        }
    }
}

pub fn wake_agent_near(pos: Vec3, radius: f32) {
    if let Ok(mut s) = state().lock() {
        let r_sq = radius * radius;
        for a in s.agents.values_mut() {
            if a.is_dormant && sq_dist(&a.position, &pos) < r_sq {
                a.is_dormant = false;
                a.energy = 1.0;
            }
        }
    }
}

pub fn get_all_active_positions() -> Vec<Vec3> {
    if let Ok(s) = state().lock() {
        s.agents.values()
            .filter(|a| !a.is_dormant)
            .map(|a| a.position)
            .collect()
    } else {
        Vec::new()
    }
}

/// Retorna una copia del agente con el ID dado.
pub fn get_agent(id: u32) -> Option<SwarmAgent> {
    state().lock().ok()?.agents.get(&id).cloned()
}

/// Retorna el número total de agentes registrados.
pub fn agent_count() -> usize {
    state().lock().map(|s| s.agents.len()).unwrap_or(0)
}

/// Elimina el agente con el ID dado del enjambre.
pub fn remove_agent(id: u32) -> bool {
    state()
        .lock()
        .map(|mut s| s.agents.remove(&id).is_some())
        .unwrap_or(false)
}

// ─── Tick Optimizado ─────────────────────────────────────────────────────────

pub fn tick(dt: f32) {
    // 1. Preparar datos (Borrow inmutable para Rayon)
    let (agents_list, sleep_epsilon) = {
        let s = state().lock().unwrap();
        (s.agents.values().cloned().collect::<Vec<_>>(), s.sleep_epsilon)
    };

    if agents_list.is_empty() { return; }

    // 2. Construir Rejilla Espacial (O(N))
    // Usamos un mapa local para evitar bloqueos globales durante la construcción
    let mut grid_map: HashMap<(i32, i32, i32), Vec<u32>> = HashMap::new();
    let mut active_agents = Vec::new();
    for a in &agents_list {
        if !a.is_dormant {
            let key = (
                (a.position[0] / GRID_CELL_SIZE).floor() as i32,
                (a.position[1] / GRID_CELL_SIZE).floor() as i32,
                (a.position[2] / GRID_CELL_SIZE).floor() as i32,
            );
            grid_map.entry(key).or_insert_with(Vec::new).push(a.id);
            active_agents.push(a.clone());
        }
    }
    
    let grid = SpatialGrid { cells: grid_map };
    // Mapeo ID -> Agent para búsqueda rápida en Rayon
    let agents_map: HashMap<u32, SwarmAgent> = agents_list.iter().map(|a| (a.id, a.clone())).collect();

    // 3. Procesar en PARALELO con Rayon (O(N))
    let updates: Vec<SwarmAgent> = agents_list.par_iter().filter_map(|agent| {
        let mut updated = agent.clone();
        
        // Lógica de Despertar: si estamos dormidos, ver si hay vecinos activos cerca
        if updated.is_dormant {
            let neighbor_ids = grid.get_neighbors(&updated.position);
            let any_neighbor_awake = neighbor_ids.iter().any(|&nid| {
                if let Some(n) = agents_map.get(&nid) {
                    sq_dist(&updated.position, &n.position) < 80.0*80.0
                } else { false }
            });
            if any_neighbor_awake {
                updated.is_dormant = false;
                updated.energy = 1.0;
            } else {
                return None; // Sigue durmiendo, no hay actualización
            }
        }

        // --- Calcular Fuerzas de Boids usando la Rejilla ---
        let neighbor_ids = grid.get_neighbors(&updated.position);
        let neighbors: Vec<&SwarmAgent> = neighbor_ids.iter()
            .filter(|&&nid| nid != updated.id)
            .filter_map(|nid| agents_map.get(nid))
            .collect();

        let force = calculate_flocking_forces_optimized(&updated, &neighbors);
        
        // --- Integración Física ---
        let gravity = -980.0;
        let damping = 0.995;
        let ground_z = 0.0;

        updated.acceleration[0] = force[0] / updated.mass;
        updated.acceleration[1] = force[1] / updated.mass;
        updated.acceleration[2] = (force[2] / updated.mass) + gravity;

        updated.velocity[0] += updated.acceleration[0] * dt;
        updated.velocity[1] += updated.acceleration[1] * dt;
        updated.velocity[2] += updated.acceleration[2] * dt;

        updated.velocity[0] *= damping;
        updated.velocity[1] *= damping;
        updated.velocity[2] *= damping;

        limit_vector(&mut updated.velocity, 5000.0);

        updated.energy = updated.velocity[0]*updated.velocity[0] + 
                         updated.velocity[1]*updated.velocity[1] + 
                         updated.velocity[2]*updated.velocity[2];

        if updated.energy < sleep_epsilon {
            updated.velocity = [0.0, 0.0, 0.0];
            updated.is_dormant = true;
        } else {
            updated.position[0] += updated.velocity[0] * dt;
            updated.position[1] += updated.velocity[1] * dt;
            updated.position[2] += updated.velocity[2] * dt;

            if updated.position[2] < ground_z {
                updated.position[2] = ground_z;
                updated.velocity[2] = -updated.velocity[2] * 0.2;
                updated.velocity[0] *= 0.5;
                updated.velocity[1] *= 0.5;
            }
        }

        Some(updated)
    }).collect();

    // 4. Aplicar resultados al estado global
    if let Ok(mut s) = state().lock() {
        for upd in updates {
            s.agents.insert(upd.id, upd);
        }
    }
}

fn calculate_flocking_forces_optimized(agent: &SwarmAgent, neighbors: &[&SwarmAgent]) -> Vec3 {
    if neighbors.is_empty() { return [0.0, 0.0, 0.0]; }

    const PERCEPTION_RADIUS_SQ: f32 = 200.0 * 200.0;
    const SEP_RADIUS_SQ:   f32 = 75.0 * 75.0;
    const W_SEP: f32 = 0.8; // Reducido drásticamente (1.8 -> 0.8) para evitar el "Sky Launch"
    const W_ALI: f32 = 1.2; // Más cohesión grupal
    const W_COH: f32 = 1.4; // Menos competencia por el espacio

    let mut separation = [0.0f32; 3];
    let mut alignment  = [0.0f32; 3];
    let mut cohesion   = [0.0f32; 3];
    let mut count = 0;

    for n in neighbors {
        let sq = sq_dist(&agent.position, &n.position);
        if sq > PERCEPTION_RADIUS_SQ { continue; }
        
        count += 1;
        let d = sq.sqrt().max(0.001);
        let sep_strength = if sq < SEP_RADIUS_SQ { 2.5 / d } else { 1.2 / d };
        
        separation[0] += (agent.position[0] - n.position[0]) * sep_strength;
        separation[1] += (agent.position[1] - n.position[1]) * sep_strength;
        separation[2] += (agent.position[2] - n.position[2]) * sep_strength;

        alignment[0] += n.velocity[0];
        alignment[1] += n.velocity[1];
        alignment[2] += n.velocity[2];

        cohesion[0] += n.position[0];
        cohesion[1] += n.position[1];
        cohesion[2] += n.position[2];
    }

    if count == 0 { return [0.0, 0.0, 0.0]; }

    let inv_count = 1.0 / count as f32;
    alignment = [alignment[0] * inv_count, alignment[1] * inv_count, alignment[2] * inv_count];
    cohesion = [(cohesion[0] * inv_count - agent.position[0]) * 0.1, 
                (cohesion[1] * inv_count - agent.position[1]) * 0.1, 
                (cohesion[2] * inv_count - agent.position[2]) * 0.1];

    [
        separation[0] * W_SEP + alignment[0] * W_ALI + cohesion[0] * W_COH,
        separation[1] * W_SEP + alignment[1] * W_ALI + cohesion[1] * W_COH,
        separation[2] * W_SEP + alignment[2] * W_ALI + cohesion[2] * W_COH,
    ]
}

// ─── Auxiliares ──────────────────────────────────────────────────────────────

fn sq_dist(a: &Vec3, b: &Vec3) -> f32 {
    let dx = a[0] - b[0];
    let dy = a[1] - b[1];
    let dz = a[2] - b[2];
    dx*dx + dy*dy + dz*dz
}

fn limit_vector(v: &mut Vec3, max: f32) {
    let sq_mag = v[0]*v[0] + v[1]*v[1] + v[2]*v[2];
    if sq_mag > max*max {
        let mag = sq_mag.sqrt();
        v[0] = (v[0] / mag) * max;
        v[1] = (v[1] / mag) * max;
        v[2] = (v[2] / mag) * max;
    }
}
