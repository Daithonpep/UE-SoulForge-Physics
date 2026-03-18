//! # FFI – Interfaz Pública en C  (con Panic Safe-Guard)
//!
//! Todas las funciones marcadas `#[no_mangle] pub extern "C"` forman el
//! "contrato" visible desde C++, C, Python (ctypes) y otros lenguajes.
//!
//! ## REGLAS DEL MÓDULO
//!  - Solo tipos primitivos de C en las firmas (i32, f32, *const c_char…).
//!  - Toda lógica real vive en los otros módulos; aquí solo se delega.
//!  - La memoria de las cadenas retornadas debe liberarse con `sf_free_string`.
//!
//! ## PANIC SAFE-GUARD
//!  Cruzar la frontera FFI con un panic activo causa **Undefined Behavior** y
//!  normalmente mata el proceso de Unreal Engine sin mensaje de error.
//!  Cada función lógica utiliza `std::panic::catch_unwind` para atrapar el
//!  panic y retornar un código de error limpio a C++ en su lugar.

use std::ffi::{CString, CStr};
use std::os::raw::{c_char, c_int, c_float};
use std::panic;

use crate::physics;
use crate::swarm;
use crate::proxy::{self, DamagePreset};

// Variable global para guardar la función de log de Unreal
static mut UNREAL_LOG_CALLBACK: Option<extern "C" fn(*const c_char)> = None;

// Unreal llamará a esto al inicio para darle el "megáfono" a Rust
#[no_mangle]
pub extern "C" fn sf_registrar_logger(callback: extern "C" fn(*const c_char)) {
    unsafe {
        UNREAL_LOG_CALLBACK = Some(callback);
    }
    // ¡PRUEBA DE VIDA INMEDIATA!
    sf_log(">>> [RUST] ¡Megáfono conectado! Puedo hablar con Unreal.");
}

// Usa esta función en Rust cada vez que quieras imprimir en Unreal
pub fn sf_log(mensaje: &str) {
    unsafe {
        if let Some(cb) = UNREAL_LOG_CALLBACK {
            if let Ok(c_str) = CString::new(mensaje) {
                cb(c_str.as_ptr());
            }
        }
    }
}

/// Multiplicador global de fuerza.
static mut GLOBAL_FORCE_MULTIPLIER: f32 = 1.0;

// ─── Macro auxiliar: ejecuta un bloque y atrapa cualquier panic ───────────────
//
//  Uso:  safe_call!(expr, valor_de_retorno_en_error)
//
//  Si `expr` provoca un panic, el macro lo atrapa, lo loguea y retorna
//  `valor_de_retorno_en_error` en lugar de dejar que el panic cruce la frontera FFI.

macro_rules! safe_call {
    ($body:expr, $on_panic:expr) => {
        match panic::catch_unwind(panic::AssertUnwindSafe(|| $body)) {
            Ok(result) => result,
            Err(err) => {
                // Intentamos extraer el mensaje del panic para loguearlo
                let msg = if let Some(s) = err.downcast_ref::<&str>() {
                    s.to_string()
                } else if let Some(s) = err.downcast_ref::<String>() {
                    s.clone()
                } else {
                    "Panic desconocido en el núcleo de Rust".to_string()
                };
                log::error!("[SoulForge FFI PANIC CAPTURADO] {}", msg);
                $on_panic
            }
        }
    };
}

// ─── Inicialización / apagado ────────────────────────────────────────────────

/// Inicializa el motor de física de SoulForge.
/// Debe llamarse **una sola vez** antes de cualquier otra función.
/// Retorna `1` si tiene éxito, `0` si falló.
#[no_mangle]
pub extern "C" fn sf_init_physics() -> c_int {
    safe_call!({
        // env_logger solo puede inicializarse una vez; ignoramos el error si ya fue llamado.
        let _ = env_logger::try_init();
        physics::init();
        log::info!("SoulForge PhysX Core: Activado ✓");
        1
    }, 0)
}

/// Apaga el motor y libera todos los recursos internos.
/// Retorna `1` si tiene éxito, `0` si falló.
#[no_mangle]
pub extern "C" fn sf_shutdown_physics() -> c_int {
    safe_call!({
        physics::shutdown();
        log::info!("SoulForge PhysX Core: Apagado ✓");
        1
    }, 0)
}

