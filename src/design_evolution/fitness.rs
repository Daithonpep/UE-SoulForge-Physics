// design_evolution/fitness.rs
// Función de Fitness Multiobjetivo — Evalúa diseños en 5 dimensiones:
//   1. Funcionalidad (¿cumple su propósito?)
//   2. Estética (¿es visualmente coherente?)
//   3. Novedad (¿es diferente a diseños previos?)
//   4. Eficiencia (¿usa pocos recursos GPU?)
//   5. Estabilidad (¿es físicamente estable?)

use super::dna::*;
use super::mutation_engine::{DesignGenome, GeneValue};
use serde::{Deserialize, Serialize};
use rapier3d::prelude::*;

// ============================================================
// STRUCTS PÚBLICOS
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FitnessScore {
    pub total: f32,
    pub breakdown: FitnessBreakdown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FitnessBreakdown {
    pub functionality: f32,
    pub aesthetics: f32,
    pub novelty: f32,
    pub efficiency: f32,
    pub stability: f32,
}

/// Resultado de simulación física (viene del Gym o de Unreal)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SimulationResult {
    pub is_stable: bool,
    pub has_floating_parts: bool,
    pub max_supported_weight: f32,
    pub bounding_box: ([f32; 3], [f32; 3]),
    pub triangle_count: u32,
    pub draw_calls: u32,
    pub collision_errors: u32,
}

impl Default for SimulationResult {
    fn default() -> Self {
        Self {
            is_stable: true,
            has_floating_parts: false,
            max_supported_weight: 100.0,
            bounding_box: ([0.0; 3], [1.0, 1.0, 1.0]),
            triangle_count: 500,
            draw_calls: 10,
            collision_errors: 0,
        }
    }
}

// ============================================================
// EVALUADOR
// ============================================================

pub struct FitnessEvaluator {
    novelty_archive: Vec<DesignGenome>,
    max_archive_size: usize,
    /// Pesos configurables por dimensión
    pub weights: FitnessWeights,
    /// Validador SOFIA universal (estándares funcionales del mundo real)
    sofia_validator: crate::sofia::universal_validator::UniversalValidator,
    /// Motor de física PHOENIX
    phoenix_engine: crate::phoenix::PHOENIX,
    /// Motor de contexto HARMONIA MUSE
    harmonia_muse: crate::harmonia::context::MUSE,
    /// Fingerprints: rastrea shape+legcount combos vistos para penalizar repetición
    seen_fingerprints: std::collections::HashMap<String, u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FitnessWeights {
    pub functionality: f32,
    pub aesthetics: f32,
    pub novelty: f32,
    pub efficiency: f32,
    pub stability: f32,
}

impl Default for FitnessWeights {
    fn default() -> Self {
        Self {
            functionality: 0.35,
            aesthetics: 0.25,
            novelty: 0.20,
            efficiency: 0.10,
            stability: 0.10,
        }
    }
}

impl FitnessEvaluator {
    pub fn new(max_archive_size: usize) -> Self {
        Self {
            novelty_archive: Vec::new(),
            max_archive_size,
            weights: FitnessWeights::default(),
            sofia_validator: crate::sofia::universal_validator::UniversalValidator::new(),
            phoenix_engine: crate::phoenix::PHOENIX::new(),
            harmonia_muse: crate::harmonia::context::MUSE::new(),
            seen_fingerprints: std::collections::HashMap::new(),
        }
    }

