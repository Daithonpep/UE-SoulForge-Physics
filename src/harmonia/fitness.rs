// harmonia/fitness.rs
use serde::{Deserialize, Serialize};
use crate::sofia::universal_validator::*;
use crate::phoenix::reality_profiles::*;
use crate::phoenix::cfd_engine::*;
use crate::seismos::analysis::IntegrityAnalyzer; // Ajuste
use super::context::*;
use super::aesthetics::*;

/// Resultado completo de evaluación multiobjetivo
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MultiObjectiveFitness {
    /// Fitness total ponderado (0.0 - 1.0)
    pub total_fitness: f32,
    
    /// Desglose por objetivo
    pub breakdown: FitnessBreakdown,
    
    /// Score normalizado para cada componente
    pub normalized_scores: NormalizedScores,
    
    /// Penalizaciones aplicadas
    pub penalties: Vec<Penalty>,
    
    /// Bonificaciones aplicadas
    pub bonuses: Vec<Bonus>,
    
    /// Contexto usado para evaluación
    pub context_used: String,
    
    /// Recomendaciones de mejora priorizadas
    pub prioritized_recommendations: Vec<PrioritizedRecommendation>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FitnessBreakdown {
    /// Integridad estructural (SEISMOS)
    pub structural: StructuralFitness,
    
    /// Eficiencia económica
    pub economic: EconomicFitness,
    
    /// Rendimiento aerodinámico (CFD)
    pub aerodynamic: AerodynamicFitness,
    
    /// Armonía estética (FIBONACCI)
    pub aesthetic: AestheticFitness,
    
    /// Factor de innovación
    pub innovation: InnovationFitness,
    
    /// Manufacturabilidad
    pub manufacturing: ManufacturingFitness,
    
    /// Sostenibilidad
    pub sustainability: SustainabilityFitness,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructuralFitness {
    pub score: f32,
    pub safety_factor: f32,
    pub resonance_risk: f32,
    pub max_stress_ratio: f32,
    pub survival_probability: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomicFitness {
    pub score: f32,
    pub total_cost_usd: f32,
    pub cost_per_performance: f32,
    pub material_efficiency: f32,
    pub budget_utilization: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AerodynamicFitness {
    pub score: f32,
    pub drag_coefficient: f32,
    pub lift_coefficient: f32,
    pub efficiency_ratio: f32,
    pub turbulence_factor: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AestheticFitness {
    pub score: f32,
    pub symmetry: f32,
    pub golden_ratio: f32,
    pub harmony: f32,
    pub visual_balance: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InnovationFitness {
    pub score: f32,
    pub novelty_index: f32,
    pub cross_pollination_bonus: f32,
    pub chaos_survivor_bonus: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManufacturingFitness {
    pub score: f32,
    pub complexity_penalty: f32,
    pub assembly_difficulty: f32,
    pub tooling_cost: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SustainabilityFitness {
    pub score: f32,
    pub recyclability: f32,
    pub energy_efficiency: f32,
    pub carbon_footprint: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NormalizedScores {
    pub structural: f32,
    pub economic: f32,
    pub aerodynamic: f32,
    pub aesthetic: f32,
    pub innovation: f32,
    pub manufacturing: f32,
    pub sustainability: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Penalty {
    pub reason: String,
    pub severity: f32,
    pub impact: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Bonus {
    pub reason: String,
    pub value: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PrioritizedRecommendation {
    pub priority: u32,
    pub category: String,
    pub recommendation: String,
    pub expected_improvement: f32,
}

/// Motor principal de evaluación HARMONIA
pub struct HARMONIACore {
    pub muse: MUSE,
    seismic_analyzer: IntegrityAnalyzer,
    cfd_engine: CFDEngine,
    innovation_tracker: InnovationTracker,
}

/// Tracker de innovaciones para evitar repeticiones
struct InnovationTracker {
    seen_designs: lru::LruCache<u64, f32>,
}

impl InnovationTracker {
    fn new() -> Self {
        Self {
            seen_designs: lru::LruCache::new(std::num::NonZeroUsize::new(10000).unwrap()),
        }
    }

    fn calculate_novelty(&mut self, design: &UniversalDesign) -> f32 {
        let hash = self.hash_design(design);
        
        if let Some(&previous_score) = self.seen_designs.get(&hash) {
            // Ya visto, menor novedad
            0.3 * previous_score
        } else {
            // Nuevo diseño
            self.seen_designs.put(hash, 1.0);
            1.0
        }
    }

    fn hash_design(&self, design: &UniversalDesign) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let mut hasher = DefaultHasher::new();
        
        // Hash basado en topología
        design.primitives.len().hash(&mut hasher);
        
        for prim in &design.primitives {
            format!("{:?}", prim.primitive_type).hash(&mut hasher);
            ((prim.position[0] * 10.0) as i32).hash(&mut hasher);
            ((prim.position[1] * 10.0) as i32).hash(&mut hasher);
            ((prim.position[2] * 10.0) as i32).hash(&mut hasher);
        }

        hasher.finish()
    }
}

impl HARMONIACore {
    pub fn new(material_library: MaterialLibrary) -> Self {
        Self {
            muse: MUSE::new(),
            seismic_analyzer: IntegrityAnalyzer::new(material_library.clone()),
            cfd_engine: CFDEngine::new(32),
            innovation_tracker: InnovationTracker::new(),
        }
    }

    pub fn with_context(mut self, context_name: &str) -> Result<Self, String> {
        self.muse.set_context(context_name)?;
        Ok(self)
    }

    pub fn infer_context_from_prompt(&mut self, prompt: &str) -> String {
        self.muse.infer_context_from_prompt(prompt)
    }

    /// Evaluación completa multiobjetivo
    pub fn evaluate(
        &mut self,
        design: &UniversalDesign,
        seismic_intensity: f32,
        airflow_velocity: f32,
    ) -> MultiObjectiveFitness {
        println!("\n⚖️ HARMONIA MULTI-OBJECTIVE EVALUATION");
        println!("════════════════════════════════════════════════════");

        let context = self.muse.get_active_context().clone();
        let weights = &context.priority_weights;

        // 1. EVALUACIÓN ESTRUCTURAL
        let structural = self.evaluate_structural(design, seismic_intensity, &context);
        let structural_normalized = structural.score;

        // 2. EVALUACIÓN ECONÓMICA
        let economic = self.evaluate_economic(design, &context);
        let economic_normalized = economic.score;

        // 3. EVALUACIÓN AERODINÁMICA
        let aerodynamic = self.evaluate_aerodynamic(design, airflow_velocity, &context);
        let aerodynamic_normalized = aerodynamic.score;

        // 4. EVALUACIÓN ESTÉTICA
        let aesthetic = self.evaluate_aesthetic(design, &context.aesthetic_rules);
        let aesthetic_normalized = aesthetic.score;

        // 5. EVALUACIÓN DE INNOVACIÓN
        let innovation = self.evaluate_innovation(design);
        let innovation_normalized = innovation.score;

        // 6. EVALUACIÓN DE MANUFACTURABILIDAD
        let manufacturing = self.evaluate_manufacturing(design);
        let manufacturing_normalized = manufacturing.score;

        // 7. EVALUACIÓN DE SOSTENIBILIDAD
        let sustainability = self.evaluate_sustainability(design);
        let sustainability_normalized = sustainability.score;

        // 8. CALCULAR PENALIZACIONES Y BONIFICACIONES
        let (penalties, bonuses) = self.calculate_modifiers(
            design,
            &structural,
            &economic,
            &aerodynamic,
            &aesthetic,
            &context,
        );

        // 9. CALCULAR FITNESS TOTAL PONDERADO
        let base_fitness = 
            weights.structural_integrity * structural_normalized +
            weights.economic_efficiency * economic_normalized +
            weights.aerodynamic_performance * aerodynamic_normalized +
            weights.aesthetic_harmony * aesthetic_normalized +
            weights.innovation_factor * innovation_normalized +
            weights.manufacturability * manufacturing_normalized +
            weights.sustainability * sustainability_normalized;

        let penalty_total: f32 = penalties.iter().map(|p| p.impact).sum();
        let bonus_total: f32 = bonuses.iter().map(|b| b.value).sum();

        let total_fitness = (base_fitness - penalty_total + bonus_total).clamp(0.0, 1.0);

        // 10. GENERAR RECOMENDACIONES PRIORIZADAS
        let recommendations = self.generate_prioritized_recommendations(
            &structural,
            &economic,
            &aerodynamic,
            &aesthetic,
            &innovation,
            weights,
        );

        println!("\n   SCORES NORMALIZADOS:");
        println!("   ├─ Estructural:     {:.1}% (peso: {:.0}%)", structural_normalized * 100.0, weights.structural_integrity * 100.0);
        println!("   ├─ Económico:       {:.1}% (peso: {:.0}%)", economic_normalized * 100.0, weights.economic_efficiency * 100.0);
        println!("   ├─ Aerodinámico:    {:.1}% (peso: {:.0}%)", aerodynamic_normalized * 100.0, weights.aerodynamic_performance * 100.0);
        println!("   ├─ Estético:        {:.1}% (peso: {:.0}%)", aesthetic_normalized * 100.0, weights.aesthetic_harmony * 100.0);
        println!("   ├─ Innovación:      {:.1}% (peso: {:.0}%)", innovation_normalized * 100.0, weights.innovation_factor * 100.0);
        println!("   ├─ Manufactura:     {:.1}% (peso: {:.0}%)", manufacturing_normalized * 100.0, weights.manufacturability * 100.0);
        println!("   └─ Sostenibilidad:  {:.1}% (peso: {:.0}%)", sustainability_normalized * 100.0, weights.sustainability * 100.0);
        
        println!("\n   MODIFICADORES:");
        println!("   ├─ Penalizaciones:  -{:.1}%", penalty_total * 100.0);
        println!("   └─ Bonificaciones:  +{:.1}%", bonus_total * 100.0);

        println!("\n   🎯 FITNESS TOTAL:   {:.1}%", total_fitness * 100.0);
        println!("════════════════════════════════════════════════════\n");

        MultiObjectiveFitness {
            total_fitness,
            breakdown: FitnessBreakdown {
                structural,
                economic,
                aerodynamic,
                aesthetic,
                innovation,
                manufacturing,
                sustainability,
            },
            normalized_scores: NormalizedScores {
                structural: structural_normalized,
                economic: economic_normalized,
                aerodynamic: aerodynamic_normalized,
                aesthetic: aesthetic_normalized,
                innovation: innovation_normalized,
                manufacturing: manufacturing_normalized,
                sustainability: sustainability_normalized,
            },
            penalties,
            bonuses,
            context_used: context.name.clone(),
            prioritized_recommendations: recommendations,
        }
    }

    /// Evaluación estructural (SEISMOS)
    fn evaluate_structural(
        &mut self,
        design: &UniversalDesign,
        seismic_intensity: f32,
        context: &DesignContext,
    ) -> StructuralFitness {
        if !context.constraints.hard_physics {
            // En modo fantasía, estructura perfecta por defecto
            return StructuralFitness {
                score: 1.0,
                safety_factor: 5.0,
                resonance_risk: 0.0,
                max_stress_ratio: 0.2,
                survival_probability: 1.0,
            };
        }

        let report = self.seismic_analyzer.analyze_integrity(design, seismic_intensity);

        // Score basado en múltiples factores
        let safety_score = (report.safety_factor / 5.0).min(1.0_f32);
        let resonance_score = 1.0 - report.resonance_risk;
        let failure_score = if report.failure_points.is_empty() {
            1.0
        } else {
            (1.0 - (report.failure_points.len() as f32 / design.primitives.len().max(1) as f32)).max(0.0_f32)
        };

        let structural_score = (
            safety_score * 0.4 +
            resonance_score * 0.3 +
            failure_score * 0.3
        ).clamp(0.0, 1.0);

        // Calcular probabilidad de supervivencia
        let survival_probability = if report.safety_factor >= 1.5 && report.failure_points.is_empty() {
            0.99
        } else if report.safety_factor >= 1.0 {
            0.85
        } else {
            0.5 * report.safety_factor
        };

        // Calcular máximo ratio de estrés
        let max_stress_ratio = report.element_analysis.iter()
            .map(|e| e.utilization_ratio)
            .fold(0.0_f32, f32::max);

        StructuralFitness {
            score: structural_score,
            safety_factor: report.safety_factor,
            resonance_risk: report.resonance_risk,
            max_stress_ratio,
            survival_probability,
        }
    }

    /// Evaluación económica
    fn evaluate_economic(
        &self,
        design: &UniversalDesign,
        context: &DesignContext,
    ) -> EconomicFitness {
        // Calcular costo total
        let mut total_cost = 0.0;
        let mut total_mass = 0.0;

        for primitive in &design.primitives {
            let volume = primitive.scale[0] * primitive.scale[1] * primitive.scale[2];
            
            // Asumir material por defecto
            let density = 7850.0; // acero
            let cost_per_kg = 0.80;
            
            let mass = volume * density;
            let cost = mass * cost_per_kg;
            
            total_cost += cost;
            total_mass += mass;
        }

        // Añadir costos de manufactura (aproximación)
        let manufacturing_cost = total_cost * 0.5;
        total_cost += manufacturing_cost;

        // Calcular eficiencia
        let performance_proxy = design.primitives.len() as f32; // Simplificado
        let cost_per_performance = total_cost / performance_proxy.max(1.0_f32);

        // Material efficiency (menor masa = mejor)
        let material_efficiency = 1.0 - (total_mass / 10000.0).min(1.0_f32);

        // Budget utilization
        let budget_utilization = if let Some(budget) = context.constraints.budget_limit {
            if total_cost > budget {
                0.0 // Excede presupuesto
            } else {
                (total_cost / budget).min(1.0_f32)
            }
        } else {
            0.8 // Sin límite, score neutral-alto
        };

        let economic_score = (
            material_efficiency * 0.4 +
            budget_utilization * 0.4 +
            (1.0 - (cost_per_performance / 1000.0).min(1.0_f32)) * 0.2
        ).clamp(0.0, 1.0);

        EconomicFitness {
            score: economic_score,
            total_cost_usd: total_cost,
            cost_per_performance,
            material_efficiency,
            budget_utilization,
        }
    }

    /// Evaluación aerodinámica (CFD)
    fn evaluate_aerodynamic(
        &mut self,
        design: &UniversalDesign,
        velocity: f32,
        _context: &DesignContext,
    ) -> AerodynamicFitness {
        if velocity < 1.0 {
            // No aplica aerodinámica
            return AerodynamicFitness {
                score: 1.0,
                drag_coefficient: 0.0,
                lift_coefficient: 0.0,
                efficiency_ratio: 1.0,
                turbulence_factor: 0.0,
            };
        }

        let air_density = 1.225; // kg/m³ al nivel del mar
        let viscosity = 0.0000181; // Pa·s

        let analysis = self.cfd_engine.simulate_aerodynamics(
            design,
            velocity,
            air_density,
            viscosity,
        );

        // Score basado en objetivos típicos
        let cd_target = 0.30; // Target Cd para vehículos eficientes
        let drag_score = (1.0 - (analysis.drag_coefficient / cd_target).min(1.0_f32)).max(0.0_f32);

        let efficiency_score = (analysis.aerodynamic_efficiency / 10.0).min(1.0_f32);

        let turbulence_factor = analysis.turbulent_zones.len() as f32 / design.primitives.len().max(1) as f32;
        let turbulence_score = 1.0 - turbulence_factor.min(1.0_f32);

        let aerodynamic_score = (
            drag_score * 0.5 +
            efficiency_score * 0.3 +
            turbulence_score * 0.2
        ).clamp(0.0, 1.0);

        AerodynamicFitness {
            score: aerodynamic_score,
            drag_coefficient: analysis.drag_coefficient,
            lift_coefficient: analysis.lift_coefficient,
            efficiency_ratio: analysis.aerodynamic_efficiency,
            turbulence_factor,
        }
    }

    /// Evaluación estética (FIBONACCI)
    fn evaluate_aesthetic(
        &self,
        design: &UniversalDesign,
        rules: &AestheticRules,
    ) -> AestheticFitness {
        let aesthetic_score_obj = FIBONACCIEngine::evaluate(design, rules);

        AestheticFitness {
            score: aesthetic_score_obj.total_score,
            symmetry: aesthetic_score_obj.symmetry_score,
            golden_ratio: aesthetic_score_obj.golden_ratio_score,
            harmony: aesthetic_score_obj.harmony_score,
            visual_balance: aesthetic_score_obj.visual_balance,
        }
    }

    /// Evaluación de innovación
    fn evaluate_innovation(&mut self, design: &UniversalDesign) -> InnovationFitness {
        let novelty = self.innovation_tracker.calculate_novelty(design);

        // Bonus por cross-pollination (uso de primitivas no convencionales)
        let primitive_diversity: std::collections::HashSet<_> = design.primitives.iter()
            .map(|p| format!("{:?}", p.primitive_type))
            .collect();

        let cross_pollination_bonus = if primitive_diversity.len() > 3 {
            0.2
        } else {
            0.0
        };

        // Bonus por sobrevivir siendo caótico (si tiene propiedades de caos)
        let chaos_survivor_bonus = design.primitives.iter()
            .filter(|p| p.properties.get("chaos").is_some())
            .count() as f32 * 0.05;

        let innovation_score = (novelty * 0.7 + cross_pollination_bonus + chaos_survivor_bonus).min(1.0_f32);

        InnovationFitness {
            score: innovation_score,
            novelty_index: novelty,
            cross_pollination_bonus,
            chaos_survivor_bonus,
        }
    }

    /// Evaluación de manufacturabilidad
    fn evaluate_manufacturing(&self, design: &UniversalDesign) -> ManufacturingFitness {
        // Complejidad basada en número de primitivas
        let complexity = design.primitives.len() as f32 / 100.0;
        let complexity_penalty = complexity.min(1.0_f32);

        // Dificultad de ensamblaje (ángulos raros, solapamientos)
        let mut difficult_angles = 0;
        for prim in &design.primitives {
            if prim.rotation.iter().any(|&r| r.abs() > 45.0 && (r.abs() - 90.0).abs() > 5.0) {
                difficult_angles += 1;
            }
        }

        let assembly_difficulty = (difficult_angles as f32 / design.primitives.len().max(1) as f32).min(1.0_f32);

        // Costo de tooling (más complejo = más caro)
        let tooling_cost = complexity * 0.5 + assembly_difficulty * 0.5;

        let manufacturing_score = (1.0 - complexity_penalty * 0.4 - assembly_difficulty * 0.4 - tooling_cost * 0.2).max(0.0_f32);

        ManufacturingFitness {
            score: manufacturing_score,
            complexity_penalty,
            assembly_difficulty,
            tooling_cost,
        }
    }

    /// Evaluación de sostenibilidad
    fn evaluate_sustainability(&self, design: &UniversalDesign) -> SustainabilityFitness {
        // Reciclabilidad basada en materiales
        let recyclability = 0.8; // Asumir acero (altamente reciclable)

        // Eficiencia energética (menor masa = menos energía de transporte)
        let total_volume: f32 = design.primitives.iter()
            .map(|p| p.scale[0] * p.scale[1] * p.scale[2])
            .sum();

        let energy_efficiency = 1.0 - (total_volume / 100.0).min(1.0_f32);

        // Huella de carbono (aproximada)
        let carbon_footprint = total_volume * 7850.0 * 2.0; // kg CO2 aprox
        let carbon_score = 1.0 - (carbon_footprint / 100000.0).min(1.0_f32);

        let sustainability_score = (
            recyclability * 0.4 +
            energy_efficiency * 0.3 +
            carbon_score * 0.3
        ).clamp(0.0, 1.0);

        SustainabilityFitness {
            score: sustainability_score,
            recyclability,
            energy_efficiency,
            carbon_footprint,
        }
    }

    /// Calcula penalizaciones y bonificaciones
    fn calculate_modifiers(
        &self,
        design: &UniversalDesign,
        structural: &StructuralFitness,
        economic: &EconomicFitness,
        aerodynamic: &AerodynamicFitness,
        aesthetic: &AestheticFitness,
        context: &DesignContext,
    ) -> (Vec<Penalty>, Vec<Bonus>) {
        let mut penalties = Vec::new();
        let mut bonuses = Vec::new();

        // PENALIZACIONES CRÍTICAS
        if context.constraints.hard_physics {
            if structural.safety_factor < 1.0 {
                penalties.push(Penalty {
                    reason: "⚠️ CRÍTICO: Factor de seguridad < 1.0".to_string(),
                    severity: 1.0,
                    impact: 0.5,
                });
            }

            if structural.resonance_risk > 0.7 {
                penalties.push(Penalty {
                    reason: "⚠️ Alto riesgo de resonancia catastrófica".to_string(),
                    severity: 0.8,
                    impact: 0.2,
                });
            }
        }

        if let Some(budget) = context.constraints.budget_limit {
            if economic.total_cost_usd > budget {
                let overspend = (economic.total_cost_usd - budget) / budget;
                penalties.push(Penalty {
                    reason: format!("💰 Presupuesto excedido en {:.1}%", overspend * 100.0),
                    severity: overspend.min(1.0_f32),
                    impact: 0.3 * overspend.min(1.0_f32),
                });
            }
        }

        // PENALIZACIÓN DE PEREZA (Fitness de Complejidad Progresiva)
        let era = context.name.to_lowercase();
        let total_volume: f32 = design.primitives.iter()
            .map(|p| p.scale[0] * p.scale[1] * p.scale[2])
            .sum();
            
        let is_simple_cube = design.primitives.len() <= 2 && design.primitives.iter().all(|p| p.primitive_type == crate::sofia::primitives::FunctionalPrimitive::Container);

        if era.contains("gothic") || era.contains("renaissance") {
            if is_simple_cube {
                penalties.push(Penalty {
                    reason: "😴 Pereza: Diseño Gótico básico (Cubo simple). Requiere mejor ratio Resistencia/Peso".to_string(),
                    severity: 0.5,
                    impact: 0.20,
                });
            }
        } else if era.contains("modern") {
            let is_adaptive = design.primitives.len() > 10; // proxy de subdivisión
            if !is_adaptive {
                penalties.push(Penalty {
                    reason: "😴 Pereza: Diseño Moderno sin ergonomía o subdivisión (muy básico)".to_string(),
                    severity: 0.8,
                    impact: 0.50,
                });
            }
        } else if era.contains("futuristic") {
            let base_volume = 100.0; // aprox volumen primitivo
            if total_volume > base_volume * 0.7 {
                penalties.push(Penalty {
                    reason: "😴 Pereza: Diseño Futurista no es suficientemente ligero. Se requiere Eficiencia Topológica (FLUX).".to_string(),
                    severity: 0.9,
                    impact: 0.50, // Solo puntúa si rescata un residual
                });
            }
        }

        // BONIFICACIONES
        if structural.safety_factor > 3.0 && structural.survival_probability > 0.95 {
            bonuses.push(Bonus {
                reason: "✨ Ultra-seguro: Factor de seguridad excepcional".to_string(),
                value: 0.05,
            });
        }

        if aesthetic.symmetry > 0.9 && aesthetic.golden_ratio > 0.8 {
            bonuses.push(Bonus {
                reason: "🎨 Obra maestra estética: Fibonacci perfecto".to_string(),
                value: 0.10,
            });
        }

        if aerodynamic.drag_coefficient < 0.25 && aerodynamic.drag_coefficient > 0.0 {
            bonuses.push(Bonus {
                reason: "✈️ Aerodinámica de clase mundial".to_string(),
                value: 0.08,
            });
        }

        (penalties, bonuses)
    }

    /// Genera recomendaciones priorizadas
    fn generate_prioritized_recommendations(
        &self,
        structural: &StructuralFitness,
        economic: &EconomicFitness,
        aerodynamic: &AerodynamicFitness,
        aesthetic: &AestheticFitness,
        innovation: &InnovationFitness,
        weights: &PriorityWeights,
    ) -> Vec<PrioritizedRecommendation> {
        let mut recommendations = Vec::new();

        // Prioridad 1: Problemas críticos de seguridad
        if structural.safety_factor < 1.5 {
            recommendations.push(PrioritizedRecommendation {
                priority: 1,
                category: "Estructural".to_string(),
                recommendation: format!(
                    "🚨 URGENTE: Aumentar factor de seguridad de {:.2} a ≥1.5. Incrementar secciones transversales o usar material más resistente.",
                    structural.safety_factor
                ),
                expected_improvement: 0.3,
            });
        }

        if structural.resonance_risk > 0.5 {
            recommendations.push(PrioritizedRecommendation {
                priority: 1,
                category: "Estructural".to_string(),
                recommendation: format!(
                    "⚠️ Riesgo de resonancia {:.0}%. Añadir amortiguadores o modificar rigidez.",
                    structural.resonance_risk * 100.0
                ),
                expected_improvement: 0.25,
            });
        }

        // Prioridad 2: Optimizaciones según pesos del contexto
        if weights.aesthetic_harmony > 0.3 && aesthetic.score < 0.6 {
            recommendations.push(PrioritizedRecommendation {
                priority: 2,
                category: "Estética".to_string(),
                recommendation: format!(
                    "🎨 Mejorar estética (actual: {:.0}%). Aplicar simetría bilateral y proporciones áureas.",
                    aesthetic.score * 100.0
                ),
                expected_improvement: 0.2,
            });
        }

        if weights.economic_efficiency > 0.2 && economic.material_efficiency < 0.6 {
            recommendations.push(PrioritizedRecommendation {
                priority: 2,
                category: "Económico".to_string(),
                recommendation: format!(
                    "💰 Optimizar material (eficiencia: {:.0}%). Reducir masa innecesaria.",
                    economic.material_efficiency * 100.0
                ),
                expected_improvement: 0.15,
            });
        }

        if weights.aerodynamic_performance > 0.3 && aerodynamic.drag_coefficient > 0.35 {
            recommendations.push(PrioritizedRecommendation {
                priority: 2,
                category: "Aerodinámico".to_string(),
                recommendation: format!(
                    "✈️ Reducir arrastre de Cd={:.3} a <0.30. Suavizar superficies y reducir turbulencia.",
                    aerodynamic.drag_coefficient
                ),
                expected_improvement: 0.18,
            });
        }

        // Prioridad 3: Mejoras de innovación
        if weights.innovation_factor > 0.1 && innovation.novelty_index < 0.5 {
            recommendations.push(PrioritizedRecommendation {
                priority: 3,
                category: "Innovación".to_string(),
                recommendation: "💡 Diseño repetitivo. Explorar cross-pollination o introducir caos controlado.".to_string(),
                expected_improvement: 0.10,
            });
        }

        // Ordenar por prioridad
        recommendations.sort_by_key(|r| r.priority);

        recommendations
    }

    /// Obtiene contexto activo
    pub fn get_context(&self) -> &DesignContext {
        self.muse.get_active_context()
    }

    /// Lista contextos disponibles
    pub fn list_contexts(&self) -> Vec<String> {
        self.muse.list_contexts()
    }
}
