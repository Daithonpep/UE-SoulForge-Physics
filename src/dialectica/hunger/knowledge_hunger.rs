use crate::dialectica::agents::trident::*;

/// Sistema de aprendizaje proactivo: Daithon investiga
/// cuando detecta lagunas en su conocimiento
pub struct KnowledgeHunger {
    /// Huecos de conocimiento detectados
    pub knowledge_gaps: Vec<KnowledgeGap>,
    
    /// Investigaciones completadas proactivamente
    pub proactive_discoveries: Vec<ProactiveDiscovery>,
    
    /// ¿Daithon tiene algo que contarle al usuario?
    pub pending_insights: Vec<PendingInsight>,
}

#[derive(Debug, Clone)]
pub struct KnowledgeGap {
    pub topic: String,
    pub detected_during: String,
    pub priority: f64,
    pub investigation_status: InvestigationStatus,
}

#[derive(Debug, Clone)]
pub enum InvestigationStatus {
    Detected,
    Investigating,
    HypothesisFormed { hypotheses: Vec<String> },
    Tested { results: Vec<String> },
    Resolved { conclusion: String },
}

#[derive(Debug, Clone)]
pub struct ProactiveDiscovery {
    pub question: String,
    pub answer: String,
    pub confidence: f64,
    pub method: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct PendingInsight {
    pub topic: String,
    pub insight: String,
    pub importance: f64,
    pub greeting: String,
}

impl KnowledgeHunger {
    pub fn new() -> Self {
        Self {
            knowledge_gaps: Vec::new(),
            proactive_discoveries: Vec::new(),
            pending_insights: Vec::new(),
        }
    }

    /// Detectar un hueco de conocimiento
    pub fn detect_gap(&mut self, topic: &str, context: &str) {
        if !self.knowledge_gaps.iter().any(|g| g.topic == topic) {
            println!("[HAMBRE] Hueco detectado: '{}' (durante: {})", topic, context);

            self.knowledge_gaps.push(KnowledgeGap {
                topic: topic.to_string(),
                detected_during: context.to_string(),
                priority: 0.7,
                investigation_status: InvestigationStatus::Detected,
            });
        }
    }

    /// Investigar proactivamente un hueco de conocimiento
    pub async fn investigate_proactively(
        &mut self,
        triad: &mut TriadMind,
    ) {
        let gap = match self.knowledge_gaps.iter_mut()
            .filter(|g| matches!(g.investigation_status, InvestigationStatus::Detected))
            .max_by(|a, b| a.priority.partial_cmp(&b.priority).unwrap())
        {
            Some(g) => g,
            None => return,
        };

        println!("[HAMBRE] Investigando proactivamente: '{}'", gap.topic);
        gap.investigation_status = InvestigationStatus::Investigating;

        let context = DeliberationContext {
            user_statement: format!("Necesito entender: {}", gap.topic),
            topic: gap.topic.clone(),
            known_facts: vec![],
            recent_conversation: vec![],
            active_documents: vec![],
            daithon_previous_claims: vec![],
        };

        let deliberation = triad.deliberate(
            &format!("Investigar: {}", gap.topic),
            &context,
        );

        if !deliberation.proposed_experiments.is_empty() {
            let hypotheses: Vec<String> = deliberation.proposed_experiments.iter()
                .map(|e| e.hypothesis.clone())
                .collect();

            gap.investigation_status = InvestigationStatus::HypothesisFormed {
                hypotheses: hypotheses.clone(),
            };

            for hypothesis in &hypotheses {
                triad.create_research_mission(hypothesis, gap.priority);
            }
        }

        if deliberation.confidence > 0.6 && !deliberation.connections_found.is_empty() {
            let connection = &deliberation.connections_found[0];

            let insight = PendingInsight {
                topic: gap.topic.clone(),
                insight: format!(
                    "Descubrí que {} se conecta con {}: {}",
                    gap.topic, connection.domain_b, connection.connection
                ),
                importance: deliberation.confidence,
                greeting: format!(
                    "Estuve investigando sobre '{}' mientras no estabas, \
                     y encontré algo interesante.",
                    gap.topic
                ),
            };

            self.pending_insights.push(insight);
        }
    }

    /// ¿Tiene Daithon algo que compartir cuando el usuario vuelve?
    pub fn get_greeting_insight(&mut self) -> Option<PendingInsight> {
        if self.pending_insights.is_empty() {
            return None;
        }

        self.pending_insights.sort_by(|a, b| b.importance.partial_cmp(&a.importance).unwrap());
        Some(self.pending_insights.remove(0))
    }
}
