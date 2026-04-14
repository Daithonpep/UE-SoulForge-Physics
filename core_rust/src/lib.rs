// --- soulforge_core master library (Rust) ---
// Versión Unificada Singularidad V5: Se han eliminado 'powers' y 'swarm' para evitar interferencias.

pub mod ffi;
pub mod proxy;
pub mod fragment_sim;
pub mod types;
pub mod military;
pub mod node_buffer;

pub use ffi::*;

#[repr(C)]
pub struct CFragment {
    pub x: f32, pub y: f32, pub z: f32,
    pub pitch: f32, pub yaw: f32, pub roll: f32,
    pub sx: f32, pub sy: f32, pub sz: f32,
    pub category: i32,
}

#[repr(C)]
pub struct REngineState {
    pub active_nodes: f32,
    pub fps: f32,
    pub efficiency: f32,
    pub sim_time: f32,
}

#[no_mangle]
pub extern "C" fn sf_insight_filter_instances(
    posiciones_ptr: *const [f64; 3],
    count: i32,
    cam_x: f64,
    cam_y: f64,
    cam_z: f64,
    vision_dist: f64,
    out_indices_ptr: *mut i32,
) -> i32 {
    if posiciones_ptr.is_null() || out_indices_ptr.is_null() || count <= 0 { 
        return 0; 
    }

    use rayon::prelude::*;

    let positions = unsafe { std::slice::from_raw_parts(posiciones_ptr, count as usize) };
    let sq_dist = vision_dist * vision_dist;

    let valid_indices: Vec<i32> = positions
        .par_iter()
        .enumerate()
        .filter_map(|(i, pos)| {
            let dx = pos[0] - cam_x;
            let dy = pos[1] - cam_y;
            let dz = pos[2] - cam_z;
            if (dx*dx + dy*dy + dz*dz) <= sq_dist {
                Some(i as i32)
            } else {
                None
            }
        })
        .collect();

    let out_indices = unsafe { std::slice::from_raw_parts_mut(out_indices_ptr, valid_indices.len().min(count as usize)) };
    out_indices.copy_from_slice(&valid_indices[..out_indices.len()]);

    valid_indices.len() as i32
}