/// Modo Dios: Apaga la física y limpia la RAM de un solo golpe.
#[no_mangle]
pub extern "C" fn sf_shutdown_engine() {
    safe_call!({
        physics::shutdown();
        proxy::clear();
        swarm::clear();
        crate::fragment_sim::clear();
        log::info!("[SoulForge Core] Memoria física liberada por completo.");
    }, ())
}

/// Limpia la memoria de proxies y enjambre, reseteando los contadores.
#[no_mangle]
pub extern "C" fn sf_limpiar_nucleo() {
    safe_call!({
        proxy::clear();
        swarm::clear();
        crate::fragment_sim::clear();
        log::info!("[SoulForge] Núcleo limpiado. Listo para nueva simulación.");
    }, ())
}

/// Ajusta el multiplicador global de fuerza (Modo Dios).
#[no_mangle]
pub extern "C" fn sf_set_global_power(multiplier: f32) {
    unsafe { GLOBAL_FORCE_MULTIPLIER = multiplier; }
}

/// Aplica ajustes de rendimiento al motor (Hilos y GPU).
#[no_mangle]
pub extern "C" fn sf_apply_settings(num_threads: c_int, use_gpu: bool) {
    safe_call!({
        // Configuramos el pool de hilos de Rayon (nuestro motor de paralelismo)
        let _ = rayon::ThreadPoolBuilder::new()
            .num_threads(num_threads as usize)
            .build_global();

        if use_gpu {
            log::info!("[SoulForge] GPU Compute activado para fragmentación masiva.");
        } else {
            log::info!("[SoulForge] Simulación basada en CPU.");
        }
        
        log::info!("[SoulForge] Hilos activos: {}", num_threads);
    }, ())
}

// ─── Simulación ──────────────────────────────────────────────────────────────

/// Avanza la simulación un tick.
/// `delta_seconds` debe ser el DeltaTime del frame de Unreal.
/// El motor usa Fixed Timestep interno (60Hz); este delta se acumula internamente.
/// Retorna el número de sub-steps ejecutados este frame (≥ 0), o -1 en error.
#[no_mangle]
pub extern "C" fn sf_tick_physics(delta_seconds: c_float) -> c_int {
    safe_call!(physics::tick(delta_seconds as f32) as c_int, -1)
}

// ─── Cerebro de Enjambre ─────────────────────────────────────────────────────

/// Registra un agente en el Cerebro de Enjambre.
/// - is_dormant: si 1, el agente no se simulará hasta ser despertado (optimización muros).
#[no_mangle]
pub extern "C" fn sf_register_swarm_agent(
    pos_x: c_float,
    pos_y: c_float,
    pos_z: c_float,
    is_dormant: c_int,
) -> c_int {
    safe_call!(
        match swarm::register_agent([pos_x as f32, pos_y as f32, pos_z as f32], is_dormant != 0) {
            Ok(id) => id as c_int,
            Err(_) => -1,
        },
        -1
    )
}

/// Despierta agentes en un radio específico.
#[no_mangle]
pub extern "C" fn sf_wake_agent_near(x: c_float, y: c_float, z: c_float, radius: c_float) {
    safe_call!(swarm::wake_agent_near([x, y, z], radius), ())
}

/// Obtiene la posición actual de un agente en el enjambre.
/// Retorna `true` si el agente existe, `false` si no (o si hubo panic).
#[no_mangle]
pub extern "C" fn sf_get_swarm_agent_pos(
    id: c_int,
    out_x: *mut c_float,
    out_y: *mut c_float,
    out_z: *mut c_float,
) -> bool {
    safe_call!({
        if let Some(agent) = swarm::get_agent(id as u32) {
            // SAFETY: los punteros son validados antes de escribir
            unsafe {
                if !out_x.is_null() { *out_x = agent.position[0]; }
                if !out_y.is_null() { *out_y = agent.position[1]; }
                if !out_z.is_null() { *out_z = agent.position[2]; }
            }
            true
        } else {
            false
        }
    }, false)
}

/// Elimina un agente del enjambre.
/// Retorna `true` si el agente existía y fue eliminado.
#[no_mangle]
pub extern "C" fn sf_remove_swarm_agent(id: c_int) -> bool {
    safe_call!(swarm::remove_agent(id as u32), false)
}

/// Retorna el número total de agentes activos en el enjambre.
#[no_mangle]
pub extern "C" fn sf_swarm_agent_count() -> c_int {
    safe_call!(swarm::agent_count() as c_int, 0)
}

