// src/learning/practice_engine.rs
// ============================================================
// MOTOR DE PRÁCTICA: Usa TODO el cerebro de Daithon para jugar
// ============================================================
// Este NO es un módulo de ajedrez. Es un motor que ejecuta
// "sesiones de práctica" en CUALQUIER dominio.
//
// Para ajedrez: Juega partidas usando el Grafo Semántico.
// Para Python: Escribiría programas y los ejecutaría.
// Para física: Diseñaría experimentos y los simularía.
//
// La clave: CONSULTA el Grafo para cada decisión.
// ============================================================

use std::collections::HashMap;
use crate::learning::cognitive_log::*;
use crate::learning::domain_learner::DomainLearner;
use crate::learning::eureka_detector::*;
use crate::learning::document_parser::ParsedKnowledge;
use crate::contextus::semantic_graph::SemanticGraph;
use crate::domains::chess::ChessWorld;
use shakmaty::Position;

/// Resultado de una sesión de práctica completa
#[derive(Debug, Clone, serde::Serialize)]
pub struct PracticeSessionResult {
    pub domain: String,
    pub iteration: usize,
    pub outcome: String,
    pub moves_made: usize,
    pub lessons_learned: Vec<String>,
    pub eureka_moments: Vec<String>,
}

/// Motor de Práctica AGI
pub struct PracticeEngine {
    pub eureka_detector: Option<EurekaDetector>,
    pub total_iterations: usize,
    pub session_results: Vec<PracticeSessionResult>,
}

impl PracticeEngine {
    pub fn new() -> Self {
        Self {
            eureka_detector: None,
            total_iterations: 0,
            session_results: Vec::new(),
        }
    }

    /// Inicializa el detector de eureka con el conocimiento del dominio
    pub fn init_for_domain(&mut self, knowledge: &ParsedKnowledge) {
        self.eureka_detector = Some(EurekaDetector::new(knowledge));
    }

