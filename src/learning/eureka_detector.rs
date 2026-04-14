// src/learning/eureka_detector.rs
// ============================================================
// DETECTOR DE EUREKA: Descubre leyes que NO están en el manual
// ============================================================
// Este es el módulo que hace a Daithon SUPERIOR a memorizar.
// Analiza patrones en la experiencia de práctica y detecta
// correlaciones y leyes causales que el manual NO menciona.
//
// Ejemplo: El manual dice "el peón se mueve hacia adelante".
// Daithon descubre: "Los peones centrales aumentan la movilidad"
// Eso NO estaba en el manual. Lo DESCUBRIÓ jugando.
// ============================================================

use std::collections::HashMap;
use crate::learning::cognitive_log::{CognitiveLog, EurekaMoment};
use crate::learning::document_parser::ParsedKnowledge;

/// Registro de una observación durante la práctica
#[derive(Debug, Clone)]
pub struct PracticeObservation {
    pub iteration: usize,
    pub variables: HashMap<String, f32>,    // Variables causales medidas
    pub action_taken: String,               // Qué hizo Daithon
    pub outcome: PracticeOutcome,           // Qué pasó después
}

#[derive(Debug, Clone)]
pub enum PracticeOutcome {
    Win,
    Loss,
    Draw,
    Improvement(f32),   // Mejora respecto al estado anterior
    Degradation(f32),   // Empeoramiento
}

/// Una ley candidata descubierta
#[derive(Debug, Clone)]
pub struct DiscoveredLaw {
    pub cause_variable: String,
    pub effect_variable: String,
    pub correlation: f32,
    pub direction: String,             // "positive" | "negative"
    pub evidence_count: usize,
    pub description: String,
    pub was_in_manual: bool,
}

pub struct EurekaDetector {
    observations: Vec<PracticeObservation>,
    discovered_laws: Vec<DiscoveredLaw>,
    manual_rules: Vec<String>,         // Reglas que SÍ estaban en el manual
}

impl EurekaDetector {
    pub fn new(knowledge: &ParsedKnowledge) -> Self {
        let manual_rules: Vec<String> = knowledge.rules.iter()
            .map(|r| format!("{} {}", r.subject, r.action).to_lowercase())
            .collect();
        
        Self {
            observations: Vec::new(),
            discovered_laws: Vec::new(),
            manual_rules,
        }
    }

    /// Registra una observación de la práctica
    pub fn observe(&mut self, obs: PracticeObservation) {
        self.observations.push(obs);
    }

    /// Analiza las observaciones acumuladas y busca leyes nuevas
    pub fn analyze(&mut self, log: &mut CognitiveLog, domain: &str) -> Vec<DiscoveredLaw> {
        if self.observations.len() < 5 {
            return Vec::new(); // Necesitamos suficientes datos
        }

        let mut new_discoveries = Vec::new();

        // 1. Correlaciones entre variables causales y resultados
        let var_names: Vec<String> = if let Some(obs) = self.observations.first() {
            obs.variables.keys().cloned().collect()
        } else {
            return Vec::new();
        };

        for var_a in &var_names {
            for var_b in &var_names {
                if var_a == var_b { continue; }
                
                let correlation = self.compute_correlation(var_a, var_b);
                
                if correlation.abs() > 0.6 {
                    let description = format!(
                        "Cuando {} {}, {} tiende a {}",
                        var_a,
                        if correlation > 0.0 { "aumenta" } else { "disminuye" },
                        var_b,
                        if correlation > 0.0 { "aumentar también" } else { "disminuir" }
                    );
                    
                    let was_in_manual = self.is_known_from_manual(&description);
                    
                    if !was_in_manual {
                        let law = DiscoveredLaw {
                            cause_variable: var_a.clone(),
                            effect_variable: var_b.clone(),
                            correlation,
                            direction: if correlation > 0.0 { "positive".to_string() } else { "negative".to_string() },
                            evidence_count: self.observations.len(),
                            description: description.clone(),
                            was_in_manual: false,
                        };
                        
                        // Log como Eureka
                        log.log_eureka(EurekaMoment {
                            timestamp: std::time::SystemTime::now()
                                .duration_since(std::time::UNIX_EPOCH)
                                .unwrap().as_secs(),
                            domain: domain.to_string(),
                            discovery: description,
                            was_in_manual: false,
                            evidence: self.observations.iter()
                                .take(3)
                                .map(|o| format!("Iter {}: {}={:.2}, {}={:.2}", 
                                    o.iteration, var_a, 
                                    o.variables.get(var_a).unwrap_or(&0.0),
                                    var_b,
                                    o.variables.get(var_b).unwrap_or(&0.0)
                                ))
                                .collect(),
                            confidence: correlation.abs(),
                            iteration: self.observations.len(),
                        });
                        
                        new_discoveries.push(law);
                    }
                }
            }
        }

        // 2. Correlación action → outcome
        let win_actions = self.find_winning_patterns();
        for (action, win_rate) in &win_actions {
            if *win_rate > 0.7 {
                let description = format!(
                    "La acción '{}' conduce a victoria en {:.0}% de los casos",
                    action, win_rate * 100.0
                );
                
                if !self.is_known_from_manual(&description) {
                    log.log_eureka(EurekaMoment {
                        timestamp: std::time::SystemTime::now()
                            .duration_since(std::time::UNIX_EPOCH)
                            .unwrap().as_secs(),
                        domain: domain.to_string(),
                        discovery: description.clone(),
                        was_in_manual: false,
                        evidence: vec![format!("{} partidas analizadas", self.observations.len())],
                        confidence: *win_rate,
                        iteration: self.observations.len(),
                    });
                    
                    new_discoveries.push(DiscoveredLaw {
                        cause_variable: action.clone(),
                        effect_variable: "win_probability".to_string(),
                        correlation: *win_rate,
                        direction: "positive".to_string(),
                        evidence_count: self.observations.len(),
                        description,
                        was_in_manual: false,
                    });
                }
            }
        }

        self.discovered_laws.extend(new_discoveries.clone());
        new_discoveries
    }

