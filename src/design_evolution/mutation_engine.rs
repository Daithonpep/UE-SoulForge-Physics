// design_evolution/mutation_engine.rs
// Motor de Mutación Evolutiva — Genera variaciones coherentes de diseños
//
// Soporta:
//   - Genesis: crear primera generación desde ADN base
//   - Mutación asexual: variar genes de un padre
//   - Crossover sexual: mezclar genes de dos padres
//   - Mutación puntual: cambios mínimos aleatorios

use super::dna::*;
use serde::{Deserialize, Serialize};

// ============================================================
// GENOMA DE DISEÑO
// ============================================================

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignGenome {
    pub dna: DesignDNA,
    pub genes: Vec<Gene>,
    pub generation: u32,
    pub lineage_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Gene {
    pub trait_name: String,
    pub value: GeneValue,
    /// Qué tan expresivo es este gene (0.0 recesivo — 1.0 dominante)
    pub dominance: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GeneValue {
    Scalar(f32),
    Vector([f32; 3]),
    Integer(i32),
    Boolean(bool),
    Shape(PrimitiveShape),
    Style(String),
}

// ============================================================
// ENGINE
// ============================================================

pub struct MutationEngine {
    rng: fastrand::Rng,
}

impl MutationEngine {
    pub fn new() -> Self {
        Self {
            rng: fastrand::Rng::new(),
        }
    }

    /// Crea la primera generación a partir del ADN base
    pub fn genesis(&mut self, dna: DesignDNA) -> DesignGenome {
        let mut genes = Vec::new();

        // Gene de escala base
        genes.push(Gene {
            trait_name: "base_scale".into(),
            value: GeneValue::Vector([1.0, 1.0, 1.0]),
            dominance: 1.0,
        });

        // Genes para cada parte requerida
        for part in &dna.core_constraints.required_parts {
            let default_count = (part.quantity_range.0 + part.quantity_range.1) / 2;
            genes.push(Gene {
                trait_name: format!("{}_count", part.name.to_lowercase()),
                value: GeneValue::Integer(default_count as i32),
                dominance: if part.required { 0.9 } else { 0.5 },
            });
        }

        // Gene de decoración
        let dec_mid = (dna.aesthetic_parameters.decoration_density.0
            + dna.aesthetic_parameters.decoration_density.1) / 2.0;
        genes.push(Gene {
            trait_name: "decoration_level".into(),
            value: GeneValue::Scalar(dec_mid),
            dominance: 0.5,
        });

        // Gene de simetría
        genes.push(Gene {
            trait_name: "symmetry".into(),
            value: GeneValue::Scalar(dna.aesthetic_parameters.symmetry_preference),
            dominance: 0.6,
        });

        // Gene de complejidad
        let comp_mid = (dna.aesthetic_parameters.complexity_range.0
            + dna.aesthetic_parameters.complexity_range.1) / 2.0;
        genes.push(Gene {
            trait_name: "complexity".into(),
            value: GeneValue::Scalar(comp_mid),
            dominance: 0.5,
        });

        // Gene de forma primaria
        genes.push(Gene {
            trait_name: "primary_shape".into(),
            value: GeneValue::Shape(PrimitiveShape::Cube),
            dominance: 0.8,
        });

        // Gene de ubicación funcional (por ejemplo, margen de las patas)
        // 0.0 = borde exterior, 0.5 = concentradas en el centro (pedestal)
        genes.push(Gene {
            trait_name: "leg_inset".into(),
            value: GeneValue::Scalar(0.15), // Empieza inexperto (levemente estorboso)
            dominance: 0.7,
        });

        let lineage = self.generate_lineage_id();
        DesignGenome {
            dna,
            genes,
            generation: 0,
            lineage_id: lineage,
        }
    }

    /// Muta un genoma existente produciendo un hijo
    pub fn mutate(&mut self, parent: &DesignGenome) -> DesignGenome {
        let mut child_genes = parent.genes.clone();
        let mutations = &parent.dna.mutation_rules.allowed_mutations;
        if mutations.is_empty() {
            return DesignGenome {
                dna: parent.dna.clone(),
                genes: child_genes,
                generation: parent.generation + 1,
                lineage_id: parent.lineage_id.clone(),
            };
        }

        let mutation_count = (parent.dna.mutation_rules.mutation_rate
            * child_genes.len() as f32)
            .ceil() as usize;

        log::debug!(
            "🧬 Mutando gen {} → {} ({} mutaciones)",
            parent.generation,
            parent.generation + 1,
            mutation_count
        );

        for _ in 0..mutation_count.max(1) {
            let idx = self.rng.usize(0..mutations.len());
            let mutation_type = &mutations[idx];

            match mutation_type {
                MutationType::ScaleVariation { axis, factor_range } => {
                    self.mutate_scale(&mut child_genes, axis, factor_range);
                }
                MutationType::PartCountChange { part_name, delta_range } => {
                    self.mutate_part_count(
                        &mut child_genes,
                        part_name,
                        delta_range,
                        &parent.dna.core_constraints.required_parts,
                    );
                }
                MutationType::ShapeSubstitution { from: _, to } => {
                    self.mutate_shape(&mut child_genes, to);
                }
                MutationType::ProportionShift { aspect_ratio_delta } => {
                    self.mutate_proportion(&mut child_genes, *aspect_ratio_delta);
                }
                MutationType::DetailAddition { detail_type, probability } => {
                    if self.rng.f32() < *probability {
                        self.add_detail(&mut child_genes, detail_type);
                    }
                }
                MutationType::FractalSubdivision { depth_range } => {
                    let depth = self.rng.u32(depth_range.0..=depth_range.1);
                    self.set_or_update_gene(
                        &mut child_genes,
                        "fractal_depth",
                        GeneValue::Integer(depth as i32),
                        0.5,
                    );
                }
                MutationType::SymmetryBreak { controlled } => {
                    if *controlled {
                        if let Some(g) = child_genes.iter_mut().find(|g| g.trait_name == "symmetry") {
                            if let GeneValue::Scalar(ref mut v) = g.value {
                                *v *= 0.7 + self.rng.f32() * 0.3;
                                *v = v.clamp(0.0_f32, 1.0_f32);
                            }
                        }
                    }
                }
                MutationType::MaterialBlend => {
                    // Placeholder para futuras mezclas de material
                }
            }
        }

        DesignGenome {
            dna: parent.dna.clone(),
            genes: child_genes,
            generation: parent.generation + 1,
            lineage_id: parent.lineage_id.clone(),
        }
    }

    /// Cruza dos genomas de la misma categoría
    pub fn crossover(&mut self, p1: &DesignGenome, p2: &DesignGenome) -> DesignGenome {
        // Solo cruzar dentro de la misma categoría
        assert_eq!(
            std::mem::discriminant(&p1.dna.category),
            std::mem::discriminant(&p2.dna.category),
            "Crossover requires same DesignCategory"
        );

        let mut child_genes = Vec::new();
        let max_len = p1.genes.len().max(p2.genes.len());

        for i in 0..max_len {
            let gene = match (p1.genes.get(i), p2.genes.get(i)) {
                (Some(g1), Some(g2)) => {
                    if self.rng.bool() { g1.clone() } else { g2.clone() }
                }
                (Some(g), None) | (None, Some(g)) => g.clone(),
                (None, None) => continue,
            };

            // 10% de chances de mutación puntual
            if self.rng.f32() < 0.1 {
                child_genes.push(self.point_mutate(gene));
            } else {
                child_genes.push(gene);
            }
        }

        let new_gen = p1.generation.max(p2.generation) + 1;
        log::debug!(
            "💑 Crossover: Gen {} + Gen {} → Gen {}",
            p1.generation,
            p2.generation,
            new_gen
        );

        DesignGenome {
            dna: p1.dna.clone(),
            genes: child_genes,
            generation: new_gen,
            lineage_id: self.generate_lineage_id()
        }
    }

    /// Lógica de Branching para el Bridge ("The Genesis Jailbreak")
    pub fn execute_jailbreak_session(&mut self, base_design: &DesignGenome) -> Vec<DesignGenome> {
        let mut branches = Vec::new();
        
        // 1. Rama Conservadora: Optimización incremental
        let mut conservative_base = base_design.clone();
        conservative_base.dna.mutation_rules.mutation_rate = 0.05;
        branches.push(self.mutate(&conservative_base)); 
        
        // 2. Rama Radical: Forzar ENTROPY alto aunque baje el fitness inicial
        let mut radical_base = base_design.clone();
        radical_base.dna.mutation_rules.mutation_rate = 0.85;
        let mut radical = self.mutate(&radical_base);
        radical.apply_entropy(); // Forzamos caos y lógica invertida
        branches.push(radical);
        
        // 3. Rama Híbrida: Inyectar geometría de referencia (Planos Reales)
        let mut hybrid_base = base_design.clone();
        hybrid_base.inject_api_reference("Moderno");
        branches.push(self.mutate(&hybrid_base));
        
        branches
    }

    // =========================================================
    // MUTACIONES ESPECÍFICAS
    // =========================================================

    fn mutate_scale(&mut self, genes: &mut Vec<Gene>, axis: &Option<Axis>, range: &(f32, f32)) {
        if let Some(g) = genes.iter_mut().find(|g| g.trait_name == "base_scale") {
            if let GeneValue::Vector(ref mut scale) = g.value {
                let factor = range.0 + self.rng.f32() * (range.1 - range.0);
                match axis {
                    Some(Axis::X) => scale[0] *= factor,
                    Some(Axis::Y) => scale[1] *= factor,
                    Some(Axis::Z) => scale[2] *= factor,
                    None => {
                        scale[0] *= factor;
                        scale[1] *= factor;
                        scale[2] *= factor;
                    }
                }
                log::trace!("   ↔️ Escala mutada: {:?}", scale);
            }
        }
    }

    fn mutate_part_count(
        &mut self,
        genes: &mut Vec<Gene>,
        part_name: &str,
        delta_range: &(i32, i32),
        parts: &[PartDefinition],
    ) {
        let gene_name = format!("{}_count", part_name.to_lowercase());
        let bounds = parts
            .iter()
            .find(|p| p.name == part_name)
            .map(|p| p.quantity_range)
            .unwrap_or((1, 99));

        let delta = delta_range.0 + self.rng.i32(0..=(delta_range.1 - delta_range.0));

        if let Some(g) = genes.iter_mut().find(|g| g.trait_name == gene_name) {
            if let GeneValue::Integer(ref mut count) = g.value {
                *count = (*count + delta).clamp(bounds.0 as i32, bounds.1 as i32);
                log::trace!("   🔢 {} = {} (Δ{})", part_name, count, delta);
            }
        } else {
            let base = ((bounds.0 + bounds.1) / 2) as i32;
            genes.push(Gene {
                trait_name: gene_name,
                value: GeneValue::Integer((base + delta).clamp(bounds.0 as i32, bounds.1 as i32)),
                dominance: 0.7,
            });
        }
    }

    fn mutate_shape(&mut self, genes: &mut Vec<Gene>, to: &[PrimitiveShape]) {
        if to.is_empty() { return; }
        let new_shape = to[self.rng.usize(0..to.len())].clone();
        self.set_or_update_gene(
            genes,
            "primary_shape",
            GeneValue::Shape(new_shape),
            0.9,
        );
    }

    fn mutate_proportion(&mut self, genes: &mut Vec<Gene>, delta: f32) {
        if let Some(g) = genes.iter_mut().find(|g| g.trait_name == "base_scale") {
            if let GeneValue::Vector(ref mut scale) = g.value {
                let axis = self.rng.usize(0..3);
                scale[axis] *= 1.0 + (self.rng.f32() - 0.5) * delta;
                // Clamp para evitar degeneración
                scale[axis] = scale[axis].clamp(0.1_f32, 10.0_f32);
            }
        }
    }

    fn add_detail(&mut self, genes: &mut Vec<Gene>, detail_type: &DetailType) {
        let name = format!("detail_{:?}", detail_type);
        // Incrementar si ya existe, sino crear
        if let Some(g) = genes.iter_mut().find(|g| g.trait_name == name) {
            if let GeneValue::Scalar(ref mut v) = g.value {
                *v = (*v + self.rng.f32() * 0.3).min(1.0);
            }
        } else {
            genes.push(Gene {
                trait_name: name,
                value: GeneValue::Scalar(self.rng.f32() * 0.5),
                dominance: 0.4,
            });
        }
    }

    fn point_mutate(&mut self, mut gene: Gene) -> Gene {
        match &mut gene.value {
            GeneValue::Scalar(v) => {
                *v += (self.rng.f32() - 0.5) * 0.2;
                *v = v.clamp(0.0_f32, 1.0_f32);
            }
            GeneValue::Integer(i) => {
                *i += if self.rng.bool() { 1 } else { -1 };
                *i = (*i).max(0);
            }
            GeneValue::Vector(v) => {
                let axis = self.rng.usize(0..3);
                v[axis] *= 0.9 + self.rng.f32() * 0.2;
            }
            GeneValue::Boolean(b) => {
                if self.rng.f32() < 0.15 {
                    *b = !*b;
                }
            }
            _ => {}
        }
        gene
    }

    // =========================================================
    // HELPERS
    // =========================================================

    fn set_or_update_gene(
        &self,
        genes: &mut Vec<Gene>,
        name: &str,
        value: GeneValue,
        dominance: f32,
    ) {
        if let Some(g) = genes.iter_mut().find(|g| g.trait_name == name) {
            g.value = value;
        } else {
            genes.push(Gene {
                trait_name: name.into(),
                value,
                dominance,
            });
        }
    }

    fn generate_lineage_id(&mut self) -> String {
        format!("L{:08X}", self.rng.u32(..))
    }
}

impl DesignGenome {
    /// Extrae el valor de un gene escalar por nombre
    pub fn get_scalar(&self, name: &str) -> Option<f32> {
        self.genes.iter().find(|g| g.trait_name == name).and_then(|g| {
            if let GeneValue::Scalar(v) = g.value { Some(v) } else { None }
        })
    }

    /// Extrae el valor de un gene entero por nombre
    pub fn get_integer(&self, name: &str) -> Option<i32> {
        self.genes.iter().find(|g| g.trait_name == name).and_then(|g| {
            if let GeneValue::Integer(v) = g.value { Some(v) } else { None }
        })
    }

    /// Extrae el vector de escala base
    pub fn get_scale(&self) -> [f32; 3] {
        self.genes.iter().find(|g| g.trait_name == "base_scale").map(|g| {
            if let GeneValue::Vector(v) = g.value { v } else { [1.0, 1.0, 1.0] }
        }).unwrap_or([1.0, 1.0, 1.0])
    }

    /// Resumen legible del genoma para logs
    pub fn summary(&self) -> String {
        let mut parts = vec![format!("Gen{}", self.generation)];
        let scale = self.get_scale();
        parts.push(format!("Scale({:.2},{:.2},{:.2})", scale[0], scale[1], scale[2]));

        for gene in &self.genes {
            match &gene.value {
                GeneValue::Integer(v) if gene.trait_name.contains("count") => {
                    parts.push(format!("{}={}", gene.trait_name, v));
                }
                GeneValue::Shape(s) => {
                    parts.push(format!("Shape={:?}", s));
                }
                _ => {}
            }
        }
        parts.join(" | ")
    }

    /// The Genesis Jailbreak: Rama Radical
    pub fn apply_entropy(&mut self) {
        // Invertimos valores de simetría y cambiamos formas drásticamente
        if let Some(g) = self.genes.iter_mut().find(|g| g.trait_name == "symmetry") {
            g.value = GeneValue::Scalar(0.0); // Romper simetría
        }
        if let Some(g) = self.genes.iter_mut().find(|g| g.trait_name == "primary_shape") {
            // Forzar una forma retorcida o asimétrica
            g.value = GeneValue::Shape(PrimitiveShape::Sphere);
        }
        // Inyectamos caos en posiciones (Chaos: InvertedLogic)
        self.genes.push(Gene {
            trait_name: "caos_inverted_logic".to_string(),
            value: GeneValue::Boolean(true),
            dominance: 1.0,
        });
    }

    /// The Genesis Jailbreak: Rama Híbrida
    pub fn inject_api_reference(&mut self, style: &str) {
        self.genes.push(Gene {
            trait_name: format!("objaverse_api_style"),
            value: GeneValue::Style(style.to_string()),
            dominance: 1.0, 
        });
        // Disminuimos la dominancia de los otros estilos para que Objaverse tome el control
        for gene in self.genes.iter_mut() {
            if gene.trait_name != "objaverse_api_style" {
                gene.dominance *= 0.5;
            }
        }
    }
}
