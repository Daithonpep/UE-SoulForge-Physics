use crate::trinity::judge::naturalness_evaluator::*;
use crate::trinity::agents::conversational_agent::*;

/// Sistema de entrenamiento triangular
pub struct TriangularTrainingLoop {
    pub agent_a: ConversationalAgent,
    pub agent_b: ConversationalAgent,
    pub judge: NaturalnessJudge,
    pub iteration_count: usize,
    pub convergence_threshold: f64,
}

impl TriangularTrainingLoop {
    pub fn new() -> Self {
        Self {
            agent_a: ConversationalAgent::new("Agent_A", PersonalitySeed {
                formality: 0.4,
                verbosity: 0.6,
                humor_tendency: 0.7,
                technical_language: 0.5,
            }),
            agent_b: ConversationalAgent::new("Agent_B", PersonalitySeed {
                formality: 0.6,
                verbosity: 0.5,
                humor_tendency: 0.5,
                technical_language: 0.7,
            }),
            judge: NaturalnessJudge::new(),
            iteration_count: 0,
            convergence_threshold: 0.85,
        }
    }

    /// Ejecutar entrenamiento durante N iteraciones
    pub fn train(&mut self, iterations: usize) {
        log::info!("[TRINITY] Iniciando entrenamiento triangular...");
        log::info!("Objetivo: {} iteraciones o convergencia >{}",iterations, self.convergence_threshold);

        let mut current_semantic_node = 0; // Índice de nodo temático básico

        for i in 0..iterations {
            self.iteration_count += 1;

            // Deriva Temática (Simulada)
            let drift = if i % 15 == 0 { 2 } else { 0 }; // Salto cada 15 turnos (TDAH simulado)
            current_semantic_node = (current_semantic_node + drift) % 6;
            
            // Inyección del caos (Simulada - para prevenir "Colapso Alienígena")
            let bypass_b = i > 0 && i % 10000 == 0; 
            
            let topic = self.get_topic_from_node(current_semantic_node);
            let context_a = ConversationContext {
                previous_exchanges: vec![],
                topic: topic.clone(),
                emotional_tone: "neutral".to_string(),
                formality_level: 0.5,
            };

            let response_a = self.agent_a.generate_response(&topic, &context_a);

            // Juez evalúa a A
            let eval_a = self.judge.evaluate_naturalness("Agent_A", &response_a, &context_a);
            self.agent_a.register_feedback(
                response_a.clone(),
                eval_a.overall_score,
                vec![topic.clone()],
            );

            // B responde
            let mut context_b = context_a.clone();
            context_b.previous_exchanges.push(response_a.clone());

            let response_b = if bypass_b {
                 "Oye, acabo de ver una película increíble anoche.".to_string() // Frase de corpus inyectada
            } else {
                 self.agent_b.generate_response(&response_a, &context_b)
            };

            // Juez evalúa a B
            let eval_b = self.judge.evaluate_naturalness("Agent_B", &response_b, &context_b);
            self.agent_b.register_feedback(
                response_b.clone(),
                eval_b.overall_score,
                vec![topic.clone()],
            );

            // Mirroring: Los agentes aprenden uno del otro
            if eval_a.overall_score > 0.7 {
                self.agent_b.learned_phrases.push(LearnedPhrase {
                    phrase: response_a.clone(),
                    context_tags: vec![topic.clone()],
                    success_rate: eval_a.overall_score,
                    usage_count: 0,
                });
            }

            if eval_b.overall_score > 0.7 && !bypass_b {
                self.agent_a.learned_phrases.push(LearnedPhrase {
                    phrase: response_b,
                    context_tags: vec![topic],
                    success_rate: eval_b.overall_score,
                    usage_count: 0,
                });
            }

            if (i + 1) % 1000 == 0 {
                let avg_a = self.agent_a.recent_performance();
                let avg_b = self.agent_b.recent_performance();
                let combined = (avg_a + avg_b) / 2.0;

                log::info!("\n[Iteración {}]", i + 1);
                log::info!("  Agente A score: {:.3} (Frustración: {:.2})", avg_a, self.agent_a.emotional_state.frustration);
                log::info!("  Agente B score: {:.3} (Frustración: {:.2})", avg_b, self.agent_b.emotional_state.frustration);
                log::info!("  Score combinado: {:.3}", combined);
                log::info!("  Frases aprendidas A: {}", self.agent_a.learned_phrases.len());
                log::info!("  Frases aprendidas B: {}", self.agent_b.learned_phrases.len());

                if combined >= self.convergence_threshold {
                    log::info!("\n✓ ¡Convergencia alcanzada!");
                    break;
                }
            }
        }

        log::info!("\n[TRINITY] Entrenamiento completado.");
    }

    fn get_topic_from_node(&self, node_id: usize) -> String {
        let topics = [
            "diseño 3D abstracto",
            "estructuras de mesas",
            "materiales sostenibles",
            "la estética en los bosques",
            "teoría de colisiones",
            "interacciones humanas",
        ];
        topics[node_id % topics.len()].to_string()
    }

    pub fn fuse_into_daithon(&self) -> UnifiedDaithon {
        let mut combined_phrases = Vec::new();

        combined_phrases.extend(
            self.agent_a.learned_phrases.iter()
                .filter(|p| p.success_rate > 0.75)
                .cloned()
        );

        combined_phrases.extend(
            self.agent_b.learned_phrases.iter()
                .filter(|p| p.success_rate > 0.75)
                .cloned()
        );

        combined_phrases.sort_by(|a, b| a.phrase.cmp(&b.phrase));
        combined_phrases.dedup_by(|a, b| a.phrase == b.phrase);

        UnifiedDaithon {
            learned_natural_language: combined_phrases,
            average_naturalness: (self.agent_a.recent_performance() + self.agent_b.recent_performance()) / 2.0,
            total_conversations: self.iteration_count,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UnifiedDaithon {
    pub learned_natural_language: Vec<LearnedPhrase>,
    pub average_naturalness: f64,
    pub total_conversations: usize,
}