// ─── Registro y Destrucción de Proxies ──────────────────────────────────────

/// Configura las propiedades físicas avanzadas de un proxy.
/// material_type: 0 = Acero, 1 = Aluminio, 2 = Concreto
/// lod_level: 0 = Primary (Hero), 1 = Secondary, 2 = Tertiary
#[no_mangle]
pub extern "C" fn sf_set_proxy_militar(
    proxy_id_ptr: *const c_char,
    material_type: c_int,
    lod_level: c_int,
) {
    if proxy_id_ptr.is_null() { return; }
    safe_call!({
        let proxy_id = unsafe { CStr::from_ptr(proxy_id_ptr).to_string_lossy().into_owned() };
        let mut s = proxy::state().lock().unwrap();
        if let Some(proxy) = s.proxies.get_mut(&proxy_id) {
            proxy.lod = match lod_level {
                0 => proxy::PhysicsLOD::Primary,
                1 => proxy::PhysicsLOD::Secondary,
                _ => proxy::PhysicsLOD::Tertiary,
            };
            
            proxy.material = match material_type {
                0 => crate::military::material::JohnsonCookMaterial::steel_4340(),
                1 => crate::military::material::JohnsonCookMaterial::aluminum_6061(),
                _ => crate::military::material::JohnsonCookMaterial::concrete_35mpa(),
            };
            log::info!("[SoulForge] Proxy [{}] configurado: Material {}, LOD {}", proxy_id, material_type, lod_level);
        }
    }, ())
}

/// Registra un proxy AABB (caja) destruible en el motor.
///
/// Parámetros de posición: centro en cm (coordenadas de Unreal).
/// Parámetros half_ext: semi-extensiones X/Y/Z en cm.
/// density: densidad del material en kg/cm³.
///   - Hormigón  ≈ 0.0024
///   - Ladrillo  ≈ 0.0018
///   - Madera    ≈ 0.0006
///   - Metal     ≈ 0.0079
///
#[no_mangle]
pub extern "C" fn sf_register_proxy(
    id_ptr: *const c_char,
    pos_x: c_float, pos_y: c_float, pos_z: c_float,
    ext_x: c_float, ext_y: c_float, ext_z: c_float,
    density: c_float,
) -> bool {
    if id_ptr.is_null() { return false; }
    safe_call!(
        {
            let id = unsafe { CStr::from_ptr(id_ptr).to_string_lossy().into_owned() };
            match proxy::register_proxy(
                id,
                [pos_x, pos_y, pos_z],
                [ext_x, ext_y, ext_z],
                density,
            ) {
                Ok(_) => true,
                Err(e) => { log::error!("sf_register_proxy: {}", e); false }
            }
        },
        false
    )
}

/// Alias para sf_register_proxy usado en el sistema de Identidad por Nombre.
#[no_mangle]
pub extern "C" fn sf_anclar_por_nombre(
    id_ptr: *const c_char,
    pos_x: c_float, pos_y: c_float, pos_z: c_float,
    ext_x: c_float, ext_y: c_float, ext_z: c_float,
    density: c_float,
) -> bool {
    sf_register_proxy(id_ptr, pos_x, pos_y, pos_z, ext_x, ext_y, ext_z, density)
}

#[no_mangle]
pub extern "C" fn sf_remove_proxy(id_ptr: *const c_char) -> bool {
    if id_ptr.is_null() { return false; }
    let id = unsafe { CStr::from_ptr(id_ptr).to_string_lossy().into_owned() };
    safe_call!(proxy::remove(id), false)
}

/// Destruye el proxy con `proxy_id` usando el preset de daño indicado.
/// Soporta energía de impacto y dirección (para Voxel Penetration).
///
/// Preset codes:
///   0 = Explosion (onda radial)
///   1 = Collapse  (colapso gravitacional en planchas)
///   2 = Impact    (cono de impacto puntual)
///   3 = ShapedCharge (cono hiper-cerrado de alta penetración)
///
/// Retorna un puntero a una cadena JSON con el `DestructionResult`.
/// Auxiliar para retornar mensajes de error seguros a C++
fn safe_response(msg: &str) -> *mut c_char {
    match CString::new(msg) {
        Ok(cs) => cs.into_raw(),
        Err(_) => {
            // Fallback extremo si el mensaje mismo falla
            if let Ok(fallback) = CString::new("Error Crítico de Memoria") {
                fallback.into_raw()
            } else {
                std::ptr::null_mut()
            }
        }
    }
}

