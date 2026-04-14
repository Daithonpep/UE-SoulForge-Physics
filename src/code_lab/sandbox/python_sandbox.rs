use std::process::Command;
use std::time::{Duration, Instant};
use serde::{Deserialize, Serialize};

/// Sandbox seguro para ejecutar código Python
pub struct PythonSandbox {
    /// Timeout máximo por ejecución (segundos)
    timeout_seconds: u64,
    /// Límite de memoria (MB)
    memory_limit_mb: usize,
    /// Directorio temporal para archivos
    temp_dir: String,
    /// Módulos permitidos
    allowed_modules: Vec<String>,
    /// Módulos prohibidos
    blocked_modules: Vec<String>,
}

/// Resultado de una ejecución
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionResult {
    /// ¿Compiló/ejecutó sin errores?
    pub success: bool,
    /// Salida estándar
    pub stdout: String,
    /// Salida de error
    pub stderr: String,
    /// Tiempo de ejecución en ms
    pub execution_time_ms: u64,
    /// Memoria usada (estimada) en KB
    pub memory_used_kb: usize,
    /// Código de salida
    pub exit_code: i32,
    /// Errores parseados
    pub parsed_errors: Vec<ParsedError>,
    /// Resultados de tests si había
    pub test_results: Vec<TestResult>,
    /// Métricas de calidad del código
    pub quality_metrics: CodeQualityMetrics,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ParsedError {
    pub error_type: String,
    pub message: String,
    pub line: Option<usize>,
    pub column: Option<usize>,
    pub suggestion: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestResult {
    pub name: String,
    pub passed: bool,
    pub expected: String,
    pub actual: String,
    pub execution_time_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeQualityMetrics {
    pub lines_of_code: usize,
    pub complexity_score: f64,
    pub has_docstrings: bool,
    pub has_type_hints: bool,
    pub num_functions: usize,
    pub num_classes: usize,
}

impl PythonSandbox {
    pub fn new() -> Self {
        let temp_dir = "code_lab/sandbox_temp".to_string();
        std::fs::create_dir_all(&temp_dir).ok();

        Self {
            timeout_seconds: 10,
            memory_limit_mb: 128,
            temp_dir,
            allowed_modules: vec![
                "math".into(), "collections".into(), "itertools".into(),
                "functools".into(), "typing".into(), "dataclasses".into(),
                "json".into(), "time".into(), "random".into(),
                "heapq".into(), "bisect".into(), "array".into(),
                "struct".into(), "hashlib".into(), "re".into(),
            ],
            blocked_modules: vec![
                "os".into(), "sys".into(), "subprocess".into(),
                "shutil".into(), "socket".into(), "http".into(),
                "urllib".into(), "requests".into(), "pathlib".into(),
                "importlib".into(), "ctypes".into(), "multiprocessing".into(),
                "threading".into(), "signal".into(), "pickle".into(),
            ],
        }
    }

    /// Ejecutar código Python de forma segura
    pub fn execute(&self, code: &str) -> ExecutionResult {
        // 1. Validar seguridad del código
        if let Some(security_error) = self.security_check(code) {
            return ExecutionResult {
                success: false,
                stdout: String::new(),
                stderr: format!("SEGURIDAD: {}", security_error),
                execution_time_ms: 0,
                memory_used_kb: 0,
                exit_code: -1,
                parsed_errors: vec![ParsedError {
                    error_type: "SecurityViolation".into(),
                    message: security_error,
                    line: None,
                    column: None,
                    suggestion: "Usa solo módulos permitidos y no intentes acceder al sistema".into(),
                }],
                test_results: vec![],
                quality_metrics: self.analyze_code_quality(code),
            };
        }

        // 2. Envolver código en sandbox Python
        let sandboxed_code = self.wrap_in_sandbox(code);

        // 3. Escribir a archivo temporal con ID único para evitar colisiones en paralelo
        let thread_id = std::thread::current().id();
        let file_path = format!("{}/daithon_code_{:?}_{}.py", self.temp_dir, 
                               thread_id,
                               std::time::SystemTime::now()
                                   .duration_since(std::time::UNIX_EPOCH)
                                   .unwrap().as_millis());

        if let Err(e) = std::fs::write(&file_path, &sandboxed_code) {
            return self.error_result(&format!("No se pudo escribir archivo: {}", e));
        }

        // 4. Ejecutar con timeout
        let start = Instant::now();

        let output = Command::new("python")
            .arg(&file_path)
            .env("PYTHONDONTWRITEBYTECODE", "1")
            .env("PYTHONPATH", "")
            .output();

        let execution_time = start.elapsed();

        // 5. Limpiar archivo temporal
        std::fs::remove_file(&file_path).ok();

        match output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                let exit_code = output.status.code().unwrap_or(-1);
                let success = output.status.success();

                // Parsear errores
                let parsed_errors = if !success {
                    self.parse_python_errors(&stderr)
                } else {
                    vec![]
                };

                // Parsear tests
                let test_results = self.parse_test_results(&stdout);

                // Estimar memoria
                let memory_estimate = self.estimate_memory(&stdout);

                ExecutionResult {
                    success,
                    stdout,
                    stderr,
                    execution_time_ms: execution_time.as_millis() as u64,
                    memory_used_kb: memory_estimate,
                    exit_code,
                    parsed_errors,
                    test_results,
                    quality_metrics: self.analyze_code_quality(code),
                }
            }
            Err(e) => {
                if execution_time > Duration::from_secs(self.timeout_seconds) {
                    self.error_result("TIMEOUT: El código tardó demasiado en ejecutar")
                } else {
                    self.error_result(&format!("Error de ejecución: {}", e))
                }
            }
        }
    }

    /// Verificar seguridad del código antes de ejecutar
    fn security_check(&self, code: &str) -> Option<String> {
        let code_lower = code.to_lowercase();

        // Verificar imports bloqueados
        for blocked in &self.blocked_modules {
            if code_lower.contains(&format!("import {}", blocked))
                || code_lower.contains(&format!("from {} ", blocked))
                || code_lower.contains(&format!("from {}", blocked))
            {
                return Some(format!(
                    "Módulo '{}' no permitido en sandbox. Módulos permitidos: {:?}",
                    blocked, self.allowed_modules
                ));
            }
        }

        // Verificar operaciones peligrosas
        let dangerous = [
            ("exec(", "Función exec() no permitida"),
            ("eval(", "Función eval() no permitida"),
            ("__import__", "__import__ no permitido"),
            ("open(", "Acceso a archivos no permitido"),
            ("compile(", "compile() no permitido"),
        ];

        for (pattern, message) in &dangerous {
            if code.contains(pattern) {
                return Some(message.to_string());
            }
        }

        // Verificar tamaño del código
        if code.len() > 50000 {
            return Some("Código demasiado largo (máximo 50KB)".into());
        }

        None
    }

    /// Envolver código en sandbox con mediciones
    fn wrap_in_sandbox(&self, code: &str) -> String {
        format!(r#"
import time
import traceback

# Medir tiempo y memoria
_start_time = time.perf_counter()
_start_mem = 0
try:
    import tracemalloc
    tracemalloc.start()
    _start_mem = tracemalloc.get_traced_memory()[0]
except:
    pass

# ═══ CÓDIGO DE DAITHON ═══
try:
{indented_code}
except Exception as e:
    print(f"ERROR: {{type(e).__name__}}: {{e}}")
    traceback.print_exc()

# ═══ MÉTRICAS ═══
_end_time = time.perf_counter()
_elapsed_ms = (_end_time - _start_time) * 1000

try:
    _current_mem, _peak_mem = tracemalloc.get_traced_memory()
    tracemalloc.stop()
    print(f"\n__METRICS__")
    print(f"time_ms: {{_elapsed_ms:.3f}}")
    print(f"memory_current_kb: {{_current_mem / 1024:.1f}}")
    print(f"memory_peak_kb: {{_peak_mem / 1024:.1f}}")
except:
    print(f"\n__METRICS__")
    print(f"time_ms: {{_elapsed_ms:.3f}}")
    print(f"memory_current_kb: 0")
    print(f"memory_peak_kb: 0")
"#,
            indented_code = code.lines()
                .map(|line| format!("    {}", line))
                .collect::<Vec<_>>()
                .join("\n"),
        )
    }

    fn parse_python_errors(&self, stderr: &str) -> Vec<ParsedError> {
        let mut errors = Vec::new();

        for line in stderr.lines() {
            // Parsear errores estándar de Python
            if line.contains("Error:") || line.contains("error:") {
                let parts: Vec<&str> = line.splitn(2, ':').collect();

                let error_type = parts.first().unwrap_or(&"Unknown").trim().to_string();
                let message = parts.get(1).unwrap_or(&"").trim().to_string();

                let suggestion = self.suggest_fix(&error_type, &message);

                errors.push(ParsedError {
                    error_type,
                    message,
                    line: self.extract_line_number(stderr),
                    column: None,
                    suggestion,
                });
            }
        }

        errors
    }

    fn suggest_fix(&self, error_type: &str, message: &str) -> String {
        match error_type {
            t if t.contains("SyntaxError") => {
                if message.contains("unexpected EOF") {
                    "Falta cerrar un paréntesis, corchete o bloque de código".into()
                } else if message.contains("invalid syntax") {
                    "Revisa la sintaxis: puede faltar ':' al final de if/for/def, o un paréntesis".into()
                } else {
                    "Error de sintaxis. Revisa la estructura del código".into()
                }
            }
            t if t.contains("NameError") => {
                format!("Variable o función no definida. ¿La escribiste bien? ¿La definiste antes de usarla?")
            }
            t if t.contains("TypeError") => {
                "Tipos incompatibles. Verifica que estés operando con los tipos correctos (int vs str, etc)".into()
            }
            t if t.contains("IndexError") => {
                "Índice fuera de rango. El array/lista no tiene tantos elementos".into()
            }
            t if t.contains("KeyError") => {
                "Clave no encontrada en diccionario. Verifica que la clave existe antes de acceder".into()
            }
            t if t.contains("ZeroDivisionError") => {
                "División por cero. Añade una verificación: if divisor != 0".into()
            }
            t if t.contains("AttributeError") => {
                "El objeto no tiene ese atributo/método. Verifica el tipo del objeto".into()
            }
            t if t.contains("ImportError") || t.contains("ModuleNotFoundError") => {
                format!("Módulo no disponible. Módulos permitidos: math, collections, itertools, json, time, random")
            }
            _ => "Revisa el código y los tipos de datos".into(),
        }
    }

    fn extract_line_number(&self, stderr: &str) -> Option<usize> {
        for line in stderr.lines() {
            if line.contains("line ") {
                let parts: Vec<&str> = line.split("line ").collect();
                if let Some(num_str) = parts.get(1) {
                    if let Ok(num) = num_str.split(|c: char| !c.is_numeric()).next()
                        .unwrap_or("0").parse::<usize>()
                    {
                        // Restar las líneas del wrapper (aproximadamente 15)
                        return Some(num.saturating_sub(15));
                    }
                }
            }
        }
        None
    }

    fn parse_test_results(&self, stdout: &str) -> Vec<TestResult> {
        let mut results = Vec::new();

        for line in stdout.lines() {
            if line.starts_with("TEST_") {
                let parts: Vec<&str> = line.splitn(3, '|').collect();
                if parts.len() >= 3 {
                    let name = parts[0].trim().to_string();
                    let status = parts[1].trim();
                    let detail = parts[2].trim().to_string();

                    results.push(TestResult {
                        name,
                        passed: status == "PASS",
                        expected: String::new(),
                        actual: detail,
                        execution_time_ms: 0,
                    });
                }
            }
        }

        results
    }

    fn estimate_memory(&self, stdout: &str) -> usize {
        for line in stdout.lines() {
            if line.starts_with("memory_peak_kb:") {
                if let Some(value) = line.split(':').nth(1) {
                    if let Ok(kb) = value.trim().parse::<f64>() {
                        return kb as usize;
                    }
                }
            }
        }
        0
    }

    fn analyze_code_quality(&self, code: &str) -> CodeQualityMetrics {
        let lines: Vec<&str> = code.lines().collect();
        let code_lines = lines.iter()
            .filter(|l| !l.trim().is_empty() && !l.trim().starts_with('#'))
            .count();

        let num_functions = code.matches("def ").count();
        let num_classes = code.matches("class ").count();
        let has_docstrings = code.contains(r#"""""#) || code.contains("'''");
        let has_type_hints = code.contains("->") || code.contains(": int") 
            || code.contains(": str") || code.contains(": float");

        let complexity = (num_functions as f64 * 0.3) 
            + (code_lines as f64 * 0.01)
            + if code.contains("for ") { 0.2 } else { 0.0 }
            + if code.contains("while ") { 0.3 } else { 0.0 }
            + if code.contains("try:") { 0.1 } else { 0.0 };

        CodeQualityMetrics {
            lines_of_code: code_lines,
            complexity_score: complexity.min(10.0),
            has_docstrings,
            has_type_hints,
            num_functions,
            num_classes,
        }
    }

    fn error_result(&self, message: &str) -> ExecutionResult {
        ExecutionResult {
            success: false,
            stdout: String::new(),
            stderr: message.to_string(),
            execution_time_ms: 0,
            memory_used_kb: 0,
            exit_code: -1,
            parsed_errors: vec![ParsedError {
                error_type: "ExecutionError".into(),
                message: message.to_string(),
                line: None,
                column: None,
                suggestion: "Verifica que el código sea válido".into(),
            }],
            test_results: vec![],
            quality_metrics: CodeQualityMetrics {
                lines_of_code: 0, complexity_score: 0.0,
                has_docstrings: false, has_type_hints: false,
                num_functions: 0, num_classes: 0,
            },
        }
    }
}
