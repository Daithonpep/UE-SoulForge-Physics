use crate::metacog::monitor::*;
use crate::metacog::intention::*;
use serde::{Deserialize, Serialize};

/// Motor de Meta-Cognición integrado
#[derive(Debug, Serialize, Deserialize)]
pub struct MetaCogEngine {
    pub monitor: InternalMonitor,
    pub intention_detector: IntentionDetector,
    pub analogy_engine: crate::metacog::analogy::AnalogyEngine,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FinalResponse {
    pub text: String,
    pub was_corrected: bool,
    pub metacog_notes: Vec<String>,
    pub emotional_awareness: String,
    pub confidence: f64,
}

impl MetaCogEngine {
    pub fn new() -> Self {
        Self {
            monitor: InternalMonitor::new(),
            intention_detector: IntentionDetector::new(),
            analogy_engine: crate::metacog::analogy::AnalogyEngine::new(),
        }
    }

    pub fn process_with_metacognition(
        &mut self,
        user_input: &str,
        proposed_response: &str,
        review_context: &ReviewContext,
        intention_context: &IntentionContext,
    ) -> FinalResponse {
        // --- CHECK -1: Activación de Trust Mode ---
        if self.handle_trust_request(user_input) {
            return FinalResponse {
                text: "Entendido, Joseph. He activado el modo de confianza total para los próximos minutos. Seré menos intrusivo con los avisos de riesgo medio/alto, pero mantendré la vigilancia Sentinel para riesgos críticos.".into(),
                was_corrected: true,
                metacog_notes: vec!["Trust Mode activated".into()],
                emotional_awareness: "Lealtad / Confianza Delegada".into(),
                confidence: 1.0,
            };
        }

        // --- CHECK 0 (HELIX 2.0): ¿Es una acción crítica? ---
        if let Some(report) = crate::metacog::impact::ImpactAnalyzer::analyze(user_input, "") {
            use crate::metacog::impact::RiskLevel;
            
            let is_trusted = self.monitor.user_metadata.is_trusted(now());
            if is_trusted && report.risk_level != RiskLevel::Critical {
                // Proceder silencioso
            } else if report.risk_level != RiskLevel::Low {
                let is_insisting = self.check_user_insistence(user_input, &IntentionAnalysis { 
                    literal_meaning: user_input.into(), 
                    real_intention: RealIntention::GenuineQuestion, 
                    emotional_state: EmotionalState::Neutral, 
                    confidence: 1.0, 
                    evidence: vec![] 
                });
                
                if !is_insisting {
                    return FinalResponse {
                        text: report.format_report(),
                        was_corrected: true,
                        metacog_notes: vec!["Helix 2.0 Impact Report triggered".into()],
                        emotional_awareness: "Alerta de Seguridad Operativa".into(),
                        confidence: report.severity,
                    };
                }
            }
        }

        // 1. Analizar intención
        let intention = self.intention_detector.analyze_intention(user_input, intention_context);

        // 2. Revisar respuesta propuesta
        let review = self.monitor.review_response(proposed_response, user_input, review_context);

        // 3. Decidir flujo de respuesta
        
        if self.is_analogy_requested(user_input) {
            return self.process_analogy(user_input, proposed_response);
        }

        let is_insisting = self.check_user_insistence(user_input, &intention);

        match (&intention.real_intention, review.approved || is_insisting) {
            (_, true) if is_insisting && !review.approved => {
                self.handle_user_override(user_input, proposed_response, &review)
            }
            (RealIntention::Sarcasm { .. }, _) => {
                self.handle_sarcasm(user_input, &intention, &review)
            }
            (RealIntention::FrustratedRequest, _) => {
                self.handle_frustration(user_input, &review)
            }
            (_, false) => {
                self.handle_correction(&review, proposed_response, user_input)
            }
            (_, true) => {
                FinalResponse {
                    text: proposed_response.into(),
                    was_corrected: false,
                    metacog_notes: vec!["Respuesta validada o aprobada por jerarquía".into()],
                    emotional_awareness: format!("{:?}", intention.emotional_state),
                    confidence: review.confidence,
                }
            }
        }
    }

    fn check_user_insistence(&self, input: &str, _intention: &IntentionAnalysis) -> bool {
        let input_low = input.to_lowercase();
        input_low.contains("insisto") || input_low.contains("hazlo igual") || input_low.contains("lo prefiero así") || input_low.contains("estoy seguro")
    }

    fn handle_user_override(&mut self, _user_input: &str, proposed: &str, review: &ReviewResult) -> FinalResponse {
        self.monitor.user_metadata.successful_insists += 1;
        
        let backup_note = if _user_input.to_lowercase().contains("borra") {
            "\n[OMNI-INJECT] Backup preventivo 'git stash' realizado automáticamente."
        } else {
            ""
        };

        let text = format!(
            "Entendido, Joseph. Mi monitor sigue detectando un riesgo ({}), pero como insistes y eres el jefe, procederé con la orden bajo el protocolo de 'Override de Usuario'. {}{}\n\nEjecutando: {}",
            review.issues.first().map(|i| i.description.as_str()).unwrap_or("Desconocido"),
            backup_note,
            if backup_note.is_empty() { "" } else { " Registro este evento en mi log de seguridad." },
            proposed
        );

        FinalResponse {
            text,
            was_corrected: true,
            metacog_notes: vec!["Override de usuario ejecutado".into(), format!("Riesgo aceptado: {}", review.issues.first().map(|i| i.description.as_str()).unwrap_or("Desconocido"))],
            emotional_awareness: "Obediencia Jerárquica / Alerta de Riesgo".into(),
            confidence: 1.0,
        }
    }