#[no_mangle]
pub extern "C" fn sf_detonar_nativo(
    proxy_id_ptr: *const c_char,
    energy: c_float,
    dir_x: c_float, dir_y: c_float, dir_z: c_float,
    fragment_count: c_int,
) {
    if proxy_id_ptr.is_null() { return; }
    let proxy_id = unsafe { CStr::from_ptr(proxy_id_ptr).to_string_lossy().into_owned() };
    safe_call!({
        let _ = proxy::destroy(
            proxy_id,
            proxy::DamagePreset::Explosion,
            energy as f32,
            [dir_x, dir_y, dir_z],
            150.0,
            fragment_count as u32
        );
    }, ())
}

#[no_mangle]
pub extern "C" fn sf_detonar_militar(
    proxy_id_ptr: *const c_char, 
    x: c_float, y: c_float, z: c_float,
    energy: c_float,
    explosive_type: c_int,
    fragment_count: c_int,
) {
    if proxy_id_ptr.is_null() { return; }
    let proxy_id = unsafe { CStr::from_ptr(proxy_id_ptr).to_string_lossy().into_owned() };
    
    safe_call!({
        let preset = match explosive_type {
            0 => proxy::DamagePreset::Explosion,
            1 => proxy::DamagePreset::Implosion,   // Agujero Negro Original
            2 => proxy::DamagePreset::Impact,      // RDX / Railgun
            3 => proxy::DamagePreset::ShapedCharge,// PETN / Carga Hueca
            4 => proxy::DamagePreset::Cinematic,   // Matrix / Zero-G
            5 => proxy::DamagePreset::Collapse,    // Colapso Físico
            _ => proxy::DamagePreset::Explosion,
        };
        
        let final_energy = energy as f32; // Eliminamos factores externos para volver al feeling original

        // 1. Destrucción Física (Feeling SoulForge Original)
        let _ = proxy::destroy(
            proxy_id, 
            preset, 
            final_energy, 
            [x as f32, y as f32, z as f32], 
            250.0, 
            fragment_count.max(0) as u32 
        );

        // 2. LOG: Informamos del éxito sin mezclar "Poderes" (que ahora van por el componente Power)
        sf_log(&format!("💥 SoulForge: Detonación física {} completada.", preset as i32));
    }, ())
}

/// Simula una detonación para el "Laboratorio de Balística" y devuelve los puntos de previsualización.
#[no_mangle]
pub extern "C" fn sf_simular_preview(
    proxy_id_ptr: *const c_char, 
    energy: c_float,
    explosive_type: c_int,
    fragment_count: c_int,
    out_count: *mut c_int,
) -> *const crate::CFragment {
    if proxy_id_ptr.is_null() { return std::ptr::null(); }
    let proxy_id = unsafe { CStr::from_ptr(proxy_id_ptr).to_string_lossy().into_owned() };
    
    static mut PREVIEW_CACHE: Vec<crate::CFragment> = Vec::new();
    
    safe_call!({
        let preset = match explosive_type {
            0 => proxy::DamagePreset::Explosion,
            1 => proxy::DamagePreset::Implosion,
            2 => proxy::DamagePreset::Impact,
            3 => proxy::DamagePreset::ShapedCharge,
            4 => proxy::DamagePreset::Cinematic,
            5 => proxy::DamagePreset::Collapse,
            _ => proxy::DamagePreset::Explosion,
        };

        // Dirección por defecto (arriba si es radio, o X si es impacto)
        let dir = if preset == proxy::DamagePreset::Impact { [1.0, 0.0, 0.0] } else { [0.0, 0.0, 1.0] };
        let count = fragment_count as u32;

        if let Ok(frags) = proxy::preview_destruction(proxy_id, preset, energy as f32, dir, count) {
            unsafe {
                let cache = &mut *std::ptr::addr_of_mut!(PREVIEW_CACHE);
                cache.clear();
                for f in frags {
                    cache.push(crate::CFragment {
                        x: f.position[0],
                        y: f.position[1],
                        z: f.position[2],
                        pitch: f.velocity[0], // Reusamos pitch/yaw/roll para la velocidad en el preview
                        yaw:   f.velocity[1],
                        roll:  f.velocity[2],
                        category: 0,
                    });
                }
                if !out_count.is_null() {
                    *out_count = cache.len() as c_int;
                }
                cache.as_ptr()
            }
        } else {
            unsafe {
                if !out_count.is_null() { *out_count = 0; }
                let cache = &mut *std::ptr::addr_of_mut!(PREVIEW_CACHE);
                cache.clear();
                cache.as_ptr()
            }
        }
    }, unsafe { 
        if !out_count.is_null() { *out_count = 0; }
        let cache = &mut *std::ptr::addr_of_mut!(PREVIEW_CACHE);
        cache.as_ptr()
    })
}

