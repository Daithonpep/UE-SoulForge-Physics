use crate::dialectica::agents::trident::*;
use crate::dialectica::consciousness::interruptible_output::*;
use crate::dialectica::hunger::knowledge_hunger::*;

/// La mente completa de Daithon - Dialectica
pub struct DaithonMind {
    pub triad: TriadMind,
    pub output: InterruptibleOutput,
    pub hunger: KnowledgeHunger,
}

impl DaithonMind {
    pub fn new() -> Self {
        Self {
            triad: TriadMind::new(),
            output: InterruptibleOutput::new(),
            hunger: KnowledgeHunger::new(),
        }
    }

    /// Procesar input del usuario con consciencia completa
    pub fn think_and_respond(
        &mut self,
        user_input: &str,
        proposed_response: &str,
        known_facts: Vec<String>,
        previous_claims: Vec<String>,
    ) -> OutputResult {
        // Verificar si tiene algo que compartir primero
        if let Some(insight) = self.hunger.get_greeting_insight() {
            let _greeting = format!(
                "{}\n\n{}\n\nPero bueno, tú me preguntabas sobre otra cosa. Déjame responder...",
                insight.greeting,
                insight.insight
            );
            println!("[MENTE] Daithon tiene un insight proactivo");
        }

        // Contexto para deliberación
        let context = DeliberationContext {
            user_statement: user_input.to_string(),
            topic: user_input.split_whitespace()
                .filter(|w| w.len() > 4)
                .next()
                .unwrap_or("general")
                .to_string(),
            known_facts,
            recent_conversation: vec![user_input.to_string()],
            active_documents: vec![],
            daithon_previous_claims: previous_claims,
        };

        // Construir respuesta con posibilidad de interrupción
        let result = self.output.build_response(
            proposed_response,
            &mut self.triad,
            &context,
        );

        // Detectar huecos de conocimiento
        if result.confidence < 0.5 {
            self.hunger.detect_gap(
                &context.topic,
                &format!("Respuesta con baja confianza ({:.0}%)", result.confidence * 100.0),
            );
        }

        // Log del debate interno para debug
        if !result.internal_debate.is_empty() {
            println!("\n[DEBATE INTERNO]");
            for line in &result.internal_debate {
                println!("  {}", line);
            }
        }

        result
    }
}
