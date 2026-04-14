// src/learning/cognitive_log.rs
// ============================================================
// LOG COGNITIVO: Registro transparente de TODO el proceso mental
// ============================================================
// Este módulo registra CADA paso del razonamiento de Daithon:
// - Qué reglas consultó del Grafo
// - Qué analogías encontró
// - Por qué Senku eligió X y Xeno propuso Y
// - Qué ley causal aplicó
// - Qué descubrió (Eureka)
// ============================================================

use serde::{Serialize, Deserialize};
use std::collections::VecDeque;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CognitiveAgent {
    Senku,       // Lógica pura
    Xeno,        // Exploración caótica
    Cortex,      // Comprensión profunda
    Contextus,   // Memoria y grafo
    Causal,      // Motor de causalidad
    Eureka,      // Descubrimiento
    Analogy,     // Conexiones cross-domain
    System,      // Mensajes del sistema
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveEntry {
    pub timestamp: u64,
    pub agent: CognitiveAgent,
    pub phase: String,             // "PERCEPCIÓN", "RAZONAMIENTO", "DECISIÓN", "APRENDIZAJE"
    pub thought: String,           // El pensamiento en sí
    pub evidence: Vec<String>,     // Datos que lo soportan
    pub confidence: f32,           // Qué tan seguro está (0.0-1.0)
    pub alternatives_considered: Vec<AlternativeThought>,
    pub graph_nodes_consulted: Vec<String>, // Nodos del grafo semántico consultados
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AlternativeThought {
    pub description: String,
    pub why_rejected: String,
    pub score: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveDecisionLog {
    pub move_number: usize,
    pub chosen_move: String,
    pub chosen_by: CognitiveAgent,
    
    // ¿Por qué esta jugada?
    pub reasoning_chain: Vec<CognitiveEntry>,
    
    // ¿Qué reglas del manual aplicó?
    pub rules_applied: Vec<RuleApplication>,
    
    // ¿Qué leyes causales descubiertas usó?
    pub causal_laws_used: Vec<CausalLawApplication>,
    
    // ¿Qué analogías de otros dominios invocó?
    pub analogies_invoked: Vec<AnalogyInvocation>,
    
    // ¿Qué alternativas consideró?
    pub alternatives: Vec<MoveAlternative>,
    
    // Resultado
    pub expected_outcome: String,
    pub actual_outcome: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleApplication {
    pub rule_id: String,
    pub rule_text: String,       // "El peón se mueve 1 casilla hacia adelante"
    pub how_applied: String,     // "Moví peón de e2 a e4 (primera jugada, se permite 2)"
    pub confidence_in_rule: f32, // Qué tanto confía en esta regla
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalLawApplication {
    pub law_id: String,
    pub description: String,     // "Control del centro → Mayor movilidad"
    pub was_discovered: bool,    // ¿Daithon descubrió esto o estaba en el manual?
    pub evidence_count: u32,     // Veces que se ha validado
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalogyInvocation {
    pub source_domain: String,   // "Unreal_Physics"
    pub source_concept: String,  // "Posición de columnas en puente"
    pub target_concept: String,  // "Posición de peones en centro"
    pub similarity: f32,
    pub insight: String,         // "Los peones centrales son como pilares: sostienen la estructura"
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoveAlternative {
    pub move_desc: String,
    pub agent: CognitiveAgent,
    pub score: f32,
    pub reason: String,
    pub why_not_chosen: String,
}

/// El Log Cognitivo principal — Mantiene una cola circular de entradas
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveLog {
    pub entries: VecDeque<CognitiveEntry>,
    pub move_decisions: Vec<MoveDecisionLog>,
    pub eureka_moments: Vec<EurekaMoment>,
    pub max_entries: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EurekaMoment {
    pub timestamp: u64,
    pub domain: String,
    pub discovery: String,       // "El control del centro aumenta la movilidad un 40%"
    pub was_in_manual: bool,     // false = Daithon lo DESCUBRIÓ solo
    pub evidence: Vec<String>,
    pub confidence: f32,
    pub iteration: usize,        // En qué partida/iteración lo descubrió
}

impl CognitiveLog {
    pub fn new() -> Self {
        Self {
            entries: VecDeque::with_capacity(500),
            move_decisions: Vec::new(),
            eureka_moments: Vec::new(),
            max_entries: 500,
        }
    }

    pub fn think(&mut self, agent: CognitiveAgent, phase: &str, thought: &str) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap().as_secs();
        
        let entry = CognitiveEntry {
            timestamp: now,
            agent,
            phase: phase.to_string(),
            thought: thought.to_string(),
            evidence: Vec::new(),
            confidence: 0.5,
            alternatives_considered: Vec::new(),
            graph_nodes_consulted: Vec::new(),
        };
        
        if self.entries.len() >= self.max_entries {
            self.entries.pop_front();
        }
        self.entries.push_back(entry);
    }

    pub fn think_with_evidence(
        &mut self, 
        agent: CognitiveAgent, 
        phase: &str, 
        thought: &str,
        evidence: Vec<String>,
        confidence: f32,
        graph_nodes: Vec<String>,
    ) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap().as_secs();
        
        let entry = CognitiveEntry {
            timestamp: now,
            agent,
            phase: phase.to_string(),
            thought: thought.to_string(),
            evidence,
            confidence,
            alternatives_considered: Vec::new(),
            graph_nodes_consulted: graph_nodes,
        };
        
        if self.entries.len() >= self.max_entries {
            self.entries.pop_front();
        }
        self.entries.push_back(entry);
    }

    pub fn log_move_decision(&mut self, decision: MoveDecisionLog) {
        self.move_decisions.push(decision);
    }

    pub fn log_eureka(&mut self, moment: EurekaMoment) {
        println!("💡 ═══════════════════════════════════════");
        println!("   EUREKA MOMENT [{}]", moment.domain);
        println!("   {}", moment.discovery);
        println!("   Confianza: {:.0}% | Evidencia: {} casos", 
            moment.confidence * 100.0, moment.evidence.len());
        println!("   ¿En el manual? {}", if moment.was_in_manual { "Sí" } else { "NO — DESCUBIERTO" });
        println!("═══════════════════════════════════════════");
        self.eureka_moments.push(moment);
    }

    /// Genera un resumen JSON del log
    pub fn get_recent_log(&self, n: usize) -> serde_json::Value {
        let recent: Vec<&CognitiveEntry> = self.entries.iter().rev().take(n).collect();
        serde_json::json!({
            "thoughts": recent,
            "total_decisions": self.move_decisions.len(),
            "eureka_count": self.eureka_moments.len(),
            "eurekas": self.eureka_moments,
        })
    }

    /// Obtiene el último razonamiento de una jugada
    pub fn get_last_decision(&self) -> Option<&MoveDecisionLog> {
        self.move_decisions.last()
    }
}
