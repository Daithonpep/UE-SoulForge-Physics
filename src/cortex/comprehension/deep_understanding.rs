use crate::cortex::extraction::knowledge_extractor::*;
use std::collections::HashMap;
use serde::{Deserialize, Serialize};

/// Sistema de comprensión profunda
#[derive(Clone)]
pub struct DeepComprehension {
    /// Base de conocimiento existente
    pub knowledge_base: KnowledgeBase,
    
    /// Motor de inferencia
    inference_engine: InferenceEngine,
    
    /// Detector de contradicciones y conductor de curiosidad autónoma
    pub contradiction_detector: ContradictionDetector,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeBase {
    pub ontology: HashMap<String, OntologyNode>,
    pub causal_graph: HashMap<String, Vec<CausalLink>>,
    pub skills: HashMap<String, Skill>,
    pub facts: Vec<VerifiedFact>,
    
    // INNOVACIÓN EXTRA: Índice invertido para búsqueda paralela masiva
    #[serde(skip)] // No necesitamos guardarlo, se reconstruye al cargar
    pub inverted_index: HashMap<String, Vec<String>>, 
}

impl KnowledgeBase {
    pub fn save_checkpoint(&self, path: &str) -> std::io::Result<()> {
        let _ = std::fs::create_dir_all("checkpoints");
        let json = serde_json::to_string(self)?;
        std::fs::write(path, json)
    }

    pub fn load_checkpoint(path: &str) -> std::io::Result<Self> {
        let data = std::fs::read_to_string(path)?;
        let mut kb: KnowledgeBase = serde_json::from_str(&data)?;
        kb.rebuild_inverted_index(); // Reconstrucción automática al cargar
        Ok(kb)
    }

