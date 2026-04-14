use crate::larynx_engine::articulator::phoneme_engine::SpanishArticulator;
use crate::larynx_engine::tract::vocal_tract::VocalTract;

/// Sistema de humanización que hace que la voz suene orgánica
/// El principio: NADA en la voz humana es perfecto ni constante.
/// Todo varía, fluctúa, se mueve.
pub struct OrganicVoiceEngine {
    sample_rate: f64,
    /// Generador de ruido orgánico
    organic_noise: OrganicNoiseGenerator,
    /// Modulador de formantes en tiempo real
    formant_modulator: FormantModulator,
    /// Sistema de coarticulación
    coarticulator: Coarticulator,
    /// Modulador de pitch orgánico
    pitch_modulator: PitchModulator,
}

// ═══════════════════════════════════════════════════════════
//  MEJORA 1: Pulso Glotal Orgánico (No más onda de sierra)
// ═══════════════════════════════════════════════════════════

/// Genera pulsos glotales que varían ciclo a ciclo como las cuerdas vocales reales
pub struct OrganicGlottis {
    sample_rate: f64,
    base_f0: f64,
    phase: f64,
    cycle_count: u64,
    current_open_quotient: f64,
    current_speed_quotient: f64,
    current_amplitude: f64,
    #[allow(dead_code)]
    aspiration_noise: f64,
    vibrato_phase: f64,
    tremolo_phase: f64,
    drift_phase: f64,
}

impl OrganicGlottis {
    pub fn new(base_f0: f64, sample_rate: f64) -> Self {
        Self {
            sample_rate,
            base_f0,
            phase: 0.0,
            cycle_count: 0,
            current_open_quotient: 0.6,
            current_speed_quotient: 2.0,
            current_amplitude: 1.0,
            aspiration_noise: 0.0,
            vibrato_phase: 0.0,
            tremolo_phase: 0.0,
            drift_phase: 0.0,
        }
    }

    pub fn generate_organic(&mut self, num_samples: usize, params: &GlottalParams) -> Vec<f64> {
        let mut output = Vec::with_capacity(num_samples);
        let mut rng = FastRng::new(self.cycle_count + 1);

        for _ in 0..num_samples {
            // ─── VIBRATO NATURAL (5-7 Hz, ~0.5-1% de variación) ───
            self.vibrato_phase += params.vibrato_rate / self.sample_rate;
            let vibrato = (self.vibrato_phase * std::f64::consts::TAU).sin() 
                * params.vibrato_depth 
                * self.base_f0;

            // ─── DRIFT (variación lenta, <1 Hz) ───
            self.drift_phase += 0.3 / self.sample_rate;
            let drift = (self.drift_phase * std::f64::consts::TAU).sin() 
                * params.drift_amount 
                * self.base_f0;

            // ─── JITTER (variación aleatoria ciclo a ciclo) ───
            let jitter = (rng.next_f64() - 0.5) * params.jitter * self.base_f0 * 2.0;

            let instant_f0 = (self.base_f0 + vibrato + drift + jitter).max(50.0);

            // ─── TREMOLO (variación de amplitud, 3-5 Hz) ───
            self.tremolo_phase += params.tremolo_rate / self.sample_rate;
            let tremolo = 1.0 + (self.tremolo_phase * std::f64::consts::TAU).sin() 
                * params.tremolo_depth;

            // ─── SHIMMER (variación aleatoria de amplitud) ───
            let shimmer = 1.0 + (rng.next_f64() - 0.5) * params.shimmer * 2.0;

            // ─── PULSO GLOTAL MEJORADO (Rosenberg-Klatt) ───
            let pulse = self.rosenberg_klatt_pulse(
                self.phase,
                self.current_open_quotient,
                self.current_speed_quotient,
                params.spectral_tilt,
            );

            // ─── RUIDO DE ASPIRACIÓN ───
            let aspiration = if self.phase > self.current_open_quotient * 0.8 
                && self.phase < self.current_open_quotient 
            {
                (rng.next_f64() - 0.5) * params.aspiration * 0.3
            } else {
                0.0
            };

            let sample = (pulse * tremolo * shimmer * self.current_amplitude) + aspiration;
            output.push(sample);

            self.phase += instant_f0 / self.sample_rate;
            if self.phase >= 1.0 {
                self.phase -= 1.0;
                self.cycle_count += 1;
                self.vary_cycle_parameters(params, &mut rng);
            }
        }
        output
    }

