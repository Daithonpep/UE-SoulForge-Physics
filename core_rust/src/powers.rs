use std::collections::HashMap;
use crate::node_buffer::{NodeBuffer, FLAG_ACTIVE};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Element {
    Fire, Water, Earth, Air, Lightning, Ice, Gravity, Energy, Void, Time,
}

#[derive(Debug, Clone)]
pub struct ActivePower {
    pub power_type: u32,
    pub start_time: f64,
    pub duration: f64,
    pub origin: [f64; 3],
    pub direction: [f64; 3],
    pub intensity: f64,
}

pub trait PowerEffect: Send + Sync {
    fn name(&self) -> &str;
    fn element(&self) -> Element;
    fn apply_to_nodes(&self, nodes: &mut NodeBuffer, power: &ActivePower, current_time: f64, dt: f64);
}

// ════════════════════════════════════════════════════════════════════════════
// PODER: AGUJERO NEGRO / IMPLOSIÓN GRAVITACIONAL (PREMIUM V2)
// ════════════════════════════════════════════════════════════════════════════

pub struct BlackHoleEffect {
    pub pull_radius: f64,
    pub pull_force: f64,
}

impl PowerEffect for BlackHoleEffect {
    fn name(&self) -> &str { "Black Hole" }
    fn element(&self) -> Element { Element::Gravity }

