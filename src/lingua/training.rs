//! LINGUA — Fase 2: Entrenamiento Simulado Masivo
//!
//! Genera millones de frases internamente, las valida gramaticalmente,
//! las ancla a conceptos 3D conocidos, y selecciona las élite.

use crate::lingua::acquisition::{AcquiredWord, DesignCategory};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ────────────────────────────────────────────────────────────────
//  TIPOS DE DATOS
// ────────────────────────────────────────────────────────────────

/// Genoma de una frase generada internamente
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SentenceGenome {
    pub sentence: String,
    pub pattern_used: String,
    pub meaning: SemanticFrame,
    pub fitness: f64,
}

/// Representación semántica estructurada de una oración
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SemanticFrame {
    pub subject: Option<String>,
    pub action: Option<String>,
    pub object: Option<String>,
    pub modifiers: Vec<String>,
    pub material: Option<String>,
    pub style: Option<String>,
    /// Conceptos que se enlazan a objetos 3D reales
    pub grounded_concepts: Vec<String>,
}

/// Patrón gramatical para generación
#[derive(Debug, Clone)]
pub struct SentencePattern {
    pub id: String,
    pub slots: Vec<SlotType>,
    pub example: String,
    pub frequency: f64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SlotType {
    DesignVerb,
    Determiner,
    FurnitureNoun,
    MaterialNoun,
    Adjective,
    Preposition,
    Quantifier,
    AnyNoun,
}

/// Estadísticas de entrenamiento
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct TrainingStats {
    pub total_generated: usize,
    pub valid_sentences: usize,
    pub grounded_sentences: usize,
    pub elite_count: usize,
    pub training_time_ms: u128,
    pub sentences_per_second: f64,
}

/// Conocimiento entrenado exportable
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainedKnowledge {
    pub elite_sentences: Vec<SentenceGenome>,
    pub stats: TrainingStats,
    pub vocabulary_size: usize,
    pub grounded_concept_ids: Vec<String>,
}

// ────────────────────────────────────────────────────────────────
//  MOTOR DE ENTRENAMIENTO SIMULADO
// ────────────────────────────────────────────────────────────────

pub struct SimulatedTrainer {
    /// Vocabulario completo adquirido
    vocabulary: HashMap<String, AcquiredWord>,
    /// Conceptos de diseño conocidos (del Archetype TaxonomyTree)
    known_concepts: Vec<String>,
    /// Patrones gramaticales
    patterns: Vec<SentencePattern>,
    /// Frases élite seleccionadas
    pub elite_sentences: Vec<SentenceGenome>,
    /// Estadísticas acumuladas
    pub stats: TrainingStats,
}

impl SimulatedTrainer {
    pub fn new(
        vocabulary: HashMap<String, AcquiredWord>,
        known_concepts: Vec<String>,
    ) -> Self {
        let mut trainer = Self {
            vocabulary,
            known_concepts,
            patterns: Vec::new(),
            elite_sentences: Vec::new(),
            stats: TrainingStats::default(),
        };
        trainer.load_grammar_patterns();
        trainer
    }

    /// Cargar patrones gramaticales del español orientados a diseño
    fn load_grammar_patterns(&mut self) {
        self.patterns = vec![
            SentencePattern {
                id: "CMD_SIMPLE".into(),
                slots: vec![SlotType::DesignVerb, SlotType::Determiner, SlotType::FurnitureNoun, SlotType::Adjective],
                example: "Diseña una mesa moderna".into(),
                frequency: 0.20,
            },
            SentencePattern {
                id: "CMD_MATERIAL".into(),
                slots: vec![SlotType::DesignVerb, SlotType::Determiner, SlotType::FurnitureNoun, SlotType::Preposition, SlotType::MaterialNoun],
                example: "Crea una mesa de madera".into(),
                frequency: 0.25,
            },
            SentencePattern {
                id: "DESC_SER".into(),
                slots: vec![SlotType::Determiner, SlotType::FurnitureNoun, SlotType::Adjective],
                example: "La silla es cómoda".into(),
                frequency: 0.15,
            },
            SentencePattern {
                id: "CMD_FULL".into(),
                slots: vec![SlotType::DesignVerb, SlotType::Determiner, SlotType::FurnitureNoun, SlotType::Adjective, SlotType::Preposition, SlotType::MaterialNoun],
                example: "Diseña una silla minimalista de metal".into(),
                frequency: 0.15,
            },
            SentencePattern {
                id: "CMD_QUANTITY".into(),
                slots: vec![SlotType::FurnitureNoun, SlotType::Preposition, SlotType::Quantifier, SlotType::AnyNoun],
                example: "Mesa con cuatro patas".into(),
                frequency: 0.10,
            },
            SentencePattern {
                id: "CMD_ADJ_MAT".into(),
                slots: vec![SlotType::Determiner, SlotType::FurnitureNoun, SlotType::Adjective, SlotType::Preposition, SlotType::MaterialNoun, SlotType::Adjective],
                example: "Una mesa alta de vidrio transparente".into(),
                frequency: 0.10,
            },
            SentencePattern {
                id: "MODIFY".into(),
                slots: vec![SlotType::DesignVerb, SlotType::Determiner, SlotType::AnyNoun, SlotType::Adjective],
                example: "Modifica el respaldo más alto".into(),
                frequency: 0.05,
            },
        ];
    }

