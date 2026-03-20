//! # Explosion Solver – Efectos Especiales (Black Hole, Matrix, etc.)
//! Este módulo maneja la lógica de fuerzas radiales y temporales asociadas a explosiones.

use crate::node_buffer::NodeBuffer;

pub fn apply_explosion_effects(nodes: &mut NodeBuffer, i: usize, dt: f32) {
    let p = nodes as *mut NodeBuffer;
    unsafe {
        let d_type = *((*p).damage_type.as_ptr().add(i));
        let life_ptr = (*p).lifetime.as_mut_ptr().add(i);
        *life_ptr += dt;
        let current_life = *life_ptr;
        
        match d_type {
            4 => { // AGUJERO NEGRO / IMPLOSION
                let sx = *((*p).source_x.as_ptr().add(i));
                let sy = *((*p).source_y.as_ptr().add(i));
                let sz = *((*p).source_z.as_ptr().add(i));
                
                let px = *((*p).pos_x.as_ptr().add(i));
                let py = *((*p).pos_y.as_ptr().add(i));
                let pz = *((*p).pos_z.as_ptr().add(i));
                
                let dx = sx - px;
                let dy = sy - py;
                let dz = sz - pz;
                let dist = (dx*dx + dy*dy + dz*dz).sqrt().max(1.0);
                
                if current_life < 1.8 {
                    let suck = 2000.0 * (1.0 + current_life);
                    *((*p).vel_x.as_mut_ptr().add(i)) += (dx / dist) * suck * dt;
                    *((*p).vel_y.as_mut_ptr().add(i)) += (dy / dist) * suck * dt;
                    *((*p).vel_z.as_mut_ptr().add(i)) += (dz / dist) * suck * dt;
                } else if current_life < 2.0 {
                    *((*p).vel_x.as_mut_ptr().add(i)) *= 0.1;
                    *((*p).vel_y.as_mut_ptr().add(i)) *= 0.1;
                    *((*p).vel_z.as_mut_ptr().add(i)) *= 0.1;
                } else if current_life < 2.3 {
                    let boom = 9000.0;
                    *((*p).vel_x.as_mut_ptr().add(i)) -= (dx / dist) * boom;
                    *((*p).vel_y.as_mut_ptr().add(i)) -= (dy / dist) * boom;
                    *((*p).vel_z.as_mut_ptr().add(i)) -= (dz / dist) * boom;
                }
            },
            5 => { // MATRIX / ZERO-G
                if current_life < 6.0 {
                    *((*p).ang_vel_x.as_mut_ptr().add(i)) *= 1.03;
                }
            },
            _ => { }
        }
    }
}

pub fn get_gravity_scale(nodes: &NodeBuffer, i: usize) -> f32 {
    unsafe {
        let d_type = *(nodes.damage_type.as_ptr().add(i));
        let life = *(nodes.lifetime.as_ptr().add(i));
        match d_type {
            5 => 0.05, 
            4 => if life < 2.0 { 0.0 } else { 1.0 },
            _ => 1.0,
        }
    }
}

pub fn get_drag_scale(nodes: &NodeBuffer, i: usize) -> f32 {
    unsafe {
        let d_type = *(nodes.damage_type.as_ptr().add(i));
        let life = *(nodes.lifetime.as_ptr().add(i));
        match d_type {
            4 => if life < 1.8 { 0.95 } else { 0.999 },
            5 => 0.97,
            _ => 0.999,
        }
    }
}
