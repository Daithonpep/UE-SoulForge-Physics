// src/learning/analogy_engine.rs
// ============================================================
// MOTOR DE ANALOGÍAS: Conecta dominios nuevos con conocimiento previo
// ============================================================
// Cuando Daithon aprende ajedrez, este motor busca en su Grafo
// Semántico si algo se parece a conceptos ya conocidos.
// 
// "Un Peón bloqueando el centro es como un Pilar sosteniendo un puente"
// "Un Sacrificio de pieza es como una Demolición Controlada"
// ============================================================

use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use crate::learning::document_parser::{ParsedKnowledge, Entity, Rule};
use crate::contextus::semantic_graph::SemanticGraph;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossDomainAnalogy {
    pub new_concept: String,        // "Pawn" (del dominio nuevo)
    pub new_domain: String,         // "Chess"
    pub known_concept: String,      // "Pillar" (de un dominio conocido)
    pub known_domain: String,       // "Architecture"
    pub similarity_type: String,    // "structural_role" | "movement_pattern" | "causal_chain"
    pub similarity_score: f32,
    pub insight: String,            // Explicación de por qué son similares
}

pub struct AnalogyEngine {
    /// Patrones semánticos universales que permiten conectar dominios
    universal_patterns: Vec<UniversalPattern>,
}

#[derive(Debug, Clone)]
struct UniversalPattern {
    name: String,
    keywords: Vec<String>,
    category: String,    // "structural", "dynamic", "adversarial", "hierarchical"
}

impl AnalogyEngine {
    pub fn new() -> Self {
        Self {
            universal_patterns: Self::init_universal_patterns(),
        }
    }

    /// Busca analogías entre el conocimiento nuevo y el grafo existente
    pub fn find_cross_domain_analogies(
        &self,
        new_knowledge: &ParsedKnowledge,
        graph: &SemanticGraph,
    ) -> Vec<CrossDomainAnalogy> {
        let mut analogies = Vec::new();

        // 1. Comparar entidades del nuevo dominio con abstracciones existentes
        for entity in &new_knowledge.entities {
            let entity_pattern = self.classify_entity(entity);
            
            for (key, abstraction) in &graph.abstraction_anchors {
                let similarity = self.compute_semantic_similarity(
                    &entity_pattern,
                    &abstraction.original_concept,
                    &abstraction.source_domain,
                );
                
                if similarity > 0.4 {
                    analogies.push(CrossDomainAnalogy {
                        new_concept: entity.name.clone(),
                        new_domain: new_knowledge.domain.clone(),
                        known_concept: abstraction.original_concept.clone(),
                        known_domain: abstraction.source_domain.clone(),
                        similarity_type: "abstraction_match".to_string(),
                        similarity_score: similarity,
                        insight: format!(
                            "'{}' en {} tiene un rol similar a '{}' en {}: {}",
                            entity.name, new_knowledge.domain,
                            abstraction.original_concept, abstraction.source_domain,
                            self.generate_insight(entity, &abstraction.original_concept)
                        ),
                    });
                }
            }

            // 2. Comparar con anclas empíricas
            for (key, empirical) in &graph.empirical_anchors {
                let similarity = self.compute_semantic_similarity(
                    &entity_pattern,
                    &empirical.claim,
                    "empirical",
                );
                
                if similarity > 0.5 {
                    analogies.push(CrossDomainAnalogy {
                        new_concept: entity.name.clone(),
                        new_domain: new_knowledge.domain.clone(),
                        known_concept: empirical.claim.clone(),
                        known_domain: "empirical_knowledge".to_string(),
                        similarity_type: "empirical_match".to_string(),
                        similarity_score: similarity,
                        insight: format!(
                            "La entidad '{}' se relaciona con la observación empírica: '{}'",
                            entity.name, empirical.claim
                        ),
                    });
                }
            }
        }

        // 3. Comparar reglas con patrones universales
        for rule in &new_knowledge.rules {
            for pattern in &self.universal_patterns {
                let match_score = self.rule_matches_pattern(rule, pattern);
                if match_score > 0.3 {
                    analogies.push(CrossDomainAnalogy {
                        new_concept: format!("{}: {}", rule.subject, rule.action),
                        new_domain: new_knowledge.domain.clone(),
                        known_concept: pattern.name.clone(),
                        known_domain: "universal_patterns".to_string(),
                        similarity_type: pattern.category.clone(),
                        similarity_score: match_score,
                        insight: format!(
                            "La regla '{}' sigue el patrón universal '{}'",
                            rule.subject, pattern.name
                        ),
                    });
                }
            }
        }

        // Ordenar por relevancia
        analogies.sort_by(|a, b| b.similarity_score.partial_cmp(&a.similarity_score).unwrap_or(std::cmp::Ordering::Equal));
        analogies.truncate(10); // Top 10 analogías
        analogies
    }

