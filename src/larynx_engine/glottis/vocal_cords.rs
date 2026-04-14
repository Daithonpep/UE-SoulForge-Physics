use std::f64::consts::PI;

pub struct SyntheticGlottis {
    pub f0: f64,
    pub f0_range: (f64, f64),
    phase: f64,
    sample_rate: f64,
    pub pulse_params: GlottalPulseParams,
    last_sample: f64,
    // Estado interno para variaciones ciclo a ciclo
    current_jitter: f64,
    current_shimmer: f64,
    rng_seed: u64,
}

#[derive(Debug, Clone)]
pub struct GlottalPulseParams {
    pub open_quotient: f64,
    pub speed_quotient: f64,
    pub jitter: f64,
    pub shimmer: f64,
    pub breathiness: f64,
    pub tension: f64,
}

impl SyntheticGlottis {
    pub fn new(base_f0: f64, sample_rate: f64) -> Self {
        Self {
            f0: base_f0,
            f0_range: (base_f0 * 0.5, base_f0 * 3.0),
            phase: 0.0,
            sample_rate,
            pulse_params: GlottalPulseParams {
                open_quotient: 0.6,
                speed_quotient: 2.0,
                jitter: 0.015,   // 1.5% de variación de pitch
                shimmer: 0.04,   // 4% de variación de volumen
                breathiness: 0.1, // 10% de ruido de aire
                tension: 0.5,
            },
            last_sample: 0.0,
            current_jitter: 1.0,
            current_shimmer: 1.0,
            rng_seed: (base_f0 * 100.0) as u64,
        }
    }

    pub fn generate(&mut self, num_samples: usize) -> Vec<f64> {
        let mut output = Vec::with_capacity(num_samples);
        for _ in 0..num_samples {
            output.push(self.generate_sample());
        }
        output
    }

    #[inline(always)]
    pub fn generate_sample(&mut self) -> f64 {
        let mut rng = SimpleRng { state: self.rng_seed };
        
        // El Jitter y Shimmer se calculan una vez por ciclo para sonar natural
        if self.phase < (self.f0 / self.sample_rate) {
            self.current_jitter = 1.0 + (rng.next_f64() - 0.5) * self.pulse_params.jitter;
            self.current_shimmer = 1.0 + (rng.next_f64() - 0.5) * self.pulse_params.shimmer;
            self.rng_seed = rng.state; // Persistir semilla
        }

        let j_f0 = self.f0 * self.current_jitter;
        
        // 1. Pulso Glotal (Armónico)
        let glottal_pulse = self.lf_pulse(self.phase) * self.current_shimmer;
        
        // 2. Ruido Aspirado (No aire, solo soplado)
        // El ruido ocurre principalmente cuando la glotis está abierta (fase < oq)
        let oq = self.pulse_params.open_quotient.clamp(0.3, 0.7);
        let aspirate_mod = if self.phase < oq { 1.0 } else { 0.2 }; // Un poco de ruido residual
        let noise = (rng.next_f64() - 0.5) * self.pulse_params.breathiness * aspirate_mod;
        
        // Actualizar fase
        self.phase += j_f0 / self.sample_rate;
        if self.phase >= 1.0 { self.phase -= 1.0; }
        
        // 3. Mezcla y Filtro de Radiación (LPF simple de 1er orden)
        // Esto imita la pérdida de energía en los labios y garganta
        let out = 0.8 * (glottal_pulse + noise) + 0.2 * self.last_sample;
        self.last_sample = out;
        
        out
    }

    fn next_rng(&mut self) -> f64 {
        let mut rng = SimpleRng { state: self.rng_seed };
        let val = rng.next_f64();
        self.rng_seed = rng.state;
        val
    }    fn lf_pulse(&self, phase: f64) -> f64 {
        let oq = self.pulse_params.open_quotient.clamp(0.3, 0.7);
        // let _sq = self.pulse_params.speed_quotient.clamp(1.5, 3.0);
        if phase < oq {
            let norm = phase / oq;
            // Forma de onda glotal estándar (más armónica)
            let val = (norm * std::f64::consts::PI).sin().powf(2.0);
            val * (1.0 - (norm * std::f64::consts::PI / 2.0).cos())
        } else {
            0.0
        }
    }
}

pub struct SimpleRng { state: u64 }
impl SimpleRng {
    pub fn new(seed: u64) -> Self { Self { state: seed.max(1) } }
    pub fn next_u64(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }
    pub fn next_f64(&mut self) -> f64 { (self.next_u64() as f64) / (u64::MAX as f64) }
}
