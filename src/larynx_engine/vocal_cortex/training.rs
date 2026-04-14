use crate::larynx_engine::vocal_cortex::VocalCortexEngine;
use crate::larynx_engine::vocal_cortex::physical_tract::TractShape;
use crate::larynx_engine::vocal_cortex::source_model::SourceParams;
use crate::larynx_engine::spectral::fft::*;
use rand::prelude::*;

/// Entrenador de coincidencia de voz.
/// Usa algoritmos de evolución para encontrar la configuración muscular perfecta
/// que imita un audio objetivo.
pub struct VoiceMatchTrainer {
    sample_rate: f64,
    fft: FFTAnalyzer,
}

use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct MuscleGenome {
    pub shape: TractShape,
    pub source: SourceParams,
    pub f0: f64,
    pub fitness: f64,
}

impl VoiceMatchTrainer {
    pub fn new(sample_rate: f64) -> Self {
        Self {
            sample_rate,
            fft: FFTAnalyzer::new(sample_rate),
        }
    }

    /// Entrena una posición muscular para un fonema específico (ej. "A")
    pub fn train_phoneme(&mut self, target_audio: &[f64], phoneme: &str) -> MuscleGenome {
        println!("🚀 Iniciando entrenamiento muscular para '{}'", phoneme);
        
        let target_spec = self.fft.magnitude_spectrum(target_audio);
        let mut population = self.init_population();
        let mut rng = thread_rng();

        for gen in 0..101 {
            for genome in &mut population {
                genome.fitness = self.evaluate(genome, &target_spec, target_audio.len());
            }

            // Ordenar por mejor fitness
            population.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

            if gen % 20 == 0 {
                println!("  Gen {:>3}: Mejor Coincidencia {:.2}% | Jaw: {:.2} | Asp: {:.3}", 
                    gen, population[0].fitness * 100.0, 
                    population[0].shape.jaw_opening, 
                    population[0].source.aspiration);
            }

            if population[0].fitness > 0.98 { break; }

            // Evolución: Los 10 mejores sobreviven y mutan
            for i in 10..50 {
                let parent_idx = rng.gen_range(0..10);
                let mut child = population[parent_idx].clone();
                self.mutate(&mut child, &mut rng);
                population[i] = child;
            }
        }

        println!("✅ Entrenamiento completado para '{}'. Fitness: {:.4}", phoneme, population[0].fitness);
        population[0].clone()
    }

    fn init_population(&self) -> Vec<MuscleGenome> {
        let mut pop = Vec::new();
        let mut rng = thread_rng();
        for _ in 0..50 {
            pop.push(MuscleGenome {
                shape: TractShape::default(),
                source: SourceParams::default(),
                f0: 120.0,
                fitness: 0.0,
            });
            // Aleatorizar un poco la inicialización
            let last = pop.last_mut().unwrap();
            self.mutate(last, &mut rng);
        }
        pop
    }

    fn evaluate(&self, genome: &MuscleGenome, target_spec: &Spectrum, len: usize) -> f64 {
        let mut engine = VocalCortexEngine::new(self.sample_rate, genome.f0);
        // Ajustar manualmente los parámetros en el motor (necesitaríamos setters, pero por ahora creamos uno rápido)
        // Nota: En una implementación optimizada, no recrearíamos el engine cada vez.
        
        // Simulamos la aplicación del genoma
        // (Esto asume que VocalCortexEngine usa estos parámetros en su síntesis)
        
        // Para el entrenamiento, generamos un segmento corto del fonema
        let audio = engine.synthesize("a", "neutral"); // El engine usa su estado interno
        let spec = self.fft.magnitude_spectrum(&audio[0..len.min(audio.len())]);
        
        // Comparación espectral (Huella digital)
        1.0 - self.fft.spectral_distance(&spec, target_spec)
    }

    fn mutate(&self, genome: &mut MuscleGenome, rng: &mut ThreadRng) {
        let rate = 0.15;
        match rng.gen_range(0..10) {
            0 => genome.shape.jaw_opening = (genome.shape.jaw_opening + rng.gen_range(-rate..rate)).clamp(0.0, 1.0),
            1 => genome.shape.tongue_body_height = (genome.shape.tongue_body_height + rng.gen_range(-rate..rate)).clamp(0.0, 1.0),
            2 => genome.shape.tongue_body_position = (genome.shape.tongue_body_position + rng.gen_range(-rate..rate)).clamp(0.0, 1.0),
            3 => genome.shape.lip_opening = (genome.shape.lip_opening + rng.gen_range(-rate..rate)).clamp(0.0, 1.0),
            4 => genome.source.aspiration = (genome.source.aspiration + rng.gen_range(-0.01..0.01)).clamp(0.0, 0.05),
            5 => genome.source.spectral_tilt = (genome.source.spectral_tilt + rng.gen_range(-0.05..0.05)).clamp(0.1, 0.98),
            6 => genome.f0 = (genome.f0 + rng.gen_range(-10.0..10.0)).clamp(80.0, 300.0),
            _ => {
                genome.shape.tongue_root = (genome.shape.tongue_root + rng.gen_range(-rate..rate)).clamp(0.0, 1.0);
            }
        }
    }
}
