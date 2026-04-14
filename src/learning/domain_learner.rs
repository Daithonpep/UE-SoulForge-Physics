// src/learning/domain_learner.rs
// ============================================================
// EL MÉTODO MAESTRO: Aprende CUALQUIER dominio desde documentación
// ============================================================

use std::collections::HashMap;
use crate::learning::document_parser::{DocumentParser, ParsedKnowledge};
use crate::learning::analogy_engine::{AnalogyEngine, CrossDomainAnalogy};
use crate::learning::cognitive_log::{CognitiveLog, CognitiveAgent};
use crate::contextus::semantic_graph::{SemanticGraph, AnchorSource};
use crate::metacog::SynthesisOutput;

#[derive(Debug, Clone, serde::Serialize)]
pub struct DomainLearningResult {
    pub domain: String,
    pub entities_learned: usize,
    pub rules_learned: usize,
    pub goals_identified: usize,
    pub constraints_found: usize,
    pub analogies_discovered: Vec<CrossDomainAnalogy>,
    pub knowledge_gaps: Vec<String>,
    pub status: String,
}

pub struct DomainLearner {
    pub analogy_engine: AnalogyEngine,
    pub learned_domains: HashMap<String, ParsedKnowledge>,
}

impl DomainLearner {
    pub fn new() -> Self {
        Self {
            analogy_engine: AnalogyEngine::new(),
            learned_domains: HashMap::new(),
        }
    }

    /// 🧠 EL MÉTODO MAESTRO: Aprende un dominio nuevo desde cero
    pub fn learn_new_domain(
        &mut self,
        domain_name: &str,
        manual_text: &str,
        graph: &mut SemanticGraph,
        log: &mut CognitiveLog,
    ) -> DomainLearningResult {
        
        println!("\n╔══════════════════════════════════════════════════════╗");
        println!("║  📚 DAITHON: Aprendiendo dominio '{}'", domain_name);
        println!("╚══════════════════════════════════════════════════════╝\n");

        log.think(CognitiveAgent::System, "INICIO", 
            &format!("Recibí documentación sobre '{}'. Iniciando procesamiento...", domain_name));

        // FASE 1: COMPRENSIÓN
        log.think(CognitiveAgent::Cortex, "COMPRENSIÓN", "Fase 1: Analizando estructura del documento...");
        let knowledge = DocumentParser::parse(domain_name, manual_text);
        
        log.think_with_evidence(
            CognitiveAgent::Cortex, "COMPRENSIÓN",
            &format!("Extracción completada: {} entidades, {} reglas, {} objetivos, {} restricciones",
                knowledge.entities.len(), knowledge.rules.len(),
                knowledge.goals.len(), knowledge.constraints.len()),
            knowledge.entities.iter().map(|e| format!("Entidad: {}", e.name)).collect(),
            0.9, vec![],
        );

        // FASE 2: INTEGRACIÓN AL GRAFO SEMÁNTICO
        log.think(CognitiveAgent::Contextus, "INTEGRACIÓN", "Fase 2: Integrando conocimiento al Grafo Semántico...");
        self.integrate_to_graph(domain_name, &knowledge, graph);
        log.think(CognitiveAgent::Contextus, "INTEGRACIÓN", "Conocimiento integrado. Buscando conexiones...");

        // FASE 3: ANALOGÍAS CROSS-DOMAIN
        log.think(CognitiveAgent::Analogy, "ANALOGÍAS", "Fase 3: Buscando patrones similares en dominios conocidos...");
        let analogies = self.analogy_engine.find_cross_domain_analogies(&knowledge, graph);
        
        if !analogies.is_empty() {
            println!("\n🔥 EUREKA — Analogías descubiertas:");
            for analogy in &analogies {
                println!("   {} es como {} en {} [similitud: {:.0}%]",
                    analogy.new_concept, analogy.known_concept, 
                    analogy.known_domain, analogy.similarity_score * 100.0);
                log.think_with_evidence(
                    CognitiveAgent::Analogy, "DESCUBRIMIENTO", &analogy.insight,
                    vec![format!("{} ↔ {}", analogy.new_concept, analogy.known_concept)],
                    analogy.similarity_score, vec![analogy.known_domain.clone()],
                );
            }
        }

        // FASE 4: LAGUNAS DE CONOCIMIENTO
        log.think(CognitiveAgent::Senku, "VALIDACIÓN", "Fase 4: Buscando lagunas en mi comprensión...");
        let gaps = self.identify_knowledge_gaps(&knowledge);
        if !gaps.is_empty() {
            println!("\n❓ Lagunas de conocimiento:");
            for gap in &gaps { println!("   - {}", gap); }
            log.think_with_evidence(
                CognitiveAgent::Senku, "LAGUNAS",
                &format!("{} preguntas sin respuesta.", gaps.len()),
                gaps.clone(), 0.3, vec![],
            );
        }

        self.learned_domains.insert(domain_name.to_string(), knowledge.clone());
        println!("\n✅ Dominio '{}' aprendido. Listo para practicar.\n", domain_name);
        log.think(CognitiveAgent::System, "COMPLETADO", 
            &format!("Dominio '{}' integrado exitosamente.", domain_name));

        DomainLearningResult {
            domain: domain_name.to_string(),
            entities_learned: knowledge.entities.len(),
            rules_learned: knowledge.rules.len(),
            goals_identified: knowledge.goals.len(),
            constraints_found: knowledge.constraints.len(),
            analogies_discovered: analogies,
            knowledge_gaps: gaps,
            status: "learned".to_string(),
        }
    }

