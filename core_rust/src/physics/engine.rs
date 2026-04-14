//! # Physics Engine – Estado y Timestep Fijo.
//! Este módulo controla el flujo de tiempo y la coordinación de los sistemas físicos.

use std::sync::{Mutex, OnceLock};

pub const FIXED_DT: f32 = 1.0 / 60.0;
const MAX_SUBSTEPS: u32 = 8;

pub struct PhysicsState {
    pub running:       bool,
    pub elapsed_time:  f32,
    pub tick_count:    u64,
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

pub fn state() -> &'static Mutex<PhysicsState> {
    PHYSICS.get_or_init(|| Mutex::new(PhysicsState::new()))
}

pub fn init() {
    let mut s = state().lock().unwrap();
    if !s.running {
        s.running     = true;
        s.accumulator = 0.0;
        log::info!("[Physics] SoulForge Modular Engine V3 Ready. FIXED_DT={:.4}s", FIXED_DT);
    }
}

pub fn tick(dt_real: f32) -> u32 {
    let mut s = state().lock().unwrap();
    if !s.running { return 0; }

    let dt_clamped = dt_real.min(FIXED_DT * MAX_SUBSTEPS as f32);
    s.accumulator += dt_clamped;

    let mut substeps: u32 = 0;
    while s.accumulator >= FIXED_DT && substeps < MAX_SUBSTEPS {
        step_simulation(&mut s, FIXED_DT);
        s.accumulator  -= FIXED_DT;
        s.elapsed_time += FIXED_DT;
        s.tick_count   += 1;
        substeps       += 1;
    }
    substeps
}

fn step_simulation(s: &mut PhysicsState, dt: f32) {
    // 1. Swarm Intelligence (Boids)
    crate::swarm::tick(dt);

    // 2. Physics & Fragments (Modularized Solvers)
    // El sistema de fragmentos llama a los solvers en node_buffer
    crate::fragment_sim::tick(dt);

    // 3. Maintenance
    if s.tick_count % 300 == 0 && s.tick_count > 0 {
        crate::fragment_sim::purge_sleeping();
    }
}

pub fn shutdown() {
    let mut s = state().lock().unwrap();
    s.running = false;
}