    fn rosenberg_klatt_pulse(&self, phase: f64, oq: f64, sq: f64, tilt: f64) -> f64 {
        let tp = oq / (1.0 + sq);
        let tn = oq;
        if phase < tp {
            let norm = phase / tp;
            let base = (norm * std::f64::consts::FRAC_PI_2).sin();
            base.powf(1.0 + tilt * 0.5)
        } else if phase < tn {
            let norm = (phase - tp) / (tn - tp);
            let base = (norm * std::f64::consts::FRAC_PI_2).cos();
            base.powf(1.0 + tilt * 0.3)
        } else if phase < tn + 0.02 {
            let norm = (phase - tn) / 0.02;
            -0.1 * (norm * std::f64::consts::PI).sin() * (-norm * 8.0).exp()
        } else {
            0.0
        }
    }

    fn vary_cycle_parameters(&mut self, params: &GlottalParams, rng: &mut FastRng) {
        let oq_var = (rng.next_f64() - 0.5) * 0.05;
        self.current_open_quotient = (params.base_open_quotient + oq_var).clamp(0.3, 0.8);
        let sq_var = (rng.next_f64() - 0.5) * 0.2;
        self.current_speed_quotient = (params.base_speed_quotient + sq_var).clamp(1.0, 4.0);
        let amp_var = (rng.next_f64() - 0.5) * 0.06;
        self.current_amplitude = (1.0 + amp_var).clamp(0.85, 1.15);
    }

    pub fn set_f0(&mut self, f0: f64) {
        self.base_f0 = f0.clamp(50.0, 500.0);
    }
}

#[derive(Debug, Clone)]
pub struct GlottalParams {
    pub base_open_quotient: f64,
    pub base_speed_quotient: f64,
    pub jitter: f64,
    pub shimmer: f64,
    pub vibrato_rate: f64,
    pub vibrato_depth: f64,
    pub tremolo_rate: f64,
    pub tremolo_depth: f64,
    pub drift_amount: f64,
    pub aspiration: f64,
    pub spectral_tilt: f64,
}

impl Default for GlottalParams {
    fn default() -> Self {
        Self {
            base_open_quotient: 0.6,
            base_speed_quotient: 2.0,
            jitter: 0.005,
            shimmer: 0.025,
            vibrato_rate: 5.5,
            vibrato_depth: 0.008,
            tremolo_rate: 4.0,
            tremolo_depth: 0.02,
            drift_amount: 0.01,
            aspiration: 0.1,
            spectral_tilt: 0.3,
        }
    }
}

// ═══════════════════════════════════════════════════════════
//  MEJORA 2: Formantes Dinámicos
// ═══════════════════════════════════════════════════════════

pub struct FormantModulator {
    sample_rate: f64,
    formant_states: Vec<FormantState>,
}

#[derive(Debug, Clone)]
struct FormantState {
    target_frequency: f64,
    current_frequency: f64,
    target_bandwidth: f64,
    current_bandwidth: f64,
    smoothing: f64,
    wobble_phase: f64,
    wobble_rate: f64,
    wobble_depth: f64,
}

impl FormantModulator {
    pub fn new(sample_rate: f64) -> Self {
        Self {
            sample_rate,
            formant_states: vec![
                FormantState {
                    target_frequency: 500.0,
                    current_frequency: 500.0,
                    target_bandwidth: 100.0,
                    current_bandwidth: 100.0,
                    smoothing: 0.005,
                    wobble_phase: 0.0,
                    wobble_rate: 3.2,
                    wobble_depth: 8.0,
                },
                FormantState {
                    target_frequency: 1500.0,
                    current_frequency: 1500.0,
                    target_bandwidth: 150.0,
                    current_bandwidth: 150.0,
                    smoothing: 0.004,
                    wobble_phase: 0.7,
                    wobble_rate: 2.8,
                    wobble_depth: 15.0,
                },
                FormantState {
                    target_frequency: 2500.0,
                    current_frequency: 2500.0,
                    target_bandwidth: 250.0,
                    current_bandwidth: 250.0,
                    smoothing: 0.003,
                    wobble_phase: 1.4,
                    wobble_rate: 2.5,
                    wobble_depth: 20.0,
                },
                FormantState {
                    target_frequency: 3500.0,
                    current_frequency: 3500.0,
                    target_bandwidth: 350.0,
                    current_bandwidth: 350.0,
                    smoothing: 0.003,
                    wobble_phase: 2.1,
                    wobble_rate: 2.2,
                    wobble_depth: 25.0,
                },
            ],
        }
    }

    pub fn set_target(&mut self, idx: usize, freq: f64, bw: f64) {
        if let Some(state) = self.formant_states.get_mut(idx) {
            state.target_frequency = freq;
            state.target_bandwidth = bw;
        }
    }