    fn integrate_to_graph(&self, domain: &str, knowledge: &ParsedKnowledge, graph: &mut SemanticGraph) {
        for entity in &knowledge.entities {
            let desc = entity.properties.get("description").cloned().unwrap_or_default();
            let synthesis = SynthesisOutput {
                estado_inicial: format!("[{}] Sin conocimiento de '{}'", domain, entity.name),
                operador_causal: format!("Lectura del manual de {}", domain),
                estado_final: format!("Entidad '{}' integrada: {}", entity.name, desc),
                aplicacion_unreal: format!("Dominio {} - concepto {}", domain, entity.name),
            };
            graph.add_abstraction(domain.to_string(), entity.name.clone(), synthesis);
        }

        for rule in &knowledge.rules {
            let key = format!("domain_{}_rule_{}", domain, rule.id).to_lowercase().replace(' ', "_");
            graph.strengthen_anchor(
                key,
                &format!("{}: {} {}", rule.subject, rule.action, rule.condition),
                1.0, true, 0.0, vec![],
                format!("domain={}, source=manual", domain),
                AnchorSource::WebValidation,
            );
        }

        for goal in &knowledge.goals {
            let synthesis = SynthesisOutput {
                estado_inicial: format!("[{}] Objetivo no logrado", domain),
                operador_causal: "Práctica y aprendizaje".to_string(),
                estado_final: format!("Lograr: {}", goal.description),
                aplicacion_unreal: format!("Dominio {} - meta", domain),
            };
            graph.add_abstraction(domain.to_string(), format!("goal_{}", goal.description), synthesis);
        }
    }

    fn identify_knowledge_gaps(&self, knowledge: &ParsedKnowledge) -> Vec<String> {
        let mut gaps = Vec::new();
        for entity in &knowledge.entities {
            if entity.properties.len() <= 2 {
                gaps.push(format!("¿Propiedades detalladas de '{}'?", entity.name));
            }
        }
        for rule in &knowledge.rules {
            if rule.condition.is_empty() && rule.exceptions.is_empty() {
                gaps.push(format!("Regla '{}' para '{}': ¿Cuándo NO aplica?", rule.action, rule.subject));
            }
        }
        for goal in &knowledge.goals {
            if goal.sub_goals.is_empty() {
                gaps.push(format!("Objetivo '{}': ¿pasos intermedios?", goal.description));
            }
        }
        let defined: std::collections::HashSet<String> = knowledge.entities.iter().map(|e| e.name.to_lowercase()).collect();
        for term in &knowledge.vocabulary {
            if !defined.contains(term) && term.len() > 4 {
                gaps.push(format!("Término '{}' no definido.", term));
            }
        }
        gaps.truncate(10);
        gaps
    }

    pub fn consult_knowledge_for_decision(
        &self, domain: &str, context: &str, graph: &SemanticGraph, log: &mut CognitiveLog,
    ) -> Vec<String> {
        let mut applicable_rules = Vec::new();
        log.think(CognitiveAgent::Contextus, "CONSULTA", 
            &format!("Consultando Grafo para '{}': '{}'", domain, context));

        let domain_prefix = format!("domain_{}_rule_", domain).to_lowercase();
        for (key, anchor) in &graph.empirical_anchors {
            if key.starts_with(&domain_prefix) {
                let claim_lower = anchor.claim.to_lowercase();
                let context_lower = context.to_lowercase();
                let context_words: Vec<&str> = context_lower.split_whitespace().collect();
                let relevance = context_words.iter()
                    .filter(|w| claim_lower.contains(*w))
                    .count() as f32 / context_words.len().max(1) as f32;
                
                if relevance > 0.1 || anchor.confidence > 0.8 {
                    applicable_rules.push(anchor.claim.clone());
                    log.think_with_evidence(
                        CognitiveAgent::Contextus, "REGLA_APLICABLE",
                        &format!("Regla: {} [{:.0}%]", anchor.claim, anchor.confidence * 100.0),
                        vec![format!("Relevancia: {:.0}%", relevance * 100.0)],
                        anchor.confidence, vec![key.clone()],
                    );
                }
            }
        }

        for (_key, abstraction) in &graph.abstraction_anchors {
            if _key.contains(&domain.to_lowercase()) {
                applicable_rules.push(format!("[Analogía] {}", abstraction.original_concept));
            }
        }

        applicable_rules
    }
}
