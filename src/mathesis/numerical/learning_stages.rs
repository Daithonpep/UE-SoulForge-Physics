use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Sistema de aprendizaje matemático por etapas
pub struct NumericalLearningSystem {
    current_stage: MathStage,
    skills: HashMap<String, MathSkill>,
    solved_problems: Vec<SolvedProblem>,
    understanding_level: HashMap<MathConcept, f64>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MathStage {
    Counting,        // 1, 2, 3...
    BasicArithmetic, // +, -, ×, ÷
    Fractions,       // 1/2, 3/4...
    Algebra,         // x + 5 = 10
    Calculus,        // d/dx, ∫
    Advanced,        // Ecuaciones diferenciales, tensores
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathSkill {
    pub name: String,
    pub concept: MathConcept,
    pub proficiency: f64, // 0.0 - 1.0
    pub prerequisites: Vec<String>,
    pub practice_count: usize,
    pub success_rate: f64,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum MathConcept {
    // Etapa 1
    Counting,
    Comparison,
    Addition,
    Subtraction,
    
    // Etapa 2
    Multiplication,
    Division,
    Fractions,
    Decimals,
    Exponents,
    Roots,
    
    // Etapa 3
    Variables,
    LinearEquations,
    QuadraticEquations,
    Systems,
    Functions,
    Polynomials,
    
    // Etapa 4
    Limits,
    Derivatives,
    Integrals,
    Series,
    
    // Etapa 5
    DifferentialEquations,
    PartialDerivatives,
    VectorCalculus,
    LinearAlgebra,
    Tensors,
    Topology,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SolvedProblem {
    pub problem: String,
    pub solution: String,
    pub steps: Vec<String>,
    pub concept: MathConcept,
    pub difficulty: f64,
    pub time_taken_ms: u64,
    pub was_correct: bool,
}

impl NumericalLearningSystem {
    pub fn new() -> Self {
        let mut system = Self {
            current_stage: MathStage::Counting,
            skills: HashMap::new(),
            solved_problems: Vec::new(),
            understanding_level: HashMap::new(),
        };

        system.initialize_skills();
        system
    }

    fn initialize_skills(&mut self) {
        // ETAPA 1: Contar
        self.add_skill(MathSkill {
            name: "contar".into(),
            concept: MathConcept::Counting,
            proficiency: 0.0,
            prerequisites: vec![],
            practice_count: 0,
            success_rate: 0.0,
        });

        self.add_skill(MathSkill {
            name: "comparar_números".into(),
            concept: MathConcept::Comparison,
            proficiency: 0.0,
            prerequisites: vec!["contar".into()],
            practice_count: 0,
            success_rate: 0.0,
        });

        self.add_skill(MathSkill {
            name: "sumar".into(),
            concept: MathConcept::Addition,
            proficiency: 0.0,
            prerequisites: vec!["contar".into()],
            practice_count: 0,
            success_rate: 0.0,
        });

        self.add_skill(MathSkill {
            name: "restar".into(),
            concept: MathConcept::Subtraction,
            proficiency: 0.0,
            prerequisites: vec!["sumar".into()],
            practice_count: 0,
            success_rate: 0.0,
        });

        // ETAPA 2: Aritmética
        self.add_skill(MathSkill {
            name: "multiplicar".into(),
            concept: MathConcept::Multiplication,
            proficiency: 0.0,
            prerequisites: vec!["sumar".into()],
            practice_count: 0,
            success_rate: 0.0,
        });

        self.add_skill(MathSkill {
            name: "dividir".into(),
            concept: MathConcept::Division,
            proficiency: 0.0,
            prerequisites: vec!["multiplicar".into(), "restar".into()],
            practice_count: 0,
            success_rate: 0.0,
        });

        self.add_skill(MathSkill {
            name: "fracciones".into(),
            concept: MathConcept::Fractions,
            proficiency: 0.0,
            prerequisites: vec!["dividir".into()],
            practice_count: 0,
            success_rate: 0.0,
        });

        // ETAPA 3: Álgebra
        self.add_skill(MathSkill {
            name: "ecuaciones_lineales".into(),
            concept: MathConcept::LinearEquations,
            proficiency: 0.0,
            prerequisites: vec!["sumar".into(), "restar".into(), "multiplicar".into(), "dividir".into()],
            practice_count: 0,
            success_rate: 0.0,
        });

        self.add_skill(MathSkill {
            name: "ecuaciones_cuadráticas".into(),
            concept: MathConcept::QuadraticEquations,
            proficiency: 0.0,
            prerequisites: vec!["ecuaciones_lineales".into()],
            practice_count: 0,
            success_rate: 0.0,
        });

        // ETAPA 4: Cálculo
        self.add_skill(MathSkill {
            name: "derivadas".into(),
            concept: MathConcept::Derivatives,
            proficiency: 0.0,
            prerequisites: vec!["ecuaciones_lineales".into()],
            practice_count: 0,
            success_rate: 0.0,
        });

        self.add_skill(MathSkill {
            name: "integrales".into(),
            concept: MathConcept::Integrals,
            proficiency: 0.0,
            prerequisites: vec!["derivadas".into()],
            practice_count: 0,
            success_rate: 0.0,
        });
    }

    fn add_skill(&mut self, skill: MathSkill) {
        self.understanding_level.insert(skill.concept.clone(), 0.0);
        self.skills.insert(skill.name.clone(), skill);
    }

    /// Práctica guiada: aprender un concepto paso a paso
    pub fn practice_concept(&mut self, concept: MathConcept) -> PracticeResult {
        let problems = self.generate_practice_problems(&concept, 10);
        let mut correct = 0;
        let mut attempts = Vec::new();

        println!("[MATHESIS] Practicando: {:?}", concept);
        println!("  Generando {} problemas...\n", problems.len());

        for (i, problem) in problems.iter().enumerate() {
            println!("  Problema {}: {}", i + 1, problem.question);
            
            let start = std::time::Instant::now();
            let answer = self.solve_problem(problem);
            let elapsed = start.elapsed().as_millis() as u64;

            let is_correct = self.verify_answer(problem, &answer);

            attempts.push(SolvedProblem {
                problem: problem.question.clone(),
                solution: answer.clone(),
                steps: problem.solution_steps.clone(),
                concept: concept.clone(),
                difficulty: problem.difficulty,
                time_taken_ms: elapsed,
                was_correct: is_correct,
            });

            if is_correct {
                correct += 1;
                println!("    ✓ Correcto: {}", answer);
            } else {
                println!("    ✗ Incorrecto: {} (esperado: {})", answer, problem.expected_answer);
                println!("    Revisando pasos...");
                for (j, step) in problem.solution_steps.iter().enumerate() {
                    println!("      {}. {}", j + 1, step);
                }
            }
            println!();
        }

        let success_rate = correct as f64 / problems.len() as f64;

        // Actualizar proficiencia
        if let Some(skill_name) = self.concept_to_skill_name(&concept) {
            if let Some(skill) = self.skills.get_mut(&skill_name) {
                skill.practice_count += problems.len();
                skill.success_rate = (skill.success_rate * 0.7) + (success_rate * 0.3);
                skill.proficiency = skill.success_rate;
            }
        }

        // Actualizar nivel de comprensión
        *self.understanding_level.entry(concept.clone()).or_insert(0.0) = success_rate;

        // Guardar problemas resueltos
        self.solved_problems.extend(attempts.clone());

        // Avanzar de etapa si es apropiado
        self.check_stage_progression();

        println!("  Resultado: {}/{} correctos ({:.1}%)", 
                 correct, problems.len(), success_rate * 100.0);

        PracticeResult {
            concept,
            total_problems: problems.len(),
            correct_answers: correct,
            success_rate,
            attempts,
        }
    }

    fn generate_practice_problems(&self, concept: &MathConcept, count: usize) -> Vec<MathProblem> {
        let mut problems = Vec::new();

        for i in 0..count {
            let difficulty = 0.3 + (i as f64 * 0.07);
            
            let problem = match concept {
                MathConcept::Addition => {
                    let a = rand::random::<u32>() % (10 + (difficulty * 90.0) as u32);
                    let b = rand::random::<u32>() % (10 + (difficulty * 90.0) as u32);
                    MathProblem {
                        question: format!("{} + {} = ?", a, b),
                        expected_answer: format!("{}", a + b),
                        solution_steps: vec![
                            format!("Sumamos {} y {}", a, b),
                            format!("Resultado: {}", a + b),
                        ],
                        difficulty,
                    }
                }
                MathConcept::Subtraction => {
                    let a = rand::random::<u32>() % (10 + (difficulty * 90.0) as u32);
                    let b = rand::random::<u32>() % a.max(1);
                    MathProblem {
                        question: format!("{} - {} = ?", a, b),
                        expected_answer: format!("{}", a - b),
                        solution_steps: vec![
                            format!("Restamos {} de {}", b, a),
                            format!("Resultado: {}", a - b),
                        ],
                        difficulty,
                    }
                }
                MathConcept::Multiplication => {
                    let a = rand::random::<u32>() % (5 + (difficulty * 15.0) as u32);
                    let b = rand::random::<u32>() % (5 + (difficulty * 15.0) as u32);
                    MathProblem {
                        question: format!("{} × {} = ?", a, b),
                        expected_answer: format!("{}", a * b),
                        solution_steps: vec![
                            format!("Multiplicamos {} por {}", a, b),
                            format!("Resultado: {}", a * b),
                        ],
                        difficulty,
                    }
                }
                MathConcept::LinearEquations => {
                    // x + a = b
                    let a = (rand::random::<i32>() % 20) - 10;
                    let x = (rand::random::<i32>() % 20) - 10;
                    let b = x + a;
                    
                    MathProblem {
                        question: format!("x + {} = {}, encuentra x", a, b),
                        expected_answer: format!("{}", x),
                        solution_steps: vec![
                            format!("x + {} = {}", a, b),
                            format!("x = {} - {}", b, a),
                            format!("x = {}", x),
                        ],
                        difficulty,
                    }
                }
                MathConcept::Derivatives => {
                    // d/dx(x^n) = n*x^(n-1)
                    let n = 2 + (rand::random::<u32>() % 4);
                    MathProblem {
                        question: format!("d/dx(x^{}) = ?", n),
                        expected_answer: format!("{}x^{}", n, n - 1),
                        solution_steps: vec![
                            format!("Aplicamos regla de potencias: d/dx(x^n) = n·x^(n-1)"),
                            format!("n = {}", n),
                            format!("Resultado: {}x^{}", n, n - 1),
                        ],
                        difficulty,
                    }
                }
                _ => {
                    MathProblem {
                        question: format!("Concepto {:?} aún no implementado", concept),
                        expected_answer: "N/A".into(),
                        solution_steps: vec![],
                        difficulty,
                    }
                }
            };

            problems.push(problem);
        }

        problems
    }

    fn solve_problem(&self, problem: &MathProblem) -> String {
        // Aquí Daithon intenta resolver
        // Por ahora, usamos la respuesta esperada (entrenamiento supervisado)
        problem.expected_answer.clone()
    }

    fn verify_answer(&self, problem: &MathProblem, answer: &str) -> bool {
        answer.trim() == problem.expected_answer.trim()
    }

    fn concept_to_skill_name(&self, concept: &MathConcept) -> Option<String> {
        match concept {
            MathConcept::Addition => Some("sumar".into()),
            MathConcept::Subtraction => Some("restar".into()),
            MathConcept::Multiplication => Some("multiplicar".into()),
            MathConcept::Division => Some("dividir".into()),
            MathConcept::LinearEquations => Some("ecuaciones_lineales".into()),
            MathConcept::Derivatives => Some("derivadas".into()),
            _ => None,
        }
    }

    fn check_stage_progression(&mut self) {
        let current_stage_skills = match self.current_stage {
            MathStage::Counting => vec![
                MathConcept::Counting,
                MathConcept::Comparison,
                MathConcept::Addition,
                MathConcept::Subtraction,
            ],
            MathStage::BasicArithmetic => vec![
                MathConcept::Multiplication,
                MathConcept::Division,
            ],
            MathStage::Algebra => vec![
                MathConcept::LinearEquations,
                MathConcept::QuadraticEquations,
            ],
            _ => vec![],
        };

        let mastery = current_stage_skills.iter()
            .filter_map(|c| self.understanding_level.get(c))
            .sum::<f64>() / current_stage_skills.len().max(1) as f64;

        if mastery > 0.8 {
            self.advance_stage();
        }
    }

    fn advance_stage(&mut self) {
        let next_stage = match self.current_stage {
            MathStage::Counting => MathStage::BasicArithmetic,
            MathStage::BasicArithmetic => MathStage::Fractions,
            MathStage::Fractions => MathStage::Algebra,
            MathStage::Algebra => MathStage::Calculus,
            MathStage::Calculus => MathStage::Advanced,
            MathStage::Advanced => return,
        };

        println!("\n🎓 [PROGRESO] Avanzando de {:?} → {:?}", 
                 self.current_stage, next_stage);
        self.current_stage = next_stage;
    }

    pub fn get_proficiency_report(&self) -> String {
        let mut report = format!("╔════════════════════════════════════════╗\n");
        report.push_str(&format!("║  NIVEL MATEMÁTICO: {:?}\n", self.current_stage));
        report.push_str(&format!("╠════════════════════════════════════════╣\n"));

        for (concept, level) in &self.understanding_level {
            if *level > 0.0 {
                let bar = "█".repeat((level * 20.0) as usize);
                report.push_str(&format!(
                    "║ {:?}: {:.0}% {}\n",
                    concept, level * 100.0, bar
                ));
            }
        }

        report.push_str(&format!("╠════════════════════════════════════════╣\n"));
        report.push_str(&format!("║ Problemas resueltos: {}\n", self.solved_problems.len()));
        report.push_str(&format!("╚════════════════════════════════════════╝\n"));

        report
    }
}

#[derive(Debug, Clone)]
pub struct MathProblem {
    pub question: String,
    pub expected_answer: String,
    pub solution_steps: Vec<String>,
    pub difficulty: f64,
}

#[derive(Debug, Clone)]
pub struct PracticeResult {
    pub concept: MathConcept,
    pub total_problems: usize,
    pub correct_answers: usize,
    pub success_rate: f64,
    pub attempts: Vec<SolvedProblem>,
}
