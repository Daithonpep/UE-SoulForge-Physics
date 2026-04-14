// design_evolution/timeline.rs
// Línea Temporal Evolutiva — Simula siglos de evolución de diseño
//
// Arquitectura:
//   EvolutionSimulator controla el bucle: Genesis → Evaluación → Selección → Reproducción
//   Cada Era aplica presión estilística diferente
//   Hall of Fame preserva los mejores diseños de cada era

use super::dna::*;
use super::mutation_engine::{DesignGenome, MutationEngine, GeneValue};
use super::fitness::{FitnessEvaluator, FitnessScore, SimulationResult, estimate_simulation};
use serde::{Deserialize, Serialize};

// ============================================================
// TIPOS DE LA LÍNEA TEMPORAL
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionaryTimeline {
    pub eras: Vec<Era>,
    pub current_era: usize,
    pub hall_of_fame: Vec<HallOfFameEntry>,
    pub total_generations: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Era {
    pub name: String,
    pub year_range: (u32, u32),
    pub style_influences: Vec<String>,
    pub population: Vec<DesignGenome>,
    pub best_fitness: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HallOfFameEntry {
    pub genome: DesignGenome,
    pub fitness: FitnessScore,
    pub era: String,
    pub innovation_description: String,
}

// ============================================================
// SIMULADOR EVOLUTIVO
// ============================================================

pub struct EvolutionSimulator {
    pub mutation_engine: MutationEngine,
    pub fitness_evaluator: FitnessEvaluator,
    pub population_size: usize,
    pub selection_pressure: f32,
    /// Log de eventos para el dashboard
    pub event_log: Vec<String>,
}

impl EvolutionSimulator {
    pub fn new(population_size: usize, selection_pressure: f32) -> Self {
        // Asegurar diversidad estocástica (evitar el mismo diseño en múltiples ejecuciones).
        if let Ok(dur) = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
            fastrand::seed(dur.as_millis() as u64);
        }

        Self {
            mutation_engine: MutationEngine::new(),
            fitness_evaluator: FitnessEvaluator::new(100),
            population_size,
            selection_pressure,
            event_log: Vec::new(),
        }
    }

    /// Simula evolución completa con entrenamiento HÍBRIDO:
    /// - Fase 1 (primera mitad): SOFIA pura (aprender las bases)
    /// - Fase 2 (segunda mitad): NEXUS híbrido (innovar con caos + cross-pollination)
    pub fn simulate_evolution(
        &mut self,
        base_dna: DesignDNA,
        eras: Vec<Era>,
        generations_per_era: u32,
    ) -> EvolutionaryTimeline {
        let category = format!("{:?}", base_dna.category);
        
        // Calcular el punto de transición SOFIA → NEXUS
        let total_gens = eras.len() as u32 * generations_per_era;
        let nexus_activation_point = total_gens / 2;
        
        self.log(format!(
            "🏛️ EVOLUCIÓN HÍBRIDA — {} | {} eras × {} gen | SOFIA pura: gen 1-{} | NEXUS híbrido: gen {}-{}",
            category, eras.len(), generations_per_era, nexus_activation_point, nexus_activation_point + 1, total_gens
        ));

        // Inicializar NEXUS para la fase híbrida
        let mut nexus = crate::nexus::NEXUS::new();

        self.fitness_evaluator.reset_novelty();

        let mut timeline = EvolutionaryTimeline {
            eras: Vec::new(),
            current_era: 0,
            hall_of_fame: Vec::new(),
            total_generations: 0,
        };

        let mut population = self.create_initial_population(&base_dna);
        let mut global_gen: u32 = 0;

        for (era_idx, mut era) in eras.into_iter().enumerate() {
            self.log(format!(
                "⏳ ERA {}: {} ({}-{}) — Influencias: {:?}",
                era_idx + 1, era.name, era.year_range.0, era.year_range.1, era.style_influences
            ));

            for gen in 0..generations_per_era {
                global_gen += 1;
                let is_nexus_phase = global_gen > nexus_activation_point;

                if is_nexus_phase {
                    // ====== FASE 2: NEXUS HÍBRIDO ======
                    // Primero evolucionar normalmente
                    population = self.evolve_generation(population);

                    // Luego, ENTROPY decide si inyectar caos en algunos individuos
                    let chaos_decision = nexus.entropy.should_introduce_chaos();
                    if chaos_decision.allow_chaos {
                        // Aplicar caos a los peores 20% de la población (no destruir los mejores)
                        let chaos_start = (population.len() as f32 * 0.8) as usize;
                        for i in chaos_start..population.len() {
                            nexus.entropy.apply_chaos_to_genes(
                                &mut population[i].genes,
                                &chaos_decision.chaos_type,
                            );
                        }

                        if gen % 10 == 0 {
                            self.log(format!(
                                "   🌀 ENTROPY {:?} aplicado a {} individuos",
                                chaos_decision.chaos_type,
                                population.len() - chaos_start
                            ));
                        }
                    }
                } else {
                    // ====== FASE 1: SOFIA PURA ======
                    population = self.evolve_generation(population);
                }

                timeline.total_generations += 1;

                // Log cada 10 generaciones
                if gen % 10 == 0 || gen == generations_per_era - 1 {
                    let (_, best_fit) = self.score_best(&population);
                    let phase = if is_nexus_phase { "NEXUS" } else { "SOFIA" };
                    self.log(format!(
                        "   [{}] Gen {}/{}: Fitness={:.3} (F:{:.2} A:{:.2} N:{:.2} E:{:.2} S:{:.2})",
                        phase, gen + 1, generations_per_era,
                        best_fit.total,
                        best_fit.breakdown.functionality,
                        best_fit.breakdown.aesthetics,
                        best_fit.breakdown.novelty,
                        best_fit.breakdown.efficiency,
                        best_fit.breakdown.stability,
                    ));
                }
            }

            // Registrar el mejor de esta era
            let (best_genome, best_fitness) = self.score_best(&population);

            if best_fitness.total > 0.4 {
                let desc = self.describe_innovation(&best_genome);
                timeline.hall_of_fame.push(HallOfFameEntry {
                    genome: best_genome.clone(),
                    fitness: best_fitness.clone(),
                    era: era.name.clone(),
                    innovation_description: desc.clone(),
                });
                self.log(format!("   🏆 Hall of Fame: {} — {}", era.name, desc));
            }

            era.population = population.clone();
            era.best_fitness = best_fitness.total;
            timeline.eras.push(era);
            timeline.current_era = era_idx;
        }

        // Log final con estadísticas NEXUS
        self.log(format!(
            "✅ EVOLUCIÓN HÍBRIDA COMPLETADA — {} gen | {} diseños HoF | {} innovaciones caóticas | ENTROPY prob: {:.1}%",
            timeline.total_generations,
            timeline.hall_of_fame.len(),
            nexus.entropy.discovered_innovations.len(),
            nexus.entropy.chaos_probability * 100.0,
        ));

        timeline
    }

    /// Evoluciona una sola generación (útil para step-by-step desde el dashboard)
    pub fn step_generation(&mut self, population: Vec<DesignGenome>) -> (Vec<DesignGenome>, FitnessScore) {
        let new_pop = self.evolve_generation(population);
        let (_, best) = self.score_best(&new_pop);
        (new_pop, best)
    }

    // =========================================================
    // INTERNOS
    // =========================================================

    fn create_initial_population(&mut self, dna: &DesignDNA) -> Vec<DesignGenome> {
        (0..self.population_size)
            .map(|_| self.mutation_engine.genesis(dna.clone()))
            .collect()
    }

    fn evolve_generation(&mut self, population: Vec<DesignGenome>) -> Vec<DesignGenome> {
        // 1. Evaluar fitness de toda la población
        let mut scored: Vec<(DesignGenome, FitnessScore)> = population
            .into_iter()
            .map(|genome| {
                let sim = estimate_simulation(&genome);
                let fitness = self.fitness_evaluator.evaluate(&genome, &sim);
                (genome, fitness)
            })
            .collect();

        // 2. Ordenar por fitness (mejor primero)
        scored.sort_by(|a, b| b.1.total.partial_cmp(&a.1.total).unwrap_or(std::cmp::Ordering::Equal));

        // 3. Selección: mantener top N%
        let survivors_count = ((self.population_size as f32 * self.selection_pressure) as usize).max(2);
        let survivors: Vec<DesignGenome> = scored
            .iter()
            .take(survivors_count)
            .map(|(g, _)| g.clone())
            .collect();

        // 4. Reproducción hasta llenar la población
        let mut new_population = survivors.clone();

        // THE GENESIS JAILBREAK: Si existe estancamiento (convergencia prematura con fitness muy alto), forzar diversificación
        let best_fitness = scored[0].1.total;
        if best_fitness >= 0.99 && new_population.len() < self.population_size {
             let branches = self.mutation_engine.execute_jailbreak_session(&scored[0].0);
             new_population.extend(branches);
             self.log(format!("   🚨 THE GENESIS JAILBREAK: Convergencia detectada (Fit={:.2}). Inyectando {} ramas experimentales (Radical/Hybrid).", best_fitness, new_population.len() - survivors.len()));
        }

        while new_population.len() < self.population_size {
            let p1_idx = fastrand::usize(0..survivors.len());
            let parent1 = &survivors[p1_idx];

            if fastrand::bool() {
                // Mutación asexual
                new_population.push(self.mutation_engine.mutate(parent1));
            } else {
                // Crossover sexual
                let p2_idx = fastrand::usize(0..survivors.len());
                let parent2 = &survivors[p2_idx];
                new_population.push(self.mutation_engine.crossover(parent1, parent2));
            }
        }

        new_population
    }

    fn score_best(&mut self, population: &[DesignGenome]) -> (DesignGenome, FitnessScore) {
        let mut best_genome = population[0].clone();
        let mut best_fitness = FitnessScore {
            total: -1.0,
            breakdown: super::fitness::FitnessBreakdown {
                functionality: 0.0,
                aesthetics: 0.0,
                novelty: 0.0,
                efficiency: 0.0,
                stability: 0.0,
            },
        };

        for genome in population {
            let sim = estimate_simulation(genome);
            let fitness = self.fitness_evaluator.evaluate(genome, &sim);
            if fitness.total > best_fitness.total {
                best_fitness = fitness;
                best_genome = genome.clone();
            }
        }

        (best_genome, best_fitness)
    }

    fn describe_innovation(&self, genome: &DesignGenome) -> String {
        let mut features = Vec::new();

        for gene in &genome.genes {
            match &gene.value {
                GeneValue::Integer(count) if gene.trait_name.contains("count") && *count > 4 => {
                    features.push(format!("{}={}", gene.trait_name.replace("_count", ""), count));
                }
                GeneValue::Shape(shape) if gene.trait_name == "primary_shape" => {
                    features.push(format!("{:?}", shape));
                }
                GeneValue::Scalar(v) if gene.trait_name.starts_with("detail_") && *v > 0.5 => {
                    features.push(gene.trait_name.replace("detail_", "").to_string());
                }
                _ => {}
            }
        }

        let scale = genome.get_scale();
        if scale[1] > 2.0 {
            features.push("Alto".into());
        }
        if scale[0] > 2.0 {
            features.push("Ancho".into());
        }

        if features.is_empty() {
            "Diseño clásico optimizado".into()
        } else {
            features.join(", ")
        }
    }

    fn log(&mut self, msg: String) {
        log::info!("[DESIGN-GENESIS] {}", msg);
        self.event_log.push(msg);
        if self.event_log.len() > 200 {
            self.event_log.remove(0);
        }
    }

    // =========================================================
    // PERSISTENCIA: Guardar/Cargar Hall of Fame al disco
    // =========================================================

    /// Guarda el Hall of Fame al disco para que sobreviva reinicios
    pub fn save_hall_of_fame(hall_of_fame: &[HallOfFameEntry]) {
        let path = "checkpoints/hall_of_fame.json";
        if let Err(e) = std::fs::create_dir_all("checkpoints") {
            log::warn!("No se pudo crear 'checkpoints/': {}", e);
            return;
        }
        match serde_json::to_string_pretty(hall_of_fame) {
            Ok(json) => {
                if let Err(e) = std::fs::write(path, &json) {
                    log::warn!("Error guardando Hall of Fame: {}", e);
                } else {
                    log::info!("[DESIGN-GENESIS] 💾 Hall of Fame guardado ({} diseños) → {}", hall_of_fame.len(), path);
                }
            }
            Err(e) => log::warn!("Error serializando Hall of Fame: {}", e),
        }
    }

    /// Carga el Hall of Fame desde disco (si existe)
    pub fn load_hall_of_fame() -> Vec<HallOfFameEntry> {
        let path = "checkpoints/hall_of_fame.json";
        if let Ok(json) = std::fs::read_to_string(path) {
            match serde_json::from_str::<Vec<HallOfFameEntry>>(&json) {
                Ok(entries) => {
                    log::info!("[DESIGN-GENESIS] 📂 Hall of Fame cargado: {} diseños desde {}", entries.len(), path);
                    entries
                }
                Err(e) => {
                    log::warn!("Error deserializando Hall of Fame: {}", e);
                    Vec::new()
                }
            }
        } else {
            Vec::new()
        }
    }
}

// ============================================================
// ERAS PREDEFINIDAS
// ============================================================

pub fn create_furniture_timeline() -> Vec<Era> {
    vec![
        Era {
            name: "Primitivo".into(),
            year_range: (0, 1000),
            style_influences: vec!["Funcional".into(), "Rústico".into()],
            population: vec![],
            best_fitness: 0.0,
        },
        Era {
            name: "Medieval".into(),
            year_range: (1000, 1500),
            style_influences: vec!["Gótico".into(), "Ornamentado".into()],
            population: vec![],
            best_fitness: 0.0,
        },
        Era {
            name: "Renacimiento".into(),
            year_range: (1500, 1700),
            style_influences: vec!["Clásico".into(), "Simétrico".into()],
            population: vec![],
            best_fitness: 0.0,
        },
        Era {
            name: "Industrial".into(),
            year_range: (1800, 1950),
            style_influences: vec!["Funcional".into(), "Metal".into()],
            population: vec![],
            best_fitness: 0.0,
        },
        Era {
            name: "Moderno".into(),
            year_range: (1950, 2000),
            style_influences: vec!["Minimalista".into(), "Ergonómico".into()],
            population: vec![],
            best_fitness: 0.0,
        },
        Era {
            name: "Futurista".into(),
            year_range: (2000, 2100),
            style_influences: vec!["Biomórfico".into(), "Sostenible".into()],
            population: vec![],
            best_fitness: 0.0,
        },
    ]
}

pub fn create_architecture_timeline() -> Vec<Era> {
    vec![
        Era {
            name: "Megalítico".into(),
            year_range: (0, 500),
            style_influences: vec!["Monolítico".into(), "Masivo".into()],
            population: vec![],
            best_fitness: 0.0,
        },
        Era {
            name: "Clásico".into(),
            year_range: (500, 1400),
            style_influences: vec!["Columnas".into(), "Arcos".into()],
            population: vec![],
            best_fitness: 0.0,
        },
        Era {
            name: "Gótico".into(),
            year_range: (1400, 1600),
            style_influences: vec!["Verticalidad".into(), "Ojival".into()],
            population: vec![],
            best_fitness: 0.0,
        },
        Era {
            name: "Barroco".into(),
            year_range: (1600, 1800),
            style_influences: vec!["Ornamental".into(), "Curvas".into()],
            population: vec![],
            best_fitness: 0.0,
        },
        Era {
            name: "Art Deco".into(),
            year_range: (1900, 1940),
            style_influences: vec!["Geométrico".into(), "Lujo".into()],
            population: vec![],
            best_fitness: 0.0,
        },
        Era {
            name: "Contemporáneo".into(),
            year_range: (1950, 2100),
            style_influences: vec!["Vidrio".into(), "Minimalista".into(), "Sostenible".into()],
            population: vec![],
            best_fitness: 0.0,
        },
    ]
}

pub fn create_vehicle_timeline() -> Vec<Era> {
    vec![
        Era {
            name: "Vapor".into(),
            year_range: (1800, 1900),
            style_influences: vec!["Industrial".into(), "Masivo".into()],
            population: vec![],
            best_fitness: 0.0,
        },
        Era {
            name: "Clásico".into(),
            year_range: (1900, 1950),
            style_influences: vec!["Elegante".into(), "Curvo".into()],
            population: vec![],
            best_fitness: 0.0,
        },
        Era {
            name: "Muscle".into(),
            year_range: (1950, 1975),
            style_influences: vec!["Potente".into(), "Angular".into()],
            population: vec![],
            best_fitness: 0.0,
        },
        Era {
            name: "Aerodinámico".into(),
            year_range: (1975, 2010),
            style_influences: vec!["Eficiente".into(), "Suave".into()],
            population: vec![],
            best_fitness: 0.0,
        },
        Era {
            name: "Eléctrico".into(),
            year_range: (2010, 2100),
            style_influences: vec!["Futurista".into(), "Minimalista".into()],
            population: vec![],
            best_fitness: 0.0,
        },
    ]
}

/// Selecciona la timeline apropiada según la categoría del ADN
pub fn timeline_for_category(category: &DesignCategory) -> Vec<Era> {
    match category {
        DesignCategory::Furniture(_) => create_furniture_timeline(),
        DesignCategory::Architecture(_) => create_architecture_timeline(),
        DesignCategory::Vehicle(_) => create_vehicle_timeline(),
        DesignCategory::Nature(_) => create_furniture_timeline(), // Reutilizar por ahora
        DesignCategory::Abstract(_) => create_furniture_timeline(),
    }
}
