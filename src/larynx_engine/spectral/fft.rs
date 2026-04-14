use std::f64::consts::PI;

/// FFT implementada en Rust puro (sin dependencias externas)
pub struct FFTAnalyzer {
    pub sample_rate: f64,
}

/// Resultado del análisis espectral
#[derive(Debug, Clone)]
pub struct Spectrum {
    pub magnitudes: Vec<f64>,
    pub frequencies: Vec<f64>,
    pub fundamental_freq: f64,
    pub formants: Vec<DetectedFormant>,
    pub total_energy: f64,
}

#[derive(Debug, Clone)]
pub struct DetectedFormant {
    pub frequency: f64,
    pub amplitude: f64,
    pub bandwidth: f64,
}

impl FFTAnalyzer {
    pub fn new(sample_rate: f64) -> Self {
        Self { sample_rate }
    }

    pub fn fft(&self, signal: &[f64]) -> Vec<(f64, f64)> {
        let n = signal.len();
        let fft_size = n.next_power_of_two();
        let mut real = vec![0.0; fft_size];
        let mut imag = vec![0.0; fft_size];

        for i in 0..n {
            let window = 0.5 * (1.0 - (2.0 * PI * i as f64 / (n - 1) as f64).cos());
            real[i] = signal[i] * window;
        }

        let mut j = 0;
        for i in 0..fft_size {
            if i < j {
                real.swap(i, j);
                imag.swap(i, j);
            }
            let mut m = fft_size / 2;
            while m >= 1 && j >= m {
                j -= m;
                m /= 2;
            }
            j += m;
        }

        let mut step = 1;
        while step < fft_size {
            let half_step = step;
            step *= 2;
            let angle_step = -PI / half_step as f64;
            for k in (0..fft_size).step_by(step) {
                let mut angle: f64 = 0.0;
                for j in 0..half_step {
                    let cos_a = angle.cos();
                    let sin_a = angle.sin();
                    let idx1 = k + j;
                    let idx2 = k + j + half_step;
                    let tr = real[idx2] * cos_a - imag[idx2] * sin_a;
                    let ti = real[idx2] * sin_a + imag[idx2] * cos_a;
                    real[idx2] = real[idx1] - tr;
                    imag[idx2] = imag[idx1] - ti;
                    real[idx1] += tr;
                    imag[idx1] += ti;
                    angle += angle_step;
                }
            }
        }
        real.into_iter().zip(imag.into_iter()).collect()
    }

    pub fn magnitude_spectrum(&self, signal: &[f64]) -> Spectrum {
        let fft_result = self.fft(signal);
        let fft_size = fft_result.len();
        let half = fft_size / 2;
        let mut magnitudes = Vec::with_capacity(half);
        let mut frequencies = Vec::with_capacity(half);

        for i in 0..half {
            let (re, im) = fft_result[i];
            let mag = (re * re + im * im).sqrt() / half as f64;
            let freq = i as f64 * self.sample_rate / fft_size as f64;
            magnitudes.push(mag);
            frequencies.push(freq);
        }

        let fundamental = self.detect_fundamental(&magnitudes, &frequencies);
        let formants = self.detect_formants_from_spectrum(&magnitudes, &frequencies);
        let total_energy = magnitudes.iter().map(|m| m * m).sum::<f64>().sqrt();

        Spectrum { magnitudes, frequencies, fundamental_freq: fundamental, formants, total_energy }
    }

    fn detect_fundamental(&self, magnitudes: &[f64], frequencies: &[f64]) -> f64 {
        let min_idx = (50.0 / (self.sample_rate / magnitudes.len() as f64 / 2.0)) as usize;
        let max_idx = (500.0 / (self.sample_rate / magnitudes.len() as f64 / 2.0)) as usize;
        let max_idx = max_idx.min(magnitudes.len() - 1);
        let min_idx = min_idx.max(1);
        let mut best_idx = min_idx;
        let mut best_mag = 0.0;
        for i in min_idx..=max_idx {
            if magnitudes[i] > best_mag {
                best_mag = magnitudes[i];
                best_idx = i;
            }
        }
        frequencies[best_idx]
    }

    fn detect_formants_from_spectrum(&self, magnitudes: &[f64], frequencies: &[f64]) -> Vec<DetectedFormant> {
        let mut formants = Vec::new();
        let smoothed = self.smooth_spectrum(magnitudes, 5);
        for i in 2..smoothed.len() - 2 {
            if frequencies[i] < 100.0 || frequencies[i] > 5000.0 { continue; }
            let is_peak = smoothed[i] > smoothed[i - 1] && smoothed[i] > smoothed[i + 1] && smoothed[i] > smoothed[i - 2] && smoothed[i] > smoothed[i + 2];
            let is_significant = smoothed[i] > smoothed.iter().take(smoothed.len() / 2).sum::<f64>() / (smoothed.len() / 2) as f64 * 1.5;
            if is_peak && is_significant {
                let bandwidth = self.estimate_bandwidth(magnitudes, frequencies, i);
                formants.push(DetectedFormant { frequency: frequencies[i], amplitude: magnitudes[i], bandwidth });
            }
        }
        formants.sort_by(|a, b| a.frequency.partial_cmp(&b.frequency).unwrap());
        formants.truncate(4);
        formants
    }

    fn smooth_spectrum(&self, data: &[f64], window: usize) -> Vec<f64> {
        let mut smoothed = vec![0.0; data.len()];
        for i in 0..data.len() {
            let start = i.saturating_sub(window);
            let end = (i + window + 1).min(data.len());
            smoothed[i] = data[start..end].iter().sum::<f64>() / (end - start) as f64;
        }
        smoothed
    }

    fn estimate_bandwidth(&self, magnitudes: &[f64], frequencies: &[f64], peak_idx: usize) -> f64 {
        let peak_mag = magnitudes[peak_idx];
        let threshold = peak_mag * 0.707;
        let mut left = peak_idx;
        while left > 0 && magnitudes[left] > threshold { left -= 1; }
        let mut right = peak_idx;
        while right < magnitudes.len() - 1 && magnitudes[right] > threshold { right += 1; }
        frequencies[right] - frequencies[left]
    }

    pub fn spectral_distance(&self, spectrum_a: &Spectrum, spectrum_b: &Spectrum) -> f64 {
        let formant_distance = self.formant_distance(&spectrum_a.formants, &spectrum_b.formants);
        let envelope_distance = self.envelope_distance(&spectrum_a.magnitudes, &spectrum_b.magnitudes);
        let total = formant_distance * 0.6 + envelope_distance * 0.4;
        total.clamp(0.0, 1.0)
    }

    fn formant_distance(&self, fa: &[DetectedFormant], fb: &[DetectedFormant]) -> f64 {
        if fa.is_empty() || fb.is_empty() { return 1.0; }
        let pairs = fa.len().min(fb.len());
        let mut total = 0.0;
        for i in 0..pairs {
            let diff = (fa[i].frequency - fb[i].frequency).abs();
            total += diff / fa[i].frequency.max(fb[i].frequency);
        }
        (total / pairs as f64).min(1.0)
    }

    fn envelope_distance(&self, ma: &[f64], mb: &[f64]) -> f64 {
        let len = ma.len().min(mb.len());
        if len == 0 { return 1.0; }
        let mut sum = 0.0;
        for i in 0..len {
            sum += (ma[i] - mb[i]).powi(2);
        }
        (sum / len as f64).sqrt().min(1.0)
    }
}