    /// Entrenar generando una cantidad masiva de frases internamente
    pub fn run_massive_generation(&mut self, target_count: usize) -> TrainingStats {
        log::info!("[LINGUA-TRAIN] Iniciando generación masiva: {} frases objetivo", target_count);
        let start = std::time::Instant::now();

        // Pre-clasificar vocabulario por slot para acceso O(1)
        let slot_pools = self.build_slot_pools();

        for _ in 0..target_count {
            // 1. Seleccionar patrón ponderado por frecuencia
            let pattern = self.select_pattern();

            // 2. Generar frase desde el patrón
            if let Some(genome) = self.generate_from_pattern(&pattern, &slot_pools) {
                self.stats.total_generated += 1;

                // 3. Validar gramática
                if !self.validate_grammar(&genome) { continue; }
                self.stats.valid_sentences += 1;

                // 4. Verificar anclaje 3D
                if !genome.meaning.grounded_concepts.is_empty() {
                    self.stats.grounded_sentences += 1;
                }

                // 5. Elite selection (fitness > 0.6)
                if genome.fitness > 0.6 {
                    self.elite_sentences.push(genome);
                }
            }
        }

        let elapsed = start.elapsed();
        self.stats.training_time_ms = elapsed.as_millis();
        self.stats.elite_count = self.elite_sentences.len();
        self.stats.sentences_per_second = if elapsed.as_secs_f64() > 0.0 {
            target_count as f64 / elapsed.as_secs_f64()
        } else {
            0.0
        };

        log::info!(
            "[LINGUA-TRAIN] Completado: {} generadas, {} válidas, {} ancladas, {} élite | {:.2}s ({:.0} frases/seg)",
            self.stats.total_generated,
            self.stats.valid_sentences,
            self.stats.grounded_sentences,
            self.stats.elite_count,
            elapsed.as_secs_f64(),
            self.stats.sentences_per_second,
        );

        self.stats.clone()
    }

    /// Pre-clasificar vocabulario por tipo de slot para acceso rápido
    fn build_slot_pools(&self) -> HashMap<String, Vec<String>> {
        let mut pools: HashMap<String, Vec<String>> = HashMap::new();

        for (word, entry) in &self.vocabulary {
            let cat = entry.design_category.as_ref().cloned().unwrap_or(DesignCategory::Uncategorized);
            
            match cat {
                DesignCategory::Furniture => {
                    pools.entry("FurnitureNoun".into()).or_default().push(word.clone());
                    pools.entry("AnyNoun".into()).or_default().push(word.clone());
                }
                DesignCategory::Material => {
                    pools.entry("MaterialNoun".into()).or_default().push(word.clone());
                    pools.entry("AnyNoun".into()).or_default().push(word.clone());
                }
                DesignCategory::Shape | DesignCategory::Dimension | DesignCategory::Style
                | DesignCategory::Property | DesignCategory::Finish | DesignCategory::Color => {
                    pools.entry("Adjective".into()).or_default().push(word.clone());
                }
                DesignCategory::DesignVerb | DesignCategory::Function => {
                    pools.entry("DesignVerb".into()).or_default().push(word.clone());
                }
                DesignCategory::Connector => {
                    if entry.part_of_speech.iter().any(|p| p.contains("preposition")) {
                        pools.entry("Preposition".into()).or_default().push(word.clone());
                    }
                    if entry.part_of_speech.iter().any(|p| p.contains("determiner")) {
                        pools.entry("Determiner".into()).or_default().push(word.clone());
                    }
                }
                DesignCategory::Quantifier => {
                    pools.entry("Quantifier".into()).or_default().push(word.clone());
                }
                _ => {}
            }
        }

        // Asegurar que determiners siempre existan
        pools.entry("Determiner".into()).or_insert_with(|| {
            vec!["el".into(), "la".into(), "un".into(), "una".into(), "los".into(), "las".into()]
        });
        pools.entry("Preposition".into()).or_insert_with(|| {
            vec!["de".into(), "con".into(), "para".into(), "en".into(), "sin".into()]
        });

        pools
    }

