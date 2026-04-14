use super::pattern_library::*;

/// Verificador que analiza código generado contra la biblioteca de patrones.
/// No solo dice "está mal", sino que explica POR QUÉ y muestra la corrección.
pub struct ConceptVerifier {
    pub library: PatternLibrary,
    pub verification_log: Vec<VerificationResult>,
}

#[derive(Debug, Clone)]
pub struct VerificationResult {
    pub exercise_name: String,
    pub score: f64,           // 0.0 a 10.0
    pub violations: Vec<String>,
    pub correct_concepts: Vec<String>,
    pub lessons_learned: Vec<String>,
    pub needs_retry: bool,
}

impl ConceptVerifier {
    pub fn new() -> Self {
        Self {
            library: PatternLibrary::new(),
            verification_log: Vec::new(),
        }
    }

    /// Verificar código generado contra reglas conocidas
    pub fn verify_code(&mut self, exercise_name: &str, code: &str, requirements: &[&str]) -> VerificationResult {
        let mut score: f64 = 10.0;
        let mut violations = Vec::new();
        let mut correct_concepts = Vec::new();
        let mut lessons = Vec::new();

        // 1. Verificar anti-patrones (solo en código real, no en comentarios)
        let code_only: String = code.lines()
            .filter(|l| {
                let trimmed = l.trim();
                !trimmed.starts_with("//") && !trimmed.starts_with('#')
            })
            .collect::<Vec<_>>()
            .join("\n");
        let anti_violations = self.library.check_for_antipatterns(&code_only);
        for v in &anti_violations {
            score -= 1.5;
            violations.push(v.clone());
        }

        // 2. Verificar requisitos específicos
        for req in requirements {
            let (met, detail) = self.check_requirement(code, req);
            if met {
                correct_concepts.push(format!("✅ {}: {}", req, detail));
            } else {
                score -= 1.0;
                violations.push(format!("❌ Requisito no cumplido: {} - {}", req, detail));
                lessons.push(format!("Aprender: {}", detail));
            }
        }

        // 3. Verificaciones estructurales
        let structural = self.structural_analysis(code);
        for (concept, passed, detail) in &structural {
            if *passed {
                correct_concepts.push(format!("✅ {}", concept));
            } else {
                score -= 0.5;
                violations.push(format!("⚠️ {}: {}", concept, detail));
            }
        }

        let result = VerificationResult {
            exercise_name: exercise_name.into(),
            score: score.clamp(0.0, 10.0),
            violations,
            correct_concepts,
            lessons_learned: lessons,
            needs_retry: score < 7.0,
        };

        self.verification_log.push(result.clone());
        result
    }

