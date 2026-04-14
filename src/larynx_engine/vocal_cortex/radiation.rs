pub struct LipRadiationModel {
    prev_sample: f64,
    radiation_coefficient: f64,
}

impl LipRadiationModel {
    pub fn new(_sample_rate: f64) -> Self {
        Self {
            prev_sample: 0.0,
            radiation_coefficient: 0.88, // Bajado de 0.96 para suavizar estridencias
        }
    }

    pub fn process(&mut self, input: f64) -> f64 {
        let output = input - self.prev_sample * self.radiation_coefficient;
        self.prev_sample = input;
        output
    }
}
