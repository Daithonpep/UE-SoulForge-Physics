pub mod classic;
pub mod explosions;

use crate::node_buffer::{NodeBuffer, FLAG_ACTIVE};
use rayon::prelude::*;
use std::sync::atomic::Ordering;

/// Ejecuta el paso de física en paralelo usando solvers modulares.
pub fn run_physics_step(nodes: &mut NodeBuffer, dt: f32, ground_z: f32) {
    let n = nodes.count.load(Ordering::Relaxed).min(nodes.capacity);
    let dt_64 = dt as f64;
    let gravity = -980.0;
    
    // TRUCO DE LA FORJA: Pasamos el puntero como usize para Rayon.
    let nodes_raw_ptr = nodes as *mut NodeBuffer as usize;

    (0..n).into_par_iter().for_each(|i| {
        unsafe {
            let p = nodes_raw_ptr as *mut NodeBuffer;

            // Acceso explícito para evitar 'dangerous_implicit_autorefs'
            if *((*p).state_flags.as_ptr().add(i)) & FLAG_ACTIVE != 0 {
                
                // 1. Efectos Especiales (Modifican vel/life)
                explosions::apply_explosion_effects(&mut *p, i, dt);
                
                // 2. Integración Modular
                let g_scale = explosions::get_gravity_scale(&*p, i);
                let d_scale = explosions::get_drag_scale(&*p, i);
                classic::integrate_classic_modular(&mut *p, i, dt, gravity * g_scale, dt_64, d_scale);
                
                // 3. Colisión de suelo
                classic::handle_ground_collision(&mut *p, i, ground_z);
            }
        }
    });
    
    nodes.mark_dirty();
}
