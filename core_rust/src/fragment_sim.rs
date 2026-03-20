//! # fragment_sim.rs – Núcleo de Simulación Maestro V8.7
//! Usa el NodeBuffer central de node_buffer.rs para evitar duplicidad.

use std::sync::atomic::Ordering;
use std::sync::OnceLock;
use std::collections::HashMap;
use parking_lot::Mutex;
use crate::node_buffer::{NodeBuffer, FLAG_ACTIVE};

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
    pub output_cache: HashMap<u32, Vec<crate::CFragment>>,
}

static STATE: OnceLock<Mutex<GlobalSimState>> = OnceLock::new();

pub fn state() -> &'static Mutex<GlobalSimState> {
    STATE.get_or_init(|| Mutex::new(GlobalSimState {
        nodes: NodeBuffer::with_capacity(50_000),
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

    for i in 0..n {
        if s.nodes.state_flags[i] & FLAG_ACTIVE == 0 { continue; }
        let m = s.nodes.mass[i] as f64;
        let mut gravity_mod = -980.0;

        // Efecto Agujero Negro (V9.0: TEMPO CINEMÁTICO)
        if mode == 1 && (t - born) > 1.5 {
            let dx = epi[0] - s.nodes.pos_x[i];
            let dy = epi[1] - s.nodes.pos_y[i];
            let dz = epi[2] - s.nodes.pos_z[i];
            let d2 = (dx*dx + dy*dy + dz*dz).max(100.0);
            let dist = d2.sqrt();
            
            if dist < 60.0 { s.nodes.state_flags[i] = 0; continue; }
            
            // Ramp-up lento: 3 segundos para llegar a potencia máxima
            let time_factor = ((t - born - 1.5) as f32).min(3.0) / 3.0; 
            
            // Gravedad Magnética desacoplada de la energía exagerada
            // Usamos log10 de la energía para que no se vuelva loco con 500M de fuerza
            let energy_mod = (s.energy / 1000.0).log10().max(1.0);
            let gf = 12000.0 * energy_mod * time_factor;
            
            s.nodes.force_x[i] += (dx/dist * gf) as f64;
            s.nodes.force_y[i] += (dy/dist * gf) as f64;
            s.nodes.force_z[i] += (dz/dist * gf) as f64;
            
            gravity_mod = 0.0;
            
            // Arrastre dinámico: aumenta conforme se acerca al centro
            let drag = 0.99 - (time_factor * 0.08); 
            s.nodes.vel_x[i] *= drag; s.nodes.vel_y[i] *= drag; s.nodes.vel_z[i] *= drag;
        }

        // Efecto Matrix (V9.3: Time Freeze)
        if mode == 4 {
            gravity_mod = 0.0;
            let time_active = (t - born) as f32;
            if time_active > 0.8 {
                // Fricción temporal extrema para congelar el movimiento
                let brake = 0.80; 
                s.nodes.vel_x[i] *= brake;
                s.nodes.vel_y[i] *= brake;
                s.nodes.vel_z[i] *= brake;
                
                // Si la velocidad es insignificante, la clavamos a cero para evitar jittering
                if s.nodes.vel_x[i].abs() < 5.0 { s.nodes.vel_x[i] = 0.0; }
                if s.nodes.vel_y[i].abs() < 5.0 { s.nodes.vel_y[i] = 0.0; }
                if s.nodes.vel_z[i].abs() < 5.0 { s.nodes.vel_z[i] = 0.0; }
            }
        }

        // Efecto Reality Shatter (V9.1: Glitch / Desincronía Cuántica)
        if mode == 6 {
            gravity_mod = 0.0;
            // Vibración errática (Glitch)
            let jitter = (t * 22.0).sin() as f32;
            if jitter > 0.8 {
                s.nodes.pos_x[i] += jitter * 5.0;
                s.nodes.pos_z[i] -= jitter * 3.0;
                s.nodes.ang_vel_x[i] *= 1.2;
            }
            // Los fragmentos se mueven sin fricción pero con cambios de dirección bruscos
            if (t as i32 % 2 == 0) && i % 5 == 0 {
                s.nodes.vel_x[i] *= -0.8;
            }
        }

        // 🌪️ EFECTO VÓRTICE / TORNADO (V9.2)
        if mode == 7 && (t - born) > 0.5 {
            let dx = s.nodes.pos_x[i] - epi[0];
            let dz = s.nodes.pos_z[i] - epi[2];
            let dist = (dx*dx + dz*dz).sqrt().max(1.0);
            let _angle = dz.atan2(dx);
            
            let time_in_vortex = (t - born - 0.5) as f32;
            
            if time_in_vortex < 4.0 { // FASE: Vórtice Activo + Subida
                gravity_mod = 0.0;
                
                // Radio objetivo según el índice del fragmento para grosor del tornado
                let target_radius = 400.0 + (i % 20) as f32 * 10.0;
                let radius_force = (target_radius - dist) * 2.0;
                
                // 1. Fuerza Centrípeta/Centrífuga (mantiene el radio)
                s.nodes.force_x[i] += (dx/dist * radius_force) as f64;
                s.nodes.force_z[i] += (dz/dist * radius_force) as f64;
                
                // 2. Fuerza Tangencial (hace girar)
                let orbital_speed = 6.0;
                let tx = -dz / dist;
                let tz = dx / dist;
                s.nodes.force_x[i] += (tx * orbital_speed * 1000.0) as f64;
                s.nodes.force_z[i] += (tz * orbital_speed * 1000.0) as f64;
                
                // 3. Elevación (Lift)
                if time_in_vortex < 2.0 {
                    s.nodes.force_z[i] += 800.0; // Sube
                } else {
                    // Flota con una onda suave
                    let wave = (time_in_vortex * 2.0 + (i % 5) as f32).sin() * 200.0;
                    s.nodes.force_z[i] += (wave + 100.0) as f64;
                }
                
                // Aplicar rozamiento para estabilizar el giro
                s.nodes.vel_x[i] *= 0.96;
                s.nodes.vel_z[i] *= 0.96;
                s.nodes.vel_y[i] *= 0.98;
            } else if time_in_vortex < 5.0 { // FASE: Dispersión
                // Salen disparados en la dirección tangencial que llevaban
                s.nodes.vel_x[i] *= 1.05;
                s.nodes.vel_z[i] *= 1.05;
                gravity_mod = -400.0; // Empiezan a caer
            }
        }

        s.nodes.vel_x[i] += (s.nodes.force_x[i] / m * dt64) as f32;
        s.nodes.vel_y[i] += (s.nodes.force_y[i] / m * dt64) as f32;
        s.nodes.vel_z[i] += ((s.nodes.force_z[i]/m + gravity_mod as f64) * dt64) as f32;
        
        s.nodes.pos_x[i] += s.nodes.vel_x[i] * dt;
        s.nodes.pos_y[i] += s.nodes.vel_y[i] * dt;
        s.nodes.pos_z[i] += s.nodes.vel_z[i] * dt;

        // Rotación física
        s.nodes.rot_x[i] += s.nodes.ang_vel_x[i] * dt;
        s.nodes.rot_y[i] += s.nodes.ang_vel_y[i] * dt;
        s.nodes.rot_z[i] += s.nodes.ang_vel_z[i] * dt;
        s.nodes.ang_vel_x[i] *= 0.99; s.nodes.ang_vel_y[i] *= 0.99; s.nodes.ang_vel_z[i] *= 0.99;

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
    
    // Desestructuramos para permitir préstamos simultáneos e independientes
    let GlobalSimState { nodes, output_cache, .. } = &mut *s_g;

    let buffer = output_cache.entry(filter_hash).or_insert_with(Vec::new);
    buffer.clear();

    for i in 0..n {
        if nodes.state_flags[i] & FLAG_ACTIVE != 0 {
            // Si el hash es 0, devolvemos ABSOLUTAMENTE TODO (Debug mode / Uni-core focus)
            if filter_hash == 0 || nodes.proxy_hash[i] == filter_hash {
                buffer.push(crate::CFragment {
                    x: nodes.pos_x[i], y: nodes.pos_y[i], z: nodes.pos_z[i],
                    pitch: nodes.rot_x[i], yaw: nodes.rot_y[i], roll: nodes.rot_z[i],
                    sx: nodes.scale_x[i], sy: nodes.scale_y[i], sz: nodes.scale_z[i],
                    category: nodes.category[i],
                });
            }
        }
    }
    (buffer.as_ptr(), buffer.len())
}

pub fn register_v7(pos: [f32; 3], vel: [f32; 3], mass: f32, scale_xyz: [f32; 3], category: i32, hash: u32) {
    let mut s = state().lock();
    let idx = s.nodes.count.fetch_add(1, Ordering::Relaxed);
    if idx < s.nodes.capacity {
        s.nodes.pos_x[idx] = pos[0]; s.nodes.pos_y[idx] = pos[1]; s.nodes.pos_z[idx] = pos[2];
        s.nodes.vel_x[idx] = vel[0]; s.nodes.vel_y[idx] = vel[1]; s.nodes.vel_z[idx] = vel[2];
        s.nodes.mass[idx] = mass;
        s.nodes.scale_x[idx] = scale_xyz[0]; s.nodes.scale_y[idx] = scale_xyz[1]; s.nodes.scale_z[idx] = scale_xyz[2];
        s.nodes.category[idx] = category; s.nodes.proxy_hash[idx] = hash;
        s.nodes.state_flags[idx] = FLAG_ACTIVE as u8;
        
        // Inicializar rotación con algo de azar
        s.nodes.rot_x[idx] = (idx % 360) as f32;
        s.nodes.ang_vel_x[idx] = (idx % 100) as f32;
    }
}

pub fn trigger_auto_power(preset: i32, pos: [f32; 3], energy: f32) {
    let mut s = state().lock();
    s.epicenter = pos; s.energy = energy; s.born_time = s.sim_time; s.mode = preset;
}

pub fn set_ground_z(z: f32) { state().lock().ground_z = z; }
pub fn purge_sleeping() {}
pub fn clear() {
    let mut s = state().lock();
    s.nodes.count.store(0, Ordering::Relaxed);
    s.output_cache.clear();
}
