use serde::{Deserialize, Serialize};

/// Sistema de ejercicios progresivos
pub struct ExerciseEngine {
    exercises: Vec<Exercise>,
    current_level: SkillLevel,
    completed: Vec<CompletedExercise>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exercise {
    pub id: String,
    pub title: String,
    pub description: String,
    pub language: Language,
    pub level: SkillLevel,
    pub category: ExerciseCategory,
    pub starter_code: String,
    pub test_code: String,
    pub hints: Vec<String>,
    pub max_iterations: usize,
    pub success_criteria: SuccessCriteria,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum SkillLevel {
    Beginner,
    Junior,
    Intermediate,
    Senior,
    Expert,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Language {
    Python,
    Rust,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ExerciseCategory {
    DataStructures,
    Algorithms,
    Systems,
    Audio,
    Optimization,
    DesignPatterns,
    Concurrency,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SuccessCriteria {
    pub must_compile: bool,
    pub must_pass_tests: bool,
    pub max_execution_time_ms: Option<u64>,
    pub max_memory_kb: Option<usize>,
    pub min_quality_score: Option<f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompletedExercise {
    pub exercise_id: String,
    pub iterations_needed: usize,
    pub final_code: String,
    pub final_metrics: super::super::sandbox::python_sandbox::ExecutionResult,
    pub lessons_learned: Vec<String>,
}

impl ExerciseEngine {
    pub fn new() -> Self {
        let mut engine = Self {
            exercises: Vec::new(),
            current_level: SkillLevel::Beginner,
            completed: Vec::new(),
        };

        engine.load_curriculum();
        engine
    }

    fn load_curriculum(&mut self) {
        // ═══ PYTHON BEGINNER ═══
        self.exercises.push(Exercise {
            id: "py_001".into(),
            title: "Hola Mundo con Variables".into(),
            description: "Crea una función que reciba un nombre y retorne un saludo personalizado.".into(),
            language: Language::Python,
            level: SkillLevel::Beginner,
            category: ExerciseCategory::DataStructures,
            starter_code: r#"
def greet(name):
    # Tu código aquí
    return f"Hola, {name}!"

# Tests
result = greet("Daithon")
print(f"TEST_greet|{'PASS' if result == 'Hola, Daithon!' else 'FAIL'}|{result}")

result2 = greet("Joseph")
print(f"TEST_greet2|{'PASS' if result2 == 'Hola, Joseph!' else 'FAIL'}|{result2}")
"#.into(),
            test_code: String::new(),
            hints: vec![
                "Usa f-strings: f'Hola, {name}!'".into(),
                "Asegúrate de retornar (return), no solo imprimir (print)".into(),
            ],
            max_iterations: 10,
            success_criteria: SuccessCriteria {
                must_compile: true,
                must_pass_tests: true,
                max_execution_time_ms: Some(1000),
                max_memory_kb: None,
                min_quality_score: None,
            },
        });

        self.exercises.push(Exercise {
            id: "py_002".into(),
            title: "Lista y Búsqueda".into(),
            description: "Implementa una función que encuentre el segundo número más grande en una lista.".into(),
            language: Language::Python,
            level: SkillLevel::Beginner,
            category: ExerciseCategory::Algorithms,
            starter_code: r#"
def second_largest(numbers):
    if len(numbers) < 2: return None
    first = second = float('-inf')
    for n in numbers:
        if n > first:
            second = first
            first = n
        elif n > second and n != first:
            second = n
    return second if second != float('-inf') else None

# Tests
r1 = second_largest([1, 5, 3, 9, 2])
print(f"TEST_basic|{'PASS' if r1 == 5 else 'FAIL'}|expected=5 got={r1}")

r2 = second_largest([10, 10, 9])
print(f"TEST_duplicates|{'PASS' if r2 == 9 else 'FAIL'}|expected=9 got={r2}")

r3 = second_largest([1])
print(f"TEST_single|{'PASS' if r3 is None else 'FAIL'}|expected=None got={r3}")
"#.into(),
            test_code: String::new(),
            hints: vec![
                "Mantén dos variables: el más grande y el segundo más grande".into(),
                "Recorre la lista una sola vez (un for loop)".into(),
                "Cuidado con duplicados: [10, 10, 9] → el segundo es 9, no 10".into(),
            ],
            max_iterations: 15,
            success_criteria: SuccessCriteria {
                must_compile: true,
                must_pass_tests: true,
                max_execution_time_ms: Some(1500),
                max_memory_kb: None,
                min_quality_score: None,
            },
        });

        // ═══ PYTHON JUNIOR ═══
        self.exercises.push(Exercise {
            id: "py_010".into(),
            title: "Ring Buffer Simple".into(),
            description: "Implementa un buffer circular de tamaño fijo. Cuando se llena, sobrescribe los datos más antiguos.".into(),
            language: Language::Python,
            level: SkillLevel::Junior,
            category: ExerciseCategory::DataStructures,
            starter_code: r#"
class RingBuffer:
    def __init__(self, capacity):
        self.capacity = capacity
        self.buffer = [None] * capacity
        self.head = 0
        self.size = 0
    
    def push(self, item):
        index = (self.head + self.size) % self.capacity
        self.buffer[index] = item
        if self.size == self.capacity:
            self.head = (self.head + 1) % self.capacity
        else:
            self.size += 1
    
    def pop(self):
        if self.size == 0: return None
        item = self.buffer[self.head]
        self.head = (self.head + 1) % self.capacity
        self.size -= 1
        return item
    
    def __len__(self):
        return self.size

# Tests
buf = RingBuffer(3)
buf.push(1)
buf.push(2)
buf.push(3)
print(f"TEST_full|{'PASS' if len(buf) == 3 else 'FAIL'}|len={len(buf)}")

buf.push(4)  # Sobrescribe el 1
r1 = buf.pop()
print(f"TEST_overwrite|{'PASS' if r1 == 2 else 'FAIL'}|expected=2 got={r1}")

buf2 = RingBuffer(2)
r2 = buf2.pop()
print(f"TEST_empty|{'PASS' if r2 is None else 'FAIL'}|got={r2}")
"#.into(),
            test_code: String::new(),
            hints: vec![
                "Usa una lista fija y dos índices: head (lectura) y tail (escritura)".into(),
                "El truco está en el módulo: index = index % capacity".into(),
                "Necesitas un contador de elementos para saber si está vacío o lleno".into(),
            ],
            max_iterations: 20,
            success_criteria: SuccessCriteria {
                must_compile: true,
                must_pass_tests: true,
                max_execution_time_ms: Some(1500),
                max_memory_kb: None,
                min_quality_score: Some(3.0),
            },
        });

        // ═══════════════════════════════════════════════════════
        // RUST BEGINNER — Fundamentos del lenguaje
        // ═══════════════════════════════════════════════════════
        self.exercises.push(Exercise {
            id: "rs_001".into(),
            title: "Reloj Digital".into(),
            description: "Implementa un struct Clock que almacene horas y minutos. \
                          Implementa Display para mostrar formato HH:MM. \
                          Implementa add_minutes que maneje overflow de horas.".into(),
            language: Language::Rust,
            level: SkillLevel::Beginner,
            category: ExerciseCategory::DataStructures,
            starter_code: r#"
use std::fmt;

struct Clock {
    hours: i32,
    minutes: i32,
}

impl Clock {
    fn new(hours: i32, minutes: i32) -> Self {
        let total_minutes = hours * 60 + minutes;
        let normalized = ((total_minutes % 1440) + 1440) % 1440;
        Clock {
            hours: normalized / 60,
            minutes: normalized % 60,
        }
    }

    fn add_minutes(&self, minutes: i32) -> Self {
        Clock::new(self.hours, self.minutes + minutes)
    }
}

impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:02}:{:02}", self.hours, self.minutes)
    }
}

fn main() {
    let c1 = Clock::new(10, 30);
    let pass1 = format!("{}", c1) == "10:30";
    println!("TEST_basic|{}|{}", if pass1 { "PASS" } else { "FAIL" }, c1);

    let c2 = Clock::new(25, 0);
    let pass2 = format!("{}", c2) == "01:00";
    println!("TEST_overflow|{}|{}", if pass2 { "PASS" } else { "FAIL" }, c2);

    let c3 = Clock::new(10, 30).add_minutes(90);
    let pass3 = format!("{}", c3) == "12:00";
    println!("TEST_add|{}|{}", if pass3 { "PASS" } else { "FAIL" }, c3);

    let c4 = Clock::new(0, -30);
    let pass4 = format!("{}", c4) == "23:30";
    println!("TEST_negative|{}|{}", if pass4 { "PASS" } else { "FAIL" }, c4);
}
"#.into(),
            test_code: String::new(),
            hints: vec![
                "Normaliza los minutos totales al rango 0..1440 (24*60)".into(),
                "Usa el truco ((x % n) + n) % n para manejar valores negativos".into(),
                "fmt::Display es como ToString pero idiomático en Rust".into(),
            ],
            max_iterations: 15,
            success_criteria: SuccessCriteria {
                must_compile: true, must_pass_tests: true,
                max_execution_time_ms: Some(1500), max_memory_kb: None, min_quality_score: None,
            },
        });

        self.exercises.push(Exercise {
            id: "rs_002".into(),
            title: "Calculadora CLI".into(),
            description: "Implementa una calculadora que parsee expresiones como '3 + 5', '10 * 2', '15 / 3'. \
                          Usa pattern matching en el operador. Maneja división por cero con Result.".into(),
            language: Language::Rust,
            level: SkillLevel::Beginner,
            category: ExerciseCategory::Algorithms,
            starter_code: r#"
fn calculate(expr: &str) -> Result<f64, String> {
    let parts: Vec<&str> = expr.trim().split_whitespace().collect();
    if parts.len() != 3 {
        return Err("Formato: 'numero operador numero'".into());
    }
    let a: f64 = parts[0].parse().map_err(|_| "Primer número inválido".to_string())?;
    let b: f64 = parts[2].parse().map_err(|_| "Segundo número inválido".to_string())?;

    match parts[1] {
        "+" => Ok(a + b),
        "-" => Ok(a - b),
        "*" => Ok(a * b),
        "/" => {
            if b == 0.0 { Err("División por cero".into()) }
            else { Ok(a / b) }
        }
        op => Err(format!("Operador desconocido: {}", op)),
    }
}

fn main() {
    let t1 = calculate("3 + 5").unwrap() == 8.0;
    println!("TEST_add|{}|{}", if t1 { "PASS" } else { "FAIL" }, calculate("3 + 5").unwrap());

    let t2 = calculate("10 * 2").unwrap() == 20.0;
    println!("TEST_mul|{}|{}", if t2 { "PASS" } else { "FAIL" }, calculate("10 * 2").unwrap());

    let t3 = calculate("5 / 0").is_err();
    println!("TEST_divzero|{}|{}", if t3 { "PASS" } else { "FAIL" }, "Err");

    let t4 = calculate("bad input").is_err();
    println!("TEST_invalid|{}|{}", if t4 { "PASS" } else { "FAIL" }, "Err");
}
"#.into(),
            test_code: String::new(),
            hints: vec![
                "split_whitespace() separa por espacios".into(),
                "parse::<f64>() puede fallar, usa map_err para convertir el error".into(),
                "match en el operador con patrón _ para el default".into(),
            ],
            max_iterations: 15,
            success_criteria: SuccessCriteria {
                must_compile: true, must_pass_tests: true,
                max_execution_time_ms: Some(1500), max_memory_kb: None, min_quality_score: None,
            },
        });

        self.exercises.push(Exercise {
            id: "rs_003".into(),
            title: "HashMap: Contador de Palabras".into(),
            description: "Cuenta la frecuencia de cada palabra en un texto. Ignora mayúsculas. \
                          Retorna un HashMap<String, usize>.".into(),
            language: Language::Rust,
            level: SkillLevel::Beginner,
            category: ExerciseCategory::DataStructures,
            starter_code: r#"
use std::collections::HashMap;

fn word_count(text: &str) -> HashMap<String, usize> {
    let mut counts = HashMap::new();
    for word in text.split_whitespace() {
        let clean: String = word.to_lowercase()
            .chars()
            .filter(|c| c.is_alphanumeric())
            .collect();
        if !clean.is_empty() {
            *counts.entry(clean).or_insert(0) += 1;
        }
    }
    counts
}

fn main() {
    let text = "Hola mundo hola MUNDO cruel mundo";
    let counts = word_count(text);

    let t1 = *counts.get("hola").unwrap_or(&0) == 2;
    println!("TEST_hola|{}|count={}", if t1 { "PASS" } else { "FAIL" }, counts.get("hola").unwrap_or(&0));

    let t2 = *counts.get("mundo").unwrap_or(&0) == 3;
    println!("TEST_mundo|{}|count={}", if t2 { "PASS" } else { "FAIL" }, counts.get("mundo").unwrap_or(&0));

    let t3 = *counts.get("cruel").unwrap_or(&0) == 1;
    println!("TEST_cruel|{}|count={}", if t3 { "PASS" } else { "FAIL" }, counts.get("cruel").unwrap_or(&0));
}
"#.into(),
            test_code: String::new(),
            hints: vec![
                "HashMap::entry().or_insert(0) es el patrón idiomático".into(),
                "to_lowercase() para normalizar".into(),
            ],
            max_iterations: 10,
            success_criteria: SuccessCriteria {
                must_compile: true, must_pass_tests: true,
                max_execution_time_ms: Some(1500), max_memory_kb: None, min_quality_score: None,
            },
        });

        // ═══════════════════════════════════════════════════════
        // RUST JUNIOR — Traits, Enums, Iteradores
        // ═══════════════════════════════════════════════════════
        self.exercises.push(Exercise {
            id: "rs_010".into(),
            title: "Sistema de Formas: Trait Area".into(),
            description: "Crea un trait Shape con método area(). Implementa para Circle, Rectangle, Triangle. \
                          Crea una función que reciba &[&dyn Shape] y retorne el área total.".into(),
            language: Language::Rust,
            level: SkillLevel::Junior,
            category: ExerciseCategory::DesignPatterns,
            starter_code: r#"
use std::f64::consts::PI;

trait Shape {
    fn area(&self) -> f64;
    fn name(&self) -> &str;
}

struct Circle { radius: f64 }
struct Rectangle { width: f64, height: f64 }
struct Triangle { base: f64, height: f64 }

impl Shape for Circle {
    fn area(&self) -> f64 { PI * self.radius * self.radius }
    fn name(&self) -> &str { "Circle" }
}

impl Shape for Rectangle {
    fn area(&self) -> f64 { self.width * self.height }
    fn name(&self) -> &str { "Rectangle" }
}

impl Shape for Triangle {
    fn area(&self) -> f64 { 0.5 * self.base * self.height }
    fn name(&self) -> &str { "Triangle" }
}

fn total_area(shapes: &[&dyn Shape]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

fn largest_shape<'a>(shapes: &[&'a dyn Shape]) -> &'a dyn Shape {
    shapes.iter().max_by(|a, b| a.area().partial_cmp(&b.area()).unwrap()).unwrap()
}

fn main() {
    let c = Circle { radius: 5.0 };
    let r = Rectangle { width: 4.0, height: 6.0 };
    let t = Triangle { base: 3.0, height: 8.0 };

    let shapes: Vec<&dyn Shape> = vec![&c, &r, &t];

    let t1 = (c.area() - 78.54).abs() < 0.01;
    println!("TEST_circle|{}|area={:.2}", if t1 { "PASS" } else { "FAIL" }, c.area());

    let t2 = r.area() == 24.0;
    println!("TEST_rect|{}|area={}", if t2 { "PASS" } else { "FAIL" }, r.area());

    let total = total_area(&shapes);
    let t3 = total > 100.0;
    println!("TEST_total|{}|total={:.2}", if t3 { "PASS" } else { "FAIL" }, total);

    let big = largest_shape(&shapes);
    let t4 = big.name() == "Circle";
    println!("TEST_largest|{}|name={}", if t4 { "PASS" } else { "FAIL" }, big.name());
}
"#.into(),
            test_code: String::new(),
            hints: vec![
                "dyn Trait permite polimorfismo dinámico".into(),
                "partial_cmp para comparar floats".into(),
                "Lifetimes en largest_shape: la referencia retornada vive tanto como el slice".into(),
            ],
            max_iterations: 15,
            success_criteria: SuccessCriteria {
                must_compile: true, must_pass_tests: true,
                max_execution_time_ms: Some(1500), max_memory_kb: None, min_quality_score: None,
            },
        });

        self.exercises.push(Exercise {
            id: "rs_011".into(),
            title: "Iterador Personalizado: Fibonacci".into(),
            description: "Implementa un iterador de Fibonacci usando el trait Iterator. \
                          Debe ser lazy (generar bajo demanda). Usa .take(10).collect().".into(),
            language: Language::Rust,
            level: SkillLevel::Junior,
            category: ExerciseCategory::DataStructures,
            starter_code: r#"
struct Fibonacci {
    a: u64,
    b: u64,
}

impl Fibonacci {
    fn new() -> Self {
        Fibonacci { a: 0, b: 1 }
    }
}

impl Iterator for Fibonacci {
    type Item = u64;
    fn next(&mut self) -> Option<u64> {
        let result = self.a;
        let new_b = self.a + self.b;
        self.a = self.b;
        self.b = new_b;
        Some(result)
    }
}

fn main() {
    let fibs: Vec<u64> = Fibonacci::new().take(10).collect();
    let expected = vec![0, 1, 1, 2, 3, 5, 8, 13, 21, 34];

    let t1 = fibs == expected;
    println!("TEST_first10|{}|got={:?}", if t1 { "PASS" } else { "FAIL" }, fibs);

    let sum: u64 = Fibonacci::new().take(20).sum();
    let t2 = sum == 17710;
    println!("TEST_sum20|{}|sum={}", if t2 { "PASS" } else { "FAIL" }, sum);

    let above100: Vec<u64> = Fibonacci::new().skip_while(|&x| x < 100).take(3).collect();
    let t3 = above100[0] == 144;
    println!("TEST_skip|{}|first_above_100={}", if t3 { "PASS" } else { "FAIL" }, above100[0]);
}
"#.into(),
            test_code: String::new(),
            hints: vec![
                "Iterator::next() retorna Option<Self::Item>".into(),
                "Guarda el estado (a, b) en el struct".into(),
                "take(), skip_while(), sum(), collect() son adaptadores de Iterator".into(),
            ],
            max_iterations: 15,
            success_criteria: SuccessCriteria {
                must_compile: true, must_pass_tests: true,
                max_execution_time_ms: Some(1500), max_memory_kb: None, min_quality_score: None,
            },
        });

        self.exercises.push(Exercise {
            id: "rs_012".into(),
            title: "Enum: Máquina de Estados".into(),
            description: "Modela una máquina expendedora con estados: Idle, CoinInserted(u32), \
                          Dispensing(String). Implementa transiciones con match.".into(),
            language: Language::Rust,
            level: SkillLevel::Junior,
            category: ExerciseCategory::DesignPatterns,
            starter_code: r#"
#[derive(Debug, Clone)]
enum VendingState {
    Idle,
    CoinInserted(u32),  // centavos acumulados
    Dispensing(String),  // producto
    OutOfStock,
}

struct VendingMachine {
    state: VendingState,
    stock: std::collections::HashMap<String, (u32, u32)>, // (precio, cantidad)
}

impl VendingMachine {
    fn new() -> Self {
        let mut stock = std::collections::HashMap::new();
        stock.insert("cola".into(), (150, 5));
        stock.insert("agua".into(), (100, 3));
        stock.insert("chips".into(), (200, 0));
        VendingMachine { state: VendingState::Idle, stock }
    }

    fn insert_coin(&mut self, cents: u32) -> String {
        match &self.state {
            VendingState::Idle => {
                self.state = VendingState::CoinInserted(cents);
                format!("Insertado: {}¢", cents)
            }
            VendingState::CoinInserted(current) => {
                self.state = VendingState::CoinInserted(current + cents);
                format!("Total: {}¢", current + cents)
            }
            _ => "No puedo aceptar monedas ahora".into(),
        }
    }

    fn select(&mut self, product: &str) -> String {
        match &self.state {
            VendingState::CoinInserted(amount) => {
                let amount = *amount;
                if let Some((price, qty)) = self.stock.get_mut(product) {
                    if *qty == 0 { self.state = VendingState::OutOfStock; return "Sin stock".into(); }
                    if amount < *price { return format!("Faltan {}¢", *price - amount); }
                    *qty -= 1;
                    let change = amount - *price;
                    self.state = VendingState::Dispensing(product.into());
                    format!("Dispensando {}! Cambio: {}¢", product, change)
                } else {
                    "Producto desconocido".into()
                }
            }
            _ => "Inserta monedas primero".into(),
        }
    }

    fn collect(&mut self) -> String {
        match &self.state {
            VendingState::Dispensing(product) => {
                let msg = format!("Recogido: {}", product);
                self.state = VendingState::Idle;
                msg
            }
            _ => "Nada que recoger".into(),
        }
    }
}

fn main() {
    let mut vm = VendingMachine::new();

    let r1 = vm.insert_coin(100);
    let t1 = r1.contains("100");
    println!("TEST_insert|{}|{}", if t1 { "PASS" } else { "FAIL" }, r1);

    let r2 = vm.insert_coin(50);
    let t2 = r2.contains("150");
    println!("TEST_accumulate|{}|{}", if t2 { "PASS" } else { "FAIL" }, r2);

    let r3 = vm.select("cola");
    let t3 = r3.contains("Dispensando");
    println!("TEST_dispense|{}|{}", if t3 { "PASS" } else { "FAIL" }, r3);

    let r4 = vm.collect();
    let t4 = r4.contains("cola");
    println!("TEST_collect|{}|{}", if t4 { "PASS" } else { "FAIL" }, r4);

    let r5 = vm.select("agua");
    let t5 = r5.contains("Inserta");
    println!("TEST_no_coin|{}|{}", if t5 { "PASS" } else { "FAIL" }, r5);
}
"#.into(),
            test_code: String::new(),
            hints: vec![
                "Usa match en self.state para decidir transiciones".into(),
                "Los enums con datos son el pattern 'sum type' de ML/Haskell".into(),
            ],
            max_iterations: 20,
            success_criteria: SuccessCriteria {
                must_compile: true, must_pass_tests: true,
                max_execution_time_ms: Some(1500), max_memory_kb: None, min_quality_score: None,
            },
        });

        // ═══════════════════════════════════════════════════════
        // RUST INTERMEDIATE — Generics, Error handling, Closures
        // ═══════════════════════════════════════════════════════
        self.exercises.push(Exercise {
            id: "rs_020".into(),
            title: "Stack Genérico con Error Handling".into(),
            description: "Implementa Stack<T> genérico con push, pop (Result), peek, is_empty, len. \
                          Capacidad máxima. pop sobre vacío retorna Err, push sobre lleno retorna Err.".into(),
            language: Language::Rust,
            level: SkillLevel::Intermediate,
            category: ExerciseCategory::DataStructures,
            starter_code: r#"
use std::fmt;

#[derive(Debug)]
enum StackError {
    Overflow(usize),
    Underflow,
}

impl fmt::Display for StackError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            StackError::Overflow(cap) => write!(f, "Stack lleno (cap: {})", cap),
            StackError::Underflow => write!(f, "Stack vacío"),
        }
    }
}

struct Stack<T> {
    data: Vec<T>,
    capacity: usize,
}

impl<T> Stack<T> {
    fn new(capacity: usize) -> Self {
        Stack { data: Vec::with_capacity(capacity), capacity }
    }

    fn push(&mut self, item: T) -> Result<(), StackError> {
        if self.data.len() >= self.capacity {
            Err(StackError::Overflow(self.capacity))
        } else {
            self.data.push(item);
            Ok(())
        }
    }

    fn pop(&mut self) -> Result<T, StackError> {
        self.data.pop().ok_or(StackError::Underflow)
    }

    fn peek(&self) -> Option<&T> {
        self.data.last()
    }

    fn len(&self) -> usize { self.data.len() }
    fn is_empty(&self) -> bool { self.data.is_empty() }
}

fn main() {
    let mut s: Stack<i32> = Stack::new(3);
    s.push(1).unwrap();
    s.push(2).unwrap();
    s.push(3).unwrap();

    let t1 = s.push(4).is_err();
    println!("TEST_overflow|{}|{}", if t1 { "PASS" } else { "FAIL" }, "push on full = Err");

    let t2 = s.pop().unwrap() == 3;
    println!("TEST_pop|{}|{}", if t2 { "PASS" } else { "FAIL" }, "LIFO order");

    let t3 = *s.peek().unwrap() == 2;
    println!("TEST_peek|{}|{}", if t3 { "PASS" } else { "FAIL" }, "peek = 2");

    let mut empty: Stack<String> = Stack::new(10);
    let t4 = empty.pop().is_err();
    println!("TEST_underflow|{}|{}", if t4 { "PASS" } else { "FAIL" }, "pop on empty = Err");
}
"#.into(),
            test_code: String::new(),
            hints: vec![
                "Option::ok_or() convierte Option a Result".into(),
                "Generics: Stack<T> funciona con cualquier tipo".into(),
            ],
            max_iterations: 15,
            success_criteria: SuccessCriteria {
                must_compile: true, must_pass_tests: true,
                max_execution_time_ms: Some(1500), max_memory_kb: None, min_quality_score: None,
            },
        });

        self.exercises.push(Exercise {
            id: "rs_021".into(),
            title: "Mini Servidor HTTP (sin deps)".into(),
            description: "Implementa un handler HTTP básico que parsee un request line, \
                          extraiga método y path, y retorne una respuesta HTML correcta.".into(),
            language: Language::Rust,
            level: SkillLevel::Intermediate,
            category: ExerciseCategory::Systems,
            starter_code: r#"
#[derive(Debug)]
struct HttpRequest {
    method: String,
    path: String,
    version: String,
}

#[derive(Debug)]
struct HttpResponse {
    status_code: u16,
    status_text: String,
    body: String,
}

impl HttpResponse {
    fn to_string(&self) -> String {
        format!("HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}",
            self.status_code, self.status_text, self.body.len(), self.body)
    }
}

fn parse_request(raw: &str) -> Option<HttpRequest> {
    let first_line = raw.lines().next()?;
    let parts: Vec<&str> = first_line.split_whitespace().collect();
    if parts.len() != 3 { return None; }
    Some(HttpRequest {
        method: parts[0].into(),
        path: parts[1].into(),
        version: parts[2].into(),
    })
}

fn handle_request(req: &HttpRequest) -> HttpResponse {
    match (req.method.as_str(), req.path.as_str()) {
        ("GET", "/") => HttpResponse {
            status_code: 200, status_text: "OK".into(),
            body: "<h1>Daithon Server</h1>".into(),
        },
        ("GET", "/health") => HttpResponse {
            status_code: 200, status_text: "OK".into(),
            body: "{\"status\": \"alive\"}".into(),
        },
        ("GET", _) => HttpResponse {
            status_code: 404, status_text: "Not Found".into(),
            body: "<h1>404</h1>".into(),
        },
        _ => HttpResponse {
            status_code: 405, status_text: "Method Not Allowed".into(),
            body: "".into(),
        },
    }
}

fn main() {
    let req1 = parse_request("GET / HTTP/1.1\r\nHost: localhost\r\n");
    let t1 = req1.is_some() && req1.as_ref().unwrap().method == "GET";
    println!("TEST_parse|{}|{:?}", if t1 { "PASS" } else { "FAIL" }, req1);

    let resp1 = handle_request(&req1.unwrap());
    let t2 = resp1.status_code == 200;
    println!("TEST_root|{}|status={}", if t2 { "PASS" } else { "FAIL" }, resp1.status_code);

    let req_404 = parse_request("GET /nope HTTP/1.1\r\n").unwrap();
    let resp_404 = handle_request(&req_404);
    let t3 = resp_404.status_code == 404;
    println!("TEST_404|{}|status={}", if t3 { "PASS" } else { "FAIL" }, resp_404.status_code);

    let resp_str = resp1.to_string();
    let t4 = resp_str.contains("HTTP/1.1 200") && resp_str.contains("Content-Length");
    println!("TEST_format|{}|{}", if t4 { "PASS" } else { "FAIL" }, &resp_str[..50]);
}
"#.into(),
            test_code: String::new(),
            hints: vec![
                "HTTP request line: 'METHOD PATH VERSION'".into(),
                "Pattern matching en tuplas: (method, path)".into(),
            ],
            max_iterations: 20,
            success_criteria: SuccessCriteria {
                must_compile: true, must_pass_tests: true,
                max_execution_time_ms: Some(500), max_memory_kb: None, min_quality_score: None,
            },
        });

        // ═══════════════════════════════════════════════════════
        // RUST SENIOR — Lock-free, Unsafe, Performance
        // ═══════════════════════════════════════════════════════
        self.exercises.push(Exercise {
            id: "rs_030".into(),
            title: "SPSC Ring Buffer Lock-Free".into(),
            description: "Implementa SpscRingBuffer con UnsafeCell, AtomicUsize, N potencia de 2, \
                          &self en push/pop, cache padding, new() con assert.".into(),
            language: Language::Rust,
            level: SkillLevel::Senior,
            category: ExerciseCategory::Concurrency,
            starter_code: r#"
use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicUsize, Ordering};

#[repr(C)]
pub struct SpscRingBuffer<const N: usize> {
    write_idx: AtomicUsize,
    _pad_w: [u8; 56],
    read_idx: AtomicUsize,
    _pad_r: [u8; 56],
    buffer: UnsafeCell<[f64; N]>,
}

unsafe impl<const N: usize> Send for SpscRingBuffer<N> {}
unsafe impl<const N: usize> Sync for SpscRingBuffer<N> {}

impl<const N: usize> SpscRingBuffer<N> {
    pub fn new() -> Self {
        assert!(N.is_power_of_two(), "N must be power of 2");
        Self {
            write_idx: AtomicUsize::new(0),
            _pad_w: [0u8; 56],
            read_idx: AtomicUsize::new(0),
            _pad_r: [0u8; 56],
            buffer: UnsafeCell::new([0.0; N]),
        }
    }

    #[inline]
    fn mask(&self, val: usize) -> usize { val & (N - 1) }

    pub fn push(&self, value: f64) -> bool {
        let write = self.write_idx.load(Ordering::Relaxed);
        let next = self.mask(write + 1);
        if next == self.read_idx.load(Ordering::Acquire) { return false; }
        unsafe { (*self.buffer.get())[write] = value; }
        self.write_idx.store(next, Ordering::Release);
        true
    }

    pub fn pop(&self) -> Option<f64> {
        let read = self.read_idx.load(Ordering::Relaxed);
        if read == self.write_idx.load(Ordering::Acquire) { return None; }
        let value = unsafe { (*self.buffer.get())[read] };
        self.read_idx.store(self.mask(read + 1), Ordering::Release);
        Some(value)
    }

    pub fn capacity(&self) -> usize { N - 1 }
}

fn main() {
    let rb = SpscRingBuffer::<1024>::new();
    rb.push(1.0);
    rb.push(2.0);
    rb.push(3.0);

    let t1 = rb.pop() == Some(1.0);
    println!("TEST_fifo|{}|{:?}", if t1 { "PASS" } else { "FAIL" }, "FIFO order");

    let t2 = rb.pop() == Some(2.0);
    println!("TEST_second|{}|{:?}", if t2 { "PASS" } else { "FAIL" }, "second pop");

    let t3 = rb.capacity() == 1023;
    println!("TEST_cap|{}|cap={}", if t3 { "PASS" } else { "FAIL" }, rb.capacity());

    // Fill and test full
    let rb2 = SpscRingBuffer::<4>::new(); // cap = 3
    rb2.push(1.0);
    rb2.push(2.0);
    rb2.push(3.0);
    let t4 = !rb2.push(4.0); // Should fail - full
    println!("TEST_full|{}|{}", if t4 { "PASS" } else { "FAIL" }, "push on full = false");

    let rb3 = SpscRingBuffer::<4>::new();
    let t5 = rb3.pop().is_none();
    println!("TEST_empty|{}|{}", if t5 { "PASS" } else { "FAIL" }, "pop on empty = None");
}
"#.into(),
            test_code: String::new(),
            hints: vec![
                "UnsafeCell::get() retorna *mut T legalmente".into(),
                "Relaxed para TU índice, Acquire del OTRO, Release para publicar".into(),
                "N-1 = capacidad real (un slot se pierde)".into(),
                "& (N-1) en vez de % N para potencias de 2".into(),
            ],
            max_iterations: 25,
            success_criteria: SuccessCriteria {
                must_compile: true, must_pass_tests: true,
                max_execution_time_ms: Some(500), max_memory_kb: None, min_quality_score: None,
            },
        });

        println!("[CURRICULUM] {} ejercicios cargados (Beginner → Expert)", self.exercises.len());
    }

    /// Obtener el siguiente ejercicio según nivel
    pub fn get_next_exercise(&self) -> Option<&Exercise> {
        self.exercises.iter()
            .filter(|e| e.level <= self.current_level)
            .filter(|e| !self.completed.iter().any(|c| c.exercise_id == e.id))
            .next()
    }

    /// Registrar ejercicio completado
    pub fn complete_exercise(&mut self, completed: CompletedExercise) {
        self.completed.push(completed);

        // Subir de nivel si completó suficientes
        let completed_at_level = self.completed.iter()
            .filter(|c| {
                self.exercises.iter()
                    .find(|e| e.id == c.exercise_id)
                    .map(|e| e.level == self.current_level)
                    .unwrap_or(false)
            })
            .count();

        if completed_at_level >= 2 {
            self.level_up();
        }
    }

    fn level_up(&mut self) {
        self.current_level = match self.current_level {
            SkillLevel::Beginner => SkillLevel::Junior,
            SkillLevel::Junior => SkillLevel::Intermediate,
            SkillLevel::Intermediate => SkillLevel::Senior,
            SkillLevel::Senior => SkillLevel::Expert,
            SkillLevel::Expert => SkillLevel::Expert,
        };

        println!("[CODE LAB] ¡Nivel alcanzado: {:?}!", self.current_level);
    }

    pub fn get_progress_report(&self) -> String {
        format!(
            "Nivel: {:?} | Ejercicios completados: {} / {} | Siguiente: {}",
            self.current_level,
            self.completed.len(),
            self.exercises.len(),
            self.get_next_exercise().map(|e| e.title.as_str()).unwrap_or("¡Todos completados!")
        )
    }
}
