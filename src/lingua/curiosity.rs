//! LINGUA — Curiosity Engine
//!
//! El motor de curiosidad de Daithon. Se encarga de identificar lagunas de 
//! conocimiento y disparar procesos de "estudio" (grounding visual).

use crate::lingua::acquisition::{AcquisitionEngine, AcquiredWord, DesignCategory};
use crate::persona::integration::DaithonPersona;
use std::collections::HashMap;

pub struct CuriosityEngine {
    /// Palabras que estamos estudiando actualmente
    pub study_queue: Vec<String>,
}

impl CuriosityEngine {
    pub fn new() -> Self {
        Self { study_queue: Vec::new() }
    }

    /// Analiza una entrada y detecta palabras que Daithon no conoce o no ha "visto" (grounded)
    pub async fn explore(&mut self, 
        input: &str, 
        acq: &mut AcquisitionEngine,
        persona: &mut DaithonPersona
    ) -> Vec<GroundingDiscovery> {
        let mut discoveries = Vec::new();
        let words: Vec<String> = input.split_whitespace()
            .map(|w| w.to_lowercase().replace(|c: char| !c.is_alphanumeric(), ""))
            .filter(|w| w.len() > 3)
            .collect();

        for word in words {
            // ¿Está en nuestro vocabulario?
            if !acq.vocabulary().contains_key(&word) {
                // FASE 1: Adquisición Cruda (Wiktionary)
                log::info!("[CURIOSITY] No sé qué es '{}'. Buscando en Wiktionary...", word);
                if let Ok(new_word) = acq.fetch_from_wiktionary(&word).await {
                    self.study_queue.push(word.clone());
                    if self.should_ground_visually(&new_word) {
                        let discovery = self.study_and_ground(&word, acq, persona).await;
                        discoveries.push(discovery);
                    }
                } else {
                    // Fallback: Adquisición Sintética (si no hay internet o error de tipeo)
                    log::info!("[CURIOSITY] '{}' desconocido en la red principal. Activando Motor de Inferencia Morfológica.", word);
                    let synth_word = acq.acquire_synthetic(&word);
                    if self.should_ground_visually(&synth_word) {
                        let discovery = self.study_and_ground(&word, acq, persona).await;
                        discoveries.push(discovery);
                    }
                }
            } else {
                // Ya lo conocemos lingüísticamente, ¿pero lo hemos "visto"?
                let entry = acq.vocabulary().get(&word).unwrap();
                if !entry.is_visually_grounded && self.should_ground_visually(entry) {
                    let discovery = self.study_and_ground(&word, acq, persona).await;
                    discoveries.push(discovery);
                }
            }
        }

        discoveries
    }

    fn should_ground_visually(&self, word: &AcquiredWord) -> bool {
        match word.design_category {
            Some(DesignCategory::Furniture) | 
            Some(DesignCategory::Material) | 
            Some(DesignCategory::Shape) => true,
            Some(DesignCategory::Uncategorized) => {
                // Si es Uncategorized, no lo groundeamos visualmente bajo ninguna circunstancia.
                // Podría ser un verbo, interjección ("hola", "significa") o concepto abstracto de gramática.
                false
            },
            _ => false
        }
    }

    /// "Estudia" un objeto: busca su forma, material y función en el mundo 3D
    async fn study_and_ground(&self, 
        word_str: &str, 
        acq: &mut AcquisitionEngine,
        _persona: &mut DaithonPersona
    ) -> GroundingDiscovery {
        log::info!("[CURIOSITY] Estudiando arquitectura y forma de: {}", word_str);

        // Simulamos la extracción de "instintos de diseño" para este nuevo objeto
        // En una fase posterior esto conectará con una búsqueda de mallas (meshes) reales
        let prototype_id = format!("grounded_{}", word_str);

        // Actualizamos el vocabulario para marcarlo como aprendido visualmente
        if let Some(entry) = acq.vocabulary().get(word_str).cloned() {
            let mut updated = entry;
            updated.is_visually_grounded = true;
            updated.grounded_concept_id = Some(prototype_id.clone());
            
            // Persistimos el aprendizaje en el motor de adquisición
            acq.update_entry(updated);
        }

        GroundingDiscovery {
            word: word_str.to_string(),
            prototype_id,
            reasoning: format!("He detectado el concepto '{}'. He estudiado su geometría y lo he mapeado a mi núcleo de diseño.", word_str),
        }
    }
}

pub struct GroundingDiscovery {
    pub word: String,
    pub prototype_id: String,
    pub reasoning: String,
}
