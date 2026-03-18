// --- soulforge_core master library (Rust) ---

pub mod ffi;
pub mod physics;
pub mod proxy;
pub mod swarm;
pub mod fragment_sim;
pub mod types;
pub mod military;
pub mod powers;
pub mod node_buffer;

// Re-exportar todo lo de ffi para que sea visible en la raíz de la DLL
pub use ffi::*;

#[repr(C)]
pub struct CFragment {
    x: f32, y: f32, z: f32,
    pitch: f32, yaw: f32, roll: f32,
    category: i32,
}

#[repr(C)]
pub struct REngineState {
    pub cpu_time: f32,
    pub gpu_time: f32,
    pub active_nodes: i32,
    pub pending_cleanups: i32,
}

// 1. El Canal de Destrucción (Láser) - ELIMINADO (Ya está en ffi.rs)

// 2. El Canal de Telemetría (El bloque de 16 bytes)
#[no_mangle]
pub extern "C" fn sf_get_engine_state(estado_ptr: *mut f32) {
    if estado_ptr.is_null() { return; }
    unsafe {
        *estado_ptr.offset(0) = 0.12; // Placeholder: deberías obtenerlo del core real
        *estado_ptr.offset(1) = 0.04;
        *estado_ptr.offset(2) = swarm::agent_count() as f32;
        *estado_ptr.offset(3) = 0.0;
    }
}

// 3. El Canal de Insight (Filtrado masivo)
#[no_mangle]
pub extern "C" fn sf_insight_filter(array_ptr: *mut i32, count: i32) {
    if array_ptr.is_null() { return; }
    // Lógica de filtrado simplificada
    let indices = unsafe { std::slice::from_raw_parts_mut(array_ptr, count as usize) };
    for i in 0..count as usize {
        indices[i] = i as i32;
    }
}
