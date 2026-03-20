//! # fragment_sim.rs – Núcleo de Aislamiento V8.6 (Anti-Race)
//! Sincronizado con TWS (Ticking World Subsystem)

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::OnceLock;
use std::collections::HashMap;
use parking_lot::Mutex;

pub const FLAG_ACTIVE: u32 = 1 << 0;

pub struct GlobalSimState {
    pub nodes: NodeBuffer,
    pub epicenter: [f32; 3],
    pub energy: f32,
    pub born_time: f64,
    pub sim_time: f64,
    pub ground_z: f32,
    pub avg_fps: f32,
    pub last_tick_time: f64,
    pub mode: i32,
    /// Búferes de salida por hash para evitar colisiones de memoria en C++
    pub output_cache: HashMap<u32, Vec<crate::CFragment>>,
}

static STATE: OnceLock<Mutex<GlobalSimState>> = OnceLock::new();

pub fn state() -> &'static Mutex<GlobalSimState> {
    STATE.get_or_init(|| Mutex::new(GlobalSimState {
        nodes: NodeBuffer::new(50_000),
        epicenter: [0.0; 3],
        energy: 1000.0,
        born_time: 0.0,
        sim_time: 0.0,
        ground_z: 0.0,
        avg_fps: 60.0,
        last_tick_time: -1.0,
        mode: 0,
        output_cache: HashMap::new(),
    }))
}

pub fn tick(dt: f32) {
    if dt <= 0.0 { return; }
    let mut s_g = state().lock();
    let s = &mut *s_g;
    
    if (s.sim_time - s.last_tick_time).abs() < 0.0001 { return; }
    s.last_tick_time = s.sim_time;
    
    let cur_fps = 1.0 / dt;
    s.avg_fps = s.avg_fps * 0.9 + cur_fps * 0.1;
    s.sim_time += dt as f64;
    
    let t = s.sim_time; let born = s.born_time; let epi = s.epicenter;
    let mode = s.mode; let g_z = s.ground_z; let dt64 = dt as f64;
    let n = s.nodes.count.load(Ordering::Relaxed).min(s.nodes.capacity);

    // [FÍSICA DE ALTO RENDIMIENTO]
    for i in 0..n {
        if s.nodes.state_flags[i] & FLAG_ACTIVE == 0 { continue; }
        let m = s.nodes.mass[i] as f64;
        let mut gravity_mod = -980.0;

        // Efecto Agujero Negro / Cinematic
        if (mode == 1 || mode == 4) && (t - born) > 0.8 {
            let dx = epi[0] - s.nodes.pos_x[i];
            let dy = epi[1] - s.nodes.pos_y[i];
            let dz = epi[2] - s.nodes.pos_z[i];
            let d2 = (dx*dx + dy*dy + dz*dz).max(100.0);
            let dist = d2.sqrt();
            // Absorber fragmentos que llegan al centro
            if dist < 30.0 { s.nodes.state_flags[i] = 0; continue; }
            // Fuerza gravitacional MASIVA (calibrada para centímetros de Unreal)
            let time_factor = ((t - born - 0.8) as f32).min(3.0) / 3.0; // Ramp up over 3 seconds
            let gf = 10000.0 * (s.energy / 1000.0) * time_factor / dist;
            s.nodes.force_x[i] += (dx/dist * gf) as f64;
            s.nodes.force_y[i] += (dy/dist * gf) as f64;
            s.nodes.force_z[i] += (dz/dist * gf) as f64;
            gravity_mod = 0.0;
            // Frenado gradual para que orbiten antes de ser absorbidos
            let drag = 0.97 - (time_factor * 0.05); 
            s.nodes.vel_x[i] *= drag; s.nodes.vel_y[i] *= drag; s.nodes.vel_z[i] *= drag;
        }

        s.nodes.vel_x[i] += (s.nodes.force_x[i] / m * dt64) as f32;
        s.nodes.vel_y[i] += (s.nodes.force_y[i] / m * dt64) as f32;
        s.nodes.vel_z[i] += ((s.nodes.force_z[i]/m + gravity_mod as f64) * dt64) as f32;
        
        s.nodes.pos_x[i] += s.nodes.vel_x[i] * dt;
        s.nodes.pos_y[i] += s.nodes.vel_y[i] * dt;
        s.nodes.pos_z[i] += s.nodes.vel_z[i] * dt;

        if s.nodes.pos_z[i] < g_z {
            s.nodes.pos_z[i] = g_z;
            s.nodes.vel_z[i] *= -0.4; s.nodes.vel_x[i] *= 0.8; s.nodes.vel_y[i] *= 0.8;
        }
        s.nodes.force_x[i] = 0.0; s.nodes.force_y[i] = 0.0; s.nodes.force_z[i] = 0.0;
    }
}