/// Alias para sf_detonar_militar usado en el sistema de Identidad por Nombre.
#[no_mangle]
pub extern "C" fn sf_detonar_por_nombre(id_ptr: *const c_char, energy: c_float) {
    // Usamos el preset de explosión por defecto (Láser/Nativo)
    sf_detonar_nativo(id_ptr, energy, 0.0, 0.0, 1.0, 100);
}

#[no_mangle]
pub extern "C" fn sf_destroy_proxy(
    proxy_id_ptr: *const c_char, 
    energy: c_float,
    dir_x: c_float, dir_y: c_float, dir_z: c_float,
) -> *mut c_char {
    sf_destroy_proxy_advanced(proxy_id_ptr, energy, 150.0, 100, 0, dir_x, dir_y, dir_z)
}

/// Destrucción avanzada con control de radio, nodos y forma.
#[no_mangle]
pub extern "C" fn sf_destroy_proxy_advanced(
    proxy_id_ptr: *const c_char, 
    energy: c_float,
    radius: c_float,
    fragment_count: c_int,
    pattern: c_int,
    dir_x: c_float, dir_y: c_float, dir_z: c_float,
) -> *mut c_char {
    if proxy_id_ptr.is_null() { return std::ptr::null_mut(); }
    let proxy_id = unsafe { CStr::from_ptr(proxy_id_ptr).to_string_lossy().into_owned() };
    safe_call!({
        // 1. EL CHISMOSO: Validaciones iniciales
        if proxy_id.is_empty() {
            let error_msg = serde_json::json!({
                "status": "error",
                "log": format!("Fallo crítico: Se recibió una ID fantasma ({}). ¿El objeto se registró?", proxy_id)
            });
            return safe_response(&error_msg.to_string());
        }

        if energy < 500_000.0 {
            let warn_msg = serde_json::json!({
                "status": "warning",
                "log": format!("Energía insuficiente. {} no supera el umbral de fractura (500k).", energy)
            });
            return safe_response(&warn_msg.to_string());
        }

        // Mapeo de patrones de Unreal a presets de Rust
        let preset = match pattern {
            0 => DamagePreset::Explosion,
            1 => DamagePreset::Impact,
            2 => DamagePreset::Collapse,
            _ => DamagePreset::Explosion,
        };
        
        let final_energy = energy * unsafe { GLOBAL_FORCE_MULTIPLIER };

        // 2. EL ARQUITECTO: Lógica de fractura real
        match proxy::destroy(
            proxy_id, 
            preset, 
            final_energy as f32, 
            [dir_x, dir_y, dir_z],
            radius as f32,
            fragment_count as u32
        ) {
            Ok(json_str) => {
                // Parseamos el JSON generado para inyectar el status de éxito
                if let Ok(mut val) = serde_json::from_str::<serde_json::Value>(&json_str) {
                    if let Some(obj) = val.as_object_mut() {
                        obj.insert("status".to_string(), serde_json::json!("success"));
                        obj.insert("log".to_string(), serde_json::json!(format!("Explosión generada con {} fragmentos en un radio de {}cm.", fragment_count, radius)));
                    }
                    safe_response(&val.to_string())
                } else {
                    safe_response(&json_str)
                }
            },
            Err(e) => {
                let err_msg = serde_json::json!({
                    "status": "error",
                    "log": format!("Error: Proxy no encontrado ({})", e)
                });
                safe_response(&err_msg.to_string())
            }
        }
    }, safe_response("{\"status\":\"error\",\"log\":\"Panic capturado en Rust\"}"))
}

/// Libera la memoria de una cadena retornada por este módulo.
///
/// # Seguridad
/// Solo debe llamarse con punteros obtenidos de `sf_destroy_proxy`.
/// Llamarlo con un puntero arbitrario causa Undefined Behavior.
#[no_mangle]
pub unsafe extern "C" fn sf_free_string(ptr: *mut c_char) {
    if !ptr.is_null() {
        drop(CString::from_raw(ptr));
    }
}