    /// Evalúa el fitness total de un diseño
    pub fn evaluate(
        &mut self,
        genome: &DesignGenome,
        sim: &SimulationResult,
    ) -> FitnessScore {
        let functionality = self.evaluate_functionality(genome, sim);
        let mut _integration_parameters = IntegrationParameters::default();
        let mut _physics_pipeline = PhysicsPipeline::new();
        let mut _island_manager = IslandManager::new();
        let mut _broad_phase = DefaultBroadPhase::new();
        let mut _narrow_phase = NarrowPhase::new();
        let mut _impulse_joint_set = ImpulseJointSet::new();
        let mut _multibody_joint_set = MultibodyJointSet::new();
        let mut _ccd_solver = CCDSolver::new();
        
        let aesthetics = self.evaluate_aesthetics(genome);
        let novelty = self.evaluate_novelty(genome);
        let efficiency = self.evaluate_efficiency(genome, sim);
        let stability = self.evaluate_stability(sim);

        let w = &self.weights;

        // 🧠 SOFIA: Validación universal contra estándares funcionales del mundo real
        let category_name = match &genome.dna.category {
            DesignCategory::Furniture(FurnitureType::Table) => "table",
            DesignCategory::Furniture(FurnitureType::Chair) => "chair",
            DesignCategory::Furniture(FurnitureType::Bed) => "bed",
            DesignCategory::Vehicle(VehicleType::Car) => "car",
            DesignCategory::Vehicle(VehicleType::Aircraft) => "airplane",
            DesignCategory::Architecture(ArchitectureType::Building) => "building",
            DesignCategory::Architecture(ArchitectureType::Door) => "door",
            _ => "",
        };
        
        let sofia_score = if !category_name.is_empty() {
            self.sofia_validator.sofia_fitness_score(category_name, &genome.genes)
        } else {
            1.0 // Sin template SOFIA, no penalizar
        };

        // SOFIA influencia DIRECTAMENTE la funcionalidad: si SOFIA dice que no cumple el propósito,
        // el fitness total se reduce drásticamente
        let adjusted_functionality = functionality * 0.4 + sofia_score * 0.6;

        let universal_design = self.sofia_validator.genome_to_universal_design(category_name, &genome.genes);

        // 🎨 HARMONIA / FIBONACCI: Evaluación estética avanzada
        let active_context = self.harmonia_muse.get_active_context();
        let fibonacci_score = crate::harmonia::aesthetics::FIBONACCIEngine::evaluate(
            &universal_design, 
            &active_context.aesthetic_rules
        );
        
        let advanced_aesthetics = (aesthetics * 0.3) + (fibonacci_score.total_score * 0.7);

        // ⚖️ HARMONIA / MUSE: Pesos dinámicos basados en contexto
        let ctx_w = &active_context.priority_weights;
        
        // Mapeo adaptativo
        let base_total = ctx_w.structural_integrity * stability
            + ctx_w.economic_efficiency * efficiency // Proxy para manufacturabilidad/economía
            + ctx_w.aerodynamic_performance * (if category_name == "vehicle" || category_name == "car" { efficiency } else { 1.0 })
            + ctx_w.aesthetic_harmony * advanced_aesthetics
            + ctx_w.innovation_factor * novelty;

        // 🔥 PHOENIX: Multiplicador de física realista
        let physics_modifier = self.phoenix_engine.quick_physics_modifier(&universal_design);

        // THE GENESIS JAILBREAK: Multiplicador de Caos y Novedad
        // Si el diseño fue infundido por "ENTROPY" (caos_inverted_logic presente), darle un fuerte bonus para que suba y rompa la norma.
        let chaos_bonus = if genome.genes.iter().any(|g| g.trait_name == "caos_inverted_logic") { 1.35 } else { 1.0 };
        
        let mut total = (base_total * physics_modifier * adjusted_functionality * chaos_bonus).clamp(0.0, 1.0);

        // FITNESS OF PROGRESSIVE COMPLEXITY:
        // Penalizar fuertemente soluciones primitivas (como cilindros básicos de 7 patas) repetidas
        // Usamos el nombre del contexto activo como proxy de "era"
        let era = active_context.name.as_str();
        if era != "furniture_general" {
            // Si el diseño no tiene suficiente complejidad geométrica, castigarlo
            if sim.triangle_count < 1200 {
                total *= 0.65; // Penalización por "Pereza Generativa"
            }
        }
        
        // 🎯 DIVERSIDAD OBLIGATORIA: Evitar que el fitness siempre sea 1.0 
        // agregando un multiplicador agresivo de Novedad
        if novelty < 0.3 {
            total *= 0.6; // Castigar clones rotundos
        } else if novelty < 0.5 {
            total *= 0.8; // Penalizar baja variación
        }

        // Agregar al archivo de novedad si es suficientemente único
        if novelty > 0.7 && self.novelty_archive.len() < self.max_archive_size {
            self.novelty_archive.push(genome.clone());
        }

        FitnessScore {
            total,
            breakdown: FitnessBreakdown {
                functionality,
                aesthetics,
                novelty,
                efficiency,
                stability,
            },
        }
    }