    pub fn get_current_formants(&mut self) -> Vec<(f64, f64)> {
        let mut res = Vec::new();
        for s in &mut self.formant_states {
            s.current_frequency += (s.target_frequency - s.current_frequency) * s.smoothing;
            s.current_bandwidth += (s.target_bandwidth - s.current_bandwidth) * s.smoothing;
            s.wobble_phase += s.wobble_rate / self.sample_rate;
            let wobble = (s.wobble_phase * std::f64::consts::TAU).sin() * s.wobble_depth;
            res.push((s.current_frequency + wobble, s.current_bandwidth));
        }
        res
    }
}

// ═══════════════════════════════════════════════════════════
//  MEJORA 3: Coarticulación
// ═══════════════════════════════════════════════════════════

pub struct Coarticulator {
    phoneme_queue: Vec<CoarticulatedPhoneme>,
    overlap_ratio: f64,
}

pub struct CoarticulatedPhoneme {
    #[allow(dead_code)]
    pub phoneme: String,
    pub target_formants: Vec<(f64, f64)>,
    pub duration_samples: usize,
    pub is_voiced: bool,
    pub energy: f64,
    pub pitch_target: f64,
}

impl Coarticulator {
    pub fn new() -> Self {
        Self {
            phoneme_queue: Vec::new(),
            overlap_ratio: 0.15,
        }
    }

    pub fn add_phoneme(&mut self, p: CoarticulatedPhoneme) {
        self.phoneme_queue.push(p);
    }

    pub fn generate_coarticulated(
        &self,
        glottis: &mut OrganicGlottis,
        formant_mod: &mut FormantModulator,
        tract: &mut VocalTract,
        params: &GlottalParams,
    ) -> Vec<f64> {
        if self.phoneme_queue.is_empty() { return Vec::new(); }
        let total_samples: usize = self.phoneme_queue.iter().map(|p| p.duration_samples).sum();
        let mut output = vec![0.0; total_samples];
        let mut write_pos: usize = 0;

        for (idx, p) in self.phoneme_queue.iter().enumerate() {
            let overlap = if idx > 0 {
                (self.phoneme_queue[idx-1].duration_samples as f64 * self.overlap_ratio) as usize
            } else { 0 };

            let start = write_pos.saturating_sub(overlap);
            for (i, &(f, b)) in p.target_formants.iter().enumerate() {
                formant_mod.set_target(i, f, b);
            }
            glottis.set_f0(p.pitch_target);

            for off in 0..p.duration_samples {
                let pos = start + off;
                if pos >= output.len() { break; }
                let cf = formant_mod.get_current_formants();
                for (i, &(f, b)) in cf.iter().enumerate() {
                    if i < tract.formants.len() {
                        tract.formants[i].frequency = f;
                        tract.formants[i].bandwidth = b;
                    }
                }
                tract.update_filters();

                let g_sample = if p.is_voiced {
                    glottis.generate_organic(1, params)[0]
                } else {
                    let mut rng = FastRng::new(pos as u64 + 42);
                    (rng.next_f64() - 0.5) * 0.3
                };

                let vocalized = tract.process_sample(g_sample);
                let env = self.calculate_envelope(off, p.duration_samples, overlap);
                let sample = vocalized * p.energy * env;

                if off < overlap && idx > 0 {
                    let blend = off as f64 / overlap as f64;
                    output[pos] = output[pos] * (1.0 - blend) + sample * blend;
                } else {
                    output[pos] += sample;
                }
            }
            write_pos = start + p.duration_samples;
        }
        output
    }

    fn calculate_envelope(&self, pos: usize, total: usize, overlap: usize) -> f64 {
        let attack = overlap.max(32);
        let release = (total / 8).max(32);
        if pos < attack {
            let t = pos as f64 / attack as f64;
            0.5 * (1.0 - (t * std::f64::consts::PI).cos())
        } else if pos > total - release {
            let t = (total - pos) as f64 / release as f64;
            0.5 * (1.0 - (t * std::f64::consts::PI).cos())
        } else {
            1.0
        }
    }
}

// ═══════════════════════════════════════════════════════════
//  MEJORA 4: Pitch Modulator
// ═══════════════════════════════════════════════════════════

pub struct PitchModulator {
    base_f0: f64,
}

impl PitchModulator {
    pub fn new(base_f0: f64, _sr: f64) -> Self {
        Self { base_f0 }
    }

    pub fn generate_pitch_contour(&self, text: &str, count: usize, emotion: &str) -> Vec<f64> {
        let mut contour = Vec::with_capacity(count);
        let q = text.contains('?');
        for i in 0..count {
            let p = i as f64 / count.max(1) as f64;
            let mut pitch = if q {
                if p < 0.6 { self.base_f0 * (1.0 + p * 0.05) }
                else { self.base_f0 * (1.03 + (p - 0.6) / 0.4 * 0.25) }
            } else {
                self.base_f0 * (1.1 - p * 0.15)
            };

            match emotion {
                "cold_disappointment" => pitch *= 0.9,
                "experimental_excited" => pitch *= 1.2,
                "xeno_sarcasm" => pitch *= 1.0 + (p * 4.0).sin().abs() * 0.1,
                _ => {}
            }
            contour.push(pitch);
        }
        contour
    }
}

