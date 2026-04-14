use crate::dialectica::agents::trident::*;

/// Sistema de salida que puede ser interrumpido por Trinity
pub struct InterruptibleOutput {
    /// Buffer de texto en construcción
    pub output_buffer: String,
    
    /// ¿La salida fue interrumpida?
    pub was_interrupted: bool,
    
    /// Razón de la interrupción
    pub interruption_reason: Option<String>,
    
    /// Texto de corrección post-interrupción
    pub correction_text: Option<String>,
}

impl InterruptibleOutput {
    pub fn new() -> Self {
        Self {
            output_buffer: String::new(),
            was_interrupted: false,
            interruption_reason: None,
            correction_text: None,
        }
    }

    /// Construir respuesta con posibilidad de interrupción
    pub fn build_response(
        &mut self,
        proposed_text: &str,
        triad: &mut TriadMind,
        context: &DeliberationContext,
    ) -> OutputResult {
        // Deliberar profundamente sobre la intención
        let deliberation = triad.deliberate(&context.user_statement, context);
        self.output_buffer = proposed_text.to_string();

        let final_text = if deliberation.confidence < 0.4 {
            // Caso de MAESTRO DAITHON (Corrección detallada)
            self.was_interrupted = true;
            let mut steps = String::new();
            for step in &deliberation.deep_reasoning {
                steps.push_str(&format!("\n  - {}", step));
            }
            
            format!(
                "¡Cuidado Joseph! {}\n\nRAZONAMIENTO ESTRUCTURAL:\n{}\n\n{}",
                deliberation.synthesis_response,
                steps,
                if deliberation.expansion_available { "¿Deseas que profundice en los informes de laboratorio sobre esto?" } else { "" }
            )
        } else if deliberation.confidence < 0.75 {
            // Caso de DUDA o MATIZ
            format!(
                "{}... Pero espera, mi Escéptico me está dando toques de atención: {}.\n\nJoseph, sugiero proceder con cautela y verificar estos puntos.",
                proposed_text,
                deliberation.objections.get(0).unwrap_or(&"Hay matices que no cuadran".to_string())
            )
        } else {
            // Caso de ÉXITO (con posible INSIGHT)
            if deliberation.connections_found.is_empty() {
                proposed_text.to_string()
            } else {
                let insight = deliberation.synthesis_response.clone();
                format!("{}\n\n[INSIGHT PROFUNDO] {}", proposed_text, insight)
            }
        };

        self.output_buffer = final_text.clone();

        OutputResult {
            text: final_text,
            was_interrupted: self.was_interrupted,
            confidence: deliberation.confidence,
            internal_debate: deliberation.internal_dialogue,
            should_challenge_user: deliberation.should_disagree_with_user,
            challenge_reason: deliberation.disagreement_reason,
        }
    }
}

#[derive(Debug, Clone)]
pub struct OutputResult {
    pub text: String,
    pub was_interrupted: bool,
    pub confidence: f64,
    pub internal_debate: Vec<String>,
    pub should_challenge_user: bool,
    pub challenge_reason: Option<String>,
}
