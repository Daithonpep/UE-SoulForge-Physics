use std::collections::{HashMap, HashSet, VecDeque};
use serde::{Serialize, Deserialize};

/// Variable en el modelo causal del mundo
#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub struct Variable {
    pub name: String,
    pub domain: Domain,
    pub value_type: ValueType,
}

#[derive(Debug, Clone, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum Domain {
    Physics,        // gravedad, masa, velocidad
    Rendering,      // draw_calls, fps, shaders
    Structural,     // stress, tension, collapse
    Topology,       // manifold, edges, faces
    Biology,        // rna, proteins, cells
    Logic,          // condiciones, bucles
    Strategic,      // planificación, riesgo, recursos
    Chess,          // piezas, posición, táctica
    Semantic,       // significado, intención, contexto
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)] // Added Eq, Hash manually, may need f32 workaround later
pub enum ValueType {
    Continuous(String), // Changed f32 to String representation for Hash/Eq compatibility
    Discrete(i32),      
    Categorical(String), 
    Boolean,            
}

/// Relación causal entre dos variables
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CausalRelationship {
    /// A causa B directamente
    Direct { 
        strength: f32,      // 0.0 a 1.0
        delay: f32,         // retardo temporal
    },
    
    /// A previene/inhibe B
    Inhibitory { 
        strength: f32,
    },
    
    /// A y B juntos causan C (interacción)
    Interactive { 
        partner: String,
        strength: f32,
    },
    
    /// A causa B a través de un mediador
    Mediated { 
        mediator: String,
        strength: f32,
    },
}

/// Ley causal descubierta
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalLaw {
    pub id: String,
    pub cause: Variable,
    pub effect: Variable,
    pub relationship: CausalRelationship,
    pub confidence: f32,           // 0.0 a 1.0
    pub evidence: Vec<String>,     // IDs de experimentos
    pub constraints: Vec<Constraint>,
    pub timestamp: u64,
}

/// Condiciones bajo las cuales la ley aplica
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    pub variable: String,
    pub condition: ConditionType,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ConditionType {
    Equals,
    GreaterThan,
    LessThan,
    InRange { min: f32, max: f32 },
}

/// Intervención: cambio deliberado en una variable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Intervention {
    pub variable: Variable,
    pub old_value: ValueType,
    pub new_value: ValueType,
    pub timestamp: u64,
}

/// Predicción de consecuencias de una intervención
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Prediction {
    pub intervention: Intervention,
    pub consequences: Vec<ConsequencePrediction>,
    pub confidence: f32,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConsequencePrediction {
    pub variable: Variable,
    pub predicted_change: f32,
    pub old_value: ValueType,
    pub new_value: ValueType,
    pub confidence: f32,
}

/// Grafo Dirigido Acíclico de causalidad (DAG)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalDAG {
    pub nodes: HashSet<String>,           // IDs de variables
    pub edges: HashMap<String, Vec<String>>, // causa -> [efectos]
}

impl CausalDAG {
    pub fn new() -> Self {
        Self {
            nodes: HashSet::new(),
            edges: HashMap::new(),
        }
    }
    
    pub fn add_edge(&mut self, cause: &str, effect: &str) {
        self.nodes.insert(cause.to_string());
        self.nodes.insert(effect.to_string());
        self.edges.entry(cause.to_string())
            .or_insert_with(Vec::new)
            .push(effect.to_string());
    }
    
    /// Encuentra todos los descendientes de una variable
    pub fn find_descendants(&self, var: &str) -> Vec<String> {
        let mut visited = HashSet::new();
        let mut result = Vec::new();
        let mut queue = VecDeque::new();
        
        queue.push_back(var.to_string());
        
        while let Some(current) = queue.pop_front() {
            if visited.contains(&current) { continue; }
            visited.insert(current.clone());
            
            if let Some(children) = self.edges.get(&current) {
                for child in children {
                    if !visited.contains(child) {
                        result.push(child.clone());
                        queue.push_back(child.clone());
                    }
                }
            }
        }
        
        result
    }
    
    /// Verifica si agregar una arista crearía un ciclo
    pub fn would_create_cycle(&self, cause: &str, effect: &str) -> bool {
        // BFS desde effect para ver si llegamos a cause
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        
        queue.push_back(effect.to_string());
        
        while let Some(current) = queue.pop_front() {
            if current == cause {
                return true; // Ciclo detectado
            }
            
            if visited.contains(&current) { continue; }
            visited.insert(current.clone());
            
            if let Some(children) = self.edges.get(&current) {
                for child in children {
                    queue.push_back(child.clone());
                }
            }
        }
        
        false
    }
}