    /// Pearson correlation entre dos variables a lo largo de las observaciones
    fn compute_correlation(&self, var_a: &str, var_b: &str) -> f32 {
        let pairs: Vec<(f32, f32)> = self.observations.iter()
            .filter_map(|obs| {
                let a = obs.variables.get(var_a)?;
                let b = obs.variables.get(var_b)?;
                Some((*a, *b))
            })
            .collect();
        
        if pairs.len() < 3 { return 0.0; }
        
        let n = pairs.len() as f32;
        let mean_a = pairs.iter().map(|(a, _)| a).sum::<f32>() / n;
        let mean_b = pairs.iter().map(|(_, b)| b).sum::<f32>() / n;
        
        let mut numerator = 0.0;
        let mut denom_a = 0.0;
        let mut denom_b = 0.0;
        
        for (a, b) in &pairs {
            let da = a - mean_a;
            let db = b - mean_b;
            numerator += da * db;
            denom_a += da * da;
            denom_b += db * db;
        }
        
        let denom = (denom_a * denom_b).sqrt();
        if denom == 0.0 { 0.0 } else { numerator / denom }
    }

    /// Encuentra patrones de acción que llevan a victoria
    fn find_winning_patterns(&self) -> HashMap<String, f32> {
        let mut action_wins: HashMap<String, (usize, usize)> = HashMap::new(); // (wins, total)
        
        for obs in &self.observations {
            let entry = action_wins.entry(obs.action_taken.clone()).or_insert((0, 0));
            entry.1 += 1;
            if matches!(obs.outcome, PracticeOutcome::Win | PracticeOutcome::Improvement(_)) {
                entry.0 += 1;
            }
        }
        
        action_wins.into_iter()
            .filter(|(_, (_, total))| *total >= 3) // Mínimo 3 intentos
            .map(|(action, (wins, total))| (action, wins as f32 / total as f32))
            .collect()
    }

    /// Verifica si una descripción ya está cubierta por el manual
    fn is_known_from_manual(&self, description: &str) -> bool {
        let desc_lower = description.to_lowercase();
        self.manual_rules.iter().any(|rule| {
            // Verificar si hay overlap significativo de palabras
            let rule_words: std::collections::HashSet<&str> = rule.split_whitespace().collect();
            let desc_words: std::collections::HashSet<&str> = desc_lower.split_whitespace().collect();
            let overlap = rule_words.intersection(&desc_words).count();
            overlap > 3 // Más de 3 palabras en común = probablemente lo mismo
        })
    }

    pub fn get_all_discoveries(&self) -> &[DiscoveredLaw] {
        &self.discovered_laws
    }
}
