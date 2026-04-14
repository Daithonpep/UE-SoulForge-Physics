use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Representa una pieza de conocimiento extraída
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KnowledgeUnit {
    pub id: String,
    pub source: KnowledgeSource,
    pub entities: Vec<Entity>,
    pub properties: Vec<Property>,
    pub relations: Vec<Relation>,
    pub actions: Vec<Action>,
    pub causal_chains: Vec<CausalChain>,
    pub confidence: f64,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum KnowledgeSource {
    Wikipedia { article: String, section: String },
    Document { filename: String, page: usize },
    ArXiv { paper_id: String, section: String },
    Reddit { subreddit: String, thread_id: String },
    YouTube { video_id: String, timestamp: f64 },
    Code { repo: String, file: String, lines: (usize, usize) },
    UserTeaching { session_id: String },
    EmpiricalVerification { simulation_id: String }, // VERIFICADO POR GLYPHICA/UNREAL
    WebResearch { topic: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entity {
    pub name: String,
    pub entity_type: EntityType,
    pub aliases: Vec<String>,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityType {
    Object,          // Mesa, silla, manzana
    Concept,         // Gravedad, diseño, algoritmo
    Process,         // Fotosíntesis, compilación
    Material,        // Madera, acero, plástico
    Quantity,        // 5 metros, 100kg
    Location,        // Guatemala, Europa
    Person,          // Einstein, Senku
    Organization,    // NASA, Google
    Code,            // Función, clase, variable
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Property {
    pub entity: String,
    pub property_name: String,
    pub value: PropertyValue,
    pub unit: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PropertyValue {
    Text(String),
    Number(f64),
    Boolean(bool),
    Categorical(Vec<String>),
    Range { min: f64, max: f64 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relation {
    pub subject: String,
    pub relation_type: RelationType,
    pub object: String,
    pub confidence: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RelationType {
    IsA,             // Manzana ES UNA fruta
    PartOf,          // Rueda ES PARTE DE carro
    HasProperty,     // Manzana TIENE color rojo
    Causes,          // Fuego CAUSA calor
    Requires,        // Fotosíntesis REQUIERE luz
    ProducesOutput,  // Función PRODUCE resultado
    UsedFor,         // Martillo USADO PARA clavar
    LocatedIn,       // Paris UBICADO EN Francia
    CreatedBy,       // Teoría CREADA POR Einstein
    Implements,      // Clase IMPLEMENTA interfaz
    DependsOn,       // Módulo DEPENDE DE librería
    Contradicts,     // Teoría A CONTRADICE Teoría B
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    pub name: String,
    pub actor: Option<String>,
    pub inputs: Vec<String>,
    pub outputs: Vec<String>,
    pub preconditions: Vec<String>,
    pub postconditions: Vec<String>,
    pub steps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalChain {
    pub cause: String,
    pub effect: String,
    pub mechanism: Option<String>,
    pub conditions: Vec<String>,
    pub confidence: f64,
}

/// Extractor principal de conocimiento: Ojo Analítico (Neurológico)
#[derive(Clone)]
pub struct KnowledgeExtractor {
    pub use_local_llm: bool,
}

impl KnowledgeExtractor {
    pub fn new() -> Self {
        Self {
            use_local_llm: true, // Simulación de extracción semántica real
        }
    }

    /// Extraer conocimiento de un texto usando un "LLM Local Ligero" (Simulado aquí)
    pub fn extract_from_text(
        &self,
        text: &str,
        source: KnowledgeSource,
    ) -> KnowledgeUnit {
        // INNOVACIÓN 1: Extracción semántica en vez de regex.
        // En código real, haríamos fetch a un modelo local tipo Llama cuantizado
        // que retorne `KnowledgeUnit` en JSON directamente. 
        log::info!("[CORTEX Eye] Procesando texto mediante LLM semántico (Simulado)...");
        
        let mut mock_entities = Vec::new();
        let mut mock_relations = Vec::new();
        let mut mock_causal_chains = Vec::new();
        let mut mock_actions = Vec::new();

        // Análisis ligero simulado:
        let lower = text.to_lowercase();
        if lower.contains("porque") || lower.contains("causa") {
            mock_causal_chains.push(CausalChain {
                cause: "Entidad A".to_string(), // Simulado del LLM
                effect: "Efecto B".to_string(), // Simulado del LLM
                mechanism: Some("Razón extraída del LLM local".to_string()),
                conditions: vec![],
                confidence: 0.85,
            });
        }
        if lower.contains("es un") || lower.contains("es una") {
            mock_relations.push(Relation {
                subject: "Objeto extraído".to_string(),
                relation_type: RelationType::IsA,
                object: "Categoría abstracta".to_string(),
                confidence: 0.90,
            });
            mock_entities.push(Entity {
                name: "Objeto extraído".to_string(),
                entity_type: EntityType::Object,
                aliases: vec![],
                description: None,
            });
        }

        KnowledgeUnit {
            id: uuid::Uuid::new_v4().to_string(),
            source,
            entities: mock_entities,
            properties: vec![],
            relations: mock_relations,
            actions: mock_actions,
            causal_chains: mock_causal_chains,
            confidence: 0.85, // LLM confidencia
            timestamp: Self::current_timestamp(),
        }
    }

    fn current_timestamp() -> u64 {
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
}