/// Modelo completo del mundo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CausalWorldModel {
    pub laws: HashMap<String, CausalLaw>,
    pub dag: CausalDAG,
    pub fundamental_constants: HashMap<String, f32>,
    pub version: u64,
}

impl CausalWorldModel {
    pub fn new() -> Self {
        Self {
            laws: HashMap::new(),
            dag: CausalDAG::new(),
            fundamental_constants: HashMap::new(),
            version: 0,
        }
    }
    
    /// Agrega una nueva ley causal al modelo
    pub fn add_law(&mut self, law: CausalLaw) {
        // Verificar que no cree ciclo en el DAG
        if !self.dag.would_create_cycle(
            &law.cause.name, 
            &law.effect.name
        ) {
            self.dag.add_edge(&law.cause.name, &law.effect.name);
            self.laws.insert(law.id.clone(), law);
            self.version += 1;
        } else {
            eprintln!("⚠️  Rejected law: would create cycle in DAG");
        }
    }
    
    /// Predice consecuencias de una intervención
    pub fn predict_intervention(&self, intervention: &Intervention) -> Prediction {
        let affected_vars = self.dag.find_descendants(&intervention.variable.name);
        
        let mut consequences = Vec::new();
        
        for var_name in affected_vars {
            if let Some(law) = self.find_law_for_effect(&var_name) {
                let predicted_change = law.compute_effect(
                    &intervention.new_value,
                    &self.fundamental_constants
                );
                
                let predicted_change_val = match &predicted_change {
                    ValueType::Continuous(s) => s.parse::<f32>().unwrap_or(0.0),
                    ValueType::Discrete(v) => *v as f32,
                    _ => 0.0,
                };
                
                consequences.push(ConsequencePrediction {
                    variable: Variable {
                        name: var_name.clone(),
                        domain: law.effect.domain.clone(),
                        value_type: law.effect.value_type.clone(),
                    },
                    predicted_change: predicted_change_val,
                    old_value: self.get_value(&var_name),
                    new_value: predicted_change.clone(),
                    confidence: law.confidence,
                });
            }
        }
        
        Prediction {
            intervention: intervention.clone(),
            consequences,
            confidence: 0.7, // Promedio de confianzas
            timestamp: chrono::Utc::now().timestamp() as u64,
        }
    }
    
    pub fn find_law_for_effect(&self, effect_name: &str) -> Option<&CausalLaw> {
        self.laws.values()
            .find(|law| law.effect.name == effect_name)
    }
    
    fn get_value(&self, var_name: &str) -> ValueType {
        self.fundamental_constants
            .get(var_name)
            .cloned()
            .map(|v| ValueType::Continuous(v.to_string()))
            .unwrap_or(ValueType::Discrete(0))
    }
    
    /// Simula eliminación de una ley física
    pub fn simulate_law_removal(&self, law_id: &str) -> Option<SimulationResult> {
        let law = self.laws.get(law_id)?;
        
        // Crear copia sin esa ley
        let mut modified = self.clone();
        modified.laws.remove(law_id);
        
        // Recalcular predicciones
        Some(SimulationResult {
            removed_law: law.clone(),
            affected_predictions: modified.dag.find_descendants(&law.cause.name),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationResult {
    pub removed_law: CausalLaw,
    pub affected_predictions: Vec<String>,
}

impl CausalLaw {
    /// Computa el efecto esperado dado un nuevo valor de la causa
    pub fn compute_effect(
        &self,
        new_cause_value: &ValueType,
        _constants: &HashMap<String, f32>
    ) -> ValueType {
        match &self.relationship {
            CausalRelationship::Direct { strength, .. } => {
                // Efecto = causa * fuerza
                match new_cause_value {
                    ValueType::Continuous(v) => {
                        let parsed = v.parse::<f32>().unwrap_or(0.0);
                        ValueType::Continuous((parsed * strength).to_string())
                    }
                    ValueType::Discrete(v) => {
                        ValueType::Discrete((*v as f32 * strength) as i32)
                    }
                    _ => new_cause_value.clone(),
                }
            }
            CausalRelationship::Inhibitory { strength } => {
                // Efecto = -causa * fuerza
                match new_cause_value {
                    ValueType::Continuous(v) => {
                        let parsed = v.parse::<f32>().unwrap_or(0.0);
                        ValueType::Continuous((-parsed * strength).to_string())
                    }
                    _ => new_cause_value.clone(),
                }
            }
            _ => new_cause_value.clone(),
        }
    }
}