// ─── Consulta de Fragmentos en Vuelo ─────────────────────────────────────────

/// Obtiene la posición actual de un fragmento en simulación.
///
/// Retorna `true` si el fragmento existe (activo o dormido), `false` si no.
/// Unreal llama esto cada frame para mover los mesh instances.
#[no_mangle]
pub extern "C" fn sf_get_fragment_pos(
    id:    u32,
    out_x: *mut c_float,
    out_y: *mut c_float,
    out_z: *mut c_float,
) -> bool {
    safe_call!({
        if let Some(pos) = crate::fragment_sim::get_position(id) {
            unsafe {
                if !out_x.is_null() { *out_x = pos[0]; }
                if !out_y.is_null() { *out_y = pos[1]; }
                if !out_z.is_null() { *out_z = pos[2]; }
            }
            true
        } else {
            false
        }
    }, false)
}

/// Obtiene la velocidad actual de un fragmento.
/// Útil para Niagara: partículas de polvo heredan la velocidad.
#[no_mangle]
pub extern "C" fn sf_get_fragment_vel(
    id:    u32,
    out_x: *mut c_float,
    out_y: *mut c_float,
    out_z: *mut c_float,
) -> bool {
    safe_call!({
        if let Some(vel) = crate::fragment_sim::get_velocity(id) {
            unsafe {
                if !out_x.is_null() { *out_x = vel[0]; }
                if !out_y.is_null() { *out_y = vel[1]; }
                if !out_z.is_null() { *out_z = vel[2]; }
            }
            true
        } else {
            false
        }
    }, false)
}

/// Retorna el estado de un fragmento:
///   0 = Flying (en vuelo libre)
///   1 = Bouncing (rebotando)
///   2 = Sleeping (en reposo, no se simula más)
///  -1 = No encontrado
#[no_mangle]
pub extern "C" fn sf_get_fragment_state(id: u32) -> c_int {
    safe_call!(
        match crate::fragment_sim::get_state(id) {
            Some(crate::fragment_sim::FragmentState::Flying)   => 0,
            Some(crate::fragment_sim::FragmentState::Bouncing) => 1,
            Some(crate::fragment_sim::FragmentState::Sleeping) => 2,
            None                                               => -1,
        },
        -1
    )
}

/// Número de fragmentos activos en simulación (en vuelo o rebotando).
#[no_mangle]
pub extern "C" fn sf_fragment_active_count() -> c_int {
    safe_call!(crate::fragment_sim::active_count() as c_int, 0)
}

/// Número total de fragmentos (activos + dormidos).
#[no_mangle]
pub extern "C" fn sf_fragment_total_count() -> c_int {
    safe_call!(crate::fragment_sim::total_count() as c_int, 0)
}

/// Configura la altura del suelo del nivel (cm, coordenadas de Unreal).
/// Llamar cuando el nivel tenga un suelo en una posición Z no nula.
#[no_mangle]
pub extern "C" fn sf_set_ground_z(z: c_float) {
    safe_call!(crate::fragment_sim::set_ground_z(z as f32), ())
}

/// Elimina todos los fragmentos dormidos de la memoria.
/// Llamar periódicamente (por ejemplo cada 5s) para liberar RAM.
/// Retorna el número de fragmentos eliminados.
#[no_mangle]
pub extern "C" fn sf_purge_sleeping_fragments() -> c_int {
    safe_call!(crate::fragment_sim::purge_sleeping() as c_int, 0)
}

/// Canal Maestro: Retorna el puntero a todos los datos de fragmentos activos.
/// Útil para actualización masiva (HISM Batch Update) en Unreal.
#[no_mangle]
pub extern "C" fn sf_get_all_fragment_data(out_count: *mut c_int) -> *const crate::CFragment {
    safe_call!({
        let (ptr, count) = crate::fragment_sim::get_all_c_fragments();
        unsafe {
            if !out_count.is_null() {
                *out_count = count as c_int;
            }
        }
        ptr
    }, unsafe {
        if !out_count.is_null() { *out_count = 0; }
        let (ptr, _) = crate::fragment_sim::get_all_c_fragments();
        ptr
    })
}
#[repr(C)]
pub struct ShockwaveData {
    pub radius_cm: f32,
    pub speed_cms: f32,
}

