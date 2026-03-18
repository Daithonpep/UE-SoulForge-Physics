//! # Fragment Simulation – High Performance Particle Engine
//! Optimized with SoA (Structure of Arrays) and Rayon for 100k+ particles.

use std::sync::{Mutex, OnceLock};
use serde::{Deserialize, Serialize};
use crate::proxy::FragmentCategory;
use crate::node_buffer::{NodeBuffer, FLAG_ACTIVE};

// ════════════════════════════════════════════════════════════════════════════
// CONSTANTES FÍSICAS
// ════════════════════════════════════════════════════════════════════════════

const GROUND_Z: f32 = 0.0;

// ════════════════════════════════════════════════════════════════════════════
// TIPOS
// ════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum FragmentState {
    Flying,
    Bouncing,
    Sleeping,
}

#[derive(Debug, Clone)]
pub struct ExplosionEvent {
    pub time: f64,
    pub center: [f32; 3],
    pub charge_mass_kg: f64,
}

pub(crate) struct FragSimState {
    pub(crate) nodes: NodeBuffer,
    pub(crate) events: Vec<ExplosionEvent>,
    pub(crate) ground_z: f32,
    pub(crate) sim_time: f64,
    pub(crate) power_system: crate::powers::PowerSystem,
}

impl FragSimState {
    fn new() -> Self {
        Self {
            nodes: NodeBuffer::with_capacity(200_000), 
            events: Vec::new(),
            ground_z: GROUND_Z,
            sim_time: 0.0,
            power_system: crate::powers::PowerSystem::new(),
        }
    }
}

static FRAG_SIM: OnceLock<Mutex<FragSimState>> = OnceLock::new();

pub(crate) fn state() -> &'static Mutex<FragSimState> {
    FRAG_SIM.get_or_init(|| Mutex::new(FragSimState::new()))
}

// ════════════════════════════════════════════════════════════════════════════
// API PÚBLICA
// ════════════════════════════════════════════════════════════════════════════

pub fn register_fragment(
    _category: FragmentCategory,
    position: [f32; 3],
    velocity: [f32; 3],
    mass:     f32,
    scale:     [f32; 3],
    _damage_type: i32,
    material: u16,
) -> Result<u32, String> {
    let s = state().lock().map_err(|e| e.to_string())?;
    match s.nodes.add_node(position, velocity, mass, material, -1.0, scale, _category as i32, _damage_type) {
        Some(idx) => Ok(idx as u32),
        None => Err("Buffer lleno".into()),
    }
}

pub fn register_effect_node(pos: [f32; 3], vel: [f32; 3], mass: f32, _radius: f32, life: f32, material: i32) {
    if let Ok(s) = state().lock() {
        // Los efectos (chispas, etc) van a la categoría 'Dust' (3) o especial
        s.nodes.add_node(pos, vel, mass, material as u16, life, [2.0, 2.0, 2.0], 3, 0);
    }
}

pub fn clear() {
    if let Ok(mut s) = state().lock() {
        s.nodes.count.store(0, std::sync::atomic::Ordering::SeqCst);
        s.events.clear();
        s.sim_time = 0.0;
    }
}

pub fn set_ground_z(z: f32) {
    if let Ok(mut s) = state().lock() {
        s.ground_z = z;
    }
}

pub fn total_count() -> usize {
    state().lock().map(|s| s.nodes.count.load(std::sync::atomic::Ordering::Relaxed)).unwrap_or(0)
}

pub fn active_count() -> usize {
    total_count() 
}

pub fn get_position(id: u32) -> Option<[f32; 3]> {
    let s = state().lock().ok()?;
    let i = id as usize;
    if i < s.nodes.count.load(std::sync::atomic::Ordering::Relaxed) {
        if s.nodes.state_flags[i] & FLAG_ACTIVE != 0 {
             return Some([s.nodes.pos_x[i], s.nodes.pos_y[i], s.nodes.pos_z[i]]);
        }
    }
    None
}

pub fn get_velocity(id: u32) -> Option<[f32; 3]> {
    let s = state().lock().ok()?;
    let i = id as usize;
    if i < s.nodes.count.load(std::sync::atomic::Ordering::Relaxed) {
        if s.nodes.state_flags[i] & FLAG_ACTIVE != 0 {
             return Some([s.nodes.vel_x[i], s.nodes.vel_y[i], s.nodes.vel_z[i]]);
        }
    }
    None
}

pub fn get_state(id: u32) -> Option<FragmentState> {
    let s = state().lock().ok()?;
    let i = id as usize;
    if i < s.nodes.count.load(std::sync::atomic::Ordering::Relaxed) {
        if s.nodes.state_flags[i] & FLAG_ACTIVE != 0 {
            return Some(FragmentState::Flying);
        } else {
            return Some(FragmentState::Sleeping);
        }
    }
    None
}

