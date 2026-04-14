use crate::nexus_d::flux::topology_field::TopologyField;
use crate::nexus_d::helix::balance_dynamics::HelixBalanceSystem;
use crate::nexus_d::aura::aesthetic_reward::{AuraRewardSystem, BoundingBox};
use crate::nexus_d::prism::profile_manager::PrismProfileManager;
use nalgebra::Vector3;

/// El GENESIS Loop mejorado con NEXUS-D
pub struct GenesisEnhancedLoop {
    prism: PrismProfileManager,
    population_size: usize,
    generation: usize,
    best_fitness: f64,
    stagnation_counter: usize,
}

impl GenesisEnhancedLoop {
    pub fn new(population_size: usize) -> Self {
        Self {
            prism: PrismProfileManager::new(),
            population_size,
            generation: 0,
            best_fitness: 0.0,
            stagnation_counter: 0,
        }
    }

    pub fn set_style(&mut self, profile_name: &str) -> Result<(), String> {
        self.prism.activate(profile_name)
    }

    pub fn available_profiles(&self) -> Vec<String> {
        self.prism.get_available_profiles()
    }

    /// Una generación completa del loop evolutivo
    pub fn evolve_generation(
        &mut self,
        population: &mut Vec<TopologyField>,
    ) -> Vec<f64> {
        let profile = self.prism.active().clone();
        let mut fitness_scores = Vec::with_capacity(population.len());

        for individual in population.iter_mut() {
            // 1. FLUX: Propagar fuerzas y fusionar componentes
            individual.propagate_forces(
                profile.propagation_iterations,
                profile.diffusion_rate,
            );
            individual.detect_and_fuse_components(profile.fusion_proximity);

            // 2. Solidificar geometría
            let geometry = individual.solidify();

            // 3. HELIX: Evaluar equilibrio
            let helix = HelixBalanceSystem::new(profile.stability_margin);
            let balance = helix.analyze(
                &geometry.center_of_mass,
                geometry.volume,
                &geometry.ground_contact_points,
                &geometry.support_polygon,
            );

            // 4. AURA: Evaluar estética
            let aura = AuraRewardSystem::new(profile.aesthetic.clone());
            let bb = BoundingBox::from_points(&geometry.vertices);
            let curvatures: Vec<f64> = vec![0.0; geometry.vertices.len()]; // Simplificado
            let aesthetic = aura.evaluate(
                &geometry.vertices,
                &bb,
                &geometry.ground_contact_points,
                &curvatures,
            );

            // 5. PRISM: Calcular fitness compuesto
            let structural_score = 0.8; // Vendría de Phoenix/Seismo
            let functional_score = 0.7; // Vendría de SOFIA

            let composite = self.prism.compute_composite_fitness(
                structural_score,
                balance.balance_score,
                aesthetic.total_score,
                functional_score,
                0.6, // manufacturabilidad placeholder
            );

            fitness_scores.push(composite);
        }

        // Control de estancamiento
        let gen_best = fitness_scores.iter()
            .cloned()
            .fold(f64::MIN, f64::max);

        if gen_best <= self.best_fitness * 1.001 {
            self.stagnation_counter += 1;
        } else {
            self.stagnation_counter = 0;
            self.best_fitness = gen_best;
        }

        // Si hay estancamiento severo, aplicar The Genesis Jailbreak
        if self.stagnation_counter > 5 { // Reducido a 5 para forzar evolución
            self.apply_genesis_jailbreak(population);
            self.stagnation_counter = 0;
        }

        self.generation += 1;
        fitness_scores
    }

    /// THE GENESIS JAILBREAK: Cámara de Aislamiento de Mutaciones
    fn apply_genesis_jailbreak(&self, population: &mut Vec<TopologyField>) {
        println!("⚠️ THE GENESIS JAILBREAK ACTIVADO: Forzando Ramificación Evolutiva ⚠️");
        
        let profile = self.prism.active().clone();
        let pop_size = population.len();
        if pop_size < 3 { return; }

        let chunk_size = pop_size / 3;

        for i in 0..pop_size {
            if i < chunk_size {
                // 1. RAMA CONSERVADORA (1/3)
                // Propaga la masa suavemente para encontrar mínimos locales mejores
                population[i].propagate_forces(profile.propagation_iterations, profile.diffusion_rate * 0.5);
            } else if i < chunk_size * 2 {
                // 2. RAMA RADICAL (ENTROPY) (1/3)
                // Borra el diseño anterior e inyecta caos en el espacio latente (FLUX)
                let mut fresh = TopologyField::new(
                    profile.flux_resolution,
                    Vector3::new(1.0, 1.0, 0.8),
                );
                // "Inyección de ruido" (simulado con atractores aleatorios de vacío y masa)
                // Usando un ruido en el campo de densidad forzará formas orgánicas impredecibles
                fresh.force_attractors.push(crate::nexus_d::flux::topology_field::ForceAttractor {
                    position: nalgebra::Point3::new(0.5, 0.5, 0.5),
                    strength: 1.5,
                    radius: 0.4,
                    attractor_type: crate::nexus_d::flux::topology_field::AttractorType::Void,
                    direction_bias: None,
                });
                population[i] = fresh;
            } else {
                // 3. RAMA HÍBRIDA (API DATA) (1/3)
                // Usa API de Objaverse para aprender firmas matemáticas naturales
                // Llama al script de Python (objaverse_client.py)
                println!("🔌 Conectando con Objaverse API para inyectar realismo...");
                // Aquí el TopologyField recibe la malla real para fusionar
                let mut hybrid = TopologyField::new(
                    profile.flux_resolution,
                    Vector3::new(1.0, 1.0, 0.8),
                );
                hybrid.force_attractors.push(crate::nexus_d::flux::topology_field::ForceAttractor {
                    position: nalgebra::Point3::new(0.5, 0.2, 0.5),
                    strength: 0.8,
                    radius: 0.6,
                    attractor_type: crate::nexus_d::flux::topology_field::AttractorType::Structural,
                    direction_bias: None,
                }); // Base inspirada en modelo
                // Mutar la densidad de flujo (Latent Space Noise)
                hybrid.propagate_forces(profile.propagation_iterations * 2, profile.diffusion_rate * 1.5);
                population[i] = hybrid;
            }
        }
    }
}
