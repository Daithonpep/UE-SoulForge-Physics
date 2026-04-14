// forge/chrome_pattern.rs
use std::collections::HashMap;
use crate::contextus::semantic_graph::{SemanticGraph, EmpiricalAnchor};
use crate::forge::experimental_lab::{UnrealExperiment, StructureType, StressTest};
use crate::forge::senku_calculator::SenkuAnalysis;
use crate::knowledge::physics_laws::PhysicsKnowledgeBase;

#[derive(Debug, Clone)]
pub struct ChromeAnalysis {
    pub mode: ChromeMode,
    pub similar_experiments: Vec<SimilarExperiment>,
    pub patterns: Vec<Pattern>,
    pub cross_domain_connections: Vec<CrossDomainLink>,
    pub consensus_prediction: Option<ConsensusPrediction>,
    pub experience_level: ExperienceLevel,
    pub contribution: ChromeContribution,
    pub summary: String,
    pub survival_expectation: f32,
    pub similar_count: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ChromeMode { Bootstrap, Learning, Experienced }

#[derive(Debug, Clone)]
pub struct SimilarExperiment {
    pub anchor_key: String,
    pub similarity_score: f64,
    pub survived: bool,
    pub deformation: f64,
    pub conditions: String,
    pub difference: String,
}

#[derive(Debug, Clone)]
pub struct Pattern {
    pub description: String,
    pub confidence: f64,
    pub evidence_count: usize,
    pub pattern_type: PatternType,
}

#[derive(Debug, Clone)]
pub enum PatternType { SurvivalRate, FailureCondition, GeometryCorrelation, MaterialBehavior, CrossDomain }

#[derive(Debug, Clone)]
pub struct CrossDomainLink {
    pub from_domain: String,
    pub to_domain: String,
    pub connection: String,
    pub relevance: f64,
}

#[derive(Debug, Clone)]
pub struct ConsensusPrediction {
    pub will_survive: bool,
    pub confidence: f64,
    pub based_on: usize,
    pub reasoning: String,
}

#[derive(Debug, Clone)]
pub enum ExperienceLevel { None, Minimal(usize), Moderate(usize), Rich(usize) }

#[derive(Debug, Clone)]
pub struct ChromeContribution {
    pub text: String,
    pub adds_value: bool,
    pub defers_to: Option<String>,
}

pub struct ChromeReview { pub summary: String, pub new_patterns: Vec<String> }

pub struct ChromePatternFinder {
    pub physics_kb: PhysicsKnowledgeBase,
}

impl ChromePatternFinder {
    pub fn new() -> Self { Self { physics_kb: PhysicsKnowledgeBase::initialize() } }

    pub fn analyze(&self, experiment: &UnrealExperiment, graph: &SemanticGraph, senku: Option<&SenkuAnalysis>) -> ChromeAnalysis {
        let anchors = self.find_relevant_anchors(experiment, graph);
        let total = anchors.iter().map(|a| a.reproduction_count as usize).sum();
        let exp_level = match total { 0 => ExperienceLevel::None, 1..=5 => ExperienceLevel::Minimal(total), 6..=20 => ExperienceLevel::Moderate(total), _ => ExperienceLevel::Rich(total) };
        let mode = match exp_level { ExperienceLevel::None => ChromeMode::Bootstrap, ExperienceLevel::Minimal(_) => ChromeMode::Learning, _ => ChromeMode::Experienced };

        let mut res = match mode {
            ChromeMode::Bootstrap => self.bootstrap_mode(experiment, senku, exp_level),
            ChromeMode::Learning => self.learning_mode(experiment, anchors, exp_level),
            ChromeMode::Experienced => self.experienced_mode(experiment, anchors, exp_level),
        };
        res.summary = res.contribution.text.clone();
        res.similar_count = total;
        res.survival_expectation = if let Some(c) = &res.consensus_prediction { if c.will_survive { 0.8 } else { 0.2 } } else { 0.5 };
        res
    }

    fn find_relevant_anchors(&self, _exp: &UnrealExperiment, graph: &SemanticGraph) -> Vec<EmpiricalAnchor> {
        graph.empirical_anchors.values().cloned().collect()
    }

    fn bootstrap_mode(&self, _exp: &UnrealExperiment, _senku: Option<&SenkuAnalysis>, level: ExperienceLevel) -> ChromeAnalysis {
        ChromeAnalysis {
            mode: ChromeMode::Bootstrap, similar_experiments: vec![], patterns: vec![], cross_domain_connections: vec![],
            consensus_prediction: None, experience_level: level,
            contribution: ChromeContribution { text: "[Bootstrap] Sin datos propios.".into(), adds_value: false, defers_to: Some("Senku".into()) },
            summary: "".into(), survival_expectation: 0.5, similar_count: 0,
        }
    }

    fn learning_mode(&self, _exp: &UnrealExperiment, anchors: Vec<EmpiricalAnchor>, level: ExperienceLevel) -> ChromeAnalysis {
        ChromeAnalysis {
            mode: ChromeMode::Learning, similar_experiments: vec![], patterns: vec![], cross_domain_connections: vec![],
            consensus_prediction: Some(ConsensusPrediction { will_survive: true, confidence: 0.4, based_on: anchors.len(), reasoning: "Incertidumbre alta.".into() }),
            experience_level: level,
            contribution: ChromeContribution { text: "[Learning] Tendencia preliminar.".into(), adds_value: true, defers_to: None },
            summary: "".into(), survival_expectation: 0.5, similar_count: anchors.len(),
        }
    }

    fn experienced_mode(&self, _exp: &UnrealExperiment, anchors: Vec<EmpiricalAnchor>, level: ExperienceLevel) -> ChromeAnalysis {
        ChromeAnalysis {
            mode: ChromeMode::Experienced, similar_experiments: vec![], patterns: vec![], cross_domain_connections: vec![],
            consensus_prediction: Some(ConsensusPrediction { will_survive: true, confidence: 0.8, based_on: anchors.len(), reasoning: "Patrón claro.".into() }),
            experience_level: level,
            contribution: ChromeContribution { text: "[Experienced] Datos sólidos.".into(), adds_value: true, defers_to: None },
            summary: "".into(), survival_expectation: 0.8, similar_count: anchors.len(),
        }
    }

    pub fn find_emerging_patterns(&self, experiments: &[UnrealExperiment]) -> ChromeReview {
        ChromeReview { summary: format!("Analizando {} experimentos.", experiments.len()), new_patterns: vec![] }
    }

    pub fn summarize_for_debate(&self, res: &ChromeAnalysis) -> String { res.summary.clone() }
}
