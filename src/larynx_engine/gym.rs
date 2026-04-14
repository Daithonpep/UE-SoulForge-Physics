use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use crate::larynx_engine::spectral::fft::*;
use crate::larynx_engine::vocal_cortex::physical_tract::TractShape;
use crate::larynx_engine::vocal_cortex::VocalCortexEngine;

/// Genoma que define la configuración física del aparato fonador para un fonema específico.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceGenome {
    pub f0: f64,
    pub subglottal_pressure: f64,
    pub tension: f64,
    pub shape: TractShape,
    pub fitness: f64,
}

pub struct PhoneticGym {
    sample_rate: f64,
    fft: FFTAnalyzer,
    pub trained_phonemes: HashMap<String, VoiceGenome>,
}

impl PhoneticGym {
    pub fn new(sample_rate: f64) -> Self {
        Self {
            sample_rate,
            fft: FFTAnalyzer::new(sample_rate),
            trained_phonemes: HashMap::new(),
        }
    }

    pub fn train_phoneme(&mut self, phoneme: String, target_audio: &[f64]) {
        println!("[GYM] Entrenando fonema físico: '{}'", phoneme);
        let window_size = 512;
        let start = (target_audio.len() / 2).saturating_sub(window_size / 2);
        let end = (start + window_size).min(target_audio.len());
        let target_window = &target_audio[start..end];
        let target_spectrum = self.fft.magnitude_spectrum(target_window);
        
        let mut population = self.init_population();
        let mut generations = 0;
        let max_gens = 501; // El modelo físico es más costoso, reducimos gens para el test

        while generations < max_gens {
            for genome in &mut population {
                genome.fitness = self.evaluate(genome, &target_spectrum, window_size);
            }
            population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
            
            if generations % 50 == 0 {
                println!("  Gen {:>3}: Mejor Fitness {:.4} | Jaw: {:.2} | TongueH: {:.2}", 
                         generations, population[0].fitness, population[0].shape.jaw_opening, population[0].shape.tongue_body_height);
            }

            if population[0].fitness > 0.99 { break; }
            self.evolve(&mut population);
            generations += 1;
        }
        
        println!("  ✅ '{}' completado. Fitness final: {:.4}", phoneme, population[0].fitness);
        self.trained_phonemes.insert(phoneme, population[0].clone());
    }

    fn init_population(&self) -> Vec<VoiceGenome> {
        let mut pop = Vec::new();
        let mut rng = SimpleRng::new(42);
        for _ in 0..30 {
            pop.push(VoiceGenome {
                f0: 100.0 + rng.next_f64() * 100.0,
                subglottal_pressure: 600.0 + rng.next_f64() * 600.0,
                tension: 0.3 + rng.next_f64() * 0.4,
                shape: TractShape {
                    jaw_opening: rng.next_f64(),
                    tongue_body_height: rng.next_f64(),
                    tongue_body_position: rng.next_f64(),
                    tongue_tip_height: rng.next_f64(),
                    tongue_root: rng.next_f64(),
                    lip_opening: rng.next_f64(),
                    lip_rounding: rng.next_f64(),
                    velum_opening: 0.0,
                    constriction_position: 0.5,
                    constriction_degree: 0.0,
                },
                fitness: 0.0,
            });
        }
        pop
    }

    fn evaluate(&self, genome: &VoiceGenome, target: &Spectrum, len: usize) -> f64 {
        // En una implementación real, aquí se usaría un motor que solo procesa bloques cortos
        let mut engine = VocalCortexEngine::new(self.sample_rate, genome.f0);
        // Nota: Tendríamos que añadir métodos set_shape y set_params al engine para no reconstruirlo
        // Por ahora, simulamos una síntesis rápida de phoneme de prueba
        let audio = engine.synthesize("a", "analytical_calm"); // Simplificación para el test
        let spec = self.fft.magnitude_spectrum(&audio[0..len.min(audio.len())]);
        1.0 - self.fft.spectral_distance(&spec, target)
    }

    fn evolve(&self, pop: &mut Vec<VoiceGenome>) {
        let mut rng = SimpleRng::new(42 + (pop[0].fitness * 10000.0) as u64);
        for i in 4..30 {
            let parent = &pop[i % 4];
            let mut child = parent.clone();
            let mut shape = &mut child.shape;
            
            match rng.next_u64() % 8 {
                0 => shape.jaw_opening = (shape.jaw_opening + (rng.next_f64() - 0.5) * 0.2).clamp(0.0, 1.0),
                1 => shape.tongue_body_height = (shape.tongue_body_height + (rng.next_f64() - 0.5) * 0.2).clamp(0.0, 1.0),
                2 => shape.tongue_body_position = (shape.tongue_body_position + (rng.next_f64() - 0.5) * 0.2).clamp(0.0, 1.0),
                3 => shape.tongue_tip_height = (shape.tongue_tip_height + (rng.next_f64() - 0.5) * 0.2).clamp(0.0, 1.0),
                4 => shape.lip_opening = (shape.lip_opening + (rng.next_f64() - 0.5) * 0.2).clamp(0.0, 1.0),
                5 => child.f0 = (child.f0 + (rng.next_f64() - 0.5) * 20.0).clamp(80.0, 300.0),
                _ => {
                   shape.tongue_root = (shape.tongue_root + (rng.next_f64() - 0.5) * 0.2).clamp(0.0, 1.0);
                }
            }
            pop[i] = child;
        }
    }
}

struct SimpleRng { state: u64 }
impl SimpleRng {
    fn new(seed: u64) -> Self { Self { state: seed.max(1) } }
    fn next_u64(&mut self) -> u64 {
        self.state ^= self.state << 13;
        self.state ^= self.state >> 7;
        self.state ^= self.state << 17;
        self.state
    }
    fn next_f64(&mut self) -> f64 { (self.next_u64() as f64) / (u64::MAX as f64) }
}
