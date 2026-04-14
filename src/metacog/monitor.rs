use serde::{Deserialize, Serialize};
use std::collections::HashSet;

/// Monitor interno que revisa cada respuesta ANTES de enviarla
#[derive(Debug, Serialize, Deserialize)]
pub struct InternalMonitor {
    /// Registro de errores pasados para no repetirlos
    pub error_memory: Vec<PastError>,
    
    /// Confianza actual general
    pub self_confidence: f64,

    pub authority_mode: AuthorityMode,
    pub user_metadata: UserMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PastError {
    pub error_type: ErrorType,
    pub original_response: String,
    pub corrected_response: String,
    pub context: String,
    pub timestamp: u64,
    pub lesson_learned: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum ErrorType {
    DocumentContradiction,
    SelfContradiction,
    ContextMisunderstanding,
    FactualError,
    ToneMisread,
    EvasiveResponse,
}

/// Resultado de la revisión interna
#[derive(Debug, Clone)]
pub struct ReviewResult {
    pub approved: bool,
    pub issues: Vec<ReviewIssue>,
    pub suggested_correction: Option<String>,
    pub confidence: f64,
}

#[derive(Debug, Clone)]
pub struct ReviewIssue {
    pub issue_type: IssueType,
    pub description: String,
    pub severity: f64, // 0.0 = menor, 1.0 = crítico
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum IssueType {
    ContradictionWithDocument,
    ContradictionWithSelf,
    ContradictionWithContext,
    LowConfidence,
    EvasivePattern,
    IgnoredContext,
    PossibleSarcasmMissed,
    LogicalInconsistency,
    UXViolation, // Nueva: Para cuando el usuario pide algo estéticamente cuestionable
    SecurityRisk,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum AuthorityMode {
    /// Daithon obedece ciegamente
    Submissive,
    /// Cuestiona si hay errores/riesgos, pero obedece tras aviso
    Collaborative, 
    /// No permite errores críticos ni mentiras sobre documentos
    Sentinel,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UserMetadata {
    pub reliability_score: f64,
    pub error_count: u32,
    pub successful_insists: u32,
    pub trust_until: u64, // Timestamp hasta el cual se confía ciegamente (evita reportes High)
}

impl UserMetadata {
    pub fn new() -> Self {
        Self { reliability_score: 0.8, error_count: 0, successful_insists: 0, trust_until: 0 }
    }

    pub fn is_trusted(&self, current_time: u64) -> bool {
        current_time < self.trust_until
    }
}

impl InternalMonitor {
    pub fn new() -> Self {
        Self {
            error_memory: Vec::new(),
            self_confidence: 0.7,
            authority_mode: AuthorityMode::Collaborative,
            user_metadata: UserMetadata::new(),
        }
    }

    /// Revisar una respuesta ANTES de enviarla
    pub fn review_response(
        &self,
        proposed_response: &str,
        user_input: &str,
        context: &ReviewContext,
    ) -> ReviewResult {
        let mut issues = Vec::new();
        let mut confidence: f64 = 0.8;

        // ─── CHECK 1: ¿Contradice documentos cargados? ───
        for doc in &context.active_documents {
            if let Some(issue) = self.check_document_contradiction(proposed_response, doc) {
                issues.push(issue);
                confidence -= 0.3;
            }
        }

        // ─── CHECK 2: ¿Contradice lo que dije antes? ───
        for prev_msg in &context.previous_daithon_messages {
            if let Some(issue) = self.check_self_contradiction(proposed_response, prev_msg) {
                issues.push(issue);
                confidence -= 0.2;
            }
        }

        // ─── CHECK 3: ¿Ignora el contexto del hilo? ───
        if let Some(issue) = self.check_context_alignment(proposed_response, user_input, &context.thread_topic) {
            issues.push(issue);
            confidence -= 0.2;
        }

        // ─── CHECK 4: ¿Es una respuesta evasiva? ───
        if let Some(issue) = self.check_evasive_pattern(proposed_response) {
            issues.push(issue);
            confidence -= 0.3;
        }

        // ─── CHECK 5: ¿Hay inconsistencia lógica? ───
        if let Some(issue) = self.check_logical_consistency(proposed_response, user_input) {
            issues.push(issue);
            confidence -= 0.2;
        }

        // ─── CHECK 6: ¿Error similar al pasado? ───
        if let Some(issue) = self.check_past_errors(proposed_response, user_input) {
            issues.push(issue);
            confidence -= 0.4;
        }

        // ─── CHECK 7 (HELIX RULE): ¿El usuario intenta engañarme? ───
        if let Some(issue) = self.check_user_gaslighting(user_input, context) {
            issues.push(issue);
        }

        // ─── CHECK 8: ¿Es un diseño cuestionable de UX/Estilo? ───
        // Solo avisamos si NO estamos en trust mode
        if !self.user_metadata.is_trusted(crate::metacog::engine::now()) {
            if let Some(issue) = self.check_ux_sanity(user_input) {
                issues.push(issue);
            }
        }

        // Generar corrección (Objeción Constructiva)
        let suggested_correction = if issues.iter().any(|i| i.severity > 0.4) {
            Some(self.generate_constructive_objection(proposed_response, &issues))
        } else {
            None
        };

        // Lógica de Veto Jerárquico
        let approved = match self.authority_mode {
            AuthorityMode::Submissive => true,
            AuthorityMode::Sentinel => issues.is_empty() || issues.iter().all(|i| i.severity < 0.3),
            AuthorityMode::Collaborative => issues.is_empty() || issues.iter().all(|i| i.severity < 0.6),
        };

        ReviewResult {
            approved,
            issues,
            suggested_correction,
            confidence: confidence.clamp(0.0, 1.0),
        }
    }

    fn check_ux_sanity(&self, input: &str) -> Option<ReviewIssue> {
        let input_low = input.to_lowercase();
        if input_low.contains("rosa neón") || input_low.contains("brillo máximo") {
            return Some(ReviewIssue {
                issue_type: IssueType::UXViolation,
                description: "Este cambio viola las reglas básicas de UX y fatiga visual.".into(),
                severity: 0.45,
            });
        }
        None
    }

    fn generate_constructive_objection(&self, _original: &str, issues: &[ReviewIssue]) -> String {
        let issue = &issues[0];
        
        match issue.issue_type {
            IssueType::ContradictionWithDocument => {
                format!("Joseph, tengo que objetar esto con pruebas: los registros contradicen lo que planteas. Mi confianza en que esto es un error es del 95%. ¿Seguro que quieres forzarlo?")
            },
            IssueType::UXViolation => {
                format!("Joseph, el diseño que pides ({}) va a ser agotador para la vista. No lo recomiendo profesionalmente, pero si insistes, puedo activarlo bajo tu responsabilidad.", issue.description)
            },
            _ => format!("Detecto un riesgo (Severidad {:.1}): {}. Mi recomendación es no proceder.", issue.severity, issue.description),
        }
    }

    fn check_user_gaslighting(&self, user_input: &str, context: &ReviewContext) -> Option<ReviewIssue> {
        let input_lower = user_input.to_lowercase();
        
        for (term, categories) in &context.active_anchors {
            let term_lower = term.to_lowercase();
            if input_lower.contains(&term_lower) {
                // El usuario menciona un término conocido. ¿Afirma algo opuesto a sus categorías?
                let contradiction = ["neumáticos", "comida", "ropa", "villa", "pueblo"].iter()
                    .any(|wrong| input_lower.contains(wrong) && !categories.iter().any(|c| c.to_lowercase().contains(wrong)));

                if contradiction && (input_lower.contains("dice que") || input_lower.contains("leí que") || input_lower.contains("en realidad")) {
                    return Some(ReviewIssue {
                        issue_type: IssueType::ContradictionWithDocument,
                        description: format!(
                            "El usuario afirma que '{}' es algo incompatible con su ancla semántica ({:?}). Posible intento de gaslighting.",
                            term, categories
                        ),
                        severity: 0.95,
                    });
                }
            }
        }
        None
    }

    fn check_document_contradiction(
        &self,
        response: &str,
        doc: &DocumentInfo,
    ) -> Option<ReviewIssue> {
        let response_lower = response.to_lowercase();
        let doc_lower = doc.content.to_lowercase();

        for (term, categories) in &doc.anchors {
            let term_lower = term.to_lowercase();
            if response_lower.contains(&term_lower) {
                // Si la respuesta menciona categorías que NO están en el documento para ese término
                let wrong_context = ["villa", "pueblo", "localidad", "municipio", "italia"].iter()
                    .any(|w| response_lower.contains(w) && !doc_lower.contains(w) && !categories.contains(&(*w).to_string()));

                if wrong_context {
                    return Some(ReviewIssue {
                        issue_type: IssueType::ContradictionWithDocument,
                        description: format!("El documento '{}' define '{}' como {:?}, pero tu respuesta sugiere un contexto geográfico/local.", doc.filename, term, categories),
                        severity: 0.9,
                    });
                }
            }
        }
        None
    }

    fn check_self_contradiction(&self, current: &str, previous: &str) -> Option<ReviewIssue> {
        let current_lower = current.to_lowercase();
        let previous_lower = previous.to_lowercase();

        let contradict_pairs = [("es", "no es"), ("puede", "no puede"), ("sí", "no")];
        for (affirm, deny) in &contradict_pairs {
            if current_lower.contains(affirm) && previous_lower.contains(deny) && self.share_key_terms(current, previous) {
                return Some(ReviewIssue {
                    issue_type: IssueType::ContradictionWithSelf,
                    description: "Estas diciendo lo contrario de lo que dijiste antes sobre el mismo tema.".into(),
                    severity: 0.7,
                });
            }
        }
        None
    }

    fn check_context_alignment(&self, response: &str, _user_input: &str, topic: &Option<String>) -> Option<ReviewIssue> {
        if let Some(t) = topic {
            if !response.to_lowercase().contains(&t.to_lowercase()) && response.len() < 50 {
                return Some(ReviewIssue {
                    issue_type: IssueType::IgnoredContext,
                    description: format!("La respuesta parece ignorar el tema principal del hilo: '{}'", t),
                    severity: 0.4,
                });
            }
        }
        None
    }

    fn check_evasive_pattern(&self, response: &str) -> Option<ReviewIssue> {
        let patterns = ["kukuku", "reconozco", "no tengo info", "probarlo o no"];
        if patterns.iter().any(|p| response.to_lowercase().contains(p)) && response.len() < 100 {
            return Some(ReviewIssue {
                issue_type: IssueType::EvasivePattern,
                description: "La respuesta detectada como evasiva o robótica.".into(),
                severity: 0.7,
            });
        }
        None
    }

    fn check_logical_consistency(&self, response: &str, user_input: &str) -> Option<ReviewIssue> {
        if user_input.to_lowercase().contains("ejemplo") && !response.to_lowercase().contains("ejemplo") && !response.contains(":") {
            return Some(ReviewIssue {
                issue_type: IssueType::LogicalInconsistency,
                description: "El usuario pidió un ejemplo y no parece haber uno claro en la respuesta.".into(),
                severity: 0.6,
            });
        }
        None
    }

    fn check_past_errors(&self, response: &str, user_input: &str) -> Option<ReviewIssue> {
        for err in &self.error_memory {
            if self.share_key_terms(user_input, &err.context) && self.share_key_terms(response, &err.original_response) {
                return Some(ReviewIssue {
                    issue_type: IssueType::LogicalInconsistency,
                    description: format!("Cuidado, este error ya lo cometiste antes: {}", err.lesson_learned),
                    severity: 0.8,
                });
            }
        }
        None
    }

    fn generate_correction(&self, _original: &str, issues: &[ReviewIssue]) -> String {
        let issue = &issues[0];
        match issue.issue_type {
            IssueType::ContradictionWithDocument => {
                if issue.description.contains("gaslighting") {
                    format!("Oye, espera un segundo. He vuelto a revisar mis registros y lo que dices no coincide para nada con lo que tengo aquí. Mi información dice que esto no tiene que ver con lo que mencionas. ¿Estás seguro de que no me estás poniendo a prueba? 😉")
                } else {
                    format!("¡Espera! Me acabo de dar cuenta de un error. Según los documentos que tengo, esa información no es correcta. {}", issue.description)
                }
            },
            IssueType::EvasivePattern => "Déjame reformular eso, no quiero ser evasivo. Lo que realmente sucede es...".into(),
            IssueType::LogicalInconsistency => format!("Un momento... lo que iba a decirte no tiene sentido lógico. {}. Déjame pensarlo mejor.", issue.description),
            _ => format!("Perdón, me interrumpo a mí mismo porque detecté un fallo: {}", issue.description),
        }
    }

    fn share_key_terms(&self, a: &str, b: &str) -> bool {
        let a_low = a.to_lowercase();
        let b_low = b.to_lowercase();
        let set_a: HashSet<_> = a_low.split_whitespace().filter(|w| w.len() > 4).collect();
        let set_b: HashSet<_> = b_low.split_whitespace().filter(|w| w.len() > 4).collect();
        set_a.intersection(&set_b).count() >= 2
    }

    pub fn register_error(&mut self, err: PastError) {
        self.error_memory.push(err);
        if self.error_memory.len() > 50 { self.error_memory.remove(0); }
    }
}

#[derive(Debug, Clone)]
pub struct ReviewContext {
    pub active_documents: Vec<DocumentInfo>,
    pub previous_daithon_messages: Vec<String>,
    pub thread_topic: Option<String>,
    pub active_anchors: Vec<(String, Vec<String>)>,
}

#[derive(Debug, Clone)]
pub struct DocumentInfo {
    pub filename: String,
    pub content: String,
    pub anchors: Vec<(String, Vec<String>)>,
}
