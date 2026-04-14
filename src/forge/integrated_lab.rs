use crate::forge::ast::logic_tree::*;
use crate::forge::ast::code_generator::*;
use crate::forge::reasoning::problem_solver::*;
use crate::forge::debugger::auto_debugger::*;
use crate::forge::tools::system_tools::*;

/// Laboratorio integrado con AST + Sandbox + Auto-Debug + Tools
pub struct IntegratedCodeLab {
    pub solver: ProblemSolver,
    pub tools: SystemTools,
    pub rust_sandbox: crate::code_lab::sandbox::rust_sandbox::RustSandbox,
    pub iteration_count: usize,
    pub knowledge_base: Vec<String>,
}

impl IntegratedCodeLab {
    pub fn new() -> Self {
        Self {
            solver: ProblemSolver::new(),
            tools: SystemTools::new(),
            rust_sandbox: crate::code_lab::sandbox::rust_sandbox::RustSandbox::new(),
            iteration_count: 0,
            knowledge_base: Vec::new(),
        }
    }

    /// Resolver un problema con el ciclo completo y aprendizaje activo
    pub fn solve_problem(
        &mut self,
        description: &str,
        problem: ProblemDescription,
        max_iterations: usize,
    ) -> SolutionResult {
        // 1. Generar AST inicial
        let mut current_ast = self.solver.solve(&problem);
        
        let mut iterations = Vec::new();
        let mut last_error = String::new();

        for iter in 0..max_iterations {
            self.iteration_count += 1;

            // 2. Aplicar auto-correcciones proactivas (aprendizaje)
            if iter > 0 {
                self.apply_learning_step(&mut current_ast, &problem, &last_error);
            }

            // 3. Traducir AST a Rust y Python
            let python_code = CodeGenerator::to_python(&current_ast, 0);
            let rust_code = CodeGenerator::to_rust(&current_ast, 0);

            // 4. Ejecutar en sandbox de RUST (esto revelará los errores reales de concurrencia/lógica)
            let result = self.rust_sandbox.execute(&rust_code);

            // 5. Analizar resultado del compilador/tests
            if result.success {
                
                // Guardar lección aprendida
                self.knowledge_base.push(format!("Aprendí a resolver '{}' usando patrones de {:?}", description, problem.intent));

                return SolutionResult {
                    success: true,
                    final_ast: current_ast,
                    python_code,
                    rust_code,
                    iterations: iter + 1,
                    lessons: self.knowledge_base.clone(),
                };
            }

            // 6. Si falla, registrar el error y buscar ayuda (auto-fix)
            if !result.parsed_errors.is_empty() {
                let error = &result.parsed_errors[0];
                last_error = error.message.clone();

                if let Some(fix) = AutoDebugger::diagnose_and_fix(&current_ast, error) {
                    self.apply_ast_fix(&mut current_ast, fix);
                }
            } else {
                // Si compile OK pero fallan tests, es un error lógico
                last_error = "LOGIC_FAIL".to_string();
            }

            iterations.push((rust_code, result));
        }

        let final_code = CodeGenerator::to_python(&current_ast, 0);
        let rust_code = CodeGenerator::to_rust(&current_ast, 0);

        SolutionResult {
            success: false,
            final_ast: current_ast,
            python_code: final_code,
            rust_code,
            iterations: max_iterations,
            lessons: vec!["Necesito más datos sobre este dominio lógico".into()],
        }
    }

    /// Aplica una corrección física al árbol AST
    fn apply_ast_fix(&mut self, node: &mut LogicNode, fix: ASTFix) {
        match fix.fix_type {
            FixType::AddDeclaration { var_name, .. } => {
                // En un sistema real, buscaríamos dónde falta e inyectaríamos el nodo DeclareVar
                self.knowledge_base.push(format!("FIX: Declarada variable faltante '{}'", var_name));
            }
            FixType::AddGuard { condition } => {
                self.knowledge_base.push(format!("FIX: Añadida guarda contra división por cero: '{}'", condition));
            }
            _ => {}
        }
        
        // Simulación de parcheo de AST (esto se expandiría con recursión sobre el árbol)
        self.patch_recursive(node);
    }

    /// Busca Nodos "TODO" o placeholders y los reemplaza con lógica real si hay pistas
    fn apply_learning_step(&mut self, node: &mut LogicNode, problem: &ProblemDescription, last_error: &str) {
        // Si el problema es de filtro y estamos en un TODO, inyectar condición lógica base
        if matches!(problem.intent, ProblemIntent::Filter) && last_error.contains("SyntaxError") {
            self.knowledge_base.push("AUTOCORRECCIÓN: Reemplazado placeholder de condición por lógica de comparación numérica".into());
            self.replace_placeholders_in_ast(node);
        }
    }

    fn patch_recursive(&mut self, node: &mut LogicNode) {
        match node {
            LogicNode::Program { body, .. } => {
                for n in body { self.patch_recursive(n); }
            }
            LogicNode::FunctionDef { body, .. } => {
                for n in body { self.patch_recursive(n); }
            }
            // ... resto de nodos recurrentes
            _ => {}
        }
    }

    fn replace_placeholders_in_ast(&mut self, node: &mut LogicNode) {
        match node {
            LogicNode::Program { body, .. } => {
                for n in body { self.replace_placeholders_in_ast(n); }
            }
            LogicNode::FunctionDef { body, .. } => {
                for n in body { self.replace_placeholders_in_ast(n); }
            }
            LogicNode::ForLoop { body, .. } => {
                for n in body { self.replace_placeholders_in_ast(n); }
            }
            LogicNode::IfElse { condition, then_body, else_body } => {
                // Si la condición es un comentario de "TODO", reemplazar por x > 0
                if let LogicNode::Comment(text) = &**condition {
                    if text.contains("CONDICIÓN") {
                        *condition = Box::new(LogicNode::BinaryOp {
                            op: BinaryOperator::GreaterThan,
                            left: Box::new(LogicNode::Variable("item".into())),
                            right: Box::new(LogicNode::IntLiteral(0)),
                        });
                    }
                }
                for n in then_body { self.replace_placeholders_in_ast(n); }
                if let Some(eb) = else_body {
                    for n in eb { self.replace_placeholders_in_ast(n); }
                }
            }
            _ => {}
        }
    }

    fn count_nodes(&self, _node: &LogicNode) -> usize {
        // En un sistema completo haríamos una cuenta recursiva
        1 
    }
}

#[derive(Debug, Clone)]
pub struct SolutionResult {
    pub success: bool,
    pub final_ast: LogicNode,
    pub python_code: String,
    pub rust_code: String,
    pub iterations: usize,
    pub lessons: Vec<String>,
}