    /// Cantidad de diseños en el archivo de novedad
    pub fn novelty_archive_size(&self) -> usize {
        self.novelty_archive.len()
    }

    /// Resetear archivo de novedad (al cambiar de categoría)
    pub fn reset_novelty(&mut self) {
        self.novelty_archive.clear();
        // NO limpiamos seen_fingerprints — queremos memoria persistente entre runs
    }

    // =========================================================
    // EVALUACIONES INDIVIDUALES
    // =========================================================

    fn evaluate_functionality(&self, genome: &DesignGenome, sim: &SimulationResult) -> f32 {
        let mut score = 0.0;
        let rules = &genome.dna.core_constraints.physics_rules;
        let rule_weight = if rules.is_empty() { 1.0 } else { 1.0 / rules.len() as f32 };

        for rule in rules {
            match rule {
                PhysicsRule::MustSupportWeight { min_kg } => {
                    if sim.max_supported_weight >= *min_kg {
                        score += rule_weight;
                    } else {
                        // Puntuación parcial por acercarse
                        score += rule_weight * (sim.max_supported_weight / min_kg).min(1.0);
                    }
                }
                PhysicsRule::CenterOfGravityStable => {
                    if sim.is_stable {
                        score += rule_weight;
                    }
                }
                PhysicsRule::NoFloatingParts => {
                    if !sim.has_floating_parts {
                        score += rule_weight;
                    }
                }
                PhysicsRule::MinimumContactArea { min_area_m2: _ } => {
                    // Placeholder — asumir que se cumple si es estable
                    if sim.is_stable {
                        score += rule_weight * 0.8;
                    }
                }
                PhysicsRule::SymmetryRequired { axis: _ } => {
                    // Evaluar vía gene de simetría
                    if let Some(sym) = genome.get_scalar("symmetry") {
                        score += rule_weight * sym;
                    }
                }
            }
        }

        // Bonus: tamaño dentro de bounds
        let (min_b, max_b) = &genome.dna.core_constraints.size_bounds;
        let scale = genome.get_scale();
        let in_bounds = (0..3).all(|i| scale[i] >= min_b[i] * 0.5 && scale[i] <= max_b[i] * 2.0);
        if in_bounds {
            score = (score + 0.1).min(1.0);
        }

        // 🧠 Lógica Ergonómica: Si es un Mueble para humanos, las patas no deben estorbar
        if let Some(inset) = genome.get_scalar("leg_inset") {
            // Bueno: inset cerca de 0.0 (en esquinas, espacio libre al medio)
            // Bueno: inset cerca de 0.5 (pedestal central, bordes libres)
            // Malo: inset ~0.25 (estorba rodillas si la gente se sienta al borde)
            let is_awkward = inset > 0.15 && inset < 0.4;
            
            if is_awkward {
                score -= 0.3; // Fuerte castigo por chocar rodillas
            } else {
                score += 0.2; // Premio a la innovación útil (pedestal o esquineras limpias)
            }
        }

        score.clamp(0.0_f32, 1.0_f32)
    }

