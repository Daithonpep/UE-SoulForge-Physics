use crate::voice::{VoiceGenerator, AudioStream, EmotionContext};
use crate::larynx_engine::vocal_cortex::synthesis_pipeline::SynthesisPipeline;

pub struct VoxisPhysicalService {
    pipeline: SynthesisPipeline,
    sample_rate: f64,
}

impl VoxisPhysicalService {
    pub fn new(sample_rate: f64) -> Self {
        Self {
            pipeline: SynthesisPipeline::new(sample_rate, 120.0),
            sample_rate,
        }
    }
}

impl VoiceGenerator for VoxisPhysicalService {
    fn generate_speech_stream(&mut self, text: String, _emotion: EmotionContext) -> Vec<AudioStream> {
        // El pipeline ahora maneja la fonética real
        let samples = self.pipeline.synthesize(&text);

        vec![AudioStream {
            samples: samples.into_iter().map(|s| s as f32).collect(),
            sample_rate: self.sample_rate as u32,
            is_final: true,
        }]
    }
}

