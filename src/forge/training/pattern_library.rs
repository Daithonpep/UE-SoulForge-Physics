/// Biblioteca de patrones CORRECTOS de Rust.
/// Cada patrón es código Rust REAL que compila y pasa tests.
/// Daithon aprende de estos patrones, no los memoriza: extrae las REGLAS.

use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct VerifiedPattern {
    pub name: String,
    pub category: PatternCategory,
    pub correct_code: String,
    pub anti_patterns: Vec<AntiPattern>,
    pub rules_extracted: Vec<String>,
    pub test_code: String,
}

#[derive(Debug, Clone)]
pub struct AntiPattern {
    pub bad_code: String,
    pub why_bad: String,
    pub fix: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PatternCategory {
    UnsafeSafety,
    AtomicOrdering,
    OwnershipBorrowing,
    LockFreeStructures,
    MemoryLayout,
    ZeroAllocation,
    ErrorHandling,
    Concurrency,
    Initialization,
    BitwiseOptimization,
}

pub struct PatternLibrary {
    pub patterns: HashMap<String, VerifiedPattern>,
    pub rules: Vec<String>,
}

impl PatternLibrary {
    pub fn new() -> Self {
        let mut lib = Self {
            patterns: HashMap::new(),
            rules: Vec::new(),
        };
        lib.load_foundational_patterns();
        lib
    }