pub fn get_all_c_fragments(filter_hash: u32) -> (*const crate::CFragment, usize) {
    let mut s_g = state().lock();
    let n = s_g.nodes.count.load(Ordering::Relaxed).min(s_g.nodes.capacity);
    
    // Desestructuramos para permitir préstamos simultáneos
    let GlobalSimState { nodes, output_cache, .. } = &mut *s_g;
    
    let buffer = output_cache.entry(filter_hash).or_insert_with(|| Vec::with_capacity(1000));
    buffer.clear();

    for i in 0..n {
        if nodes.state_flags[i] & FLAG_ACTIVE != 0 {
            if filter_hash == 0 || nodes.proxy_hash[i] == filter_hash {
                buffer.push(crate::CFragment {
                    x: nodes.pos_x[i], y: nodes.pos_y[i], z: nodes.pos_z[i],
                    pitch: 0.0, yaw: 0.0, roll: 0.0,
                    sx: nodes.scale[i], sy: nodes.scale[i], sz: nodes.scale[i],
                    category: nodes.category[i],
                });
            }
        }
    }
    (buffer.as_ptr(), buffer.len())
}

pub fn register_v7(pos: [f32; 3], vel: [f32; 3], mass: f32, scale: f32, category: i32, hash: u32) {
    let mut s = state().lock();
    let idx = s.nodes.count.fetch_add(1, Ordering::Relaxed);
    if idx < s.nodes.capacity {
        s.nodes.pos_x[idx] = pos[0]; s.nodes.pos_y[idx] = pos[1]; s.nodes.pos_z[idx] = pos[2];
        s.nodes.vel_x[idx] = vel[0]; s.nodes.vel_y[idx] = vel[1]; s.nodes.vel_z[idx] = vel[2];
        s.nodes.mass[idx] = mass; s.nodes.scale[idx] = scale;
        s.nodes.category[idx] = category; s.nodes.proxy_hash[idx] = hash;
        s.nodes.state_flags[idx] = FLAG_ACTIVE;
    }
}

pub fn trigger_auto_power(preset: i32, pos: [f32; 3], energy: f32) {
    let mut s = state().lock();
    s.epicenter = pos; s.energy = energy; s.born_time = s.sim_time; s.mode = preset;
}

pub fn set_ground_z(z: f32) { state().lock().ground_z = z; }
pub fn total_count() -> usize { state().lock().nodes.count.load(Ordering::Relaxed) }

pub struct NodeBuffer {
    pub pos_x: Vec<f32>, pub pos_y: Vec<f32>, pub pos_z: Vec<f32>,
    pub vel_x: Vec<f32>, pub vel_y: Vec<f32>, pub vel_z: Vec<f32>,
    pub force_x: Vec<f64>, pub force_y: Vec<f64>, pub force_z: Vec<f64>,
    pub mass: Vec<f32>, pub scale: Vec<f32>, pub category: Vec<i32>,
    pub state_flags: Vec<u32>, pub proxy_hash: Vec<u32>,
    pub count: AtomicUsize, pub capacity: usize,
}
impl NodeBuffer {
    pub fn new(capacity: usize) -> Self {
        Self {
            pos_x: vec![0.0; capacity], pos_y: vec![0.0; capacity], pos_z: vec![0.0; capacity],
            vel_x: vec![0.0; capacity], vel_y: vec![0.0; capacity], vel_z: vec![0.0; capacity],
            force_x: vec![0.0; capacity], force_y: vec![0.0; capacity], force_z: vec![0.0; capacity],
            mass: vec![1.0; capacity], scale: vec![1.0; capacity], category: vec![0; capacity],
            state_flags: vec![0; capacity], proxy_hash: vec![0; capacity],
            count: AtomicUsize::new(0), capacity,
        }
    }
}
pub fn purge_sleeping() {}
pub fn clear() {
    let mut s = state().lock();
    s.nodes.count.store(0, Ordering::Relaxed);
    s.output_cache.clear();
}