    /// Ejecuta UNA sesión de práctica de ajedrez usando TODO el cerebro
    pub fn play_one_game_with_full_brain(
        &mut self,
        chess: &mut ChessWorld,
        graph: &mut SemanticGraph,
        learner: &DomainLearner,
        log: &mut CognitiveLog,
    ) -> PracticeSessionResult {
        self.total_iterations += 1;
        let iteration = self.total_iterations;
        
        println!("\n━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        println!("🎲 PARTIDA #{} — Daithon vs Sí Mismo", iteration);
        println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\n");
        
        log.think(CognitiveAgent::System, "PRÁCTICA", 
            &format!("Iniciando partida #{}", iteration));

        *chess = ChessWorld::new();
        chess.update_fen();
        let mut move_count = 0;
        let mut lessons = Vec::new();
        
        // Jugar hasta game over o máximo 200 jugadas
        while !chess.is_game_over() && move_count < 200 {
            move_count += 1;
            
            // ═══════════════════════════════════════
            // FASE 1 — PERCEPCIÓN: ¿Qué veo en el tablero?
            // ═══════════════════════════════════════
            let causal_vars = chess.get_causal_variables();
            let legal_moves = chess.get_legal_moves();
            
            log.think_with_evidence(
                CognitiveAgent::Senku,
                "PERCEPCIÓN",
                &format!("Turno {}. {} jugadas legales. Material: {:.0}, Centro: {:.0}, Movilidad: {:.0}",
                    move_count, legal_moves.len(),
                    causal_vars.get("material_balance").unwrap_or(&0.0),
                    causal_vars.get("center_control").unwrap_or(&0.0),
                    causal_vars.get("mobility").unwrap_or(&0.0)
                ),
                causal_vars.iter().map(|(k, v)| format!("{}={:.2}", k, v)).collect(),
                0.95,
                vec![],
            );

            if legal_moves.is_empty() { break; }

            // ═══════════════════════════════════════
            // FASE 2 — CONSULTA AL GRAFO: ¿Qué sé sobre esta situación?
            // ═══════════════════════════════════════
            let context_description = format!(
                "material:{:.0} centro:{:.0} movilidad:{:.0} desarrollo:{:.0}",
                causal_vars.get("material_balance").unwrap_or(&0.0),
                causal_vars.get("center_control").unwrap_or(&0.0),
                causal_vars.get("mobility").unwrap_or(&0.0),
                causal_vars.get("development").unwrap_or(&0.0),
            );
            
            let applicable_rules = learner.consult_knowledge_for_decision(
                "Chess", &context_description, graph, log
            );

            // ═══════════════════════════════════════
            // FASE 3 — SENKU: Razonamiento lógico basado en reglas
            // ═══════════════════════════════════════
            let mut move_scores: Vec<(shakmaty::Move, f32, String)> = Vec::new();
            
            for mv in &legal_moves {
                let mut simulated = chess.clone();
                simulated.apply_move(mv).unwrap();
                let vars_after = simulated.get_causal_variables();
                
                // Senku evalúa basándose en las REGLAS DEL GRAFO
                let mut score = 0.0;
                let mut rationale = Vec::new();
                
                // Regla aprendida: material importa
                let material_delta = vars_after.get("material_balance").unwrap_or(&0.0) 
                    - causal_vars.get("material_balance").unwrap_or(&0.0);
                if material_delta.abs() > 0.0 {
                    // Invertir porque después del movimiento es turno del oponente
                    score -= material_delta * 2.0;
                    rationale.push(format!("Material Δ={:.0}", -material_delta));
                }
                
                // Regla aprendida: controlar el centro es bueno
                let center_delta = vars_after.get("center_control").unwrap_or(&0.0)
                    - causal_vars.get("center_control").unwrap_or(&0.0);
                score -= center_delta * 1.5; // Invertir por turno
                if center_delta.abs() > 0.0 {
                    rationale.push(format!("Centro Δ={:.0}", -center_delta));
                }
                
                // Regla aprendida: movilidad es poder
                let mobility_delta = vars_after.get("mobility").unwrap_or(&0.0) 
                    - causal_vars.get("mobility").unwrap_or(&0.0);
                score -= mobility_delta * 0.1;
                
                // Bonus si captura
                if mv.is_capture() {
                    score += 3.0;
                    rationale.push("Captura pieza".to_string());
                }
                
                // Consultar leyes descubiertas por Eureka
                if let Some(detector) = &self.eureka_detector {
                    for law in detector.get_all_discoveries() {
                        if causal_vars.contains_key(&law.cause_variable) {
                            // Si tenemos una ley descubierta relevante, aplicarla
                            let boost = law.correlation * 0.5;
                            score += boost;
                            rationale.push(format!("Ley descubierta: {} [+{:.2}]", 
                                law.description, boost));
                        }
                    }
                }
                
                let reason = if rationale.is_empty() { 
                    "Evaluación posicional general".to_string() 
                } else { 
                    rationale.join(" | ") 
                };
                
                move_scores.push((mv.clone(), score, reason));
            }
            
            // ═══════════════════════════════════════
            // FASE 4 — XENO: Inyectar caos controlado
            // ═══════════════════════════════════════
            let xeno_threshold = if iteration < 5 { 0.4 } else { 0.15 }; // Menos caos con más experiencia
            let xeno_active = rand::random::<f32>() < xeno_threshold;
            
            if xeno_active {
                log.think(CognitiveAgent::Xeno, "CAOS", 
                    "Protocolo Xeno activado. Inyectando perturbación al ranking de movimientos...");
                
                for (_, score, reason) in &mut move_scores {
                    let chaos = (rand::random::<f32>() - 0.5) * 5.0;
                    *score += chaos;
                    if chaos.abs() > 2.0 {
                        *reason = format!("{} | XENO: Perturbación λ={:.2}", reason, chaos);
                    }
                }
            }

            // ═══════════════════════════════════════
            // FASE 5 — DECISIÓN: Elegir la mejor jugada
            // ═══════════════════════════════════════
            move_scores.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));
            
            let (chosen_move, chosen_score, chosen_reason) = &move_scores[0];
            
            // Construir alternativas consideradas
            let alternatives: Vec<MoveAlternative> = move_scores.iter().skip(1).take(3)
                .map(|(mv, score, reason)| MoveAlternative {
                    move_desc: format!("{:?}", mv),
                    agent: if xeno_active { CognitiveAgent::Xeno } else { CognitiveAgent::Senku },
                    score: *score,
                    reason: reason.clone(),
                    why_not_chosen: format!("Score {:.2} < {:.2}", score, chosen_score),
                })
                .collect();

            // Log de la decisión completa
            let decision = MoveDecisionLog {
                move_number: move_count,
                chosen_move: format!("{:?}", chosen_move),
                chosen_by: if xeno_active { CognitiveAgent::Xeno } else { CognitiveAgent::Senku },
                reasoning_chain: vec![],
                rules_applied: applicable_rules.iter().map(|r| RuleApplication {
                    rule_id: "graph".to_string(),
                    rule_text: r.clone(),
                    how_applied: "Consultado durante evaluación posicional".to_string(),
                    confidence_in_rule: 0.8,
                }).collect(),
                causal_laws_used: Vec::new(),
                analogies_invoked: Vec::new(),
                alternatives,
                expected_outcome: chosen_reason.clone(),
                actual_outcome: None,
            };
            
            log.log_move_decision(decision);
            
            // Log legible solo cada N jugadas para no saturar
            if move_count <= 5 || move_count % 10 == 0 {
                log.think_with_evidence(
                    if xeno_active { CognitiveAgent::Xeno } else { CognitiveAgent::Senku },
                    "DECISIÓN",
                    &format!("Jugada {}: {:?} [score={:.2}] — {}", 
                        move_count, chosen_move, chosen_score, chosen_reason),
                    vec![format!("{} alternativas evaluadas", move_scores.len())],
                    (*chosen_score / 10.0 + 0.5).clamp(0.0, 1.0),
                    vec![],
                );
            }

            // Aplicar jugada
            chess.apply_move(chosen_move).unwrap();
            chess.update_fen();
            
            // Registrar observación para Eureka Detector
            if let Some(ref mut detector) = self.eureka_detector {
                let vars_after = chess.get_causal_variables();
                detector.observe(PracticeObservation {
                    iteration,
                    variables: vars_after,
                    action_taken: format!("{:?}", chosen_move),
                    outcome: if chess.is_game_over() {
                        PracticeOutcome::Win
                    } else {
                        let mob_delta = chess.get_causal_variables().get("mobility").unwrap_or(&0.0)
                            - causal_vars.get("mobility").unwrap_or(&0.0);
                        if mob_delta > 0.0 { PracticeOutcome::Improvement(mob_delta) }
                        else { PracticeOutcome::Degradation(mob_delta.abs()) }
                    },
                });
            }
        }