pub fn purge_sleeping() -> usize { 0 }

pub fn add_explosion_event(position: [f32; 3], energy_joules: f64) {
    if let Ok(mut s) = state().lock() {
        let charge_kg = energy_joules / 4_184_000.0;
        let current_time = s.sim_time;
        s.events.push(ExplosionEvent {
            time: current_time,
            center: position,
            charge_mass_kg: charge_kg,
        });
    }
}

pub fn trigger_auto_power(preset: i32, origin: [f32; 3], energy: f32) {
    if let Ok(mut s) = state().lock() {
        let current_time = s.sim_time;
        if preset == 4 { // DamagePreset::Implosion (Agujero Negro)
            s.power_system.active_powers.push(crate::powers::ActivePower {
                power_type: 1, start_time: current_time, duration: 6.0, origin: [origin[0] as f64, origin[1] as f64, origin[2] as f64], direction: [0.0, 0.0, 1.0], intensity: energy as f64 / 1000.0
            });
        } else if preset == 5 { // DamagePreset::Cinematic (Matrix / ZeroGravity)
            s.power_system.active_powers.push(crate::powers::ActivePower {
                power_type: 2, start_time: current_time, duration: 8.0, origin: [origin[0] as f64, origin[1] as f64, origin[2] as f64], direction: [0.0, 0.0, 1.0], intensity: energy as f64 / 1000.0
            });
        }
    }
}


pub fn tick(dt: f32) {
    let mut s = state().lock().unwrap();
    let dt_64 = dt as f64;
    let current_time = s.sim_time;
    
    // 1. Integrar poderes
    let mut power_system = s.power_system.clone(); 
    power_system.update(&mut s.nodes, current_time, dt_64);
    
    // 2. Física en paralelo (Incluye integración y colisión de suelo)
    let gz = s.ground_z;
    s.nodes.step_parallel(dt, gz);
    
    s.sim_time += dt_64;
}

// ════════════════════════════════════════════════════════════════════════════
// CACHES PARA FFI
// ════════════════════════════════════════════════════════════════════════════

static mut POS_CACHE: Vec<f32> = Vec::new();
static mut FRAG_CACHE: Vec<crate::CFragment> = Vec::new();
static mut TEMP_CACHE: Vec<f32> = Vec::new();

pub fn get_interleaved_positions() -> (*const f32, usize) {
    if let Ok(s) = state().lock() {
        unsafe {
            let cache = &mut *std::ptr::addr_of_mut!(POS_CACHE);
            cache.clear();
            let n = s.nodes.count.load(std::sync::atomic::Ordering::Relaxed);
            for i in 0..n {
                if s.nodes.state_flags[i] & FLAG_ACTIVE != 0 {
                    cache.push(s.nodes.pos_x[i]);
                    cache.push(s.nodes.pos_y[i]);
                    cache.push(s.nodes.pos_z[i]);
                }
            }
            (cache.as_ptr(), cache.len() / 3)
        }
    } else {
        (std::ptr::null(), 0)
    }
}

pub fn get_all_c_fragments() -> (*const crate::CFragment, usize) {
    if let Ok(s) = state().lock() {
        unsafe {
            let cache = &mut *std::ptr::addr_of_mut!(FRAG_CACHE);
            cache.clear();
            let n = s.nodes.count.load(std::sync::atomic::Ordering::Relaxed);
            for i in 0..n {
                if s.nodes.state_flags[i] & FLAG_ACTIVE != 0 {
                    cache.push(crate::CFragment {
                        x: s.nodes.pos_x[i],
                        y: s.nodes.pos_y[i],
                        z: s.nodes.pos_z[i],
                        pitch: s.nodes.scale_x[i],
                        yaw:   s.nodes.scale_y[i],
                        roll:  s.nodes.scale_z[i],
                        category: s.nodes.category[i],
                    });
                }
            }
            (cache.as_ptr(), cache.len())
        }
    } else {
        (std::ptr::null(), 0)
    }
}

pub fn get_active_temperatures() -> (*const f32, usize) {
    if let Ok(s) = state().lock() {
        unsafe {
            let cache = &mut *std::ptr::addr_of_mut!(TEMP_CACHE);
            cache.clear();
            let n = s.nodes.count.load(std::sync::atomic::Ordering::Relaxed);
            for i in 0..n {
                if s.nodes.state_flags[i] & FLAG_ACTIVE != 0 {
                    cache.push(20.0); // Placeholder: 20°C
                }
            }
            (cache.as_ptr(), cache.len())
        }
    } else {
        (std::ptr::null(), 0)
    }
}
