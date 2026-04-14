//! # proxy.rs – Física Modular V8.7 (Hash Fix + Efectos Independientes)
//! Cada efecto (TNT, Agujero Negro, etc.) tiene su propia física.
//! Las globales (gravedad, colisión suelo) son compartidas.

use std::collections::HashMap;
use std::sync::{Mutex, OnceLock};
use serde::{Serialize, Deserialize};
use crate::fragment_sim;
use rand::Rng;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[repr(i32)]
pub enum DamagePreset {
    Explosion = 0,
    Implosion = 1,
    Impact = 2,
    ShapedCharge = 3,
    Cinematic = 4,
    Collapse = 5,
    RealityShatter = 6,
    Vortex = 7,
}

impl DamagePreset {
    pub fn from_i32(val: i32) -> Self {
        match val {
            1 => Self::Implosion,
            2 => Self::Impact,
            3 => Self::ShapedCharge,
            4 => Self::Cinematic,
            5 => Self::Collapse,
            6 => Self::RealityShatter,
            7 => Self::Vortex,
            _ => Self::Explosion,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FragmentCategory { Slab = 0, Chunk = 1, Gravel = 2, Dust = 3 }

impl FragmentCategory {
    pub fn size_scale(&self) -> f32 {
        match self {
            // Escalas aumentadas para visibilidad de Júpiter
            Self::Slab => 0.85, Self::Chunk => 0.45, Self::Gravel => 0.25, Self::Dust => 0.12,
        }
    }
    pub fn base_mass(&self) -> f32 {
        match self {
            Self::Slab => 10.0, Self::Chunk => 2.5, Self::Gravel => 0.5, Self::Dust => 0.05,
        }
    }
}

pub struct FragmentLayout {
    pub slabs: u32, pub chunks: u32, pub gravels: u32, pub dusts: u32,
}

impl FragmentLayout {
    pub fn for_preset(preset: DamagePreset, frag_level: f32, target_count: u32) -> Self {
        // Ratios base por categoría (suman 1.0)
        let (r_slab, r_chunk, r_gravel, r_dust) = match preset {
            DamagePreset::Explosion    => (0.05, 0.15, 0.30, 0.50),
            DamagePreset::Implosion    => (0.05, 0.15, 0.30, 0.50),
            DamagePreset::Impact       => (0.03, 0.13, 0.34, 0.50),
            DamagePreset::ShapedCharge => (0.02, 0.04, 0.31, 0.63),
            DamagePreset::Cinematic    => (0.06, 0.16, 0.29, 0.49),
            DamagePreset::Collapse     => (0.00, 0.16, 0.28, 0.56),
            DamagePreset::RealityShatter => (0.03, 0.16, 0.27, 0.54),
            DamagePreset::Vortex         => (0.05, 0.20, 0.35, 0.40),
        };
        
        // Escalamos al target_count (mínimo 10 fragmentos)
        let total = (target_count.max(10)) as f32;
        let mut l = Self {
            slabs:   (total * r_slab).max(if r_slab > 0.0 { 1.0 } else { 0.0 }) as u32,
            chunks:  (total * r_chunk).max(1.0) as u32,
            gravels: (total * r_gravel).max(1.0) as u32,
            dusts:   (total * r_dust).max(1.0) as u32,
        };
        
        // Multiplicador extra de Júpiter para Collapse
        if preset == DamagePreset::Collapse {
            l.chunks  = (l.chunks  as f32 * frag_level).max(1.0) as u32;
            l.gravels = (l.gravels as f32 * frag_level * 1.5).max(1.0) as u32;
            l.dusts   = (l.dusts   as f32 * frag_level * 2.0).max(1.0) as u32;
        }
        l
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PhysicsProxy {
    pub id: String, pub position: [f32; 3], pub half_ext: [f32; 3], pub density: f32,
}

impl PhysicsProxy {
    pub fn new(id: String, pos: [f32; 3], ext: [f32; 3], den: f32) -> Self {
        Self { id, position: pos, half_ext: ext, density: den }
    }
}

/// Calcula el hash FNV-1a idéntico al que C++ usa en BeginPlay
fn fnv1a_hash(s: &str) -> u32 {
    let mut h: u32 = 2166136261;
    for b in s.bytes() {
        h ^= b as u32;
        h = h.wrapping_mul(16777619);
    }
    h
}

// ═══════════════════════════════════════════════════════════
//  FÍSICA MODULAR POR EFECTO
// ═══════════════════════════════════════════════════════════

/// Cada preset define su propia velocidad, dispersión, gravedad
struct EffectPhysics {
    radial_force: f32,     // Fuerza base radial
    vertical_boost: f32,   // Impulso vertical extra
    randomness: f32,       // Dispersión aleatoria
    gravity_override: f32, // -980 = normal, 0 = sin gravedad, +980 = antigravedad
}

fn get_effect_physics(preset: DamagePreset, energy: f32) -> EffectPhysics {
    // Escala logarítmica: 10000→2.3, 100000→3.3, 1000000→5.0
    let power = (energy.max(100.0).log10() - 2.0).max(0.5);
    match preset {
        DamagePreset::Explosion => EffectPhysics {
            // TNT: Explosión VIOLENTA radial
            radial_force: 1500.0 * power,
            vertical_boost: 800.0 * power,
            randomness: 500.0,
            gravity_override: -980.0,
        },
        DamagePreset::Implosion => EffectPhysics {
            // Agujero Negro: Empuje inicial VIOLENTO para que vuelen lejos (1300)
            radial_force: 1300.0 * power,
            vertical_boost: 600.0 * power,
            randomness: 500.0,
            gravity_override: 0.0,
        },
        DamagePreset::Impact => EffectPhysics {
            radial_force: 1200.0 * power,
            vertical_boost: 300.0,
            randomness: 200.0,
            gravity_override: -980.0,
        },
        DamagePreset::ShapedCharge => EffectPhysics {
            radial_force: 2500.0 * power,
            vertical_boost: 100.0,
            randomness: 150.0,
            gravity_override: -980.0,
        },
        DamagePreset::Cinematic => EffectPhysics {
            // Matrix: Explosión inicial potente + Congelación
            radial_force: 1400.0 * power,
            vertical_boost: 600.0 * power,
            randomness: 800.0,
            gravity_override: 0.0,
        },
        DamagePreset::Collapse => EffectPhysics {
            // Júpiter: colapso pesado
            radial_force: 1000.0 * power,
            vertical_boost: 400.0,
            randomness: 300.0,
            gravity_override: -1500.0,
        },
        DamagePreset::RealityShatter => EffectPhysics {
            // Ruptura de Realidad: Caos total, gravedad cero (1500)
            radial_force: 1500.0 * power,
            vertical_boost: 800.0 * power,
            randomness: 2000.0, // Caos extremo
            gravity_override: 0.0,
        },
        DamagePreset::Vortex => EffectPhysics {
            // Vórtice: Expansión moderada + Tornado dinámico
            radial_force: 600.0 * power,
            vertical_boost: 300.0 * power,
            randomness: 400.0,
            gravity_override: 0.0,
        },
    }
}

// ═══════════════════════════════════════════════════════════
//  DETONACIÓN
// ═══════════════════════════════════════════════════════════

pub fn destroy(
    id: &str, 
    preset: DamagePreset, 
    energy: f32, 
    frag_level: f32,
    custom_count: i32
) -> (u32, u32) {
    let proxy = {
        let Ok(mut s) = crate::proxy::state().lock() else { return (0, 0) };
        let Some(p) = s.proxies.remove(id) else { return (0, 0) };
        p
    };

    let hash = fnv1a_hash(id);
    let layout = FragmentLayout::for_preset(preset, frag_level, custom_count as u32);
    let physics = get_effect_physics(preset, energy);
    let mut rng = rand::thread_rng();

    let categories = [
        (FragmentCategory::Slab,   layout.slabs),
        (FragmentCategory::Chunk,  layout.chunks),
        (FragmentCategory::Gravel, layout.gravels),
        (FragmentCategory::Dust,   layout.dusts),
    ];

    for (cat, count) in &categories {
        for _ in 0..*count {
            // Posición dentro del volumen del objeto
            let px = proxy.position[0] + rng.gen_range(-proxy.half_ext[0]..proxy.half_ext[0]);
            let py = proxy.position[1] + rng.gen_range(-proxy.half_ext[1]..proxy.half_ext[1]);
            let pz = proxy.position[2] + rng.gen_range(-proxy.half_ext[2]..proxy.half_ext[2]);
            
            // Dirección desde el centro del objeto (explosión radial)
            let dx = px - proxy.position[0];
            let dy = py - proxy.position[1];
            let dz = pz - proxy.position[2];
            let d = (dx*dx + dy*dy + dz*dz).sqrt().max(0.1);
            
            // Velocidad = dirección radial × fuerza del efecto + ruido
            let vx = (dx/d) * physics.radial_force + rng.gen_range(-physics.randomness..physics.randomness);
            let vy = (dy/d) * physics.radial_force + rng.gen_range(-physics.randomness..physics.randomness);
            let vz = (dz/d) * physics.radial_force + physics.vertical_boost + rng.gen_range(-physics.randomness..physics.randomness);

            let s = cat.size_scale() * rng.gen_range(0.8..1.2);
            fragment_sim::register_v7(
                [px, py, pz], 
                [vx, vy, vz], 
                cat.base_mass(), 
                [s, s, s], // Escapa inicial uniforme
                *cat as i32, 
                hash
            );
        }
    }

    // El modo del efecto controla la física post-detonación en fragment_sim
    fragment_sim::trigger_auto_power(preset as i32, proxy.position, energy);
    
    let total_count = layout.slabs + layout.chunks + layout.gravels + layout.dusts;
    (hash, total_count)
}

// ═══════════════════════════════════════════════════════════
//  ESTADO GLOBAL
// ═══════════════════════════════════════════════════════════

pub struct ProxyState { pub proxies: HashMap<String, PhysicsProxy> }
static STATE: OnceLock<Mutex<ProxyState>> = OnceLock::new();
pub fn state() -> &'static Mutex<ProxyState> {
    STATE.get_or_init(|| Mutex::new(ProxyState { proxies: HashMap::new() }))
}

pub fn register(id: String, pos: [f32; 3], ext: [f32; 3], den: f32) {
    if let Ok(mut s) = state().lock() {
        s.proxies.insert(id.clone(), PhysicsProxy::new(id, pos, ext, den));
    }
}

pub fn remove(id: &str) {
    if let Ok(mut s) = state().lock() {
        s.proxies.remove(id);
    }
}

pub fn clear() {
    if let Ok(mut s) = state().lock() { s.proxies.clear(); }
}
