// harmonia/genetic_algorithm.rs
use serde::{Deserialize, Serialize};
use crate::sofia::universal_validator::*;
use super::fitness::*;
use super::batch_evaluator::*;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

/// Configuración del algoritmo genético
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneticConfig {
    /// Tamaño de la población
    pub population_size: usize,
    
    /// Número de generaciones
    pub num_generations: u32,
    
    /// Tasa de mutación (0.0 - 1.0)
    pub mutation_rate: f32,
    
    /// Tasa de crossover (0.0 - 1.0)
    pub crossover_rate: f32,
    
    /// Presión de selección (0.0 - 1.0)
    pub selection_pressure: f32,
    
    /// Elitismo (% de mejores que pasan directo)
    pub elitism_rate: f32,
    
    /// Método de mutación
    pub mutation_strategy: MutationStrategy,
    
    /// Método de selección
    pub selection_method: SelectionMethod,
    
    /// Parámetros de diversidad
    pub diversity_preservation: DiversityConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MutationStrategy {
    /// Mutación uniforme (todos los genes con igual probabilidad)
    Uniform { intensity: f32 },
    
    /// Mutación adaptativa (intensidad disminuye con fitness)
    Adaptive { min_intensity: f32, max_intensity: f32 },
    
    /// Mutación guiada (basada en gradientes de fitness)
    Guided { learning_rate: f32 },
    
    /// Mutación topológica (añadir/quitar primitivas)
    Topological { add_probability: f32, remove_probability: f32 },
    
    /// Combinación de estrategias
    Hybrid(Vec<MutationStrategy>),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SelectionMethod {
    /// Selección por torneo
    Tournament { size: usize },
    
    /// Selección por ruleta (proporcional al fitness)
    Roulette,
    
    /// Selección por ranking
    Rank { bias: f32 },
    
    /// Selección estocástica universal
    StochasticUniversal,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiversityConfig {
    /// Mantener diversidad genética mínima
    pub min_diversity: f32,
    
    /// Penalización por similitud
    pub similarity_penalty: f32,
    
    /// Nichos (sub-poblaciones)
    pub niching_enabled: bool,
    pub niche_radius: f32,
}

impl Default for GeneticConfig {
    fn default() -> Self {
        Self {
            population_size: 50,
            num_generations: 100,
            mutation_rate: 0.15,
            crossover_rate: 0.7,
            selection_pressure: 0.3,
            elitism_rate: 0.1,
            mutation_strategy: MutationStrategy::Adaptive { 
                min_intensity: 0.05, 
                max_intensity: 0.3 
            },
            selection_method: SelectionMethod::Tournament { size: 3 },
            diversity_preservation: DiversityConfig {
                min_diversity: 0.2,
                similarity_penalty: 0.1,
                niching_enabled: true,
                niche_radius: 0.15,
            },
        }
    }
}

/// Individuo en la población
#[derive(Debug, Clone)]
pub struct Individual {
    pub genome: UniversalDesign,
    pub fitness: f32,
    pub age: u32,
    pub lineage_id: String,
}

impl PartialEq for Individual {
    fn eq(&self, other: &Self) -> bool {
        self.fitness == other.fitness
    }
}

impl Eq for Individual {}

impl PartialOrd for Individual {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.fitness.partial_cmp(&other.fitness)
    }
}

impl Ord for Individual {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}

/// Resultado de evolución
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvolutionResult {
    pub generations: Vec<GenerationSnapshot>,
    pub best_individual: UniversalDesign,
    pub best_fitness: f32,
    pub convergence_generation: u32,
    pub diversity_history: Vec<f32>,
    pub average_fitness_history: Vec<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationSnapshot {
    pub generation: u32,
    pub best_fitness: f32,
    pub average_fitness: f32,
    pub worst_fitness: f32,
    pub diversity: f32,
    pub mutation_count: u32,
    pub crossover_count: u32,
}

/// Motor de algoritmo genético
pub struct GENESISLoop {
    config: GeneticConfig,
    rng: fastrand::Rng,
    lineage_counter: u64,
}

impl GENESISLoop {
    pub fn new(config: GeneticConfig) -> Self {
        Self {
            config,
            rng: fastrand::Rng::new(),
            lineage_counter: 0,
        }
    }

    /// Ejecuta la evolución completa
    pub fn evolve(
        &mut self,
        initial_design: &UniversalDesign,
        evaluator: &mut HARMONIABatchEvaluator,
        context_name: &str,
        seismic_intensity: f32,
        airflow_velocity: f32,
    ) -> EvolutionResult {
        println!("\n🧬 GENESIS LOOP - EVOLUTIONARY OPTIMIZATION");
        println!("════════════════════════════════════════════════════");
        println!("   Population: {}", self.config.population_size);
        println!("   Generations: {}", self.config.num_generations);
        println!("   Mutation rate: {:.1}%", self.config.mutation_rate * 100.0);
        println!("   Crossover rate: {:.1}%", self.config.crossover_rate * 100.0);
        println!("════════════════════════════════════════════════════\n");

        // 1. Inicializar población
        let mut population = self.initialize_population(initial_design);

        // 2. Evaluar población inicial
        let initial_fitness = self.evaluate_population(
            &population,
            evaluator,
            context_name,
            seismic_intensity,
            airflow_velocity,
        );

        for (i, individual) in population.iter_mut().enumerate() {
            individual.fitness = initial_fitness[i];
        }

        let mut generations = Vec::new();
        let mut diversity_history = Vec::new();
        let mut average_fitness_history = Vec::new();
        let mut best_ever = population.iter().max().unwrap().clone();
        let mut convergence_generation = self.config.num_generations;

        // 3. Bucle de evolución
        for gen in 0..self.config.num_generations {
            println!("\n🔄 Generation {}/{}", gen + 1, self.config.num_generations);

            // 3.1. Calcular diversidad
            let diversity = self.calculate_diversity(&population);
            diversity_history.push(diversity);

            // 3.2. Selección
            let selected = self.selection(&population);

            // 3.3. Crossover
            let mut offspring = self.crossover(&selected);

            // 3.4. Mutación
            let mutation_count = self.mutate(&mut offspring, gen);

            // 3.5. Evaluación
            let offspring_fitness = self.evaluate_population(
                &offspring,
                evaluator,
                context_name,
                seismic_intensity,
                airflow_velocity,
            );

            for (i, individual) in offspring.iter_mut().enumerate() {
                individual.fitness = offspring_fitness[i];
            }

            // 3.6. Reemplazo (elitismo + nueva generación)
            population = self.replacement(&population, &offspring);

            // 3.7. Estadísticas
            let best = population.iter().max().unwrap();
            let worst = population.iter().min().unwrap();
            let avg_fitness: f32 = population.iter().map(|i| i.fitness).sum::<f32>() 
                / population.len() as f32;

            average_fitness_history.push(avg_fitness);

            if best.fitness > best_ever.fitness {
                best_ever = best.clone();
                convergence_generation = gen;
            }

            println!("   Best: {:.3} | Avg: {:.3} | Worst: {:.3} | Diversity: {:.2}", 
                best.fitness, avg_fitness, worst.fitness, diversity);

            generations.push(GenerationSnapshot {
                generation: gen,
                best_fitness: best.fitness,
                average_fitness: avg_fitness,
                worst_fitness: worst.fitness,
                diversity,
                mutation_count,
                crossover_count: offspring.len() as u32,
            });

            // Criterio de convergencia temprana
            if gen > 20 && diversity < 0.05 {
                println!("\n⚠️ Convergencia prematura detectada en generación {}", gen);
                break;
            }

            // Inyección de diversidad si es necesario
            if diversity < self.config.diversity_preservation.min_diversity {
                println!("   💉 Inyectando diversidad...");
                self.inject_diversity(&mut population, initial_design);
            }
        }

        println!("\n✅ EVOLUCIÓN COMPLETADA");
        println!("   Mejor fitness: {:.3}", best_ever.fitness);
        println!("   Alcanzado en generación: {}", convergence_generation);

        EvolutionResult {
            generations,
            best_individual: best_ever.genome,
            best_fitness: best_ever.fitness,
            convergence_generation,
            diversity_history,
            average_fitness_history,
        }
    }

    /// Inicializa la población con variaciones del diseño inicial
    fn initialize_population(&mut self, initial_design: &UniversalDesign) -> Vec<Individual> {
        let mut population = Vec::with_capacity(self.config.population_size);

        // Añadir diseño original
        population.push(Individual {
            genome: initial_design.clone(),
            fitness: 0.0,
            age: 0,
            lineage_id: self.generate_lineage_id(),
        });

        // Generar variaciones
        for _ in 1..self.config.population_size {
            let mut variant = initial_design.clone();
            
            // Mutación inicial agresiva
            self.apply_mutation(&mut variant, 0.3);

            population.push(Individual {
                genome: variant,
                fitness: 0.0,
                age: 0,
                lineage_id: self.generate_lineage_id(),
            });
        }

        population
    }

    /// Evalúa toda la población en paralelo
    fn evaluate_population(
        &self,
        population: &[Individual],
        evaluator: &mut HARMONIABatchEvaluator,
        context_name: &str,
        seismic_intensity: f32,
        airflow_velocity: f32,
    ) -> Vec<f32> {
        let genomes: Vec<_> = population.iter().map(|i| &i.genome).collect();
        
        let batch_result = evaluator.evaluate_batch(
            &genomes.iter().map(|&g| g.clone()).collect::<Vec<_>>(),
            context_name,
            seismic_intensity,
            airflow_velocity,
        );

        batch_result.fitness_scores
    }

    /// Selección de padres
    fn selection(&mut self, population: &[Individual]) -> Vec<Individual> {
        let selection_size = (population.len() as f32 * self.config.selection_pressure * 2.0) as usize;
        let mut selected = Vec::with_capacity(selection_size);

        let selection_method = self.config.selection_method.clone();
        match selection_method {
            SelectionMethod::Tournament { size } => {
                for _ in 0..selection_size {
                    let winner = self.tournament_selection(population, size);
                    selected.push(winner);
                }
            }

            SelectionMethod::Roulette => {
                for _ in 0..selection_size {
                    let winner = self.roulette_selection(population);
                    selected.push(winner);
                }
            }

            SelectionMethod::Rank { bias } => {
                for _ in 0..selection_size {
                    let winner = self.rank_selection(population, bias);
                    selected.push(winner);
                }
            }

            SelectionMethod::StochasticUniversal => {
                selected = self.stochastic_universal_selection(population, selection_size);
            }
        }

        selected
    }

    fn tournament_selection(&mut self, population: &[Individual], tournament_size: usize) -> Individual {
        let mut best = &population[self.rng.usize(0..population.len())];

        for _ in 1..tournament_size {
            let contestant = &population[self.rng.usize(0..population.len())];
            if contestant.fitness > best.fitness {
                best = contestant;
            }
        }

        best.clone()
    }

    fn roulette_selection(&mut self, population: &[Individual]) -> Individual {
        let total_fitness: f32 = population.iter().map(|i| i.fitness.max(0.0)).sum();
        let threshold = self.rng.f32() * total_fitness;

        let mut cumulative = 0.0;
        for individual in population {
            cumulative += individual.fitness.max(0.0);
            if cumulative >= threshold {
                return individual.clone();
            }
        }

        population.last().unwrap().clone()
    }

    fn rank_selection(&mut self, population: &[Individual], bias: f32) -> Individual {
        let mut sorted: Vec<_> = population.iter().collect();
        sorted.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

        let n = sorted.len() as f32;
        let rank_sum: f32 = (1..=sorted.len()).map(|r| {
            let rank = r as f32;
            (2.0 - bias) + 2.0 * (bias - 1.0) * (rank - 1.0) / (n - 1.0)
        }).sum();

        let threshold = self.rng.f32() * rank_sum;
        let mut cumulative = 0.0;

        for (i, individual) in sorted.iter().enumerate() {
            let rank = (i + 1) as f32;
            cumulative += (2.0 - bias) + 2.0 * (bias - 1.0) * (rank - 1.0) / (n - 1.0);
            
            if cumulative >= threshold {
                return (*individual).clone();
            }
        }

        (*sorted.last().unwrap()).clone()
    }

    fn stochastic_universal_selection(&mut self, population: &[Individual], count: usize) -> Vec<Individual> {
        let total_fitness: f32 = population.iter().map(|i| i.fitness.max(0.0)).sum();
        let spacing = total_fitness / count.max(1) as f32;
        let start = self.rng.f32() * spacing;

        let mut selected = Vec::with_capacity(count);
        let mut cumulative = 0.0;
        let mut current = start;
        let mut i = 0;

        while selected.len() < count && i < population.len() {
            cumulative += population[i].fitness.max(0.0);
            
            while current <= cumulative && selected.len() < count {
                selected.push(population[i].clone());
                current += spacing;
            }

            i += 1;
        }

        selected
    }

    /// Crossover (cruce genético)
    fn crossover(&mut self, selected: &[Individual]) -> Vec<Individual> {
        let mut offspring = Vec::new();

        for i in (0..selected.len()).step_by(2) {
            if i + 1 >= selected.len() {
                break;
            }

            let parent1 = &selected[i];
            let parent2 = &selected[i + 1];

            if self.rng.f32() < self.config.crossover_rate {
                let (child1, child2) = self.perform_crossover(parent1, parent2);
                offspring.push(child1);
                offspring.push(child2);
            } else {
                offspring.push(parent1.clone());
                offspring.push(parent2.clone());
            }
        }

        offspring
    }

    fn perform_crossover(&mut self, parent1: &Individual, parent2: &Individual) -> (Individual, Individual) {
        // Crossover de un punto en las primitivas
        let p1_prims = &parent1.genome.primitives;
        let p2_prims = &parent2.genome.primitives;

        let min_len = p1_prims.len().min(p2_prims.len());
        if min_len <= 1 {
            return (parent1.clone(), parent2.clone());
        }

        let crossover_point = self.rng.usize(1..min_len);

        let mut child1_prims = p1_prims[..crossover_point].to_vec();
        child1_prims.extend_from_slice(&p2_prims[crossover_point..]);

        let mut child2_prims = p2_prims[..crossover_point].to_vec();
        child2_prims.extend_from_slice(&p1_prims[crossover_point..]);

        let child1 = Individual {
            genome: UniversalDesign {
                object_type: parent1.genome.object_type.clone(),
                primitives: child1_prims,
                bounding_box: parent1.genome.bounding_box.clone(),
            },
            fitness: 0.0,
            age: 0,
            lineage_id: format!("{}-{}", parent1.lineage_id, parent2.lineage_id),
        };

        let child2 = Individual {
            genome: UniversalDesign {
                object_type: parent2.genome.object_type.clone(),
                primitives: child2_prims,
                bounding_box: parent2.genome.bounding_box.clone(),
            },
            fitness: 0.0,
            age: 0,
            lineage_id: format!("{}-{}", parent2.lineage_id, parent1.lineage_id),
        };

        (child1, child2)
    }

    /// Mutación
    fn mutate(&mut self, offspring: &mut [Individual], generation: u32) -> u32 {
        let mut mutation_count = 0;

        for individual in offspring {
            if self.rng.f32() < self.config.mutation_rate {
                let intensity = match &self.config.mutation_strategy {
                    MutationStrategy::Uniform { intensity } => *intensity,
                    
                    MutationStrategy::Adaptive { min_intensity, max_intensity } => {
                        // Disminuir intensidad con el tiempo
                        let progress = generation as f32 / self.config.num_generations.max(1) as f32;
                        max_intensity - (max_intensity - min_intensity) * progress
                    }

                    MutationStrategy::Guided { learning_rate } => {
                        // Basado en fitness (mayor fitness = menor mutación)
                        learning_rate * (1.0 - individual.fitness)
                    }

                    MutationStrategy::Topological { .. } => 0.2,

                    MutationStrategy::Hybrid(strategies) => {
                        // Usar estrategia aleatoria
                        if let Some(strategy) = strategies.get(self.rng.usize(0..strategies.len().max(1))) {
                            match strategy {
                                MutationStrategy::Uniform { intensity } => *intensity,
                                _ => 0.15,
                            }
                        } else {
                            0.15
                        }
                    }
                };

                self.apply_mutation(&mut individual.genome, intensity);
                mutation_count += 1;
            }
        }

        mutation_count
    }

    /// Aplica mutación a un diseño
    fn apply_mutation(&mut self, design: &mut UniversalDesign, intensity: f32) {
        match &self.config.mutation_strategy {
            MutationStrategy::Topological { add_probability, remove_probability } => {
                // Añadir primitiva
                if self.rng.f32() < *add_probability && design.primitives.len() < 50 {
                    if let Some(template) = design.primitives.first() {
                        let mut new_prim = template.clone();
                        
                        // Posición aleatoria cercana
                        new_prim.position[0] += (self.rng.f32() - 0.5) * 5.0;
                        new_prim.position[1] += (self.rng.f32() - 0.5) * 5.0;
                        new_prim.position[2] += (self.rng.f32() - 0.5) * 5.0;

                        design.primitives.push(new_prim);
                    }
                }

                // Quitar primitiva
                if self.rng.f32() < *remove_probability && design.primitives.len() > 3 {
                    let remove_idx = self.rng.usize(0..design.primitives.len());
                    design.primitives.remove(remove_idx);
                }
            }

            _ => {
                // Mutación estándar de parámetros
                for primitive in &mut design.primitives {
                    // Mutar posición
                    if self.rng.f32() < 0.5 {
                        for i in 0..3 {
                            primitive.position[i] += (self.rng.f32() - 0.5) * intensity * 2.0;
                        }
                    }

                    // Mutar escala
                    if self.rng.f32() < 0.5 {
                        for i in 0..3 {
                            let factor = 1.0 + (self.rng.f32() - 0.5) * intensity;
                            primitive.scale[i] *= factor;
                            primitive.scale[i] = primitive.scale[i].max(0.1_f32);
                        }
                    }

                    // Mutar rotación
                    if self.rng.f32() < 0.3 {
                        for i in 0..3 {
                            primitive.rotation[i] += (self.rng.f32() - 0.5) * intensity * 90.0;
                        }
                    }
                }
            }
        }

        // Actualizar bounding box
        self.update_bounding_box(design);
    }

    fn update_bounding_box(&self, design: &mut UniversalDesign) {
        let mut min = [f32::INFINITY; 3];
        let mut max = [f32::NEG_INFINITY; 3];

        for prim in &design.primitives {
            for i in 0..3 {
                min[i] = min[i].min(prim.position[i] - prim.scale[i] / 2.0);
                max[i] = max[i].max(prim.position[i] + prim.scale[i] / 2.0);
            }
        }

        design.bounding_box = BoundingBox {
            width: max[0] - min[0],
            height: max[1] - min[1],
            depth: max[2] - min[2],
        };
    }

    /// Reemplazo generacional con elitismo
    fn replacement(&self, old_population: &[Individual], offspring: &[Individual]) -> Vec<Individual> {
        let elite_count = (old_population.len() as f32 * self.config.elitism_rate) as usize;

        // Ordenar población anterior por fitness
        let mut sorted_old: Vec<_> = old_population.iter().collect();
        sorted_old.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());

        // Tomar elite
        let mut new_population: Vec<Individual> = sorted_old.iter()
            .take(elite_count)
            .map(|&i| {
                let mut clone = i.clone();
                clone.age += 1;
                clone
            })
            .collect();

        // Añadir offspring
        let remaining = self.config.population_size - elite_count;
        new_population.extend(offspring.iter().take(remaining).cloned());

        // Si no hay suficientes, completar con individuos aleatorios de la población antigua
        while new_population.len() < self.config.population_size {
            let idx = fastrand::usize(..old_population.len());
            new_population.push(old_population[idx].clone());
        }

        new_population.truncate(self.config.population_size);
        new_population
    }

    /// Calcula diversidad de la población
    fn calculate_diversity(&self, population: &[Individual]) -> f32 {
        if population.len() < 2 {
            return 1.0;
        }

        let mut total_distance = 0.0;
        let mut comparisons = 0;

        for i in 0..population.len() {
            for j in (i + 1)..population.len() {
                let distance = self.genome_distance(&population[i].genome, &population[j].genome);
                total_distance += distance;
                comparisons += 1;
            }
        }

        if comparisons == 0 {
            return 0.0;
        }

        (total_distance / comparisons as f32).min(1.0_f32)
    }

    fn genome_distance(&self, g1: &UniversalDesign, g2: &UniversalDesign) -> f32 {
        let mut distance = 0.0;

        // Diferencia en número de primitivas
        distance += ((g1.primitives.len() as i32 - g2.primitives.len() as i32).abs() as f32) * 0.1;

        // Diferencia en posiciones
        let min_len = g1.primitives.len().min(g2.primitives.len());
        for i in 0..min_len {
            let p1 = &g1.primitives[i];
            let p2 = &g2.primitives[i];

            let pos_dist = ((p1.position[0] - p2.position[0]).powi(2) +
                           (p1.position[1] - p2.position[1]).powi(2) +
                           (p1.position[2] - p2.position[2]).powi(2)).sqrt();

            distance += pos_dist * 0.1;
        }

        distance
    }

    /// Inyecta diversidad cuando la población converge demasiado
    fn inject_diversity(&mut self, population: &mut Vec<Individual>, template: &UniversalDesign) {
        let injection_count = population.len() / 5; // 20% de la población

        for _ in 0..injection_count {
            let idx = self.rng.usize(0..population.len());
            
            // Crear individuo completamente nuevo
            let mut new_genome = template.clone();
            self.apply_mutation(&mut new_genome, 0.5); // Mutación agresiva

            population[idx] = Individual {
                genome: new_genome,
                fitness: 0.0,
                age: 0,
                lineage_id: self.generate_lineage_id(),
            };
        }
    }

    fn generate_lineage_id(&mut self) -> String {
        self.lineage_counter += 1;
        format!("L{:08X}", self.lineage_counter)
    }
}
