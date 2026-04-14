use super::physical_tract::TractShape;

pub struct ArticulationController {
    sample_rate: f64,
    current_shape: TractShape,
    target_shape: TractShape,
    smoothing_rate: f64,
    pub muscle_library: std::collections::HashMap<String, crate::larynx_engine::vocal_cortex::training::MuscleGenome>,
}

#[derive(Debug, Clone)]
pub struct ArticulationTarget {
    pub phoneme: String,
    pub shape: TractShape,
    pub source_params: Option<crate::larynx_engine::vocal_cortex::source_model::SourceParams>,
    pub duration_samples: usize,
    pub is_voiced: bool,
    pub pitch: f64,
    #[allow(dead_code)]
    pub energy: f64,
}

impl ArticulationController {
    pub fn new(sample_rate: f64) -> Self {
        Self {
            sample_rate,
            current_shape: TractShape::default(),
            target_shape: TractShape::default(),
            smoothing_rate: 0.008,
            muscle_library: std::collections::HashMap::new(),
        }
    }

    pub fn set_target(&mut self, target: &ArticulationTarget) {
        self.target_shape = target.shape.clone();
    }

    pub fn get_current_shape(&mut self) -> TractShape {
        let r = self.smoothing_rate;
        self.current_shape.jaw_opening += (self.target_shape.jaw_opening - self.current_shape.jaw_opening) * r;
        self.current_shape.tongue_body_height += (self.target_shape.tongue_body_height - self.current_shape.tongue_body_height) * r;
        self.current_shape.tongue_body_position += (self.target_shape.tongue_body_position - self.current_shape.tongue_body_position) * r;
        self.current_shape.tongue_tip_height += (self.target_shape.tongue_tip_height - self.current_shape.tongue_tip_height) * r;
        self.current_shape.tongue_root += (self.target_shape.tongue_root - self.current_shape.tongue_root) * r;
        self.current_shape.lip_opening += (self.target_shape.lip_opening - self.current_shape.lip_opening) * r;
        self.current_shape.lip_rounding += (self.target_shape.lip_rounding - self.current_shape.lip_rounding) * r;
        self.current_shape.velum_opening += (self.target_shape.velum_opening - self.current_shape.velum_opening) * r * 0.5;
        self.current_shape.clone()
    }

    pub fn text_to_targets(&self, text: &str, emotion: &str) -> Vec<ArticulationTarget> {
        let mut targets = Vec::new();
        let base_f0 = 120.0;
        let chars: Vec<char> = text.to_lowercase().chars().collect();
        for (i, &c) in chars.iter().enumerate() {
            let progress = i as f64 / chars.len().max(1) as f64;
            let (shape, voiced, ms, source) = self.char_to_articulation(c);
            let p = self.calculate_pitch(base_f0, progress, text, emotion);
            targets.push(ArticulationTarget {
                phoneme: c.to_string(),
                shape,
                source_params: source,
                duration_samples: (ms / 1000.0 * self.sample_rate) as usize,
                is_voiced: voiced,
                pitch: p,
                energy: self.calculate_energy(c, emotion),
            });
        }
        targets
    }

    fn char_to_articulation(&self, c: char) -> (TractShape, bool, f64, Option<crate::larynx_engine::vocal_cortex::source_model::SourceParams>) {
        if let Some(genome) = self.muscle_library.get(&c.to_string()) {
            return (genome.shape.clone(), true, 85.0, Some(genome.source.clone()));
        }

        match c {
            'a' | 'á' => (TractShape { jaw_opening: 0.85, tongue_body_height: 0.2, tongue_body_position: 0.5, tongue_tip_height: 0.1, tongue_root: 0.4, lip_opening: 0.8, lip_rounding: 0.0, velum_opening: 0.0, constriction_position: 0.5, constriction_degree: 0.0 }, true, 85.0, None),
            'e' | 'é' => (TractShape { jaw_opening: 0.6, tongue_body_height: 0.55, tongue_body_position: 0.65, tongue_tip_height: 0.3, tongue_root: 0.5, lip_opening: 0.65, lip_rounding: 0.0, velum_opening: 0.0, constriction_position: 0.5, constriction_degree: 0.0 }, true, 80.0, None),
            'i' | 'í' => (TractShape { jaw_opening: 0.3, tongue_body_height: 0.85, tongue_body_position: 0.8, tongue_tip_height: 0.5, tongue_root: 0.6, lip_opening: 0.4, lip_rounding: 0.0, velum_opening: 0.0, constriction_position: 0.5, constriction_degree: 0.0 }, true, 75.0, None),
            'o' | 'ó' => (TractShape { jaw_opening: 0.6, tongue_body_height: 0.4, tongue_body_position: 0.3, tongue_tip_height: 0.2, tongue_root: 0.3, lip_opening: 0.5, lip_rounding: 0.7, velum_opening: 0.0, constriction_position: 0.5, constriction_degree: 0.0 }, true, 80.0, None),
            'u' | 'ú' => (TractShape { jaw_opening: 0.35, tongue_body_height: 0.7, tongue_body_position: 0.2, tongue_tip_height: 0.2, tongue_root: 0.3, lip_opening: 0.3, lip_rounding: 0.9, velum_opening: 0.0, constriction_position: 0.5, constriction_degree: 0.0 }, true, 75.0, None),
            'm' => (TractShape { jaw_opening: 0.1, tongue_body_height: 0.5, tongue_body_position: 0.5, tongue_tip_height: 0.3, tongue_root: 0.5, lip_opening: 0.0, lip_rounding: 0.0, velum_opening: 0.8, constriction_position: 0.5, constriction_degree: 0.0 }, true, 70.0, None),
            'n' => (TractShape { jaw_opening: 0.2, tongue_body_height: 0.6, tongue_body_position: 0.7, tongue_tip_height: 0.9, tongue_root: 0.5, lip_opening: 0.3, lip_rounding: 0.0, velum_opening: 0.8, constriction_position: 0.75, constriction_degree: 0.9 }, true, 65.0, None),
            's' | 'z' => (TractShape { jaw_opening: 0.25, tongue_body_height: 0.7, tongue_body_position: 0.75, tongue_tip_height: 0.8, tongue_root: 0.5, lip_opening: 0.3, lip_rounding: 0.0, velum_opening: 0.0, constriction_position: 0.8, constriction_degree: 0.85 }, false, 90.0, None),
            ' ' => (TractShape::default(), false, 100.0, None),
            _ => (TractShape::default(), false, 50.0, None),
        }
    }

    fn calculate_pitch(&self, base_f0: f64, progress: f64, text: &str, emotion: &str) -> f64 {
        let q = text.contains('?');
        let mut p = if q && progress > 0.7 { base_f0 * (1.0 + (progress - 0.7) / 0.3 * 0.35) } else { base_f0 * (1.05 - progress * 0.12) };
        match emotion {
            "cold_disappointment" => p *= 0.85,
            "experimental_excited" => p *= 1.2,
            "xeno_sarcasm" => p *= 1.0 + ((progress * 4.0).sin() * 0.06).abs(),
            _ => {}
        }
        p
    }

    fn calculate_energy(&self, c: char, emotion: &str) -> f64 {
        let b = match c { 'a' | 'o' => 1.0, 'e' | 'u' | 'i' => 0.85, 'm' | 'n' => 0.6, 's' => 0.4, _ => 0.5 };
        match emotion { "cold_disappointment" => b * 0.7, "experimental_excited" => b * 1.3, _ => b }
    }
}
