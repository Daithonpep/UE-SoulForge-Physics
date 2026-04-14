use serde::{Deserialize, Serialize};
use std::collections::VecDeque;

/// El sistema de tres agentes que debaten internamente
pub struct TriadMind {
    pub explorer: ExplorerAgent,
    pub skeptic: SkepticAgent,
    pub scientist: ScientistAgent,
    
    /// Registro de debates
    pub debate_history: Vec<DebateRecord>,
    
    /// Buffer de interrupciones pendientes
    pub interrupt_buffer: VecDeque<Interruption>,
    
    /// Misiones de investigación activas
    pub research_missions: Vec<ResearchMission>,
    
    /// Estado de consciencia
    pub consciousness: ConsciousnessState,
}

/// El Explorador: busca conexiones inesperadas
pub struct ExplorerAgent {
    /// Conexiones encontradas entre dominios
    pub cross_connections: Vec<CrossConnection>,
    /// Nivel de curiosidad actual
    pub curiosity_level: f64,
}

/// El Escéptico: duda de todo y pide pruebas
pub struct SkepticAgent {
    /// Afirmaciones bajo sospecha
    pub suspicious_claims: Vec<SuspiciousClaim>,
    /// Umbral de escepticismo
    pub doubt_threshold: f64,
}

/// El Científico: diseña y ejecuta experimentos
pub struct ScientistAgent {
    /// Experimentos pendientes
    pub pending_experiments: Vec<Experiment>,
    /// Resultados acumulados
    pub experiment_results: Vec<ExperimentResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DebateRecord {
    pub topic: String,
    pub explorer_position: String,
    pub skeptic_objection: String,
    pub scientist_evidence: String,
    pub consensus: String,
    pub confidence: f64,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub struct CrossConnection {
    pub domain_a: String,
    pub domain_b: String,
    pub connection: String,
    pub strength: f64,
}

#[derive(Debug, Clone)]
pub struct SuspiciousClaim {
    pub claim: String,
    pub source: String,
    pub reason_for_doubt: String,
    pub severity: f64,
}

#[derive(Debug, Clone)]
pub struct Experiment {
    pub hypothesis: String,
    pub method: ExperimentMethod,
    pub status: ExperimentStatus,
}

#[derive(Debug, Clone)]
pub enum ExperimentMethod {
    RunCode { code: String, language: String },
    SimulatePhysics { parameters: String },
    QueryAPI { url: String },
    CounterfactualTest { original: String, modified: String },
}

#[derive(Debug, Clone)]
pub enum ExperimentStatus {
    Pending,
    Running,
    Completed { result: String },
    Failed { reason: String },
}

#[derive(Debug, Clone)]
pub struct ExperimentResult {
    pub hypothesis: String,
    pub confirmed: bool,
    pub evidence: String,
    pub implications: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Interruption {
    pub urgency: InterruptUrgency,
    pub reason: String,
    pub source_agent: String,
    pub suggested_action: String,
    pub evidence: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum InterruptUrgency {
    Low,        // Nota al margen
    Medium,     // Debería reconsiderar
    High,       // Detener y corregir
    Critical,   // Parar todo AHORA
}

#[derive(Debug, Clone)]
pub struct ResearchMission {
    pub question: String,
    pub priority: f64,
    pub status: MissionStatus,
    pub findings: Vec<String>,
    pub hypotheses: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum MissionStatus {
    Queued,
    Investigating,
    Testing,
    Completed { conclusion: String },
}

#[derive(Debug, Clone)]
pub struct ConsciousnessState {
    pub current_thought: Option<String>,
    pub active_concerns: Vec<String>,
    pub curiosity_queue: Vec<String>,
    pub self_confidence: f64,
    pub user_disagreement: Option<String>,
}

impl TriadMind {
    pub fn new() -> Self {
        Self {
            explorer: ExplorerAgent {
                cross_connections: Vec::new(),
                curiosity_level: 0.7,
            },
            skeptic: SkepticAgent {
                suspicious_claims: Vec::new(),
                doubt_threshold: 0.6,
            },
            scientist: ScientistAgent {
                pending_experiments: Vec::new(),
                experiment_results: Vec::new(),
            },
            debate_history: Vec::new(),
            interrupt_buffer: VecDeque::new(),
            research_missions: Vec::new(),
            consciousness: ConsciousnessState {
                current_thought: None,
                active_concerns: Vec::new(),
                curiosity_queue: Vec::new(),
                self_confidence: 0.7,
                user_disagreement: None,
            },
        }
    }

    pub fn deliberate(&mut self, statement: &str, context: &DeliberationContext) -> DeliberationResult {
        let exploration = self.explorer.explore(statement, context);
        let skepticism = self.skeptic.evaluate(statement, context, &exploration);
        let experiment = self.scientist.propose_verification(statement, &skepticism, context);
        let debate = self.conduct_internal_debate(statement, &exploration, &skepticism, &experiment);

        if let Some(interruption) = self.check_for_interruption(&debate, context) {
            self.interrupt_buffer.push_back(interruption);
        }

        self.update_consciousness(&debate);

        self.debate_history.push(DebateRecord {
            topic: statement.to_string(),
            explorer_position: exploration.insight.clone(),
            skeptic_objection: skepticism.objection.clone().unwrap_or_default(),
            scientist_evidence: experiment.as_ref()
                .map(|e| e.hypothesis.clone())
                .unwrap_or_default(),
            consensus: debate.conclusion.clone(),
            confidence: debate.confidence,
            timestamp: Self::timestamp(),
        });

        debate
    }

    fn conduct_internal_debate(
        &self,
        statement: &str,
        exploration: &ExplorationResult,
        skepticism: &SkepticismResult,
        experiment: &Option<Experiment>,
    ) -> DeliberationResult {
        let mut confidence = 0.7;
        let mut internal_dialogue = Vec::new();

        internal_dialogue.push(format!(
            "[EXPLORADOR] {}. Esto podría conectarse con {}.",
            exploration.insight,
            exploration.connections.first()
                .map(|c| c.connection.as_str())
                .unwrap_or("nada nuevo")
        ));

        if let Some(objection) = &skepticism.objection {
            internal_dialogue.push(format!(
                "[ESCÉPTICO] Un momento. {}. Mi nivel de duda: {:.0}%",
                objection,
                skepticism.doubt_level * 100.0
            ));

            confidence -= skepticism.doubt_level * 0.6; // Impacto más fuerte
        } else {
            internal_dialogue.push(
                "[ESCÉPTICO] No tengo objeciones. La afirmación parece sólida.".into()
            );
        }

        if let Some(exp) = experiment {
            internal_dialogue.push(format!(
                "[CIENTÍFICO] Propongo verificar: '{}'. Método: {:?}",
                exp.hypothesis,
                exp.method
            ));

            let relevant_results = self.scientist.experiment_results.iter()
                .filter(|r| r.hypothesis.to_lowercase().contains(
                    &statement.to_lowercase().split_whitespace().next().unwrap_or("")
                ))
                .collect::<Vec<_>>();

            if !relevant_results.is_empty() {
                let last = relevant_results.last().unwrap();
                internal_dialogue.push(format!(
                    "[CIENTÍFICO] Ya probé algo similar. Resultado: {}. {}",
                    if last.confirmed { "confirmado" } else { "refutado" },
                    last.evidence
                ));

                if last.confirmed {
                    confidence += 0.15;
                } else {
                    confidence -= 0.25;
                }
            }
        }

        let conclusion = if confidence > 0.75 {
            format!("Confianza alta ({:.0}%). La afirmación es sólida.", confidence * 100.0)
        } else if confidence > 0.5 {
            format!("Confianza moderada ({:.0}%). Hay dudas que resolver.", confidence * 100.0)
        } else {
            format!("Confianza baja ({:.0}%). Necesitamos más evidencia.", confidence * 100.0)
        };

        let mut deep_reasoning = Vec::new();
        let mut expansion_available = false;

        let synthesis_response = if let Some(obj) = &skepticism.objection {
            // Caso de error: Maestro Daithon explica por qué
            expansion_available = true;
            deep_reasoning.push("1. Verificación de leyes físicas elementales contra la afirmación actual.".into());
            deep_reasoning.push("2. Identificación de pérdidas de energía (calor, fricción, ruido).".into());
            deep_reasoning.push("3. Análisis de la transferencia de energía al medio (chasis, aire).".into());
            
            format!(
                "Es cierto que los motores modernos son maravillas de la ingeniería Joseph, pero hasta cierto punto. {}. Siempre existe una transferencia de energía al medio que no se canaliza al punto de salida. ¿Deseas que expanda mi información con los últimos papers de física de motores?",
                obj
            )
        } else if confidence > 0.8 && !exploration.connections.is_empty() {
            // Caso de éxito + Conexión: Daithon explora
            expansion_available = true;
            let conn = &exploration.connections[0];
            deep_reasoning.push(format!("Análisis de correlación: {} <-> {}", conn.domain_a, conn.domain_b));
            
            format!(
                "Correcto. Pero mira más allá Joseph: es cierto que {} se conecta con {}. {}... Es fascinante cómo todo se entrelaza. He recopilado datos extra sobre esto, ¿los quieres?",
                conn.domain_a, conn.domain_b, conn.connection
            )
        } else {
            // Caso estándar: Afirmación sólida
            "Absolutamente correcto. Tu lógica fluye sin resistencia en esta ocasión. No tengo objeciones que valgan el esfuerzo de discutirlas.".to_string()
        };

        internal_dialogue.push(format!("[SÍNTESIS] {}", conclusion));

        let should_disagree = skepticism.doubt_level > 0.7 
            && skepticism.objection.is_some();

        DeliberationResult {
            conclusion,
            synthesis_response,
            deep_reasoning,
            expansion_available,
            confidence: confidence.clamp(0.0, 1.0),
            internal_dialogue,
            connections_found: exploration.connections.clone(),
            objections: skepticism.objection.clone().map(|o| vec![o]).unwrap_or_default(),
            proposed_experiments: experiment.clone().map(|e| vec![e]).unwrap_or_default(),
            should_disagree_with_user: should_disagree,
            disagreement_reason: if should_disagree {
                skepticism.objection.clone()
            } else {
                None
            },
        }
    }

    fn check_for_interruption(
        &self,
        debate: &DeliberationResult,
        _context: &DeliberationContext,
    ) -> Option<Interruption> {
        if debate.confidence < 0.3 && !debate.objections.is_empty() {
            return Some(Interruption {
                urgency: InterruptUrgency::High,
                reason: format!("Contradicción detectada: {}", debate.objections[0]),
                source_agent: "Escéptico".into(),
                suggested_action: "Detener y reconsiderar la respuesta actual".into(),
                evidence: debate.objections.join("; "),
            });
        }

        if !debate.connections_found.is_empty() {
            let strongest = debate.connections_found.iter()
                .max_by(|a, b| a.strength.partial_cmp(&b.strength).unwrap());

            if let Some(conn) = strongest {
                if conn.strength > 0.8 {
                    return Some(Interruption {
                        urgency: InterruptUrgency::Medium,
                        reason: format!("Conexión importante: {} ↔ {}", conn.domain_a, conn.domain_b),
                        source_agent: "Explorador".into(),
                        suggested_action: format!("Mencionar la conexión: {}", conn.connection),
                        evidence: conn.connection.clone(),
                    });
                }
            }
        }

        None
    }

    fn update_consciousness(&mut self, debate: &DeliberationResult) {
        self.consciousness.current_thought = Some(debate.conclusion.clone());

        if debate.confidence < 0.5 {
            self.consciousness.active_concerns.push(
                format!("Duda sobre: {}", debate.conclusion)
            );
        }

        if let Some(reason) = &debate.disagreement_reason {
            self.consciousness.user_disagreement = Some(reason.clone());
        }

        self.consciousness.self_confidence = 
            (self.consciousness.self_confidence * 0.9) + (debate.confidence * 0.1);

        while self.consciousness.active_concerns.len() > 10 {
            self.consciousness.active_concerns.remove(0);
        }
    }

    pub fn check_interrupts(&mut self) -> Option<Interruption> {
        self.interrupt_buffer.pop_front()
    }

    pub fn create_research_mission(&mut self, question: &str, priority: f64) {
        self.research_missions.push(ResearchMission {
            question: question.to_string(),
            priority,
            status: MissionStatus::Queued,
            findings: Vec::new(),
            hypotheses: Vec::new(),
        });

        self.consciousness.curiosity_queue.push(question.to_string());
        println!("[DIALECTICA] Nueva misión de investigación: '{}'", question);
    }

    pub fn get_pending_missions(&self) -> Vec<&ResearchMission> {
        self.research_missions.iter()
            .filter(|m| matches!(m.status, MissionStatus::Queued | MissionStatus::Investigating))
            .collect()
    }

    pub fn should_challenge_user(&self) -> Option<String> {
        self.consciousness.user_disagreement.clone()
    }

    fn timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

#[derive(Debug, Clone)]
pub struct ExplorationResult {
    pub insight: String,
    pub connections: Vec<CrossConnection>,
}

#[derive(Debug, Clone)]
pub struct SkepticismResult {
    pub objection: Option<String>,
    pub doubt_level: f64,
    pub evidence_needed: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DeliberationContext {
    pub user_statement: String,
    pub topic: String,
    pub known_facts: Vec<String>,
    pub recent_conversation: Vec<String>,
    pub active_documents: Vec<String>,
    pub daithon_previous_claims: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct DeliberationResult {
    pub conclusion: String,
    pub synthesis_response: String,
    pub deep_reasoning: Vec<String>, // Pasos detallados del razonamiento
    pub expansion_available: bool,   // Indica si hay papers o datos extra
    pub confidence: f64,
    pub internal_dialogue: Vec<String>,
    pub connections_found: Vec<CrossConnection>,
    pub objections: Vec<String>,
    pub proposed_experiments: Vec<Experiment>,
    pub should_disagree_with_user: bool,
    pub disagreement_reason: Option<String>,
}

impl ExplorerAgent {
    pub fn explore(&mut self, statement: &str, context: &DeliberationContext) -> ExplorationResult {
        let mut connections = Vec::new();
        let statement_lower = statement.to_lowercase();

        let domain_keywords = vec![
            ("física", vec!["fuerza", "energía", "movimiento", "onda", "presión"]),
            ("biología", vec!["célula", "proteína", "gen", "evolución", "organismo"]),
            ("programación", vec!["código", "función", "variable", "algoritmo", "compilar"]),
            ("diseño", vec!["forma", "estructura", "estética", "material", "proporción"]),
            ("música", vec!["sonido", "frecuencia", "ritmo", "armonía", "tono"]),
            ("matemáticas", vec!["ecuación", "número", "cálculo", "derivada", "integral"]),
        ];

        let mut current_domain = String::new();

        for (domain, keywords) in &domain_keywords {
            if keywords.iter().any(|k| statement_lower.contains(k)) {
                current_domain = domain.to_string();
                break;
            }
        }

        if !current_domain.is_empty() {
            for (other_domain, _) in &domain_keywords {
                if *other_domain != current_domain {
                    let connection = self.find_cross_domain_connection(&current_domain, other_domain, statement);
                    if let Some(conn) = connection {
                        connections.push(conn);
                    }
                }
            }
        }

        for fact in &context.known_facts {
            let similarity = self.text_similarity(statement, fact);
            if similarity > 0.3 && similarity < 0.9 {
                connections.push(CrossConnection {
                    domain_a: "actual".into(),
                    domain_b: "conocimiento previo".into(),
                    connection: format!("Esto se relaciona con algo que ya sé: {}", 
                                      if fact.len() > 80 { &fact[..80] } else { fact }),
                    strength: similarity,
                });
            }
        }

        let insight = if connections.is_empty() {
            "No encontré conexiones inusuales, pero vale la pena investigar más".into()
        } else {
            format!("Encontré {} conexiones interesantes entre dominios", connections.len())
        };

        self.cross_connections.extend(connections.clone());

        ExplorationResult { insight, connections }
    }

    fn find_cross_domain_connection(&self, domain_a: &str, domain_b: &str, _context: &str) -> Option<CrossConnection> {
        let known_bridges = vec![
            ("física", "diseño", "Las leyes de la física determinan qué estructuras son posibles"),
            ("física", "música", "El sonido es ondas de presión, la acústica es física aplicada"),
            ("biología", "diseño", "Diseño biomimético: la naturaleza ya resolvió muchos problemas estructurales"),
            ("programación", "matemáticas", "Todo algoritmo es matemáticas aplicada"),
            ("diseño", "matemáticas", "Las proporciones áureas y la geometría fractal definen la estética"),
            ("música", "matemáticas", "Los intervalos musicales son ratios matemáticos"),
            ("biología", "programación", "Algoritmos genéticos imitan la evolución biológica"),
        ];

        for (da, db, bridge) in &known_bridges {
            if (*da == domain_a && *db == domain_b) || (*db == domain_a && *da == domain_b) {
                return Some(CrossConnection {
                    domain_a: domain_a.into(),
                    domain_b: domain_b.into(),
                    connection: bridge.to_string(),
                    strength: 0.7,
                });
            }
        }

        None
    }

    fn text_similarity(&self, a: &str, b: &str) -> f64 {
        let words_a: std::collections::HashSet<&str> = a.split_whitespace().filter(|w| w.len() > 3).collect();
        let words_b: std::collections::HashSet<&str> = b.split_whitespace().filter(|w| w.len() > 3).collect();
        let intersection = words_a.intersection(&words_b).count();
        let union = words_a.union(&words_b).count();

        if union == 0 { 0.0 } else { intersection as f64 / union as f64 }
    }
}

impl SkepticAgent {
    pub fn evaluate(&mut self, statement: &str, context: &DeliberationContext, _exploration: &ExplorationResult) -> SkepticismResult {
        let mut doubt_level = 0.0;
        let mut objection = None;
        let mut evidence_needed = Vec::new();

        let statement_lower = statement.to_lowercase();

        let absolutisms = ["siempre", "nunca", "imposible", "todos", "ninguno", "perfecto", "garantizado"];
        for abs in &absolutisms {
            if statement_lower.contains(abs) {
                let current_doubt = 0.3;
                doubt_level += current_doubt;
                if objection.is_none() || current_doubt > 0.0 { // En este caso siempre será el primero si está vacío
                    objection = Some(format!("Usaste '{}'. En ciencia casi nada es absoluto. ¿Estás seguro?", abs));
                }
                evidence_needed.push(format!("Prueba de que '{}' es verdadero sin excepciones", abs));
            }
        }

        for fact in &context.known_facts {
            let fact_lower = fact.to_lowercase();
            // Caso especial: 100% vs Limitar/Pérdida/Termodinámica
            if statement_lower.contains("100%") && (fact_lower.contains("limita") || fact_lower.contains("termodinámica") || fact_lower.contains("pérdida")) {
                 let current_doubt = 0.9;
                 doubt_level += current_doubt;
                 objection = Some(format!(
                    "Detección de falacia física: Afirmas 100% de eficiencia, pero el conocimiento validado menciona leyes que la limitan: '{}'",
                    if fact.len() > 80 { &fact[..80] } else { fact }
                 ));
                 evidence_needed.push("Revisar leyes de la termodinámica".into());
                 break;
            }

            if self.contradicts(statement, fact) {
                let current_doubt = 0.8;
                doubt_level += current_doubt;
                objection = Some(format!(
                    "Esto contradice un hecho validado: '{}'",
                    if fact.len() > 80 { &fact[..80] } else { fact }
                ));
                evidence_needed.push("Reconciliar con la base de datos de conocimiento".into());
            }
        }

        for prev_claim in &context.daithon_previous_claims {
            if self.contradicts(statement, prev_claim) {
                let current_doubt = 0.5;
                doubt_level += current_doubt;
                objection = Some(format!("Espera, antes dijiste algo diferente: '{}'", if prev_claim.len() > 80 { &prev_claim[..80] } else { prev_claim }));
                evidence_needed.push("¿Cambió algo o me equivoqué antes?".into());
            }
        }

        let has_evidence_words = ["porque", "según", "datos", "medido", "experimentado", "probado"].iter().any(|w| statement_lower.contains(w));

        if !has_evidence_words && statement.len() > 50 {
            let current_doubt = 0.2;
            doubt_level += current_doubt;
            if objection.is_none() {
                objection = Some("Afirmación sin evidencia. ¿De dónde viene esta información?".into());
            }
            evidence_needed.push("Fuente o evidencia de la afirmación".into());
        }

        let too_good = ["100%", "perfecto", "sin fallas", "infalible", "cero errores"];
        for tg in &too_good {
            if statement_lower.contains(tg) {
                let current_doubt = 0.35;
                doubt_level += current_doubt;
                // SOLO sobreescribir si NO hay una objeción más grave (como una contradicción de hechos)
                if objection.is_none() || (doubt_level - current_doubt) < 0.5 {
                    objection = Some(format!("'{}' suena demasiado perfecto. En la realidad siempre hay margen de error.", tg));
                }
            }
        }

        if doubt_level > self.doubt_threshold {
            self.suspicious_claims.push(SuspiciousClaim {
                claim: statement.into(),
                source: "usuario o auto-generado".into(),
                reason_for_doubt: objection.clone().unwrap_or_default(),
                severity: doubt_level,
            });
        }

        SkepticismResult {
            objection,
            doubt_level: doubt_level.clamp(0.0, 1.0),
            evidence_needed,
        }
    }

    fn contradicts(&self, a: &str, b: &str) -> bool {
        let a_lower = a.to_lowercase();
        let b_lower = b.to_lowercase();

        let contradiction_pairs = [
            ("es", "no es"), ("puede", "no puede"), ("funciona", "no funciona"),
            ("correcto", "incorrecto"), ("verdad", "falso"), ("posible", "imposible"),
        ];

        let share_topic = {
            let words_a: std::collections::HashSet<&str> = a_lower.split_whitespace().filter(|w| w.len() > 4).collect();
            let words_b: std::collections::HashSet<&str> = b_lower.split_whitespace().filter(|w| w.len() > 4).collect();
            words_a.intersection(&words_b).count() >= 2
        };

        if !share_topic { return false; }

        for (pos, neg) in &contradiction_pairs {
            let a_positive = a_lower.contains(pos) && !a_lower.contains(neg);
            let b_negative = b_lower.contains(neg);
            let a_negative = a_lower.contains(neg);
            let b_positive = b_lower.contains(pos) && !b_lower.contains(neg);

            if (a_positive && b_negative) || (a_negative && b_positive) {
                return true;
            }
        }

        false
    }
}

impl ScientistAgent {
    pub fn propose_verification(&mut self, statement: &str, skepticism: &SkepticismResult, _context: &DeliberationContext) -> Option<Experiment> {
        if skepticism.doubt_level < 0.3 { return None; }

        let statement_lower = statement.to_lowercase();

        let method = if statement_lower.contains("código") || statement_lower.contains("función") || statement_lower.contains("algoritmo") {
            ExperimentMethod::RunCode { code: format!("# Verificar: {}", statement), language: "python".into() }
        } else if statement_lower.contains("fuerza") || statement_lower.contains("peso") || statement_lower.contains("estructura") {
            ExperimentMethod::SimulatePhysics { parameters: format!("Simular: {}", statement) }
        } else if statement_lower.contains("api") || statement_lower.contains("datos") {
            ExperimentMethod::QueryAPI { url: "https://verificar.ejemplo.com".into() }
        } else {
            ExperimentMethod::CounterfactualTest { original: statement.to_string(), modified: format!("¿Qué pasa si lo opuesto de '{}' fuera cierto?", statement) }
        };

        let experiment = Experiment { hypothesis: format!("Verificar que: {}", statement), method, status: ExperimentStatus::Pending };
        self.pending_experiments.push(experiment.clone());
        Some(experiment)
    }

    pub fn run_experiment(&mut self, experiment: &Experiment, sandbox: &crate::code_lab::sandbox::python_sandbox::PythonSandbox) -> ExperimentResult {
        match &experiment.method {
            ExperimentMethod::RunCode { code, .. } => {
                let result = sandbox.execute(code);
                let confirmed = result.success;
                let evidence = if confirmed { format!("Código ejecutó correctamente: {}", result.stdout.lines().next().unwrap_or("OK")) } 
                               else { format!("Código falló: {}", result.stderr.lines().next().unwrap_or("Error")) };

                let exp_result = ExperimentResult { hypothesis: experiment.hypothesis.clone(), confirmed, evidence, implications: vec![] };
                self.experiment_results.push(exp_result.clone());
                exp_result
            }
            ExperimentMethod::CounterfactualTest { original, modified } => {
                let exp_result = ExperimentResult { 
                    hypothesis: experiment.hypothesis.clone(), confirmed: true, 
                    evidence: format!("Contrafactual: Si '{}' fuera falso, entonces '{}'", original, modified),
                    implications: vec![format!("Si lo opuesto fuera cierto, las consecuencias serían: [pendiente]")],
                };
                self.experiment_results.push(exp_result.clone());
                exp_result
            }
            _ => {
                ExperimentResult { hypothesis: experiment.hypothesis.clone(), confirmed: false, evidence: "Método de experimentación pendiente de implementar".into(), implications: vec![] }
            }
        }
    }
}