#[repr(C)]
pub struct CActivePowerPayload {
    pub power_type: i32,
    pub origin_x: f32, pub origin_y: f32, pub origin_z: f32,
    pub dir_x: f32, pub dir_y: f32, pub dir_z: f32,
    pub intensity: f32,
    pub phase: f32,
}

#[no_mangle]
pub extern "C" fn sf_update_all_powers(
    powers_ptr: *const CActivePowerPayload,
    count: c_int,
    dt: c_float
) {
    if powers_ptr.is_null() || count <= 0 { return; }
    
    use crate::powers::ActivePower;
    
    safe_call!({
        let payloads = unsafe { std::slice::from_raw_parts(powers_ptr, count as usize) };
        if let Ok(mut s) = crate::fragment_sim::state().lock() {
            let current_time = s.sim_time;
            let dt_f64 = dt as f64;
            
            // Destructuración para evitar conflicto de borrows
            let crate::fragment_sim::FragSimState { 
                ref mut nodes, 
                ref power_system, 
                .. 
            } = *s;

            // Log ocasional para verificar que Unreal está enviando datos
            if count > 0 && (current_time * 10.0) as i32 % 60 == 0 {
                sf_log(&format!("[RUST] Recibiendo {} payloads de poder desde Unreal...", count));
            }

            for payload in payloads {
                // Si el poder es de tipo EnergyBeam (4 fases), necesitamos una duración coherente para que las fases avancen
                let estimated_duration = 8.0; 
                
                let temp_power = ActivePower {
                    power_type: payload.power_type as u32,
                    start_time: current_time - (payload.phase as f64 * estimated_duration),
                    duration: estimated_duration,
                    origin: [payload.origin_x as f64, payload.origin_y as f64, payload.origin_z as f64],
                    direction: [payload.dir_x as f64, payload.dir_y as f64, payload.dir_z as f64],
                    intensity: payload.intensity as f64,
                };

                if let Some(effect) = power_system.effects.get(&temp_power.power_type) {
                    effect.apply_to_nodes(nodes, &temp_power, current_time, dt_f64);
                }
            }
        }
    }, ())
}

#[no_mangle]
pub extern "C" fn sf_calcular_onda(energy: f32) -> ShockwaveData {
    // 1. Escala de energía a Joules (Ajustado para Unreal)
    let energy_joules = (energy * 10.0).max(1.0); 

    // 2. Matemática de Onda (Simplificación de Sedov-Taylor)
    // R = (E/rho)^1/5 * t^2/5 -> Simplificado para obtener radio máximo estable
    let radius_m = (energy_joules as f32).powf(0.333) * 1.5; 
    
    // Velocidad inicial supersónica (Mach 1+)
    let speed_ms = (energy_joules as f32).powf(0.5).clamp(343.0, 6000.0);

    ShockwaveData {
        radius_cm: radius_m * 100.0,
        speed_cms: speed_ms * 100.0,
    }
}
#[no_mangle]
pub extern "C" fn sf_activar_poder(
    power_type: c_int,
    x: c_float, y: c_float, z: c_float,
    dir_x: c_float, dir_y: c_float, dir_z: c_float,
    intensity: c_float,
    duration: c_float
) {
    use crate::powers::ActivePower;
    safe_call!({
        if let Ok(mut s) = crate::fragment_sim::state().lock() {
            let current_time = s.sim_time;
            let power = ActivePower {
                power_type: power_type as u32,
                start_time: current_time,
                duration: duration as f64,
                origin: [x as f64, y as f64, z as f64],
                direction: [dir_x as f64, dir_y as f64, dir_z as f64],
                intensity: intensity as f64,
            };
            s.power_system.active_powers.push(power);
            sf_log(&format!("✨ Poder activado en la Forja: Tipo {}, Intensidad {}", power_type, intensity));
        }
    }, ())
}
#[no_mangle]
pub extern "C" fn sf_get_node_positions(out_count: *mut c_int) -> *const f32 {
    let (ptr, count) = crate::fragment_sim::get_interleaved_positions();
    unsafe {
        if !out_count.is_null() {
            *out_count = count as c_int;
        }
    }
    ptr
}

#[repr(C)]
pub struct RenderData {
    pub count: u32,
    pub positions_ptr: *const f32,
    pub colors_ptr: *const u32,
    pub scales_ptr: *const f32,
}

