use std::time::Instant;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum HypothesisSource {
    UserBehaviorPattern,  
    UnrealStateAnomaly,   
    KnowledgeGap,         
    ConflictingAnchors,   
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum HypothesisStatus {
    Active,
    Confirmed,
    Refuted,
    Stale,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenHypothesis {
    pub id: String, // Cambiado de Uuid a String para evitar errores de serialización
    pub claim: String,
    pub source: HypothesisSource,
    pub confidence: f32,
    pub evidence_count: u32,
    pub created_at: i64,
    pub status: HypothesisStatus,
}

pub struct HypothesisEngine {
    pub active_hypotheses: Vec<OpenHypothesis>,
    pub max_active: usize,
}

impl HypothesisEngine {
    pub fn new() -> Self {
        Self {
            active_hypotheses: Vec::new(),
            max_active: 10,
        }
    }

    pub fn propose(&mut self, claim: &str, source: HypothesisSource) {
        if self.active_hypotheses.len() < self.max_active {
            let new_hyp = OpenHypothesis {
                id: format!("hyp_{}", chrono::Utc::now().timestamp_millis()),
                claim: claim.to_string(),
                source,
                confidence: 0.5,
                evidence_count: 0,
                created_at: chrono::Utc::now().timestamp(),
                status: HypothesisStatus::Active,
            };
            self.active_hypotheses.push(new_hyp);
            println!("  [HYPOTHESIS] Nueva sospecha activa: {}", claim);
        }
    }

    pub fn update_with_evidence(&mut self, evidence_found: bool, hypothesis_id: &str) {
        if let Some(hyp) = self.active_hypotheses.iter_mut().find(|h| h.id == hypothesis_id) {
            hyp.evidence_count += 1;
            if evidence_found {
                hyp.confidence += 0.15;
            } else {
                hyp.confidence -= 0.10;
            }

            if hyp.confidence > 0.90 {
                hyp.status = HypothesisStatus::Confirmed;
                println!("  [HYPOTHESIS] ¡Hipótesis Confirmada! {}", hyp.claim);
            } else if hyp.confidence < 0.20 {
                hyp.status = HypothesisStatus::Refuted;
                println!("  [HYPOTHESIS] Hipótesis Refutada: {}", hyp.claim);
            }
        }
    }

    pub fn resolve(&mut self, hypothesis_id: &str, confirmed: bool) {
        if let Some(hyp) = self.active_hypotheses.iter_mut().find(|h| h.id == hypothesis_id) {
            hyp.status = if confirmed { HypothesisStatus::Confirmed } else { HypothesisStatus::Refuted };
            println!("  [HYPOTHESIS] Hipótesis {}: {}", 
                     if confirmed { "CONFIRMADA ✅" } else { "REFUTADA ❌" }, 
                     hyp.claim);
        }
    }

    pub fn cleanup(&mut self) {
        self.active_hypotheses.retain(|h| h.status == HypothesisStatus::Active);
    }
}
