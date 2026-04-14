use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EmotionContext {
    Analytical,
    Sarcastic,
    Frustrated,
    Excited,
    Disappointed,
    Neutral,
}

pub struct AudioStream {
    pub samples: Vec<f32>,
    pub sample_rate: u32,
    pub is_final: bool,
}

pub trait VoiceGenerator {
    fn generate_speech_stream(&mut self, text: String, emotion: EmotionContext) -> Vec<AudioStream>;
}

pub mod voxis_physical;
pub use voxis_physical::*;
