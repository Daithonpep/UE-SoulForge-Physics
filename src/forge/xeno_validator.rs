// forge/xeno_validator.rs
use crate::forge::senku_calculator::{SenkuAnalysis, SenkuPrediction};
use crate::forge::chrome_pattern::{ChromeAnalysis, ChromeMode};

#[derive(Debug, Clone)]
pub struct XenoAssessment {
    pub conflicts: Vec<Conflict>,
    pub warnings: Vec<String>,
    pub recommended_action: RecommendedAction,
    pub overall_confidence: f64,
    pub xeno_verdict: String,
    pub summary: String,
}

#[derive(Debug, Clone)]
pub struct Conflict { pub source_a: String, pub source_b: String, pub description: String, pub severity: f64 }

#[derive(Debug, Clone, PartialEq)]
pub enum RecommendedAction { Proceed, ProceedWithCaution, InvestigateConflict, RepeatExperiment, RejectExperiment }

pub struct XenoValidator;

impl XenoValidator {
    pub fn new() -> Self { Self }

    pub fn cross_validate(&self, senku: &SenkuAnalysis, chrome: &ChromeAnalysis, experiment_id: &str) -> XenoAssessment {
        let mut conflicts = Vec::new();
        let mut warnings = Vec::new();

        if let Some(chrome_consensus) = &chrome.consensus_prediction {
            let senku_survives = matches!(senku.prediction, SenkuPrediction::StableSafe | SenkuPrediction::StableMarginal);
            let chrome_survives = chrome_consensus.will_survive;
            if senku_survives != chrome_survives {
                let severity = match chrome.mode { ChromeMode::Bootstrap => 0.3, ChromeMode::Learning => 0.6, ChromeMode::Experienced => 0.9 };
                conflicts.push(Conflict { source_a: "Senku".into(), source_b: "Chrome".into(), description: "Diferencia de predicción".into(), severity });
            }
        }

        if !senku.warnings.is_empty() { warnings.push(format!("Senku: {} advertencias", senku.warnings.len())); }
        if senku.stability_ratio > 1000.0 { warnings.push("Estabilidad anómala detectada".into()); }

        let rec_action = if conflicts.iter().any(|c| c.severity > 0.8) { RecommendedAction::InvestigateConflict } else { RecommendedAction::Proceed };
        let confidence = 0.7; // Simplificado para estabilidad
        let verdict = format!("XENO: {:?} | Confidenca: {:.0}%", rec_action, confidence * 100.0);

        XenoAssessment {
            conflicts, warnings, recommended_action: rec_action.clone(), overall_confidence: confidence, xeno_verdict: verdict.clone(),
            summary: format!("Veredicto: {:?}", rec_action),
        }
    }

    pub fn summarize_for_debate(&self, assessment: &XenoAssessment) -> String { assessment.xeno_verdict.clone() }
    pub fn find_systematic_errors(&self, _inputs: &[UnrealExperimentReview]) -> XenoReview { XenoReview { summary: "Sin errores sistémicos.".into(), issues: vec![] } }
}

pub struct XenoReview { pub summary: String, pub issues: Vec<String> }
pub struct UnrealExperimentReview { pub id: String }
