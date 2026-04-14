//! # Physics – Motor de Física Principal (Fixed Timestep)
//!
//! ## Responsabilidades
//!  - Mantener el estado global de la simulación.
//!  - Ejecutar la simulación en **pasos de tiempo fijos** (60 Hz).
//!  - Coordinar el Cerebro de Enjambre y la destrucción de proxies.
//!
//! ## Fixed Timestep
//!  La idea central del timestep fijo es **desacoplar la frecuencia de
//!  renderizado de la frecuencia de simulación**:
//!
//!  ```text
//!  acumulador += dt_real_del_frame
//!  while acumulador >= FIXED_DT:
//!      simular(FIXED_DT)       ← paso constante, independiente de FPS
//!      acumulador -= FIXED_DT
//!  ```
//!
//!  Resultado: a 20 FPS o a 144 FPS, los escombros se comportan igual.
//!  FIXED_DT = 1/60 ≈ 0.01667 s

use std::sync::{Mutex, OnceLock};

// ─── Constantes ───────────────────────────────────────────────────────────────

/// Tamaño del sub-paso de simulación: 60 Hz.
pub const FIXED_DT: f32 = 1.0 / 60.0;

/// Número máximo de sub-pasos por frame (evita el "espiral de la muerte"
/// si el frame tarda mucho: nunca podemos simular más de MAX_SUBSTEPS *
/// FIXED_DT segundos de física en un solo frame).
const MAX_SUBSTEPS: u32 = 8;

// ─── Estado Global ────────────────────────────────────────────────────────────

pub struct PhysicsState {
    /// ¿Está el motor activo?
    pub running:       bool,
    /// Tiempo físico total transcurrido (suma de FIXED_DT * ticks ejecutados).
    pub elapsed_time:  f32,
    /// Número total de sub-pasos de física ejecutados desde el inicio.
    pub tick_count:    u64,
    /// Acumulador de tiempo restante del frame actual.
    /// Cuando supera FIXED_DT, se ejecuta un sub-paso y se descuenta.
    pub accumulator:   f32,
}

impl PhysicsState {
    fn new() -> Self {
        Self {
            running:     false,
            elapsed_time: 0.0,
            tick_count:   0,
            accumulator:  0.0,
        }
    }
}

static PHYSICS: OnceLock<Mutex<PhysicsState>> = OnceLock::new();

fn state() -> &'static Mutex<PhysicsState> {
    PHYSICS.get_or_init(|| Mutex::new(PhysicsState::new()))
}

// ─── API interna ──────────────────────────────────────────────────────────────

/// Inicializa el motor. Solo tiene efecto la primera vez.
pub fn init() {
    let mut s = state().lock().unwrap();
    if !s.running {
        s.running     = true;
        s.accumulator = 0.0;
        log::info!(
            "[Physics] Motor inicializado | FIXED_DT={:.4}s ({}Hz) | MAX_SUBSTEPS={}",
            FIXED_DT, (1.0 / FIXED_DT) as u32, MAX_SUBSTEPS
        );
    }
}

/// Avanza la simulación usando un Fixed Timestep de 60 Hz.
///
/// - `dt_real` es el DeltaTime real del frame de Unreal (variable por FPS).
/// - Internamente el motor acumula `dt_real` y ejecuta tantos sub-pasos de
///   `FIXED_DT` como sean necesarios, hasta un máximo de `MAX_SUBSTEPS`.
///
/// Retorna el número de sub-pasos ejecutados este frame.
pub fn tick(dt_real: f32) -> u32 {
    let mut s = state().lock().unwrap();
    if !s.running {
        log::warn!("[Physics] tick() llamado antes de init(). Ignorado.");
        return 0;
    }

    // Guard: descartamos deltas absurdos (pausa del editor, breakpoints, etc.)
    let dt_clamped = dt_real.min(FIXED_DT * MAX_SUBSTEPS as f32);

    s.accumulator += dt_clamped;

    let mut substeps: u32 = 0;

    while s.accumulator >= FIXED_DT && substeps < MAX_SUBSTEPS {
        // ── Sub-paso de física a FIXED_DT constante ───────────────────────────
        step_simulation(&mut s, FIXED_DT);

        s.accumulator  -= FIXED_DT;
        s.elapsed_time += FIXED_DT;
        s.tick_count   += 1;
        substeps       += 1;
    }

    if substeps > 0 {
        log::debug!(
            "[Physics] Frame: dt_real={:.4}s | sub-pasos={} | tick_total={}",
            dt_real, substeps, s.tick_count
        );
    }

    substeps
}

/// Lógica de un único sub-paso de simulación a tiempo constante `dt`.
/// Aquí se llama a todos los sistemas que necesitan un timestep estable.
fn step_simulation(s: &mut PhysicsState, dt: f32) {
    // 1. Cerebro de Enjambre (Boids) a paso fijo
    crate::swarm::tick(dt);

    // 2. Simulación de vuelo de fragmentos: Verlet + Gravedad + Rebotes
    crate::fragment_sim::tick(dt);

    // 3. Cada 300 ticks (~5s a 60Hz) purgar fragmentos dormidos
    //    para liberar memoria y mantener el HashMap compacto.
    if s.tick_count % 300 == 0 && s.tick_count > 0 {
        crate::fragment_sim::purge_sleeping();
    }
}


/// Apaga el motor y resetea el estado.
pub fn shutdown() {
    let mut s = state().lock().unwrap();
    s.running = false;
    log::info!(
        "[Physics] Motor apagado | tick_total={} | tiempo_físico={:.4}s | FIXED_DT={}Hz",
        s.tick_count,
        s.elapsed_time,
        (1.0 / FIXED_DT) as u32
    );
}