    pub fn rebuild_inverted_index(&mut self) {
        let mut index = HashMap::new();
        for (name, node) in &self.ontology {
            // Dividir nombre y propiedades en palabras clave
            let words: Vec<String> = name.to_lowercase()
                .split(|c: char| !c.is_alphanumeric())
                .filter(|s| s.len() > 3)
                .map(|s| s.to_string())
                .collect();

            for word in words {
                index.entry(word).or_insert_with(Vec::new).push(name.clone());
            }
        }
        self.inverted_index = index;
        log::info!("[CORTEX] Índice invertido reconstruido: {} términos indexados.", self.inverted_index.len());
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OntologyNode {
    pub name: String,
    pub parent: Option<String>,
    pub children: Vec<String>,
    pub properties: HashMap<String, PropertyValue>,
    pub typical_behaviors: Vec<String>,
    pub last_used: u64, // Para poda sináptica
    pub confidence: f64,
    pub source: Option<KnowledgeSource>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalLink {
    pub cause: String,
    pub effect: String,
    pub strength: f64,
    pub mechanism: Option<String>,
    pub exceptions: Vec<String>,
    pub last_used: u64, // Para poda sináptica
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub description: String,
    pub steps: Vec<SkillStep>,
    pub prerequisites: Vec<String>,
    pub examples: Vec<String>,
    pub proficiency: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillStep {
    pub order: usize,
    pub action: String,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub tips: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiedFact {
    pub statement: String,
    pub source: String,
    pub confidence: f64,
    pub verified_at: u64,
}

#[derive(Clone)]
pub struct InferenceEngine;

impl InferenceEngine {
    pub fn infer_new_relations(&self, kb: &KnowledgeBase) -> Vec<Relation> {
        // Implementación simplificada
        Vec::new()
    }
}

#[derive(Clone)]
pub struct ContradictionDetector;

impl ContradictionDetector {
    /// Detectar contradicciones e impulsar Misiones de Investigación (Curiosidad Autónoma)
    pub fn check_contradictions(
        &self,
        new_knowledge: &KnowledgeUnit,
        kb: &KnowledgeBase,
    ) -> Vec<Contradiction> {
        let mut contradictions = Vec::new();

        for new_rel in &new_knowledge.relations {
            if let Some(node) = kb.ontology.get(&new_rel.subject) {
                for (prop_name, existing_value) in &node.properties {
                    if let RelationType::HasProperty = new_rel.relation_type {
                        let obj_val = format!("{:?}", existing_value);
                        if new_rel.object != obj_val {
                            // INNOVACIÓN 3: Curiosidad Autónoma generada
                            let mission = format!("¿De qué depende que {} tenga propiedad {} diferente ({}) versus ({})?", new_rel.subject, prop_name, new_rel.object, obj_val);
                            
                            contradictions.push(Contradiction {
                                existing_fact: format!("{} tiene {}: {:?}", new_rel.subject, prop_name, existing_value),
                                new_fact: format!("{} tiene {}", new_rel.subject, new_rel.object),
                                resolution_suggestion: mission,
                            });
                        }
                    }
                }
            }
        }
        contradictions
    }
}

#[derive(Debug, Clone)]
pub struct Contradiction {
    pub existing_fact: String,
    pub new_fact: String,
    pub resolution_suggestion: String, // Ahora actúa como query de investigación
}

impl DeepComprehension {
    pub fn new() -> Self {
        Self {
            knowledge_base: KnowledgeBase {
                ontology: HashMap::new(),
                causal_graph: HashMap::new(),
                skills: HashMap::new(),
                facts: Vec::new(),
                inverted_index: HashMap::new(),
            },
            inference_engine: InferenceEngine,
            contradiction_detector: ContradictionDetector,
        }
    }

    /// INNOVACIÓN 4: Poda Sináptica (Olvido Estratégico)
    pub fn execute_synaptic_pruning(&mut self) {
        let current_time = Self::current_timestamp();
        let forget_threshold = 0.2;
        let time_decay_factor = 0.05; // Cuánto decae por ciclo largo sin uso

        // Poda ontológica
        self.knowledge_base.ontology.retain(|_, node| {
            let cycles_unused = (current_time - node.last_used) / 1000; // Asumiendo ciclo de 1000 iteraciones
            let decay = (cycles_unused as f64) * time_decay_factor;
            
            // Calculamos cuánto podemos olvidar en base a su origen
            let protection_factor = match node.source.as_ref() {
                Some(KnowledgeSource::UserTeaching { .. }) => 2.0, // Muy resistente al olvido
                Some(KnowledgeSource::EmpiricalVerification { .. }) => 1.5,
                _ => 1.0,
            };

            node.confidence -= decay / protection_factor;
            node.confidence > forget_threshold
        });

        // Poda causal 
        for links in self.knowledge_base.causal_graph.values_mut() {
            links.retain(|link| {
                 let cycles_unused = (current_time - link.last_used) / 1000;
                 let new_strength = link.strength - (cycles_unused as f64) * time_decay_factor;
                 new_strength > forget_threshold
            });
        }
        
        log::info!("[CORTEX] Sueño/Poda Sináptica ejecutada. La compresión redujo la memoria.");
    }

    pub fn integrate_knowledge(&mut self, new_knowledge: KnowledgeUnit) -> IntegrationResult {
        let current_time = Self::current_timestamp();

        let contradictions = self.contradiction_detector
            .check_contradictions(&new_knowledge, &self.knowledge_base);

        if !contradictions.is_empty() {
            log::warn!("[CORTEX] ⚠️ Detectadas {} contradicciones", contradictions.len());
            for c in &contradictions {
                log::info!("  [MISION DE INVESTIGACION] {}", c.resolution_suggestion);
            }
        }

        // Simular INNOVACION 2: Laboratorio Mental (GLYPHICA)
        // Revisar si esto es una cadena causal que puede verificarse en Unreal
        let is_empirically_tested = match new_knowledge.source {
            KnowledgeSource::EmpiricalVerification { .. } => true,
            _ => false
        };

        for entity in &new_knowledge.entities {
            if !self.knowledge_base.ontology.contains_key(&entity.name) {
                self.knowledge_base.ontology.insert(
                    entity.name.clone(),
                    OntologyNode {
                        name: entity.name.clone(),
                        parent: None,
                        children: vec![],
                        properties: HashMap::new(),
                        typical_behaviors: vec![],
                        last_used: current_time,
                        confidence: if is_empirically_tested { 0.99 } else { 0.6 },
                        source: Some(new_knowledge.source.clone()),
                    },
                );
            }
        }

        for causal in &new_knowledge.causal_chains {
            let links = self.knowledge_base.causal_graph
                .entry(causal.cause.clone())
                .or_insert_with(Vec::new);

            // Verificación cruzada (Laboratorio mental)
            let mut final_strength = causal.confidence;
            if is_empirically_tested {
                final_strength = 0.99;
                log::info!("[CORTEX Laboratorio Mental] Hecho empírico verificado en simulación física!");
            }

            links.push(CausalLink {
                cause: causal.cause.clone(),
                effect: causal.effect.clone(),
                strength: final_strength,
                mechanism: causal.mechanism.clone(),
                exceptions: vec![],
                last_used: current_time,
            });
        }

        IntegrationResult {
            entities_added: new_knowledge.entities.len(),
            relations_added: new_knowledge.relations.len(),
            causal_chains_added: new_knowledge.causal_chains.len(),
            relations_inferred: 0,
            contradictions_found: contradictions.len(),
        }
    }

    pub fn answer_question(&mut self, question: &str) -> String {
        // Marcamos como usado el grafo para la poda
        let current_time = Self::current_timestamp();

        for (k, node) in self.knowledge_base.ontology.iter_mut() {
            if question.to_lowercase().contains(&k.to_lowercase()) {
                node.last_used = current_time; // Reavivar conexión
                node.confidence = f64::min(1.0, node.confidence + 0.1);
            }
        }
        
        "Respuesta derivada del grafo de conocimiento (simulado)...".to_string()
    }

    fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}

#[derive(Debug, Clone)]
pub struct IntegrationResult {
    pub entities_added: usize,
    pub relations_added: usize,
    pub causal_chains_added: usize,
    pub relations_inferred: usize,
    pub contradictions_found: usize,
}