    fn apply_to_nodes(&self, nodes: &mut NodeBuffer, power: &ActivePower, current_time: f64, _dt: f64) {
        let elapsed = current_time - power.start_time;
        
        // El agujero negro tiene una curva de potencia: nace suave, máximo al medio, y colapsa al final.
        let life_phase = elapsed / power.duration;
        if life_phase > 1.0 { return; }
        
        let strength_curve = if life_phase < 0.2 {
            life_phase / 0.2 // Crecimiento
        } else if life_phase < 0.8 {
            1.0 // Mesera estable
        } else {
            (1.0 - life_phase) / 0.2 // Colapso final
        };

        let strength = strength_curve * power.intensity;
        let n = nodes.count.load(std::sync::atomic::Ordering::Relaxed);
        let origin = power.origin;

        for i in 0..n {
            if nodes.state_flags[i] & FLAG_ACTIVE == 0 { continue; }
            let dx = origin[0] - nodes.pos_x[i] as f64;
            let dy = origin[1] - nodes.pos_y[i] as f64;
            let dz = origin[2] - nodes.pos_z[i] as f64;
            let d2 = dx*dx + dy*dy + dz*dz;
            let d = d2.sqrt().max(1.0);

            if d < self.pull_radius {
                // 1. Atracción Radial (Succión)
                let force = (self.pull_force * strength) / (d * 0.05 + 10.0);
                
                // 2. Efecto de Vórtice (Remolino) - Añade belleza al caos
                let swirl_intensity = strength * 0.2;
                let swirl_x = -dy * swirl_intensity;
                let swirl_y = dx * swirl_intensity;

                nodes.force_x[i] += (dx/d) * force + swirl_x;
                nodes.force_y[i] += (dy/d) * force + swirl_y;
                nodes.force_z[i] += (dz/d) * force;

                // 3. Estabilización en el núcleo: El "Horizonte de Eventos"
                if d < 100.0 {
                    let friction = 0.85; 
                    nodes.vel_x[i] *= friction;
                    nodes.vel_y[i] *= friction;
                    nodes.vel_z[i] *= friction;
                    
                    // Pequeña elevación para que no se peguen al suelo dentro del agujero
                    nodes.force_z[i] += 500.0; 
                }
            }
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// PODER: GRAVEDAD CERO / MATRIX (PREMIUM V2)
// ════════════════════════════════════════════════════════════════════════════

pub struct ZeroGravityEffect {
    pub effect_radius: f64,
}

impl PowerEffect for ZeroGravityEffect {
    fn name(&self) -> &str { "Anti-Gravity Field" }
    fn element(&self) -> Element { Element::Air }

    fn apply_to_nodes(&self, nodes: &mut NodeBuffer, power: &ActivePower, current_time: f64, _dt: f64) {
        let n = nodes.count.load(std::sync::atomic::Ordering::Relaxed);
        let origin = [power.origin[0], power.origin[1], power.origin[2]];
        let r_sq = self.effect_radius * self.effect_radius;
        let t = current_time as f32;

        for i in 0..n {
            if nodes.state_flags[i] & FLAG_ACTIVE == 0 { continue; }
            let dx = origin[0] - nodes.pos_x[i] as f64;
            let dy = origin[1] - nodes.pos_y[i] as f64;
            let dz = origin[2] - nodes.pos_z[i] as f64;
            let d2 = dx*dx + dy*dy + dz*dz;

            if d2 < r_sq {
                // 1. CANCELAR GRAVEDAD (Antigravedad pura)
                // Unreal usa -980.0, nosotros aplicamos +980.0 para flotabilidad perfecta.
                nodes.force_z[i] += 980.0 * nodes.mass[i] as f64;

                // 2. EFECTO MATRIX (Damping Masivo)
                // Ralentiza los escombros hasta dejarlos casi inmóviles (Time Freeze)
                let matrix_damping = 0.88; 
                nodes.vel_x[i] *= matrix_damping;
                nodes.vel_y[i] *= matrix_damping;
                nodes.vel_z[i] *= matrix_damping;

                // 3. MICRO-LEVITACIÓN OSCILANTE
                // Añade un pequeño movimiento "flotante" para que no parezca una foto estática
                let wobble = (t * 2.0 + (i % 10) as f32).sin() * 50.0;
                nodes.force_z[i] += wobble as f64;
                
                // 4. Intensidad adicional (Push suave hacia afuera para formar la esfera)
                let d = d2.sqrt().max(1.0);
                let push = (power.intensity * 100.0) / (d * 0.1 + 1.0);
                nodes.force_x[i] -= (dx/d) * push;
                nodes.force_y[i] -= (dy/d) * push;
                nodes.force_z[i] -= (dz/d) * push;
            }
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// PODER: KAMEHAMEHA / ENERGY BEAM
// ════════════════════════════════════════════════════════════════════════════

pub struct EnergyBeam {
    pub beam_radius: f64,
    pub push_force: f64,
}

impl PowerEffect for EnergyBeam {
    fn name(&self) -> &str { "Energy Beam" }
    fn element(&self) -> Element { Element::Energy }
    
    fn apply_to_nodes(&self, nodes: &mut NodeBuffer, power: &ActivePower, current_time: f64, _dt: f64) {
        let elapsed = current_time - power.start_time;
        let origin = power.origin;
        let dir = power.direction;
        let n = nodes.count.load(std::sync::atomic::Ordering::Relaxed);

        // Fase de carga: Atraer escombros al inicio del rayo
        if elapsed < 1.0 {
            let pull_radius = 800.0;
            for i in 0..n {
                if nodes.state_flags[i] & FLAG_ACTIVE == 0 { continue; }
                let dx = origin[0] - nodes.pos_x[i] as f64;
                let dy = origin[1] - nodes.pos_y[i] as f64;
                let dz = origin[2] - nodes.pos_z[i] as f64;
                let d = (dx*dx+dy*dy+dz*dz).sqrt().max(1.0);
                if d < pull_radius {
                    let f = 500000.0 * power.intensity / (d + 1.0);
                    nodes.force_x[i] += (dx/d)*f;
                    nodes.force_y[i] += (dy/d)*f;
                    nodes.force_z[i] += (dz/d)*f;
                }
            }
        } else {
            // Fase de disparo: Empujar todo en el cilindro del rayo
            for i in 0..n {
                if nodes.state_flags[i] & FLAG_ACTIVE == 0 { continue; }
                let dx = nodes.pos_x[i] as f64 - origin[0];
                let dy = nodes.pos_y[i] as f64 - origin[1];
                let dz = nodes.pos_z[i] as f64 - origin[2];
                let dist_proj = dx*dir[0] + dy*dir[1] + dz*dir[2];
                
                if dist_proj > 0.0 && dist_proj < 15000.0 {
                    let perp_x = dx - dist_proj * dir[0];
                    let perp_y = dy - dist_proj * dir[1];
                    let perp_z = dz - dist_proj * dir[2];
                    let dist_perp_sq = perp_x*perp_x + perp_y*perp_y + perp_z*perp_z;
                    
                    if dist_perp_sq < self.beam_radius * self.beam_radius {
                        let f = self.push_force * power.intensity;
                        nodes.force_x[i] += dir[0] * f;
                        nodes.force_y[i] += dir[1] * f;
                        nodes.force_z[i] += dir[2] * f;
                    }
                }
            }
        }
    }
}

// ════════════════════════════════════════════════════════════════════════════
// GESTIÓN DE SISTEMAS
// ════════════════════════════════════════════════════════════════════════════

pub struct PowerSystem {
    pub active_powers: Vec<ActivePower>,
    pub effects: HashMap<u32, Box<dyn PowerEffect>>,
}

impl PowerSystem {
    pub fn new() -> Self {
        let mut effects: HashMap<u32, Box<dyn PowerEffect>> = HashMap::new();
        
        // Registro de habilidades:
        // 0: Kamehameha (Energy Beam)
        // 1: Black Hole (Gravity)
        // 2: Zero Gravity / Matrix
        effects.insert(0, Box::new(EnergyBeam { beam_radius: 200.0, push_force: 80000.0 }));
        effects.insert(1, Box::new(BlackHoleEffect { pull_radius: 2500.0, pull_force: 800000.0 }));
        effects.insert(2, Box::new(ZeroGravityEffect { effect_radius: 3000.0 }));

        Self { active_powers: Vec::new(), effects }
    }

    pub fn update(&mut self, nodes: &mut NodeBuffer, current_time: f64, dt: f64) {
        self.active_powers.retain(|p| current_time < p.start_time + p.duration);
        for p in &self.active_powers {
            if let Some(effect) = self.effects.get(&p.power_type) {
                effect.apply_to_nodes(nodes, p, current_time, dt);
            }
        }
    }
}

impl Clone for PowerSystem {
    fn clone(&self) -> Self {
        Self::new()
    }
}