    /// Clasifica una entidad en patrones semánticos universales
    fn classify_entity(&self, entity: &Entity) -> String {
        let desc = entity.properties.get("description").cloned().unwrap_or_default().to_lowercase();
        let name = entity.name.to_lowercase();
        
        let mut pattern = String::new();
        
        if desc.contains("move") || desc.contains("mueve") || desc.contains("position") {
            pattern.push_str("mobile_agent ");
        }
        if desc.contains("attack") || desc.contains("captur") || desc.contains("atac") {
            pattern.push_str("offensive ");
        }
        if desc.contains("protect") || desc.contains("defend") || desc.contains("proteg") {
            pattern.push_str("defensive ");
        }
        if desc.contains("block") || desc.contains("obstru") || desc.contains("bloque") {
            pattern.push_str("structural_barrier ");
        }
        if desc.contains("jump") || desc.contains("salt") || desc.contains("over") {
            pattern.push_str("non_linear_traversal ");
        }
        if desc.contains("any direction") || desc.contains("cualquier") {
            pattern.push_str("omnidirectional ");
        }
        if desc.contains("forward") || desc.contains("adelante") || desc.contains("straight") {
            pattern.push_str("linear_progression ");
        }
        
        if pattern.is_empty() {
            pattern = format!("generic_{}", name);
        }
        
        pattern.trim().to_string()
    }

    /// Calcula similitud semántica simplificada entre dos conceptos
    fn compute_semantic_similarity(&self, pattern_a: &str, concept_b: &str, _domain_b: &str) -> f32 {
        let words_a: std::collections::HashSet<&str> = pattern_a.split_whitespace().collect();
        let concept_lower = concept_b.to_lowercase();
        let words_b: std::collections::HashSet<&str> = concept_lower.split(|c: char| !c.is_alphanumeric()).collect();
        
        if words_a.is_empty() || words_b.is_empty() {
            return 0.0;
        }
        
        let intersection = words_a.intersection(&words_b).count() as f32;
        let union = words_a.union(&words_b).count() as f32;
        
        let base_sim: f32 = if union > 0.0 { intersection / union } else { 0.0 };
        
        let semantic_bonus: f32 = if pattern_a.contains("structural") && concept_b.contains("structur") { 0.3 }
            else if pattern_a.contains("mobile") && concept_b.contains("move") { 0.2 }
            else if pattern_a.contains("offensive") && concept_b.contains("force") { 0.2 }
            else { 0.0 };
        
        (base_sim + semantic_bonus).min(1.0_f32)
    }

    fn rule_matches_pattern(&self, rule: &Rule, pattern: &UniversalPattern) -> f32 {
        let rule_text = format!("{} {} {}", rule.subject, rule.action, rule.condition).to_lowercase();
        let mut score: f32 = 0.0;
        
        for keyword in &pattern.keywords {
            if rule_text.contains(&keyword.to_lowercase()) {
                score += 0.3;
            }
        }
        
        score.min(1.0)

    }

    fn generate_insight(&self, entity: &Entity, _known_concept: &str) -> String {
        let desc = entity.properties.get("description").cloned().unwrap_or_default();
        format!("Ambos cumplen una función de {} en sus respectivos dominios", 
            if desc.contains("move") { "movimiento y posicionamiento" }
            else if desc.contains("protect") { "protección y defensa" }
            else if desc.contains("attack") { "ataque y disrupción" }
            else { "soporte estructural" }
        )
    }

    /// Patrones universales que conectan dominios
    fn init_universal_patterns() -> Vec<UniversalPattern> {
        vec![
            UniversalPattern {
                name: "Linear Progression".to_string(),
                keywords: vec!["forward".into(), "step".into(), "advance".into(), "adelante".into(), "avanza".into()],
                category: "dynamic".to_string(),
            },
            UniversalPattern {
                name: "Non-Linear Traversal".to_string(),
                keywords: vec!["jump".into(), "over".into(), "skip".into(), "salta".into(), "L-shape".into()],
                category: "dynamic".to_string(),
            },
            UniversalPattern {
                name: "Structural Support".to_string(),
                keywords: vec!["support".into(), "hold".into(), "base".into(), "foundation".into(), "soporte".into()],
                category: "structural".to_string(),
            },
            UniversalPattern {
                name: "Controlled Sacrifice".to_string(),
                keywords: vec!["sacrifice".into(), "trade".into(), "exchange".into(), "sacrificio".into()],
                category: "adversarial".to_string(),
            },
            UniversalPattern {
                name: "Territorial Control".to_string(),
                keywords: vec!["control".into(), "territory".into(), "center".into(), "dominate".into(), "centro".into()],
                category: "adversarial".to_string(),
            },
            UniversalPattern {
                name: "Hierarchical Value".to_string(),
                keywords: vec!["value".into(), "important".into(), "king".into(), "queen".into(), "priority".into()],
                category: "hierarchical".to_string(),
            },
            UniversalPattern {
                name: "Combinatorial Complexity".to_string(),
                keywords: vec!["combine".into(), "multiple".into(), "chain".into(), "sequence".into(), "cadena".into()],
                category: "dynamic".to_string(),
            },
            UniversalPattern {
                name: "Constraint Satisfaction".to_string(),
                keywords: vec!["cannot".into(), "must".into(), "legal".into(), "valid".into(), "restricción".into()],
                category: "structural".to_string(),
            },
        ]
    }
}
