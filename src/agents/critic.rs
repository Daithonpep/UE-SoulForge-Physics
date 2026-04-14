// src/agents/critic.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PerformanceReport {
    pub score: f32,                    // 0.0 a 10.0
    pub verdict: String,               // "Excelente", "Aceptable", "Necesita revisión"
    pub issues: Vec<Issue>,
    pub suggestions: Vec<String>,
    pub metrics: PerformanceMetrics,
    pub should_iterate: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PerformanceMetrics {
    pub draw_calls: u32,
    pub triangle_count: u64,
    pub fps_estimate: f32,
    pub memory_mb: f32,
    pub nanite_usage: f32,
    pub aesthetic_coherence: f32,      // 0-1
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Issue {
    pub severity: u8,                  // 1-5
    pub category: String,              // "performance", "aesthetic", "technical"
    pub description: String,
}

pub struct Critic {
    style_weights: std::collections::HashMap<String, f32>,
}

impl Critic {
    pub fn new() -> Self {
        Critic {
            style_weights: std::collections::HashMap::from([
                ("gothic".to_string(), 0.9),
                ("scifi".to_string(), 0.85),
            ]),
        }
    }

    pub fn evaluate(
        &self,
        geometric_result: &crate::agents::geometer::GeometricInstruction,
        original_prompt: &str,
        telemetry: Option<PerformanceMetrics>,
    ) -> PerformanceReport {
        let mut issues = Vec::new();
        let mut suggestions = Vec::new();
        let mut score = 8.5f32;

        let metrics = telemetry.unwrap_or(PerformanceMetrics {
            draw_calls: geometric_result.estimated_draw_calls,
            triangle_count: geometric_result.total_objects as u64 * 8000,
            fps_estimate: 65.0,
            memory_mb: geometric_result.memory_estimate_mb,
            nanite_usage: 0.6,
            aesthetic_coherence: 0.82,
        });

        // Evaluación de rendimiento
        if metrics.draw_calls > 3500 {
            score -= 2.5;
            issues.push(Issue {
                severity: 4,
                category: "performance".to_string(),
                description: "Demasiados draw calls. Se recomienda usar Hierarchical Instanced Static Mesh (HISM) o Nanite.".to_string(),
            });
            suggestions.push("Reducir densidad en un 35%".to_string());
        }

        if metrics.triangle_count > 15_000_000 {
            score -= 3.0;
            issues.push(Issue {
                severity: 5,
                category: "performance".to_string(),
                description: "Exceso de geometría. Se debe activar Nanite en meshes grandes.".to_string(),
            });
        }

        // Evaluación estética y Lógica Espacial (Enjambre Jerárquico)
        let mut style_score = 8.0;
        
        let has_furniture = geometric_result.tasks.iter().any(|o| o.object_id.contains("sofa") || o.object_id.contains("bed"));
        let has_car_parts = geometric_result.tasks.iter().any(|o| o.object_id.contains("wheel") || o.object_id.contains("chassis"));
        
        if has_furniture && has_car_parts {
            style_score -= 4.0;
            issues.push(Issue {
                severity: 5,
                category: "aesthetic".to_string(),
                description: "Discriminador Estético: Mezcla ilógica de partes de coche y mobiliario de interior.".to_string(),
            });
        }

        if original_prompt.to_lowercase().contains("gótico") || 
           original_prompt.to_lowercase().contains("cementerio") {
            if geometric_result.tasks.iter().any(|o| o.object_id.contains("gravestone")) {
                style_score = 9.2;
            } else { 
                style_score -= 1.5; 
            }
        }

        // Monitor de Colisiones Sintético (Cálculo Matemático Real)
        let total_objects = geometric_result.tasks.len();
        let mut overlap_count = 0;
        
        for i in 0..total_objects {
            for j in (i+1)..total_objects {
                let a = &geometric_result.tasks[i];
                let b = &geometric_result.tasks[j];
                
                let dx = a.transform.location[0] - b.transform.location[0];
                let dy = a.transform.location[1] - b.transform.location[1];
                let dz = a.transform.location[2] - b.transform.location[2];
                let dist_sq = dx*dx + dy*dy + dz*dz;
                
                // Radio base heurístico (~100 unidades por scale 1)
                let radius_a = a.scale * 100.0;
                let radius_b = b.scale * 100.0;
                let min_dist = radius_a + radius_b;
                
                if dist_sq < min_dist * min_dist {
                    overlap_count += 1;
                }
            }
        }

        if overlap_count > 0 {
            issues.push(Issue {
                severity: 4,
                category: "spatial".to_string(),
                description: format!("Monitor Sintético: {} colisiones graves detectadas. Mallas solapadas.", overlap_count),
            });
            style_score -= (overlap_count as f32 * 0.5).min(5.0);
        }

        let final_score = (score * 0.6 + style_score * 0.4).min(10.0);

        let verdict = if final_score >= 8.5 {
            "Excelente".to_string()
        } else if final_score >= 6.5 {
            "Aceptable".to_string()
        } else {
            "Necesita revisión".to_string()
        };

        PerformanceReport {
            score: final_score,
            verdict,
            issues,
            suggestions,
            metrics,
            should_iterate: final_score < 6.8,
        }
    }
}
