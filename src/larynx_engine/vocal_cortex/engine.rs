use super::physical_tract::PhysicalVocalTract;
use super::source_model::ComplexGlottalSource;
use super::radiation::LipRadiationModel;
use super::articulation::ArticulationController;

pub struct VocalCortexEngine {
    pub sample_rate: f64,
    pub tract: PhysicalVocalTract,
    pub source: ComplexGlottalSource,
    pub radiation: LipRadiationModel,
    pub articulation: ArticulationController,
}

impl VocalCortexEngine {
    pub fn new(sample_rate: f64, base_f0: f64) -> Self {
        Self {
            sample_rate,
            tract: PhysicalVocalTract::new(sample_rate),
            source: ComplexGlottalSource::new(base_f0, sample_rate),
            radiation: LipRadiationModel::new(sample_rate),
            articulation: ArticulationController::new(sample_rate),
        }
    }

    pub fn synthesize(&mut self, text: &str, emotion: &str) -> Vec<f64> {
        let targets = self.articulation.text_to_targets(text, emotion);
        let total_samples = targets.iter().map(|t| t.duration_samples).sum::<usize>();
        let mut output = Vec::with_capacity(total_samples);

        for target in &targets {
            self.articulation.set_target(target);
            self.source.set_f0(target.pitch);
            if let Some(params) = &target.source_params {
                self.source.params = params.clone();
            }
            
            for _ in 0..target.duration_samples {
                let shape = self.articulation.get_current_shape();
                self.tract.set_shape(&shape);
                let glottal = self.source.next_sample(&shape);
                let tract_output = self.tract.propagate(glottal);
                let radiated = self.radiation.process(tract_output);
                output.push(radiated);
            }
        }
        self.normalize(&mut output);
        output
    }

    fn normalize(&self, audio: &mut Vec<f64>) {
        if audio.is_empty() { return; }
        let dc = audio.iter().sum::<f64>() / audio.len() as f64;
        let peak = audio.iter().map(|s| (s - dc).abs()).fold(0.0_f64, f64::max).max(1e-10);
        for s in audio.iter_mut() {
            *s = (*s - dc) / peak * 0.85;
        }
    }
}