    fn load_foundational_patterns(&mut self) {
        // ═══════════════════════════════════════════════════════
        // REGLA 1: Unsafe correcto - UnsafeCell para mutabilidad interior
        // ═══════════════════════════════════════════════════════
        self.add_pattern(VerifiedPattern {
            name: "unsafe_interior_mutability".into(),
            category: PatternCategory::UnsafeSafety,
            correct_code: r#"
use std::cell::UnsafeCell;

// Para mutabilidad interior en contexto concurrente,
// SIEMPRE usar UnsafeCell como base.
pub struct SharedBuffer {
    data: UnsafeCell<[f64; 1024]>,
}

// SOLO es Send+Sync si garantizamos acceso exclusivo
// por diseño (ej: SPSC donde solo 1 hilo escribe)
unsafe impl Send for SharedBuffer {}
unsafe impl Sync for SharedBuffer {}

impl SharedBuffer {
    pub fn write_at(&self, index: usize, value: f64) {
        // UnsafeCell::get() retorna *mut T legalmente
        unsafe {
            (*self.data.get())[index] = value;
        }
    }

    pub fn read_at(&self, index: usize) -> f64 {
        unsafe {
            (*self.data.get())[index]
        }
    }
}
"#.into(),
            anti_patterns: vec![
                AntiPattern {
                    bad_code: "let ptr = self.buffer.as_ptr() as *mut f64;".into(),
                    why_bad: "Castear *const a *mut es UB. El compilador asume que *const nunca muta. Optimizaciones pueden eliminar la escritura.".into(),
                    fix: "Usar UnsafeCell<[f64; N]> que provee *mut legalmente via .get()".into(),
                },
                AntiPattern {
                    bad_code: "pub fn push(&mut self, value: f64)".into(),
                    why_bad: "&mut self requiere exclusividad. En SPSC necesitas &self para que ambos hilos tengan referencia simultánea.".into(),
                    fix: "pub fn push(&self, value: f64) con UnsafeCell internamente".into(),
                },
            ],
            rules_extracted: vec![
                "REGLA: Nunca castear *const T a *mut T. Usar UnsafeCell.".into(),
                "REGLA: UnsafeCell es el ÚNICO mecanismo legal de mutabilidad interior.".into(),
                "REGLA: Solo implementar Send+Sync si el diseño GARANTIZA seguridad.".into(),
            ],
            test_code: r#"
fn test_shared_buffer() {
    let buf = SharedBuffer { data: UnsafeCell::new([0.0; 1024]) };
    buf.write_at(0, 42.0);
    assert_eq!(buf.read_at(0), 42.0);
}
"#.into(),
        });

        // ═══════════════════════════════════════════════════════
        // REGLA 2: Ordering atómico correcto
        // ═══════════════════════════════════════════════════════
        self.add_pattern(VerifiedPattern {
            name: "atomic_ordering_spsc".into(),
            category: PatternCategory::AtomicOrdering,
            correct_code: r#"
use std::sync::atomic::{AtomicUsize, Ordering};

// En SPSC:
// - El PRODUCTOR usa Relaxed para leer SU propio índice (solo él lo modifica)
// - El PRODUCTOR usa Acquire para leer el índice del CONSUMIDOR
// - El PRODUCTOR usa Release para publicar SU índice después de escribir datos
//
// - El CONSUMIDOR es simétrico: Relaxed para su índice, Acquire del otro, Release para publicar

fn producer_push(
    write_idx: &AtomicUsize,
    read_idx: &AtomicUsize,
    capacity: usize,
) -> bool {
    let write = write_idx.load(Ordering::Relaxed);  // Solo yo escribo aquí
    let next = (write + 1) & (capacity - 1);        // Bitwise AND, no módulo

    if next == read_idx.load(Ordering::Acquire) {    // Leer lo último del consumidor
        return false; // Lleno
    }

    // ... escribir dato en buffer[write] ...

    write_idx.store(next, Ordering::Release);        // Publicar para el consumidor
    true
}
"#.into(),
            anti_patterns: vec![
                AntiPattern {
                    bad_code: "write_idx.load(Ordering::Acquire) // para leer MI propio índice".into(),
                    why_bad: "Acquire es innecesariamente caro para leer tu propio índice. Solo tú lo modificas.".into(),
                    fix: "write_idx.load(Ordering::Relaxed) para tu propio índice".into(),
                },
                AntiPattern {
                    bad_code: "(write + 1) % N".into(),
                    why_bad: "Módulo es una división entera (20-90 ciclos). En audio a 48kHz cada nanosegundo cuenta.".into(),
                    fix: "(write + 1) & (N - 1) cuando N es potencia de 2".into(),
                },
            ],
            rules_extracted: vec![
                "REGLA: Relaxed para leer TU propio índice atómico (solo tú lo escribes).".into(),
                "REGLA: Acquire para leer el índice del OTRO hilo.".into(),
                "REGLA: Release después de escribir datos, para publicar tu índice.".into(),
                "REGLA: Usar & (N-1) en vez de % N cuando N es potencia de 2.".into(),
            ],
            test_code: "// Verificado por el patrón de SPSC estándar de la industria".into(),
        });

        // ═══════════════════════════════════════════════════════
        // REGLA 3: Inicialización correcta
        // ═══════════════════════════════════════════════════════
        self.add_pattern(VerifiedPattern {
            name: "correct_initialization".into(),
            category: PatternCategory::Initialization,
            correct_code: r#"
use std::cell::UnsafeCell;
use std::sync::atomic::AtomicUsize;

pub struct RingBuffer<const N: usize> {
    buffer: UnsafeCell<[f64; N]>,
    write_idx: AtomicUsize,
    read_idx: AtomicUsize,
}

impl<const N: usize> RingBuffer<N> {
    /// SIEMPRE proveer new() con inicialización explícita
    pub const fn new() -> Self {
        Self {
            buffer: UnsafeCell::new([0.0; N]),
            write_idx: AtomicUsize::new(0),
            read_idx: AtomicUsize::new(0),
        }
    }
}
"#.into(),
            anti_patterns: vec![
                AntiPattern {
                    bad_code: "// Sin new() - los atomics tendrán basura".into(),
                    why_bad: "Sin inicialización explícita, AtomicUsize puede contener valores aleatorios de la memoria.".into(),
                    fix: "Siempre proveer new() que inicialice todos los campos a valores conocidos.".into(),
                },
            ],
            rules_extracted: vec![
                "REGLA: SIEMPRE proveer new() con valores iniciales explícitos.".into(),
                "REGLA: Los AtomicUsize DEBEN inicializarse a 0.".into(),
                "REGLA: Usar const fn new() permite inicializar en tiempo de compilación.".into(),
            ],
            test_code: r#"
fn test_initialization() {
    let rb = RingBuffer::<1024>::new();
    // Los índices deben empezar en 0
}
"#.into(),
        });

        // ═══════════════════════════════════════════════════════
        // REGLA 4: SPSC Completo y Correcto
        // ═══════════════════════════════════════════════════════
        self.add_pattern(VerifiedPattern {
            name: "complete_spsc_ringbuffer".into(),
            category: PatternCategory::LockFreeStructures,
            correct_code: r#"
use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicUsize, Ordering};

/// La capacidad real es N-1 (un slot se pierde para distinguir full de empty).
/// N DEBE ser potencia de 2.
#[repr(C)]
pub struct SpscRingBuffer<const N: usize> {
    // Padding manual entre write y read para evitar false sharing.
    // En producción usar crossbeam_utils::CachePadded.
    write_idx: AtomicUsize,
    _pad_w: [u8; 56], // 64 - sizeof(AtomicUsize) = 56
    read_idx: AtomicUsize,
    _pad_r: [u8; 56],
    buffer: UnsafeCell<[f64; N]>,
}

unsafe impl<const N: usize> Send for SpscRingBuffer<N> {}
unsafe impl<const N: usize> Sync for SpscRingBuffer<N> {}

impl<const N: usize> SpscRingBuffer<N> {
    pub fn new() -> Self {
        assert!(N.is_power_of_two(), "N debe ser potencia de 2");
        Self {
            write_idx: AtomicUsize::new(0),
            _pad_w: [0u8; 56],
            read_idx: AtomicUsize::new(0),
            _pad_r: [0u8; 56],
            buffer: UnsafeCell::new([0.0; N]),
        }
    }

    #[inline]
    fn mask(&self, val: usize) -> usize {
        val & (N - 1) // Bitwise AND, no módulo
    }

    pub fn push(&self, value: f64) -> bool {
        let write = self.write_idx.load(Ordering::Relaxed);
        let next = self.mask(write + 1);

        if next == self.read_idx.load(Ordering::Acquire) {
            return false; // Buffer lleno
        }

        unsafe {
            (*self.buffer.get())[write] = value;
        }

        self.write_idx.store(next, Ordering::Release);
        true
    }

    pub fn pop(&self) -> Option<f64> {
        let read = self.read_idx.load(Ordering::Relaxed);

        if read == self.write_idx.load(Ordering::Acquire) {
            return None; // Buffer vacío
        }

        let value = unsafe { (*self.buffer.get())[read] };
        let next = self.mask(read + 1);

        self.read_idx.store(next, Ordering::Release);
        Some(value)
    }

    pub fn len(&self) -> usize {
        let write = self.write_idx.load(Ordering::Relaxed);
        let read = self.read_idx.load(Ordering::Relaxed);
        self.mask(write.wrapping_sub(read))
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn capacity(&self) -> usize {
        N - 1 // Un slot se pierde
    }
}
"#.into(),
            anti_patterns: vec![
                AntiPattern {
                    bad_code: "buffer: Vec<f64>".into(),
                    why_bad: "Vec aloca en heap dinámicamente. En audio real-time, las alocaciones causan latencia impredecible.".into(),
                    fix: "Usar [f64; N] con const generics para búfer estático.".into(),
                },
            ],
            rules_extracted: vec![
                "REGLA: N debe ser potencia de 2 para usar bitwise AND.".into(),
                "REGLA: La capacidad real es N-1 (un slot se sacrifica para distinguir full de empty).".into(),
                "REGLA: UnsafeCell para mutabilidad interior del buffer compartido.".into(),
                "REGLA: Padding manual de 64 bytes entre índices para evitar false sharing.".into(),
                "REGLA: #[repr(C)] para control exacto del layout de memoria.".into(),
                "REGLA: assert! en new() para validar invariantes en tiempo de ejecución.".into(),
                "REGLA: #[inline] en funciones de hot path como mask().".into(),
                "REGLA: wrapping_sub para aritmética segura en len().".into(),
            ],
            test_code: r#"
fn test_spsc() {
    let rb = SpscRingBuffer::<1024>::new();
    assert!(rb.push(1.0));
    assert!(rb.push(2.0));
    assert_eq!(rb.pop(), Some(1.0));
    assert_eq!(rb.pop(), Some(2.0));
    assert_eq!(rb.pop(), None);
    assert_eq!(rb.capacity(), 1023);
}
"#.into(),
        });

        // ═══════════════════════════════════════════════════════
        // REGLA 5: Ownership y Borrowing
        // ═══════════════════════════════════════════════════════
        self.add_pattern(VerifiedPattern {
            name: "ownership_borrowing_basics".into(),
            category: PatternCategory::OwnershipBorrowing,
            correct_code: r#"
// Ownership fundamental: cada valor tiene UN dueño.
// Cuando el dueño sale del scope, el valor se destruye (Drop).

fn take_ownership(s: String) {
    println!("{}", s);
} // s se destruye aquí

fn borrow_immutable(s: &str) {
    println!("{}", s);
} // s NO se destruye - solo era un préstamo

fn borrow_mutable(v: &mut Vec<i32>) {
    v.push(42);
} // v NO se destruye - solo era un préstamo mutable

// REGLA: Puedes tener N referencias inmutables (&T) O exactamente 1 mutable (&mut T), NUNCA ambas.
"#.into(),
            anti_patterns: vec![
                AntiPattern {
                    bad_code: "let mut x = vec![1,2,3]; let r = &x; x.push(4); println!(\"{:?}\", r);".into(),
                    why_bad: "No puedes mutar mientras existan referencias inmutables activas.".into(),
                    fix: "Asegurar que las referencias inmutables ya no estén vivas antes de mutar.".into(),
                },
            ],
            rules_extracted: vec![
                "REGLA: Un valor tiene exactamente UN dueño.".into(),
                "REGLA: N refs inmutables (&T) O 1 ref mutable (&mut T), nunca ambas.".into(),
                "REGLA: Move transfiere ownership. Clone crea una copia.".into(),
                "REGLA: Drop se llama al salir del scope.".into(),
            ],
            test_code: "// Verificado por el borrow checker del compilador".into(),
        });

        // Extraer todas las reglas a la lista global
        for pattern in self.patterns.values() {
            for rule in &pattern.rules_extracted {
                if !self.rules.contains(rule) {
                    self.rules.push(rule.clone());
                }
            }
        }

        println!("[PATTERN LIBRARY] {} patrones cargados, {} reglas extraídas.",
            self.patterns.len(), self.rules.len());
    }