    fn check_requirement(&self, code: &str, requirement: &str) -> (bool, String) {
        match requirement {
            "unsafe_cell" => {
                if code.contains("UnsafeCell") {
                    (true, "Usa UnsafeCell para mutabilidad interior".into())
                } else if code.contains("as *mut") {
                    (false, "Castea *const a *mut en vez de usar UnsafeCell. Esto es UB.".into())
                } else {
                    (false, "No usa UnsafeCell para el buffer compartido".into())
                }
            }
            "atomic_ordering" => {
                let has_relaxed_own = code.contains("Relaxed");
                let has_acquire = code.contains("Acquire");
                let has_release = code.contains("Release");
                if has_relaxed_own && has_acquire && has_release {
                    (true, "Usa Relaxed/Acquire/Release correctamente".into())
                } else if has_acquire && has_release {
                    (false, "Usa Acquire donde debería ser Relaxed para el índice propio".into())
                } else {
                    (false, "Falta semántica de ordenamiento atómico completa".into())
                }
            }
            "no_vec_hot_path" => {
                if code.contains("Vec<f64>") || code.contains("Vec<f32>") {
                    (false, "Usa Vec en el hot path. Vec aloca dinámicamente.".into())
                } else {
                    (true, "No usa Vec en el hot path".into())
                }
            }
            "power_of_two" => {
                if code.contains("& (N - 1)") || code.contains("&(N-1)") || code.contains(".is_power_of_two()") {
                    (true, "Usa bitwise AND / verifica potencia de 2".into())
                } else if code.contains("% N") || code.contains("% capacity") {
                    (false, "Usa módulo (%) en vez de bitwise AND. 20-90x más lento.".into())
                } else {
                    (false, "No implementa optimización de potencia de 2".into())
                }
            }
            "const_generics" => {
                if code.contains("const N: usize") || code.contains("const CAP: usize") {
                    (true, "Usa const generics para capacidad en tiempo de compilación".into())
                } else {
                    (false, "No usa const generics. La capacidad debería ser fija en compilación.".into())
                }
            }
            "initialization" => {
                if code.contains("fn new()") || code.contains("fn new(") {
                    if code.contains("AtomicUsize::new(0)") {
                        (true, "Tiene new() con inicialización explícita de atómicos a 0".into())
                    } else {
                        (false, "Tiene new() pero no inicializa atómicos explícitamente".into())
                    }
                } else {
                    (false, "Falta constructor new(). Los campos pueden tener basura.".into())
                }
            }
            "shared_ref_push" => {
                if code.contains("fn push(&self") {
                    (true, "push toma &self (compartible entre hilos)".into())
                } else if code.contains("fn push(&mut self") {
                    (false, "&mut self requiere exclusividad. Imposible en SPSC con 2 hilos.".into())
                } else {
                    (false, "Falta método push".into())
                }
            }
            "send_sync" => {
                if code.contains("unsafe impl") && code.contains("Send") && code.contains("Sync") {
                    (true, "Implementa Send + Sync manualmente (necesario con UnsafeCell)".into())
                } else {
                    (false, "Falta impl Send/Sync. UnsafeCell no es Sync por defecto.".into())
                }
            }
            "cache_padding" => {
                if code.contains("CachePadded") || code.contains("_pad") || code.contains("#[repr(C)]") {
                    (true, "Tiene padding para evitar false sharing entre cache lines".into())
                } else {
                    (false, "Sin padding. Los índices pueden compartir cache line = false sharing.".into())
                }
            }
            "full_empty_distinction" => {
                let mentions_capacity_minus_1 = code.contains("N - 1") || code.contains("capacity - 1");
                let mentions_slot_loss = code.contains("slot") || code.contains("capacidad real");
                if mentions_capacity_minus_1 || mentions_slot_loss {
                    (true, "Distingue buffer full vs empty sacrificando un slot".into())
                } else {
                    (false, "No maneja la distinción full/empty. Un slot debe sacrificarse.".into())
                }
            }
            _ => (false, format!("Requisito desconocido: {}", requirement)),
        }
    }

    fn structural_analysis(&self, code: &str) -> Vec<(String, bool, String)> {
        let mut results = Vec::new();

        // Verificar que no haya allocaciones dinámicas en funciones críticas
        let has_box_in_fn = code.contains("Box::new") && (code.contains("fn push") || code.contains("fn pop"));
        results.push((
            "Zero allocation en hot path".into(),
            !has_box_in_fn,
            if has_box_in_fn { "Box::new dentro de push/pop causa alocación" } else { "OK" }.into(),
        ));

        // Verificar que push retorne bool
        if code.contains("fn push") {
            let returns_bool = code.contains("-> bool");
            results.push((
                "push retorna bool".into(),
                returns_bool,
                if returns_bool { "OK" } else { "push debe retornar bool (false si lleno)" }.into(),
            ));
        }

        // Verificar que pop retorne Option
        if code.contains("fn pop") {
            let returns_option = code.contains("-> Option<f64>") || code.contains("-> Option<f32>");
            results.push((
                "pop retorna Option".into(),
                returns_option,
                if returns_option { "OK" } else { "pop debe retornar Option<f64> (None si vacío)" }.into(),
            ));
        }

        // Verificar #[inline]
        let has_inline = code.contains("#[inline]");
        results.push((
            "Hot path con #[inline]".into(),
            has_inline,
            if has_inline { "OK" } else { "Funciones de hot path deberían tener #[inline]" }.into(),
        ));

        results
    }

    /// Obtener un resumen del progreso
    pub fn progress_summary(&self) -> TrainingProgress {
        let total = self.verification_log.len();
        let passed = self.verification_log.iter().filter(|r| !r.needs_retry).count();
        let avg_score = if total > 0 {
            self.verification_log.iter().map(|r| r.score).sum::<f64>() / total as f64
        } else {
            0.0
        };

        let mut all_lessons: Vec<String> = self.verification_log.iter()
            .flat_map(|r| r.lessons_learned.clone())
            .collect();
        all_lessons.dedup();

        TrainingProgress {
            total_exercises: total,
            passed,
            failed: total - passed,
            average_score: avg_score,
            accumulated_lessons: all_lessons,
        }
    }
}

#[derive(Debug, Clone)]
pub struct TrainingProgress {
    pub total_exercises: usize,
    pub passed: usize,
    pub failed: usize,
    pub average_score: f64,
    pub accumulated_lessons: Vec<String>,
}
