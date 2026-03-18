//! # Types – Tipos de Datos Compartidos
//!
//! Estructuras que pueden ser serializadas a JSON y leídas por
//! cualquiera de los tres núcleos (Rust, C++, Python).

use serde::{Deserialize, Serialize};

// ─── Vectores y Transforms ────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct SfVec3 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl SfVec3 {
    pub fn new(x: f32, y: f32, z: f32) -> Self { Self { x, y, z } }
    pub fn zero() -> Self { Self::new(0.0, 0.0, 0.0) }
    pub fn length(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
}

impl From<[f32; 3]> for SfVec3 {
    fn from(a: [f32; 3]) -> Self { Self::new(a[0], a[1], a[2]) }
}

// ─── Mensajes de Escena (Rust → Python) ──────────────────────────────────────

/// Mensaje que Rust emite con el estado de la escena para que Python lo consuma.
#[derive(Debug, Serialize, Deserialize)]
pub struct SceneSnapshot {
    pub tick:           u64,
    pub elapsed_secs:   f32,
    pub agent_count:    usize,
    pub proxy_count:    usize,
}

// ─── Configuración de Simulación ──────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimConfig {
    /// Gravedad en cm/s² (Unreal usa cm como unidad base)
    pub gravity:            SfVec3,
    /// Factor de amortiguación lineal (0.0 = sin fricción, 1.0 = detención inmediata)
    pub linear_damping:     f32,
    /// Número máximo de agentes del enjambre
    pub max_swarm_agents:   u32,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            gravity:          SfVec3::new(0.0, 0.0, -980.0), // ≈ -9.8 m/s² en cm
            linear_damping:   0.01,
            max_swarm_agents: 1024,
        }
    }
}