// ═══════════════════════════════════════════════════════════
//  MEJORA 5: Organic Noise
// ═══════════════════════════════════════════════════════════

pub struct OrganicNoiseGenerator {
    #[allow(dead_code)]
    sample_rate: f64,
}

impl OrganicNoiseGenerator {
    pub fn new(sr: f64) -> Self { Self { sample_rate: sr } }
}

pub struct FastRng { state: u64 }
impl FastRng {
    pub fn new(seed: u64) -> Self { Self { state: seed.max(1) } }
    pub fn next_u64(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }
    pub fn next_f64(&mut self) -> f64 { (self.next_u64() as f64) / (u64::MAX as f64) }
}

impl OrganicVoiceEngine {
    pub fn new(base_f0: f64, sr: f64) -> Self {
        Self {
            sample_rate: sr,
            organic_noise: OrganicNoiseGenerator::new(sr),
            formant_modulator: FormantModulator::new(sr),
            coarticulator: Coarticulator::new(),
            pitch_modulator: PitchModulator::new(base_f0, sr),
        }
    }

    pub fn synthesize_humanized(&mut self, text: &str, emotion: &str, params: &GlottalParams) -> Vec<f64> {
        let articulator = SpanishArticulator::new();
        let seq = articulator.text_to_phonemes(text);
        let contour = self.pitch_modulator.generate_pitch_contour(text, seq.phonemes.len(), emotion);
        let f_table = Self::get_spanish_formant_table();
        self.coarticulator = Coarticulator::new();

        for (idx, p) in seq.phonemes.iter().enumerate() {
            let dur = (p.duration_ms as f64 / 1000.0 * self.sample_rate) as usize;
            let fmts = f_table.get(p.phoneme.as_str()).cloned().unwrap_or(vec![(500.0, 100.0), (1500.0, 150.0), (2500.0, 250.0), (3500.0, 350.0)]);
            let pitch = contour.get(idx).copied().unwrap_or(120.0);
            
            self.coarticulator.add_phoneme(CoarticulatedPhoneme {
                phoneme: p.phoneme.clone(),
                target_formants: fmts,
                duration_samples: dur,
                is_voiced: p.is_voiced,
                energy: self.calculate_energy(&p.phoneme, p.stress, emotion),
                pitch_target: pitch,
            });
        }

        let mut glottis = OrganicGlottis::new(contour[0], self.sample_rate);
        let mut tract = VocalTract::new(self.sample_rate);
        let audio = self.coarticulator.generate_coarticulated(&mut glottis, &mut self.formant_modulator, &mut tract, params);
        self.final_humanize(&audio)
    }

    fn get_spanish_formant_table() -> std::collections::HashMap<&'static str, Vec<(f64, f64)>> {
        let mut t = std::collections::HashMap::new();
        t.insert("a", vec![(730.0, 100.0), (1090.0, 150.0), (2440.0, 250.0), (3400.0, 350.0)]);
        t.insert("e", vec![(530.0, 80.0), (1840.0, 150.0), (2480.0, 250.0), (3400.0, 350.0)]);
        t.insert("i", vec![(270.0, 80.0), (2290.0, 150.0), (3010.0, 250.0), (3400.0, 350.0)]);
        t.insert("o", vec![(570.0, 100.0), (840.0, 150.0), (2410.0, 250.0), (3400.0, 350.0)]);
        t.insert("u", vec![(300.0, 100.0), (870.0, 150.0), (2240.0, 250.0), (3400.0, 350.0)]);
        t
    }

    fn calculate_energy(&self, ph: &str, stress: f64, _em: &str) -> f64 {
        let base = match ph { 
            "a" | "o" => 0.8, "e" | "u" | "i" => 0.7, 
            "s" => 0.3, "_" => 0.0, _ => 0.5 
        };
        base * (0.8 + stress * 0.4)
    }

    fn final_humanize(&self, audio: &[f64]) -> Vec<f64> {
        let mut output = audio.to_vec();
        // Normalización y soft-clip simple
        let peak = output.iter().map(|s| s.abs()).fold(0.0_f64, f64::max);
        if peak > 0.0 {
            for s in &mut output {
                *s = (*s / peak) * 0.8;
                if *s > 0.5 { *s = 0.5 + (*s - 0.5) * 0.2; }
                if *s < -0.5 { *s = -0.5 + (*s + 0.5) * 0.2; }
            }
        }
        output
    }
}