    fn evaluate_aesthetics(&self, genome: &DesignGenome) -> f32 {
        let mut score = 0.0;
        let mut factors = 0;

        // 1. Simetría vs preferencia
        if let Some(sym) = genome.get_scalar("symmetry") {
            let pref = genome.dna.aesthetic_parameters.symmetry_preference;
            score += 1.0 - (sym - pref).abs();
            factors += 1;
        }

        // 2. Complejidad dentro del rango
        if let Some(complexity) = genome.get_scalar("complexity") {
            let (min_c, max_c) = genome.dna.aesthetic_parameters.complexity_range;
            if complexity >= min_c && complexity <= max_c {
                score += 1.0;
            } else {
                let dist = if complexity < min_c { min_c - complexity } else { complexity - max_c };
                score += (1.0 - dist).max(0.0);
            }
            factors += 1;
        }

        // 3. Proporción áurea (ratio ~1.618)
        let scale = genome.get_scale();
        if scale[2] > 0.001 {
            let ratio = scale[0] / scale[2];
            let golden = 1.618;
            let dist_golden = (ratio - golden).abs();
            score += 1.0 - dist_golden.min(1.0);
            factors += 1;
        }

        // 4. Decoración dentro del rango
        if let Some(dec) = genome.get_scalar("decoration_level") {
            let (min_d, max_d) = genome.dna.aesthetic_parameters.decoration_density;
            if dec >= min_d && dec <= max_d {
                score += 1.0;
            }
            factors += 1;
        }

        if factors > 0 {
            (score / factors as f32).clamp(0.0_f32, 1.0_f32)
        } else {
            0.5 // neutral si no hay datos
        }
    }

    fn evaluate_novelty(&mut self, genome: &DesignGenome) -> f32 {
        // Crear fingerprint basado en shape + leg_count + scale_bucket
        let shape_str = genome.genes.iter()
            .find(|g| g.trait_name == "primary_shape")
            .map(|g| format!("{:?}", g.value))
            .unwrap_or_else(|| "none".to_string());
        let leg_count = genome.get_integer("leg_count").unwrap_or(0);
        let scale = genome.get_scale();
        let scale_bucket = format!("{:.0}_{:.0}_{:.0}", scale[0]*10.0, scale[1]*10.0, scale[2]*10.0);
        let fingerprint = format!("{}_{}_{}_{}", shape_str, leg_count, scale_bucket,
            genome.get_scalar("symmetry").map(|s| format!("{:.1}", s)).unwrap_or_else(|| "x".to_string()));

        let count = self.seen_fingerprints.entry(fingerprint).or_insert(0);
        *count += 1;

        // Cuantas más veces se ha visto este fingerprint, peor es la novedad
        let repetition_penalty = match *count {
            1 => 1.0,       // Primera vez: totalmente novedoso
            2 => 0.7,       // Ya visto una vez
            3 => 0.4,       // Comenzando a repetirse
            4..=6 => 0.2,   // Claramente repetitivo
            _ => 0.05,      // Convergencia total — castigo severo
        };

        // Combinar con distancia al archivo de novedad (si existe)
        let archive_novelty = if self.novelty_archive.is_empty() {
            0.8 // NO dar 1.0 al primero — evita bias inicial
        } else {
            let total_distance: f32 = self.novelty_archive
                .iter()
                .map(|archived| self.genome_distance(genome, archived))
                .sum();
            let avg = total_distance / self.novelty_archive.len() as f32;
            (avg / 5.0).min(1.0)
        };

        (archive_novelty * repetition_penalty).clamp(0.0, 1.0)
    }

    fn genome_distance(&self, g1: &DesignGenome, g2: &DesignGenome) -> f32 {
        let mut distance = 0.0;

        for gene1 in &g1.genes {
            if let Some(gene2) = g2.genes.iter().find(|g| g.trait_name == gene1.trait_name) {
                distance += match (&gene1.value, &gene2.value) {
                    (GeneValue::Scalar(v1), GeneValue::Scalar(v2)) => (v1 - v2).abs(),
                    (GeneValue::Vector(a), GeneValue::Vector(b)) => {
                        ((a[0]-b[0]).powi(2) + (a[1]-b[1]).powi(2) + (a[2]-b[2]).powi(2)).sqrt()
                    }
                    (GeneValue::Integer(a), GeneValue::Integer(b)) => (a - b).unsigned_abs() as f32 * 0.1,
                    (GeneValue::Shape(a), GeneValue::Shape(b)) => {
                        if a == b { 0.0 } else { 1.0 }
                    }
                    _ => 0.0,
                };
            } else {
                distance += 0.5; // Gene ausente = distancia media
            }
        }

        distance
    }

