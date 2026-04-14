pub struct VocalTract {
    pub formants: Vec<Formant>,
    sample_rate: f64,
    filters: Vec<BiquadFilter>,
}

#[derive(Debug, Clone)]
pub struct Formant {
    pub frequency: f64,
    pub bandwidth: f64,
    pub amplitude: f64,
}

#[derive(Debug, Clone)]
struct BiquadFilter {
    // Coeficientes del filtro (Fórmula estándar de Robert Bristow-Johnson)
    a1: f64,
    a2: f64,
    b0: f64,
    b1: f64,
    b2: f64,
    // Estado interno (Buffer de muestras anteriores)
    x1: f64,
    x2: f64,
    y1: f64,
    y2: f64,
}

impl BiquadFilter {
    fn new_basic() -> Self {
        Self { a1: 0.0, a2: 0.0, b0: 1.0, b1: 0.0, b2: 0.0, x1: 0.0, x2: 0.0, y1: 0.0, y2: 0.0 }
    }

    /// Recalcula coeficientes para un filtro de pico (Band-Pass)
    fn update(&mut self, freq: f64, bw: f64, sr: f64) {
        // Salvaguarda para evitar frecuencias inválidas
        let freq = freq.clamp(20.0, sr / 2.1);
        let bw = bw.clamp(10.0, freq * 2.0);

        let omega = 2.0 * std::f64::consts::PI * freq / sr;
        let alpha = omega.sin() * (bw / freq) / 2.0;

        // Coeficientes para un Band-Pass (pico de formante)
        // Usamos la fórmula de RBJ para un filtro con pico constante
        let a0 = 1.0 + alpha;
        self.b0 = alpha / a0;
        self.b1 = 0.0;
        self.b2 = -alpha / a0;
        self.a1 = -2.0 * omega.cos() / a0;
        self.a2 = (1.0 - alpha) / a0;
    }

    /// Filtro pasa-bajos para diagnóstico (RBJ LPF)
    fn update_lowpass(&mut self, freq: f64, q: f64, sr: f64) {
        let omega = 2.0 * std::f64::consts::PI * freq / sr;
        let alpha = omega.sin() / (2.0 * q);
        let cos_w = omega.cos();

        let b0 = (1.0 - cos_w) / 2.0;
        let b1 = 1.0 - cos_w;
        let b2 = (1.0 - cos_w) / 2.0;
        let a0 = 1.0 + alpha;

        self.b0 = b0 / a0;
        self.b1 = b1 / a0;
        self.b2 = b2 / a0;
        self.a1 = (-2.0 * cos_w) / a0;
        self.a2 = (1.0 - alpha) / a0;
    }

    #[inline(always)]
    fn process_sample(&mut self, x: f64) -> f64 {
        // Ecuación de diferencia (Direct Form I)
        let y = self.b0 * x + self.b1 * self.x1 + self.b2 * self.x2 - self.a1 * self.y1 - self.a2 * self.y2;
        
        // Actualizar estados
        self.x2 = self.x1;
        self.x1 = x;
        self.y2 = self.y1;
        self.y1 = y;

        y
    }
}

impl VocalTract {
    pub fn new(sr: f64) -> Self {
        let formants = vec![
            Formant { frequency: 700.0, bandwidth: 80.0, amplitude: 1.0 },
            Formant { frequency: 1200.0, bandwidth: 120.0, amplitude: 0.7 },
            Formant { frequency: 2500.0, bandwidth: 150.0, amplitude: 0.5 },
            Formant { frequency: 3500.0, bandwidth: 200.0, amplitude: 0.3 },
        ];
        let mut filters = Vec::new();
        // 4 para formantes + 1 para diagnóstico LPF
        for _ in 0..5 { filters.push(BiquadFilter::new_basic()); }
        
        let mut tract = Self { formants, sample_rate: sr, filters };
        tract.update_filters();
        tract
    }

    pub fn update_filters(&mut self) {
        for i in 0..self.formants.len() {
            let f = &self.formants[i];
            self.filters[i].update(f.frequency, f.bandwidth, self.sample_rate);
        }
        
        // Diagnóstico: Filtro pasa-bajos a 500Hz en el quinto filtro
        self.filters[4].update_lowpass(500.0, 0.707, self.sample_rate);
    }

    pub fn process(&mut self, glottal: &[f64]) -> Vec<f64> {
        self.update_filters();
        glottal.iter().map(|&s| self.process_sample(s)).collect()
    }

    #[inline(always)]
    pub fn process_sample(&mut self, s: f64) -> f64 {
        let mut output_sample = 0.0;
        
        // 1. Procesar formantes en paralelo
        for (i, filter) in self.filters[0..4].iter_mut().enumerate() {
            output_sample += filter.process_sample(s) * self.formants[i].amplitude;
        }
        
        // 2. SUGERENCIA DIAGNÓSTICO: Activar la línea de abajo para probar el filtro sordo de 500Hz
        // output_sample = self.filters[4].process_sample(output_sample);
        
        output_sample.clamp(-0.9, 0.9)
    }

    pub fn set_phoneme(&mut self, ph: &str) {
        let targets = match ph {
            "a" => vec![700.0, 1200.0, 2500.0, 3500.0],
            "e" => vec![500.0, 1800.0, 2500.0, 3500.0],
            "i" => vec![300.0, 2300.0, 3000.0, 3500.0],
            "o" => vec![500.0, 800.0, 2400.0, 3400.0],
            "u" => vec![300.0, 600.0, 2300.0, 3300.0],
            _ => vec![500.0, 1500.0, 2500.0, 3500.0],
        };

        for (i, &freq) in targets.iter().enumerate() {
            if i < self.formants.len() {
                self.formants[i].frequency = freq;
            }
        }
        self.update_filters();
    }
}