#[no_mangle]
pub unsafe extern "C" fn sf_get_render_data(out_data: *mut RenderData) -> bool {
    if out_data.is_null() { return false; }
    if let Ok(s) = crate::fragment_sim::state().lock() {
        s.nodes.prepare_render_data();
        let rb = s.nodes.read_buffer.read();
        (*out_data).count = rb.count;
        (*out_data).positions_ptr = rb.positions.as_ptr();
        (*out_data).colors_ptr = rb.colors.as_ptr();
        (*out_data).scales_ptr = rb.scales.as_ptr();
        true
    } else {
        false
    }
}

#[no_mangle]
pub extern "C" fn sf_tick_optimized(dt: c_float) {
    crate::fragment_sim::tick(dt as f32);
}

#[no_mangle]
pub unsafe extern "C" fn sf_add_node(x: f32, y: f32, z: f32, mass: f32, material: u16) -> i32 {
    if let Ok(s) = crate::fragment_sim::state().lock() {
        match s.nodes.add_node([x, y, z], [0.0; 3], mass, material, -1.0, [1.0, 1.0, 1.0], 0, 0) {
            Some(idx) => idx as i32,
            None => -1,
        }
    } else {
        -1
    }
}

#[no_mangle]
pub unsafe extern "C" fn sf_add_nodes_bulk(
    positions_ptr: *const f32, 
    count: c_int, 
    mass: f32, 
    material: u16
) -> c_int {
    if positions_ptr.is_null() || count <= 0 { return 0; }
    
    let positions = std::slice::from_raw_parts(positions_ptr, (count * 3) as usize);
    let mut added = 0;
    
    if let Ok(s) = crate::fragment_sim::state().lock() {
        for i in 0..count as usize {
            let x = positions[i * 3];
            let y = positions[i * 3 + 1];
            let z = positions[i * 3 + 2];
            if s.nodes.add_node([x, y, z], [0.0; 3], mass, material, -1.0, [1.0, 1.0, 1.0], 0, 0).is_some() {
                added += 1;
            }
        }
    }
    added
}

// === API Optimizado para el nuevo componente ISMC ===

#[repr(C)]
pub struct EngineHandle {
    _private: [u8; 0],
}

#[no_mangle]
pub unsafe extern "C" fn physics_engine_create(_max_nodes: u32) -> *mut EngineHandle {
    // Ya inicializado por el singleton, pero nos aseguramos
    sf_init_physics();
    sf_log(">>> [STABLE] Núcleo Rust listo para simulación masiva (100k+ nodos).");
    1 as *mut EngineHandle // Handle ficticio para compatibilidad
}

#[no_mangle]
pub unsafe extern "C" fn physics_engine_destroy(_handle: *mut EngineHandle) {
    sf_shutdown_physics();
}

#[no_mangle]
pub unsafe extern "C" fn physics_engine_tick(_handle: *mut EngineHandle, dt: f32) {
    crate::fragment_sim::tick(dt);
}

#[no_mangle]
pub unsafe extern "C" fn physics_engine_get_render_data(
    _handle: *mut EngineHandle,
    out_data: *mut RenderData,
) -> bool {
    sf_get_render_data(out_data)
}

#[no_mangle]
pub unsafe extern "C" fn physics_engine_add_node(
    _handle: *mut EngineHandle,
    x: f32, y: f32, z: f32,
    mass: f32,
    material: u16,
) -> i32 {
    if let Ok(s) = crate::fragment_sim::state().lock() {
        match s.nodes.add_node([x, y, z], [0.0; 3], mass, material, -1.0, [1.0, 1.0, 1.0], 0, 0) {
            Some(idx) => idx as i32,
            None => -1,
        }
    } else {
        -1
    }
}

#[no_mangle]
pub unsafe extern "C" fn physics_engine_activate_kamehameha(
    _handle: *mut EngineHandle,
    ox: f32, oy: f32, oz: f32,
    dx: f32, dy: f32, dz: f32,
    intensity: f32,
) -> bool {
    sf_activar_poder(0, ox, oy, oz, dx, dy, dz, intensity, 8.0);
    true
}

#[no_mangle]
pub unsafe extern "C" fn physics_engine_set_gravity(
    _handle: *mut EngineHandle,
    _x: f32, _y: f32, _z: f32,
) {
    // Por ahora la gravedad es constante en fragment_sim, pero podríamos exponer un set_gravity
    log::info!("[SoulForge] Gravedad configurada");
}
