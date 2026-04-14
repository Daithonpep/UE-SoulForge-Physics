use super::physical_tract::TractShape;

pub struct ComplexGlottalSource {
    sample_rate: f64,
    base_f0: f64,
    mass1_position: f64,
    mass1_velocity: f64,
    mass2_position: f64,
    mass2_velocity: f64,
    subglottal_pressure: f64,
    #[allow(dead_code)]
    cord_tension: f64,
    cord_mass: f64,
    cord_damping: f64,
    #[allow(dead_code)]
    cord_stiffness: f64,
    coupling_spring: f64,
    phase: f64,
    #[allow(dead_code)]
    cycle_count: u64,
    rng: FastRng,
    pub params: SourceParams,
    // Filtrado de doble etapa para calidez extrema
    tilt_state1: f64,
    tilt_state2: f64,
}
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SourceParams {
    pub vibrato_rate: f64,
    pub vibrato_depth: f64,
    pub jitter: f64,
    pub shimmer: f64,
    pub aspiration: f64,
    pub spectral_tilt: f64,
    pub pressure_variation: f64,
}

impl Default for SourceParams {
    fn default() -> Self {
        Self {
            vibrato_rate: 5.5,
            vibrato_depth: 0.008,
            jitter: 0.003,
            shimmer: 0.015,
            aspiration: 0.005, // Drástica reducción: de 0.04 a 0.005
            spectral_tilt: 0.94, // De 0.25 a 0.94 para calidez total
            pressure_variation: 0.02,
        }
    }
}

impl ComplexGlottalSource {
    pub fn new(base_f0: f64, sample_rate: f64) -> Self {
        let cord_mass = 0.125;
        let cord_stiffness = (2.0 * std::f64::consts::PI * base_f0).powi(2) * cord_mass;
        Self {
            sample_rate,
            base_f0,
            mass1_position: 0.0,
            mass1_velocity: 0.0,
            mass2_position: 0.0,
            mass2_velocity: 0.0,
            subglottal_pressure: 1000.0, // Subida a 1000 Pa para más potencia de cierre
            cord_tension: 0.5,
            cord_mass,
            cord_damping: 0.01,
            cord_stiffness,
            coupling_spring: cord_stiffness * 0.5,
            phase: 0.0,
            cycle_count: 0,
            rng: FastRng::new(42),
            params: SourceParams::default(),
            tilt_state1: 0.0,
            tilt_state2: 0.0,
        }
    }

    pub fn next_sample(&mut self, _tract_shape: &TractShape) -> f64 {
        let dt = 1.0 / self.sample_rate;
        let vibrato = (self.phase * self.params.vibrato_rate * std::f64::consts::TAU).sin() * self.params.vibrato_depth * self.base_f0;
        let jitter = (self.rng.next_f64() - 0.5) * self.params.jitter * self.base_f0 * 2.0;
        let p_var = 1.0 + (self.rng.next_f64() - 0.5) * self.params.pressure_variation;
        let c_p = self.subglottal_pressure * p_var;

        let g_area = (self.mass1_position + self.mass2_position + 0.1).max(0.0);
        let p_force = if g_area > 0.001 { c_p * 0.5 } else { c_p };
        let i_f0 = self.base_f0 + vibrato + jitter;
        let i_stiffness = (2.0 * std::f64::consts::PI * i_f0).powi(2) * self.cord_mass;

        let s_f1 = -i_stiffness * self.mass1_position;
        let c_f1 = -self.coupling_spring * (self.mass1_position - self.mass2_position);
        let d_f1 = -self.cord_damping * self.mass1_velocity;
        let coll1 = if self.mass1_position < -0.05 { -self.mass1_position * i_stiffness * 3.0 } else { 0.0 };
        let accel1 = (p_force + s_f1 + c_f1 + d_f1 + coll1) / self.cord_mass;
        self.mass1_velocity += accel1 * dt;
        self.mass1_position += self.mass1_velocity * dt;

        let s_f2 = -i_stiffness * 0.8 * self.mass2_position;
        let c_f2 = -self.coupling_spring * (self.mass2_position - self.mass1_position);
        let d_f2 = -self.cord_damping * 1.2 * self.mass2_velocity;
        let coll2 = if self.mass2_position < -0.05 { -self.mass2_position * i_stiffness * 3.0 } else { 0.0 };
        let accel2 = (p_force * 0.8 + s_f2 + c_f2 + d_f2 + coll2) / (self.cord_mass * 1.1);
        self.mass2_velocity += accel2 * dt;
        self.mass2_position += self.mass2_velocity * dt;

        // ─── Flujo No-Lineal (Suavizado) ───
        // Usar powf(1.5) suaviza el inicio/fin del flujo, eliminando el "clic/zumbido"
        let smooth_area = g_area.powf(1.5);
        let flow = if g_area > 0.001 { (2.0 * c_p / 1.14).sqrt() * smooth_area * 0.0008 } else { 0.0 };
        let shim = 1.0 + (self.rng.next_f64() - 0.5) * self.params.shimmer * 2.0;
        
        // ─── Aspiration Ultra-Sutil ───
        let asp_gate = if g_area > 0.1 { 0.04 } else { 0.005 };
        let noise = (self.rng.next_f64() * 2.0 - 1.0) * self.params.aspiration * asp_gate * g_area.sqrt();
        
        let raw_sample = (flow * shim) + noise;

        // ─── Doble Spectral Tilt (2-Stage Integrator) ───
        // Crea una caída de -12dB/oct, idéntica a la voz humana
        let alpha = self.params.spectral_tilt.clamp(0.1, 0.97);
        let s1 = raw_sample * (1.0 - alpha) + self.tilt_state1 * alpha;
        self.tilt_state1 = s1;
        
        let out = s1 * (1.0 - alpha) + self.tilt_state2 * alpha;
        self.tilt_state2 = out;

        self.phase += dt;
        if self.mass1_position > 0.0 && self.mass1_velocity < 0.0 && self.mass2_position > 0.0 {
            self.cycle_count += 1;
        }

        out
    }

    pub fn set_f0(&mut self, f0: f64) { self.base_f0 = f0.clamp(50.0, 500.0); }
    pub fn set_tension(&mut self, t: f64) { 
        self.cord_damping = 0.005 + t * 0.02; 
    }
}

struct FastRng { state: u64 }
impl FastRng {
    fn new(seed: u64) -> Self { Self { state: seed.max(1) } }
    fn next_f64(&mut self) -> f64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        (self.state as f64) / (u64::MAX as f64)
    }
}