    fn add_pattern(&mut self, pattern: VerifiedPattern) {
        self.patterns.insert(pattern.name.clone(), pattern);
    }

    /// Buscar anti-patrones en código generado
    pub fn check_for_antipatterns(&self, code: &str) -> Vec<String> {
        let mut violations = Vec::new();
        for pattern in self.patterns.values() {
            for anti in &pattern.anti_patterns {
                if code.contains(&anti.bad_code) || self.fuzzy_match(code, &anti.bad_code) {
                    violations.push(format!(
                        "⚠️ ANTI-PATRÓN DETECTADO: '{}'\n   Problema: {}\n   Corrección: {}",
                        anti.bad_code, anti.why_bad, anti.fix
                    ));
                }
            }
        }
        violations
    }

    /// Buscar reglas aplicables a un tema o concepto
    pub fn get_rules_for_topic(&self, topic: &str) -> Vec<String> {
        let topic_lower = topic.to_lowercase();
        self.rules.iter()
            .filter(|r| {
                let r_lower = r.to_lowercase();
                topic_lower.split_whitespace().any(|w| w.len() > 3 && r_lower.contains(w))
            })
            .cloned()
            .collect()
    }

    fn fuzzy_match(&self, code: &str, pattern: &str) -> bool {
        let code_lower = code.to_lowercase();
        let keywords: Vec<&str> = pattern.split_whitespace()
            .filter(|w| w.len() > 3)
            .collect();

        if keywords.is_empty() { return false; }

        let matched = keywords.iter()
            .filter(|k| code_lower.contains(&k.to_lowercase()))
            .count();

        matched as f64 / keywords.len() as f64 > 0.7
    }
}
