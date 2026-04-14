// forge/debate_session.rs
use crate::forge::senku_calculator::{SenkuCalculator, SenkuAnalysis};
use crate::forge::chrome_pattern::{ChromePatternFinder, ChromeAnalysis, ChromeReview};
use crate::forge::xeno_validator::{XenoValidator, XenoAssessment, XenoReview, UnrealExperimentReview};
use crate::contextus::semantic_graph::SemanticGraph;
use crate::forge::experimental_lab::UnrealExperiment;

pub struct LabDebateSession {
    pub senku: SenkuCalculator,
    pub chrome: ChromePatternFinder,
    pub xeno: XenoValidator,
}

impl LabDebateSession {
    pub fn new() -> Self {
        Self {
            senku: SenkuCalculator::new(),
            chrome: ChromePatternFinder::new(),
            xeno: XenoValidator::new(),
        }
    }

    pub fn review_last_n_experiments(&self, n: usize, graph: &SemanticGraph) -> DebateConclusions {
        let experiments = graph.get_recent_experiments(n);
        if experiments.is_empty() {
            return DebateConclusions {
                senku_review: "Sin datos.".into(), chrome_review: "Sin datos.".into(),
                xeno_review: "Sin datos.".into(), synthesis: "Esperando experimentos.".into(),
                learnings: vec![],
            };
        }
        let senku_review = self.senku.review_accuracy_trend(&experiments);
        let chrome_review = self.chrome.find_emerging_patterns(&experiments);
        let xeno_inputs: Vec<UnrealExperimentReview> = experiments.iter().map(|_| UnrealExperimentReview { id: "exp".into() }).collect();
        let xeno_review = self.xeno.find_systematic_errors(&xeno_inputs);
        self.synthesize(&senku_review, &chrome_review, &xeno_review)
    }

    fn synthesize(&self, senku: &crate::forge::senku_calculator::SenkuReview, chrome: &ChromeReview, xeno: &XenoReview) -> DebateConclusions {
        let mut learnings = vec![];
        if senku.accuracy_trend > 0.7 { learnings.push("Modelo de Senku altamente fiable.".into()); }
        if !chrome.new_patterns.is_empty() { learnings.push(format!("Chrome detectó {} patrones.", chrome.new_patterns.len())); }
        if !xeno.issues.is_empty() { learnings.push("Xeno identificó problemas sistémicos.".into()); }
        
        DebateConclusions {
            senku_review: senku.summary.clone(),
            chrome_review: chrome.summary.clone(),
            xeno_review: xeno.summary.clone(),
            synthesis: format!("Debate completado con {} aprendizajes.", learnings.len()),
            learnings,
        }
    }

    pub fn analyze_experiment(&self, experiment: &UnrealExperiment, graph: &SemanticGraph) -> DebateOutput {
        let senku_res = self.senku.analyze(experiment, None);
        let chrome_res = self.chrome.analyze(experiment, graph, Some(&senku_res));
        let xeno_res = self.xeno.cross_validate(&senku_res, &chrome_res, "current_exp");

        println!("\n╔══════════════════════════════════╗");
        println!("║      🧠 DEBATE CIENTÍFICO        ║");
        println!("╚══════════════════════════════════╝");
        println!("[SENKU]  {}", self.senku.summarize_for_debate(&senku_res));
        println!("[CHROME] {}", self.chrome.summarize_for_debate(&chrome_res));
        println!("[XENO]   {}", self.xeno.summarize_for_debate(&xeno_res));

        DebateOutput { senku: senku_res, chrome: chrome_res, xeno: xeno_res }
    }
}

pub struct DebateOutput {
    pub senku: SenkuAnalysis,
    pub chrome: ChromeAnalysis,
    pub xeno: XenoAssessment,
}

pub struct DebateConclusions {
    pub senku_review: String,
    pub chrome_review: String,
    pub xeno_review: String,
    pub synthesis: String,
    pub learnings: Vec<String>,
}