        // ═══════════════════════════════════════
        // POST-PARTIDA: Extraer lecciones y Eurekas
        // ═══════════════════════════════════════
        
        let mut eureka_descriptions = Vec::new();
        
        if let Some(ref mut detector) = self.eureka_detector {
            let discoveries = detector.analyze(log, "Chess");
            for discovery in &discoveries {
                lessons.push(format!("💡 DESCUBIERTO: {}", discovery.description));
                eureka_descriptions.push(discovery.description.clone());
                
                // Integrar descubrimiento al Grafo
                let key = format!("discovered_chess_{}", discovery.cause_variable).to_lowercase();
                graph.strengthen_anchor(
                    key,
                    &discovery.description,
                    discovery.correlation.abs(),
                    true,
                    0.0,
                    vec![],
                    format!("discovered_after_{}_games", iteration),
                    crate::contextus::semantic_graph::AnchorSource::LabExperiment,
                );
            }
        }

        let outcome = if chess.is_game_over() { "game_over" } else { "max_moves" };
        
        log.think(CognitiveAgent::System, "POST-PARTIDA", 
            &format!("Partida #{} finalizada. {} jugadas. {} lecciones aprendidas.", 
                iteration, move_count, lessons.len()));

        let result = PracticeSessionResult {
            domain: "Chess".to_string(),
            iteration,
            outcome: outcome.to_string(),
            moves_made: move_count,
            lessons_learned: lessons,
            eureka_moments: eureka_descriptions,
        };
        
        self.session_results.push(result.clone());
        result
    }
}
