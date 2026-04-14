use super::predictor::{WorldModelPredictor, PredictionResult};
use super::state::{WorldState, StateTransition};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CorrectionTrigger {
    pub discrepancy_threshold: f32,
    pub predictions_since_correction: u64,
    pub max_predictions_without_correction: u64,
    stats: CorrectionStatistics,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct CorrectionStatistics {
    pub total_predictions: u64,
    pub total_corrections: u64,
    pub average_discrepancy: f32,
    pub max_discrepancy: f32,
    pub correction_rate: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscrepancyReport {
    pub visual_discrepancy: f32,
    pub fps_discrepancy: f32,
    pub draw_calls_discrepancy: f32,
    pub overall_discrepancy: f32,
    pub requires_correction: bool,
    pub reason: String,
}

impl CorrectionTrigger {
    pub fn new(threshold: f32, max_predictions: u64) -> Self {
        Self {
            discrepancy_threshold: threshold,
            predictions_since_correction: 0,
            max_predictions_without_correction: max_predictions,
            stats: CorrectionStatistics::default(),
        }
    }

    pub fn evaluate(
        &mut self,
        prediction: &PredictionResult,
        actual_state: &WorldState,
    ) -> DiscrepancyReport {
        self.predictions_since_correction += 1;
        self.stats.total_predictions += 1;

        let visual_discrepancy = self.calculate_visual_discrepancy(
            &prediction.predicted_visual_state,
            &actual_state.visual_state.feature_vector,
        );

        let fps_discrepancy = self.calculate_metric_discrepancy(
            prediction.predicted_fps,
            actual_state.performance_metrics.fps,
        );

        let draw_calls_discrepancy = self.calculate_metric_discrepancy(
            prediction.predicted_draw_calls as f32,
            actual_state.performance_metrics.draw_calls as f32,
        );

        let overall_discrepancy = 
            0.6 * visual_discrepancy +
            0.3 * fps_discrepancy +
            0.1 * draw_calls_discrepancy;

        self.stats.average_discrepancy = 
            (self.stats.average_discrepancy * (self.stats.total_predictions - 1) as f32 
             + overall_discrepancy) / self.stats.total_predictions as f32;

        if overall_discrepancy > self.stats.max_discrepancy {
            self.stats.max_discrepancy = overall_discrepancy;
        }

        let (requires_correction, reason) = self.should_correct(
            overall_discrepancy,
            prediction.confidence,
        );

        if requires_correction {
            self.stats.total_corrections += 1;
            self.stats.correction_rate = 
                self.stats.total_corrections as f32 / self.stats.total_predictions as f32;
            self.predictions_since_correction = 0;

            println!(
                "⚡ CORRECCIÓN REQUERIDA: {} (Discrepancia: {:.2}%)",
                reason,
                overall_discrepancy * 100.0
            );
        }

        DiscrepancyReport {
            visual_discrepancy,
            fps_discrepancy,
            draw_calls_discrepancy,
            overall_discrepancy,
            requires_correction,
            reason,
        }
    }

    fn calculate_visual_discrepancy(&self, predicted: &[f32], actual: &[f32]) -> f32 {
        if predicted.is_empty() || actual.is_empty() {
            return 1.0; 
        }

        let min_len = predicted.len().min(actual.len());

        let dot_product: f32 = predicted[..min_len]
            .iter()
            .zip(actual[..min_len].iter())
            .map(|(p, a)| p * a)
            .sum();

        let pred_magnitude: f32 = predicted[..min_len]
            .iter()
            .map(|p| p * p)
            .sum::<f32>()
            .sqrt();

        let actual_magnitude: f32 = actual[..min_len]
            .iter()
            .map(|a| a * a)
            .sum::<f32>()
            .sqrt();

        if pred_magnitude == 0.0 || actual_magnitude == 0.0 {
            return 1.0;
        }

        let cosine_similarity = dot_product / (pred_magnitude * actual_magnitude);

        (1.0 - cosine_similarity.clamp(-1.0, 1.0)) / 2.0
    }

    fn calculate_metric_discrepancy(&self, predicted: f32, actual: f32) -> f32 {
        if actual == 0.0 {
            return if predicted == 0.0 { 0.0 } else { 1.0 };
        }

        ((predicted - actual).abs() / actual).min(1.0)
    }

    fn should_correct(&self, discrepancy: f32, confidence: f32) -> (bool, String) {
        if discrepancy > self.discrepancy_threshold {
            return (
                true,
                format!(
                    "Discrepancia ({:.1}%) > Umbral ({:.1}%)",
                    discrepancy * 100.0,
                    self.discrepancy_threshold * 100.0
                ),
            );
        }

        if confidence < 0.3 {
            return (
                true,
                format!("Confianza baja ({:.1}%)", confidence * 100.0),
            );
        }

        if self.predictions_since_correction >= self.max_predictions_without_correction {
            return (
                true,
                format!(
                    "Máx. predicciones alcanzado ({})",
                    self.max_predictions_without_correction
                ),
            );
        }

        (false, "Sin necesidad de corrección".to_string())
    }

    pub fn reset_counter(&mut self) {
        self.predictions_since_correction = 0;
    }

    pub fn get_stats(&self) -> &CorrectionStatistics {
        &self.stats
    }

    pub fn adapt_threshold(&mut self) {
        if self.stats.correction_rate > 0.5 {
            self.discrepancy_threshold *= 1.1;
            println!(
                "📈 Umbral ajustado a {:.1}% (reduciendo correcciones)",
                self.discrepancy_threshold * 100.0
            );
        }

        if self.stats.correction_rate < 0.1 {
            self.discrepancy_threshold *= 0.9;
            println!(
                "📉 Umbral ajustado a {:.1}% (aumentando validación)",
                self.discrepancy_threshold * 100.0
            );
        }
    }
}

impl Default for CorrectionTrigger {
    fn default() -> Self {
        Self::new(0.05, 20)
    }
}
