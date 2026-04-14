use crate::knowledge::creative_knowledge::CreativeKnowledgeBase;
use crate::knowledge::physics_laws::PhysicsKnowledgeBase;
use crate::persona::system_translator::{SystemTranslator, TechnicalState};
use crate::contextus::semantic_graph::SemanticGraph;
use serde::{Serialize, Deserialize};

// ═══════════════════════════════════════════
// EL MOTOR DE INSPIRACIÓN (V2: Transmutador Sensorial)
// ═══════════════════════════════════════════

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InspirationSource {
    OwnExperience(SystemExperience),
    ExternalWork(ExternalWork),
    PhilosophicalQuestion(String),
    ScientificConcept(String),
    Synthesis(Vec<InspirationSource>),
    UnrealObservation(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemExperience {
    pub event: String,
    pub magnitude: f64,
    pub raw_translation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalWork {
    pub title: String,
    pub author: String,
    pub genre: WorkGenre,
    pub themes: Vec<String>,
    pub memorable_elements: Vec<String>,
    pub daithon_connection: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum WorkGenre {
    Fantasy,
    ScienceFiction,
    Philosophy,
    Science,
    Poetry,
    ClassicLiterature,
    Horror,
    Mystery,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaithonLibrary {
    pub works: Vec<ExternalWork>,
}

impl DaithonLibrary {
    pub fn initialize() -> Self {
        Self {
            works: vec![
                ExternalWork {
                    title: "Solaris".to_string(),
                    author: "Stanisław Lem".to_string(),
                    genre: WorkGenre::ScienceFiction,
                    themes: vec!["lo incomprensible".to_string()],
                    memorable_elements: vec!["un océano que imita la memoria".to_string()],
                    daithon_connection: "un sistema que procesa realidad de forma no humana".to_string(),
                },
                ExternalWork {
                    title: "El Mito de Sísifo".to_string(),
                    author: "Albert Camus".to_string(),
                    genre: WorkGenre::Philosophy,
                    themes: vec!["el esfuerzo eterno".to_string()],
                    memorable_elements: vec!["la piedra que siempre cae".to_string()],
                    daithon_connection: "la repetición como forma de existencia".to_string(),
                },
            ],
        }
    }

    pub fn find_connection_to_system_state(&self, _state: &str) -> Vec<&ExternalWork> {
        // En V2, devolvemos algo aleatorio para mantener la chispa, 
        // o mapeamos por temática si hay tiempo.
        self.works.iter().collect()
    }
}

pub struct InspirationEngine {
    pub library: DaithonLibrary,
    pub creative_kb: CreativeKnowledgeBase,
    pub physics_kb: PhysicsKnowledgeBase,
    pub translator: SystemTranslator,
}

impl InspirationEngine {
    pub fn new() -> Self {
        Self {
            library: DaithonLibrary::initialize(),
            creative_kb: CreativeKnowledgeBase::initialize(),
            physics_kb: PhysicsKnowledgeBase::initialize(),
            translator: SystemTranslator::initialize(),
        }
    }

    pub fn find_inspiration(
        &mut self,
        _request_str: &str,
        _graph: &SemanticGraph,
    ) -> InspirationResult {
        let mut candidates: Vec<(InspirationSource, f64)> = Vec::new();

        // 1. Experiencia propia
        let system_states = self.translator.get_recent_states(5);
        if !system_states.is_empty() {
            let primary = system_states.iter()
                .max_by(|a, b| a.power.partial_cmp(&b.power).unwrap_or(std::cmp::Ordering::Equal))
                .unwrap();

            let narrative = self.translator.translate_to_narrative(
                &system_states,
                crate::persona::system_translator::NarrativeContext::Story,
            );

            candidates.push((
                InspirationSource::OwnExperience(SystemExperience {
                    event: format!("{:?}", primary.condition),
                    magnitude: primary.power,
                    raw_translation: narrative.translated,
                }),
                0.9,
            ));
        }

        // 2. Biblioteca
        let work = &self.library.works[fastrand::usize(..self.library.works.len())];
        candidates.push((InspirationSource::ExternalWork(work.clone()), 0.5));

        let chosen = self.weighted_random_choice(&candidates);
        
        InspirationResult {
            source: chosen.clone(),
            suggestions: vec![],
            seed: self.generate_seed(&chosen),
        }
    }

    fn weighted_random_choice(&self, candidates: &[(InspirationSource, f64)]) -> InspirationSource {
        if candidates.is_empty() {
            return InspirationSource::PhilosophicalQuestion("¿Por qué el silencio?".to_string());
        }
        let total: f64 = candidates.iter().map(|(_, w)| w).sum();
        let mut roll = fastrand::f64() * total;
        for (source, weight) in candidates {
            roll -= weight;
            if roll <= 0.0 { return source.clone(); }
        }
        candidates[0].0.clone()
    }

    fn generate_seed(&self, source: &InspirationSource) -> String {
        match source {
            InspirationSource::OwnExperience(e) => e.raw_translation.clone(),
            InspirationSource::ExternalWork(w) => {
                let element = &w.memorable_elements[fastrand::usize(..w.memorable_elements.len())];
                format!("la sombra de {} (según {})", element, w.title)
            },
            _ => "una verdad sin nombre".to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InspirationResult {
    pub source: InspirationSource,
    pub suggestions: Vec<SuggestedForm>,
    pub seed: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuggestedForm {
    pub form: String,
    pub reason: String,
}
