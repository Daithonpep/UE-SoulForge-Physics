use super::phonetic_database::*;
use super::physical_tract::*;
use super::source_model::*;
use super::radiation::*;

pub struct SynthesisPipeline {
    pub db: SpanishPhoneticDatabase,
    tract: PhysicalVocalTract,
    source: ComplexGlottalSource,
    radiation: LipRadiationModel,
    sample_rate: f64,
}

impl SynthesisPipeline {
    pub fn new(sample_rate: f64, base_f0: f64) -> Self {
        Self {
            db: SpanishPhoneticDatabase::new(),
            tract: PhysicalVocalTract::new(sample_rate),
            source: ComplexGlottalSource::new(base_f0, sample_rate),
            radiation: LipRadiationModel::new(sample_rate),
            sample_rate,
        }
    }

    pub fn synthesize(&mut self, text: &str) -> Vec<f64> {
        let phonemes = self.text_to_phonemes(text);
        let mut audio = Vec::new();

        for i in 0..phonemes.len() {
            let symbol = &phonemes[i];
            let prev = if i > 0 { Some(phonemes[i-1].as_str()) } else { None };
            let next = if i + 1 < phonemes.len() { Some(phonemes[i+1].as_str()) } else { None };

            if let Some(data) = self.db.get_phoneme(symbol) {
                // Cálculo de duración real basada en prosodia
                let duration_ms = data.duration_mean_ms; 
                let duration_samples = (duration_ms / 1000.0 * self.sample_rate) as usize;

                for s_idx in 0..duration_samples {
                    let progress = s_idx as f64 / duration_samples as f64;
                    
                    // Articulación base
                    let art = &data.articulation;
                    let mut shape = TractShape {
                        jaw_opening: art.jaw_opening,
                        tongue_body_height: art.tongue_height,
                        tongue_body_position: art.tongue_frontness,
                        tongue_tip_height: art.tongue_tip,
                        tongue_root: 0.5,
                        lip_opening: art.lip_opening,
                        lip_rounding: art.lip_rounding,
                        velum_opening: art.velum,
                        constriction_position: 0.5,
                        constriction_degree: 0.0,
                    };

                    self.tract.set_shape(&shape);
                    let glottal = if data.is_voiced {
                        self.source.next_sample(&shape)
                    } else {
                        (fastrand::f64() - 0.5) * 0.1 * data.relative_intensity
                    };

                    let tract_out = self.tract.propagate(glottal);
                    let radiated = self.radiation.process(tract_out);
                    
                    // Envolvente de fonema (evitar pops)
                    let env = self.calc_envelope(s_idx, duration_samples);
                    audio.push(radiated * data.relative_intensity * env);
                }
            }
        }
        self.normalize(&mut audio);
        audio
    }

    fn calc_envelope(&self, i: usize, total: usize) -> f64 {
        let edge = (total as f64 * 0.1) as usize;
        if i < edge { i as f64 / edge as f64 }
        else if i > total - edge { (total - i) as f64 / edge as f64 }
        else { 1.0 }
    }

    fn text_to_phonemes(&self, text: &str) -> Vec<String> {
        text.to_lowercase().chars().filter_map(|c| {
            match c {
                'a'|'á' => Some("a".into()),
                'e'|'é' => Some("e".into()),
                'i'|'í' => Some("i".into()),
                'o'|'ó' => Some("o".into()),
                'u'|'ú' => Some("u".into()),
                'm' => Some("m".into()),
                ' ' => Some("_pause".into()),
                _ => None
            }
        }).collect()
    }

    fn normalize(&self, audio: &mut Vec<f64>) {
        let peak = audio.iter().map(|s| s.abs()).fold(0.0_f64, f64::max).max(1e-10);
        for s in audio { *s = *s / peak * 0.8; }
    }
}
