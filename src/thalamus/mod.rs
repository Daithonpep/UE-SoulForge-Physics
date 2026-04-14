pub mod dispatcher;
pub mod fuzzy;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitiveProfile {
    pub intent: String,        // ej: "greeting", "simulation", "creation"
    pub complexity: f32,       // 0.0 a 1.0
    pub arousal: f32,          // Nivel de urgencia
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleActivation {
    pub lingua: f32,           
    pub realidad: f32,         
    pub metacognicion: f32,    
    pub creativa: f32,         
}

pub struct ThalamusEngine;

impl ThalamusEngine {
    pub fn assess_stimulus(input: &str) -> CognitiveProfile {
        let input_low = input.to_lowercase();
        
        if input_low.len() < 20 && (input_low.contains("hola") || input_low.contains("tal")) {
            CognitiveProfile { intent: "greeting".into(), complexity: 0.1, arousal: 0.2 }
        } else if input_low.contains("unreal") || input_low.contains("física") || input_low.contains("esfera") {
            CognitiveProfile { intent: "simulation".into(), complexity: 0.9, arousal: 0.8 }
        } else if input_low.contains("imagina") || input_low.contains("historia") || input_low.contains("mundo") {
            CognitiveProfile { intent: "creation".into(), complexity: 0.8, arousal: 0.6 }
        } else {
            CognitiveProfile { intent: "general_query".into(), complexity: 0.5, arousal: 0.5 }
        }
    }

    pub fn calculate_weights(profile: &CognitiveProfile) -> ModuleActivation {
        match profile.intent.as_str() {
            "greeting" => ModuleActivation {
                lingua: 1.0, realidad: 0.0, metacognicion: 0.1, creativa: 0.0
            },
            "simulation" => ModuleActivation {
                lingua: 0.4, realidad: 1.0, metacognicion: 0.9, creativa: 0.2
            },
            "creation" => ModuleActivation {
                lingua: 0.6, realidad: 0.3, metacognicion: 0.8, creativa: 1.0
            },
            _ => ModuleActivation {
                lingua: 0.8, realidad: 0.5, metacognicion: 0.5, creativa: 0.5
            }
        }
    }
}
