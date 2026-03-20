//! # ffi.rs – API Completa V8.7 (Sincronizada con SoulForgeBridge.cpp)
//! REGLA: Cada función que el Bridge busca, DEBE existir aquí.

use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_int};
use crate::proxy::{self, DamagePreset};
use crate::fragment_sim;

static mut UNREAL_LOG_CALLBACK: Option<extern "C" fn(*const c_char)> = None;

// ══════════════════════════════════════════════════════════════
//  FUNCIONES VITALES (El Bridge las busca al arrancar)
// ══════════════════════════════════════════════════════════════

#[no_mangle]
pub extern "C" fn sf_init_physics() {
    sf_log(">>> [RUST] Núcleo V8.7 inicializado.");
}

#[no_mangle]
pub extern "C" fn sf_tick_physics(dt: f32) -> i32 {
    fragment_sim::tick(dt);
    let s = fragment_sim::state().lock();
    s.nodes.count.load(std::sync::atomic::Ordering::Relaxed) as i32
}

#[no_mangle]
pub extern "C" fn sf_shutdown_physics() {
    sf_log("[RUST] Motor apagado.");
}

#[no_mangle]
pub extern "C" fn sf_shutdown_engine() {
    sf_log("[RUST] Engine shutdown.");
}

#[no_mangle]
pub extern "C" fn sf_register_proxy(
    id: *const c_char, 
    x: f32, y: f32, z: f32, 
    ex: f32, ey: f32, ez: f32, 
    density: f32
) -> bool {
    if id.is_null() { return false; }
    let sid = unsafe { CStr::from_ptr(id).to_string_lossy().into_owned() };
    proxy::register(sid, [x, y, z], [ex, ey, ez], density);
    true
}

#[no_mangle]
pub extern "C" fn sf_remove_proxy(id: *const c_char) -> bool {
    if id.is_null() { return false; }
    let sid = unsafe { CStr::from_ptr(id).to_string_lossy().into_owned() };
    if let Ok(mut s) = proxy::state().lock() {
        s.proxies.remove(&sid);
    }
    true
}

#[no_mangle]
pub extern "C" fn sf_limpiar_nucleo() {
    proxy::clear();
    fragment_sim::clear();
    sf_log("[RUST] Núcleo limpiado completamente.");
}

#[no_mangle]
pub extern "C" fn sf_set_global_power(_power: f32) {
    // Reservado para escala global futura
}

#[no_mangle]
pub extern "C" fn sf_free_string(_ptr: *const c_char) {
    // No-op: Los strings de Rust se limpian automáticamente
}

#[no_mangle]
pub extern "C" fn sf_apply_settings(_threads: i32, _gpu: bool) {
    // Reservado
}

#[no_mangle]
pub extern "C" fn sf_purge_sleeping_fragments() -> i32 { 0 }

// ══════════════════════════════════════════════════════════════
//  LOGGER
// ══════════════════════════════════════════════════════════════

#[no_mangle]
pub extern "C" fn sf_registrar_logger(callback: extern "C" fn(*const c_char)) {
    unsafe { UNREAL_LOG_CALLBACK = Some(callback); }
    sf_log(">>> [RUST] Logger conectado a Unreal (V8.7).");
}

pub fn sf_log(mensaje: &str) {
    unsafe {
        if let Some(cb) = UNREAL_LOG_CALLBACK {
            if let Ok(msg) = CString::new(mensaje) {
                cb(msg.as_ptr());
            }
        }
    }
}

// ══════════════════════════════════════════════════════════════
//  DETONACIÓN
// ══════════════════════════════════════════════════════════════

#[no_mangle]
pub extern "C" fn sf_detonar_nativo(
    id: *const c_char, energy: f32, preset_f: f32, 
    frag_level: f32, radius: f32, count: i32
) {
    if id.is_null() { return; }
    let sid = unsafe { CStr::from_ptr(id).to_string_lossy().into_owned() };
    
    let preset = match preset_f as i32 {
        1 => DamagePreset::Implosion,
        2 => DamagePreset::Impact,
        3 => DamagePreset::ShapedCharge,
        4 => DamagePreset::Cinematic,
        5 => DamagePreset::Collapse,
        6 => DamagePreset::RealityShatter,
        _ => DamagePreset::Explosion,
    };

    let _ = proxy::destroy(sid, preset, energy, [0.0,0.0,0.0], radius, frag_level, count as u32);
}

#[no_mangle]
pub extern "C" fn sf_detonar_militar(
    id: *const c_char, _x: f32, _y: f32, _z: f32, 
    energy: f32, preset_f: f32, count: i32
) {
    sf_detonar_nativo(id, energy, preset_f, 1.0, 250.0, count);
}

// ══════════════════════════════════════════════════════════════
//  TELEMETRÍA (SIN DEADLOCK)
// ══════════════════════════════════════════════════════════════

#[no_mangle]
pub extern "C" fn sf_get_engine_state(ptr: *mut f32) {
    if ptr.is_null() { return; }
    let s = fragment_sim::state().lock();
    // Leemos todo dentro del MISMO lock, sin llamar a total_count() que haría otro lock
    let active = s.nodes.count.load(std::sync::atomic::Ordering::Relaxed) as f32;
    let fps = s.avg_fps;
    let capacity_pct = (active / 50000.0).min(1.0);
    let sim_time = s.sim_time as f32;
    drop(s);
    
    unsafe {
        let out = std::slice::from_raw_parts_mut(ptr, 4);
        out[0] = active; out[1] = fps; out[2] = capacity_pct; out[3] = sim_time;
    }
}

#[no_mangle]
pub extern "C" fn sf_get_all_fragment_data(hash: u32, out_count: *mut c_int) -> *const crate::CFragment {
    let (ptr, count) = fragment_sim::get_all_c_fragments(hash);
    unsafe { if !out_count.is_null() { *out_count = count as c_int; } }
    ptr
}

// ══════════════════════════════════════════════════════════════
//  FUNCIONES AUXILIARES
// ══════════════════════════════════════════════════════════════

#[no_mangle] pub extern "C" fn sf_purge_sleeping() -> i32 { 0 }
#[no_mangle] pub extern "C" fn sf_clear_core() { proxy::clear(); }
#[no_mangle] pub extern "C" fn sf_set_ground_z(z: f32) { fragment_sim::set_ground_z(z); }

#[no_mangle] 
pub extern "C" fn sf_set_proxy_militar(id: *const c_char, _m: i32, _l: i32) {
    if !id.is_null() { let _ = unsafe { CStr::from_ptr(id) }; }
}