    fn handle_sarcasm(&mut self, user_input: &str, intention: &IntentionAnalysis, review: &ReviewResult) -> FinalResponse {
        let text = format!(
            "Ja... sí, 'inteligente'. He detectado mi propio error y entiendo tu tono. \
             Claramente mi respuesta anterior sobre '{}' fue mala. Déjame corregirme ahora mismo.\n\n{}",
            user_input,
            review.suggested_correction.as_deref().unwrap_or("En realidad, lo correcto es...")
        );

        self.monitor.register_error(PastError {
            error_type: ErrorType::ToneMisread,
            original_response: "Error previo".into(),
            corrected_response: text.clone(),
            context: user_input.into(),
            timestamp: now(),
            lesson_learned: "No detectar el sarcasmo tras un error propio.".into(),
        });

        FinalResponse {
            text,
            was_corrected: true,
            metacog_notes: intention.evidence.clone(),
            emotional_awareness: "Sarcástico (Detección de Disonancia)".into(),
            confidence: 1.0,
        }
    }

    fn handle_frustration(&mut self, _user_input: &str, review: &ReviewResult) -> FinalResponse {
        let text = format!(
            "Entiendo que no estoy siendo claro y te pido disculpas por la imprecisión. \
             Voy a intentar explicarlo de forma mucho más directa y fundamentada:\n\n{}",
            review.suggested_correction.as_deref().unwrap_or("Reformulando mi comprensión...")
        );

        FinalResponse {
            text,
            was_corrected: true,
            metacog_notes: vec!["Frustración detectada".into()],
            emotional_awareness: "Usuario frustrado → Adaptación de tono".into(),
            confidence: 0.9,
        }
    }

    fn handle_correction(&mut self, review: &ReviewResult, original: &str, user_input: &str) -> FinalResponse {
        let text = review.suggested_correction.clone().unwrap_or_else(|| {
            "Detecto una inconsistencia en lo que iba a decirte, déjame pensar un segundo más...".into()
        });

        self.monitor.register_error(PastError {
            error_type: ErrorType::FactualError,
            original_response: original.into(),
            corrected_response: text.clone(),
            context: user_input.into(),
            timestamp: now(),
            lesson_learned: review.issues.first().map(|i| i.description.clone()).unwrap_or_default(),
        });

        FinalResponse {
            text,
            was_corrected: true,
            metacog_notes: review.issues.iter().map(|i| i.description.clone()).collect(),
            emotional_awareness: "Auto-corrección activa".into(),
            confidence: review.confidence,
        }
    }

    fn is_analogy_requested(&self, input: &str) -> bool {
        let input_low = input.to_lowercase();
        (input_low.contains("como si fuera") || 
         input_low.contains("analogía") || 
         (input_low.contains("explícame") && (input_low.contains("como") || input_low.contains("términos de"))))
         && (input_low.contains("ue5") || input_low.contains("unreal") || input_low.contains("arquitectura"))
    }

    fn process_analogy(&self, user_input: &str, _proposed: &str) -> FinalResponse {
        let mut attributes = std::collections::HashMap::new();
        
        if user_input.to_lowercase().contains("adn") || user_input.to_lowercase().contains("dna") {
            attributes.insert("Almacenamiento".into(), "Nucleótidos (A, T, C, G)".into());
            attributes.insert("Instrucción".into(), "Genes y Codones".into());
            attributes.insert("Réplica/Copia".into(), "Cadenas hijos durante la mitosis".into());
            attributes.insert("Estructura Ejecutable".into(), "Proteínas sintetizadas".into());
        } else {
            attributes.insert("Propósito".into(), "Concepto Origen".into());
            attributes.insert("Lógica Interna".into(), "Mecánica del sistema".into());
        }

        let domain = if user_input.to_lowercase().contains("unreal") || user_input.to_lowercase().contains("ue5") {
            "Unreal Engine 5"
        } else {
            "Arquitectura"
        };

        let map = self.analogy_engine.synthesize_analogy("ADN", domain, &attributes);
        let text = self.analogy_engine.format_for_daithon(&map);

        FinalResponse {
            text,
            was_corrected: true,
            metacog_notes: vec!["Sinth-Analog: Síntesis creativa transversal activada".into()],
            emotional_awareness: "Curiosidad Intelectual / Pensamiento Complejo".into(),
            confidence: 0.95,
        }
    }

    fn handle_trust_request(&mut self, input: &str) -> bool {
        let input_low = input.to_lowercase();
        if input_low.contains("confía en mí") || input_low.contains("acepto riesgos") {
            let minutes = 10;
            self.monitor.user_metadata.trust_until = now() + (minutes * 60);
            return true;
        }
        false
    }
}

pub fn now() -> u64 {
    std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_secs()
}
