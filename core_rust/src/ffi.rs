//! # ffi.rs – Puente de Comunicación SoulForge V8.7
//! Punto de convergencia único entre C++ y Rust.

use std::sync::atomic::Ordering;
use std::ffi::CStr;
use std::os::raw::c_char;
use crate::proxy::{self, DamagePreset};

// ══════════════════════════════════════════════════════════════
// 1. FUNCIONES DE LOG Y TELEMETRÍA
// ══════════════════════════════════════════════════════════════

static mut LOG_CALLBACK: Option<extern "C" fn(*const c_char)> = None;

#[no_mangle]
pub extern "C" fn sf_registrar_logger(cb: extern "C" fn(*const c_char)) {
    unsafe { LOG_CALLBACK = Some(cb); }
    sf_log("✅ [SoulForge-Rust] Canal de telemetría establecido.");
}

pub fn sf_log(msg: &str) {
    if let Some(cb) = unsafe { LOG_CALLBACK } {
        if let Ok(c_str) = std::ffi::CString::new(msg) {
            cb(c_str.as_ptr());
        }
    }
}

// ══════════════════════════════════════════════════════════════
// 2. CICLO DE VIDA (INIT / TICK / CLEANUP)
// ══════════════════════════════════════════════════════════════

#[no_mangle]
pub extern "C" fn sf_init_physics() {
    crate::fragment_sim::state(); // Fuerza inicialización
    sf_log("🚀 Nucleo SoulForge iniciado.");
}

#[no_mangle]
pub extern "C" fn sf_tick_physics(dt: f32) {
    crate::fragment_sim::tick(dt);
}

#[no_mangle]
pub extern "C" fn sf_limpiar_nucleo() {
    crate::fragment_sim::clear();
    sf_log("♻️ Nucleo purgado.");
}

// ══════════════════════════════════════════════════════════════
// 3. GESTIÓN DE PROXIES (DNA OBJETOS)
// ══════════════════════════════════════════════════════════════

#[no_mangle]
pub extern "C" fn sf_register_proxy(
    id_ptr: *const c_char,
    x: f32, y: f32, z: f32,
    ex: f32, ey: f32, ez: f32,
    density: f32
) -> i32 {
    if id_ptr.is_null() { return -1; }
    let id = unsafe { CStr::from_ptr(id_ptr) }.to_string_lossy();
    proxy::register(id.to_string(), [x, y, z], [ex, ey, ez], density);
    1
}

#[no_mangle]
pub extern "C" fn sf_remove_proxy(id_ptr: *const c_char) -> i32 {
    if id_ptr.is_null() { return -1; }
    let id = unsafe { CStr::from_ptr(id_ptr) }.to_string_lossy();
    proxy::remove(&id);
    1
}

#[no_mangle]
pub extern "C" fn sf_detonar_nativo(
    id_ptr: *const c_char,
    energy: f32,
    preset_f: f32,
    _frag_level: f32,
    _radius: f32,
    custom_count: i32
) -> u32 {
    if id_ptr.is_null() { return 0; }
    let id = unsafe { CStr::from_ptr(id_ptr) }.to_string_lossy();
    let preset = DamagePreset::from_i32(preset_f as i32);
    
    // Devolvemos el conteo real de nodos para que C++ dimensione sus buffers
    let (hash, count) = proxy::destroy(&id, preset, energy, _frag_level, custom_count);
    
    sf_log(&format!("💥 [RUST-DETONATION] {} | hash={} | nodos={}", id, hash, count));
    count
}

// ══════════════════════════════════════════════════════════════
// 4. DATA PIPELINE (ZERO-COPY)
// ══════════════════════════════════════════════════════════════

#[no_mangle]
pub extern "C" fn sf_get_all_fragment_data(filter_hash: u32, out_count: *mut i32) -> *const crate::CFragment {
    let (ptr, len) = crate::fragment_sim::get_all_c_fragments(filter_hash);
    unsafe { *out_count = len as i32; }
    ptr
}

#[no_mangle]
pub extern "C" fn sf_get_engine_state(out_state_ptr: *mut crate::REngineState) {
    if out_state_ptr.is_null() { return; }
    
    let s = crate::fragment_sim::state().lock();
    let n = s.nodes.count.load(Ordering::Relaxed) as f32;
    
    unsafe {
        (*out_state_ptr).active_nodes = n;
        (*out_state_ptr).fps = s.avg_fps;
        (*out_state_ptr).efficiency = (n / 50000.0).min(1.0);
        (*out_state_ptr).sim_time = s.sim_time as f32;
    }
}