    fn evaluate_efficiency(&self, genome: &DesignGenome, sim: &SimulationResult) -> f32 {
        let mut score: f32 = 1.0;

        // Penalizar exceso ridículo de geometría
        if sim.triangle_count > 10000 {
            score -= 0.3;
        }
        if sim.draw_calls > 100 {
            score -= 0.3;
        }
        if sim.triangle_count < 1000 {
            score += 0.2;
        }

        // Penalizar exceso de redundancia funcional (Ej. demasiadas patas innecesarias)
        if let Some(leg_count) = genome.get_integer("leg_count") {
            if leg_count > 4 {
                // Fuerte penalización por desperdicio de material si la mesa es chica
                score -= 0.2 * (leg_count as f32 - 4.0); 
            } else if leg_count == 4 {
                // Recompensar la estructura estándar eficiente
                score += 0.2; 
            }
        }

        score.clamp(0.0_f32, 1.0_f32)
    }

    fn evaluate_stability(&self, sim: &SimulationResult) -> f32 {
        let mut score: f32 = 1.0;

        if !sim.is_stable { score -= 0.5; }
        if sim.has_floating_parts { score -= 0.3; }
        if sim.collision_errors > 0 {
            score -= 0.2 * (sim.collision_errors as f32).min(1.0);
        }

        score.clamp(0.0_f32, 1.0_f32)
    }
}

/// Simula un resultado básico a partir del genoma (sin Unreal)
/// NOTA: Ya no devuelve resultados perfectos — introduce imperfecciones realistas
pub fn estimate_simulation(genome: &DesignGenome) -> SimulationResult {
    let scale = genome.get_scale();
    let volume = scale[0] * scale[1] * scale[2];

    // Estimar triángulos basados en complejidad y partes
    let complexity = genome.get_scalar("complexity").unwrap_or(0.5);
    let part_count: i32 = genome.genes.iter()
        .filter(|g| g.trait_name.contains("count"))
        .filter_map(|g| if let GeneValue::Integer(v) = g.value { Some(v) } else { None })
        .sum();

    let base_tris = 200 + (part_count.max(1) as u32 * 50);
    let detail_tris: u32 = genome.genes.iter()
        .filter(|g| g.trait_name.starts_with("detail_"))
        .filter_map(|g| if let GeneValue::Scalar(v) = g.value { Some((v * 500.0) as u32) } else { None })
        .sum();

    let tri_count = ((base_tris + detail_tris) as f32 * (1.0 + complexity)) as u32;

    // Usar RAPIER3D Physics Engine para simular caída y medir estabilidad física real
    let scale_arr = [scale[0], scale[1], scale[2]];
    let rapier_stable = super::physics_sim::PhysicsSimulator::test_stability(&genome.genes, &scale_arr);

    // Fetch support count since we still need it for max_supported_weight
    let support_count = genome.get_integer("leg_count")
        .or_else(|| genome.get_integer("wheel_count"))
        .unwrap_or(4);

    let leg_instability = !rapier_stable; 

    // Proporciones extremas causan partes flotantes
    let max_ratio = (scale[0] / scale[2].max(0.01)).max(scale[2] / scale[0].max(0.01));
    let has_floating = max_ratio > 5.0; // Diseños demasiado alargados tienen problemas

    // Simetría rota = más errores de colisión
    let symmetry = genome.get_scalar("symmetry").unwrap_or(0.5);
    let collision_errs = 0; // Se resolverá con mallas reales en Unreal

    SimulationResult {
        is_stable: !leg_instability,
        has_floating_parts: has_floating,
        max_supported_weight: volume * 50.0 * support_count as f32,
        bounding_box: ([0.0; 3], [scale[0], scale[1], scale[2]]),
        triangle_count: tri_count,
        draw_calls: (tri_count / 1000).max(1),
        collision_errors: collision_errs,
    }
}