    /// Seleccionar patrón ponderado
    fn select_pattern(&self) -> SentencePattern {
        let total: f64 = self.patterns.iter().map(|p| p.frequency).sum();
        let mut r = fastrand::f64() * total;

        for pattern in &self.patterns {
            r -= pattern.frequency;
            if r <= 0.0 {
                return pattern.clone();
            }
        }

        self.patterns[0].clone()
    }

    /// Generar frase concreta a partir de un patrón y los pools de palabras
    fn generate_from_pattern(
        &self,
        pattern: &SentencePattern,
        pools: &HashMap<String, Vec<String>>,
    ) -> Option<SentenceGenome> {
        let mut words = Vec::new();
        let mut frame = SemanticFrame {
            subject: None,
            action: None,
            object: None,
            modifiers: Vec::new(),
            material: None,
            style: None,
            grounded_concepts: Vec::new(),
        };

        for slot in &pattern.slots {
            let pool_key = format!("{:?}", slot);
            let pool = pools.get(&pool_key)?;
            if pool.is_empty() { return None; }
            
            let idx = fastrand::usize(..pool.len());
            let word = pool[idx].clone();
            
            // Poblar frame semántico
            match slot {
                SlotType::DesignVerb => frame.action = Some(word.clone()),
                SlotType::FurnitureNoun => {
                    if frame.subject.is_none() {
                        frame.subject = Some(word.clone());
                    } else {
                        frame.object = Some(word.clone());
                    }
                    // Anclar a concepto 3D si es conocido
                    if self.known_concepts.iter().any(|c| c.contains(&word)) {
                        frame.grounded_concepts.push(word.clone());
                    }
                }
                SlotType::AnyNoun => {
                    frame.object = Some(word.clone());
                    if self.known_concepts.iter().any(|c| c.contains(&word)) {
                        frame.grounded_concepts.push(word.clone());
                    }
                }
                SlotType::MaterialNoun => frame.material = Some(word.clone()),
                SlotType::Adjective => frame.modifiers.push(word.clone()),
                _ => {} // Determiners / prepositions son estructura, no semántica
            }

            words.push(word);
        }

        if words.is_empty() { return None; }

        let sentence = words.join(" ");
        let fitness = self.compute_fitness(&frame, words.len());

        Some(SentenceGenome {
            sentence,
            pattern_used: pattern.id.clone(),
            meaning: frame,
            fitness,
        })
    }

    /// Validar gramática básica
    fn validate_grammar(&self, genome: &SentenceGenome) -> bool {
        let words: Vec<&str> = genome.sentence.split_whitespace().collect();
        
        // Mínimo 2 palabras
        if words.len() < 2 { return false; }
        
        // Sin palabras duplicadas consecutivas
        for i in 0..words.len().saturating_sub(1) {
            if words[i] == words[i + 1] { return false; }
        }

        // Debe tener verbo o sustantivo
        genome.meaning.action.is_some() || genome.meaning.subject.is_some()
    }

    /// Calcular fitness de una frase
    fn compute_fitness(&self, frame: &SemanticFrame, word_count: usize) -> f64 {
        let mut score: f64 = 0.0;

        // Tiene verbo + sustantivo → frase completa
        if frame.action.is_some() && frame.subject.is_some() { score += 0.3; }
        // Anclada a conceptos 3D
        if !frame.grounded_concepts.is_empty() { score += 0.35; }
        // Material especificado
        if frame.material.is_some() { score += 0.1; }
        // Tiene modificadores (estilo/adjetivos)
        if !frame.modifiers.is_empty() { score += 0.1; }
        // Longitud apropiada (3-10 palabras)
        if word_count >= 3 && word_count <= 10 { score += 0.15; }

        score.min(1.0)
    }

    /// Exportar conocimiento entrenado a JSON
    pub fn export_knowledge(&self) -> TrainedKnowledge {
        TrainedKnowledge {
            elite_sentences: self.elite_sentences.clone(),
            stats: self.stats.clone(),
            vocabulary_size: self.vocabulary.len(),
            grounded_concept_ids: self.known_concepts.clone(),
        }
    }

    /// Guardar conocimiento a disco
    pub fn save_knowledge(&self, path: &str) -> Result<(), String> {
        let knowledge = self.export_knowledge();
        let json = serde_json::to_string_pretty(&knowledge)
            .map_err(|e| format!("Serialization error: {}", e))?;
        let _ = std::fs::create_dir_all("lingua_cache");
        std::fs::write(path, json)
            .map_err(|e| format!("Write error: {}", e))?;
        log::info!("[LINGUA-TRAIN] Conocimiento exportado: {} frases élite -> {}", self.elite_sentences.len(), path);
        Ok(())
    }
}
