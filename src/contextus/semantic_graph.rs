// src/contextus/semantic_graph.rs
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use crate::metacog::SynthesisOutput;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EvidenceType { Empirical, Analytical, Theoretical, Anecdotal }

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TheoreticalSupport {
    pub concept: String,
    pub relevance: f32,
    pub source: AnchorSource,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmpiricalAnchor {
    pub claim: String,
    pub conditions: String, 
    pub result_summary: String,
    pub source: AnchorSource,
    pub reproduction_count: u32,
    pub confidence: f32,
    pub avg_deformation: f32,
    pub known_failure_points: Vec<String>,
    pub survival_rate: f32,
    pub theoretical_support: Vec<TheoreticalSupport>,
    pub accuracy_history: Vec<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AnchorSource { LabExperiment, WebValidation, UserConflict, MockSimulator, UnrealPhysics, PhilosophicalAbstraction }

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ScientificStatus {
    Hypothesis,
    EngineeringLaw,
    Poetic,
    Falsified,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbstractionAnchor {
    pub source_domain: String,
    pub original_concept: String,
    pub synthesis: SynthesisOutput,
    pub confidence: f32,
    pub timestamp: u64,
    pub status: ScientificStatus,
    pub usage_count: u32,
    pub failure_count: u32,
    pub last_used: u64,
    pub constraint_delta: f32, // Matriz R: Diferencia de restricciones detectada
}

#[derive(Debug, Clone)]
pub struct SemanticGraph {
    pub empirical_anchors: HashMap<String, EmpiricalAnchor>,
    pub abstraction_anchors: HashMap<String, AbstractionAnchor>,
}

impl SemanticGraph {
    pub fn new() -> Self { 
        Self { 
            empirical_anchors: HashMap::new(),
            abstraction_anchors: HashMap::new(),
        } 
    }

    pub fn add_abstraction(&mut self, source_domain: String, concept: String, synthesis: SynthesisOutput) {
        let key = format!("abs_{}_{}", source_domain, concept).replace(' ', "_").to_lowercase();
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        self.abstraction_anchors.insert(key, AbstractionAnchor {
            source_domain,
            original_concept: concept,
            synthesis,
            confidence: 0.9,
            timestamp: now,
            status: ScientificStatus::Hypothesis,
            usage_count: 0,
            failure_count: 0,
            last_used: now,
            constraint_delta: 0.0,
        });
    }

    /// Fase 5: Poda Semántica y Control de Entropía
    pub fn prune_abstractions(&mut self, threshold: f32) {
        let now = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
        self.abstraction_anchors.retain(|_, anchor| {
            // Decaimiento por tiempo (1% por día de inactividad)
            let days_passed = (now - anchor.last_used) / 86400;
            let decay = 0.01 * days_passed as f32;
            let current_confidence = (anchor.confidence - decay).max(0.0);
            
            // Criterios de poda:
            // 1. Confianza por debajo del umbral.
            // 2. Tasa de fallo muy alta (más de 3 fallos y éxito < 20%).
            let failure_rate = if anchor.usage_count > 0 { 
                anchor.failure_count as f32 / anchor.usage_count as f32 
            } else { 0.0 };

            if current_confidence < threshold || (anchor.usage_count > 3 && failure_rate > 0.8) {
                println!("[PRUNING] Eliminando abstracción ruidosa: {}", anchor.original_concept);
                false
            } else {
                true
            }
        });
    }

    pub fn mark_abstraction_use(&mut self, key: &str, success: bool) {
        if let Some(anchor) = self.abstraction_anchors.get_mut(key) {
            anchor.usage_count += 1;
            anchor.last_used = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs();
            
            if success {
                anchor.confidence = (anchor.confidence + 0.05).min(1.0);
                if anchor.usage_count > 5 && anchor.confidence > 0.8 {
                    anchor.status = ScientificStatus::EngineeringLaw;
                }
            } else {
                anchor.failure_count += 1;
                anchor.confidence = (anchor.confidence - 0.2).max(0.0);
                if anchor.confidence < 0.4 {
                    anchor.status = ScientificStatus::Poetic;
                }
            }
        }
    }

    pub fn strengthen_anchor(&mut self, key: String, claim: &str, accuracy: f32, survived: bool, deformation: f32, failure_points: Vec<String>, conditions: String, source: AnchorSource) {
        let entry = self.empirical_anchors.entry(key.clone()).or_insert(EmpiricalAnchor {
            claim: claim.to_string(), conditions, result_summary: String::new(), source, reproduction_count: 0,
            confidence: 0.1, avg_deformation: 0.0, known_failure_points: vec![], survival_rate: 0.0,
            theoretical_support: vec![], accuracy_history: vec![],
        });
        let n = entry.reproduction_count as f32;
        entry.accuracy_history.push(accuracy);
        entry.reproduction_count += 1;
        entry.avg_deformation = (entry.avg_deformation * n + deformation) / (n + 1.0);
        entry.survival_rate = (entry.survival_rate * n + if survived { 1.0 } else { 0.0 }) / (n + 1.0);
        let mean_acc = entry.accuracy_history.iter().sum::<f32>() / entry.accuracy_history.len() as f32;
        entry.confidence = (mean_acc * (1.0 - (1.0 / (1.0 + n)))).clamp(0.0, 1.0);
    }

    pub fn add_theoretical_support(&mut self, anchor_key: &str, concept: &str, relevance: f32) {
        if let Some(anchor) = self.empirical_anchors.get_mut(anchor_key) {
            anchor.theoretical_support.push(TheoreticalSupport {
                concept: concept.to_string(),
                relevance,
                source: AnchorSource::LabExperiment,
            });
            anchor.confidence = (anchor.confidence + relevance * 0.1).min(1.0);
        }
    }

    pub fn get_recent_experiments(&self, n: usize) -> Vec<crate::forge::experimental_lab::UnrealExperiment> {
        self.empirical_anchors.values().take(n).map(|a| {
            let structure_type = if a.claim.contains("pyramid") {
                crate::forge::experimental_lab::StructureType::Pyramid(crate::forge::experimental_lab::PyramidParams { base_width: 10.0, height: 10.0, material_density: 2400.0 })
            } else {
                crate::forge::experimental_lab::StructureType::Arch(crate::forge::experimental_lab::ArchParams { span: 10.0, radius: 5.0, keystone_weight: 100.0 })
            };

            crate::forge::experimental_lab::UnrealExperiment {
                structure_type,
                placement: crate::forge::experimental_lab::Placement {
                    position: [0.0, 0.0, 0.0],
                    rotation: [0.0, 0.0, 0.0],
                    on_surface: crate::forge::experimental_lab::SurfaceType::Flat,
                },
                material: "concrete".into(),
                stress_test: crate::forge::experimental_lab::StressTest::Seismic(crate::forge::experimental_lab::SeismicParams { magnitude: 5.0, frequency: 1.0 }),
                force_direction: crate::forge::experimental_lab::ForceDirection::Lateral,
                parameters: HashMap::new(),
                duration_seconds: 3,
            }
        }).collect()
    }

    pub fn count_experiments_for(&self, key: &str) -> u32 {
        self.empirical_anchors.iter()
            .filter(|(k, _)| k.contains(key))
            .map(|(_, a)| a.reproduction_count)
            .sum()
    }

    pub fn has_any_collapse(&self) -> bool {
        self.empirical_anchors.values().any(|a| a.survival_rate < 0.9)
    }

    pub fn has_orientation_experiments(&self) -> bool {
        self.empirical_anchors.values().any(|a| a.conditions.contains("rotation") || a.conditions.contains("Placement"))
    }

    pub fn get_max_survived_stress(&self, _structure: &crate::forge::experimental_lab::StructureType) -> Option<f32> {
        // En una implementación real, buscaríamos por tipo de estructura
        // Por ahora, devolvemos el máximo global de supervivencia con alta intensidad
        self.empirical_anchors.values()
            .filter(|a| a.survival_rate > 0.8)
            .map(|a| {
                // Extracción muy básica de la magnitud
                if a.conditions.contains("magnitude: ") {
                    a.conditions.split("magnitude: ").nth(1).and_then(|s| s.split(',').next()).and_then(|s| s.parse::<f32>().ok()).unwrap_or(5.0)
                } else {
                    5.0
                }
            })
            .max_by(|a, b| a.partial_cmp(b).unwrap())
    }

    pub fn purge_mock_data(&mut self) {
        self.empirical_anchors.retain(|_, a| a.source != AnchorSource::MockSimulator);
    }

    /// Encuentra el experimento más "interesante" (anómalos o catastróficos)
    pub fn get_most_interesting_experiment(&self) -> Option<InterestingExperiment> {
        self.empirical_anchors.values().map(|a| {
            // El interés es la suma de su fallo de predicción (delta) y su tasa de colapso
            let delta = a.accuracy_history.last().cloned().unwrap_or(1.0);
            let interest_score = (1.0 - delta) + (1.0 - a.survival_rate);
            
            (a, interest_score)
        })
        .max_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal))
        .map(|(a, _)| InterestingExperiment {
            claim: a.claim.clone(),
            delta: a.accuracy_history.last().cloned().unwrap_or(1.0),
            survival_rate: a.survival_rate,
        })
    }
}

pub struct InterestingExperiment {
    pub claim: String,
    pub delta: f32,
    pub survival_rate: f32,
}
