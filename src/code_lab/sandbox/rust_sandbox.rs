use std::process::Command;
use std::time::Instant;

/// Sandbox para compilar y ejecutar código Rust
pub struct RustSandbox {
    workspace_dir: String,
    timeout_seconds: u64,
}

impl RustSandbox {
    pub fn new() -> Self {
        let workspace_dir = "code_lab/rust_workspace".to_string();

        // Crear proyecto Rust temporal si no existe
        if !std::path::Path::new(&format!("{}/Cargo.toml", workspace_dir)).exists() {
            std::fs::create_dir_all(&workspace_dir).ok();
            std::fs::create_dir_all(&format!("{}/src", workspace_dir)).ok();

            let cargo_toml = r#"[package]
name = "daithon_experiment"
version = "0.1.0"
edition = "2021"
"#;
            std::fs::write(format!("{}/Cargo.toml", workspace_dir), cargo_toml).ok();
            // Crear un src/main.rs básico para que sea un binario válido
            std::fs::write(format!("{}/src/main.rs", workspace_dir), "fn main() {}").ok();
        }

        Self {
            workspace_dir,
            timeout_seconds: 30,
        }
    }

    /// Compilar y ejecutar código Rust
    pub fn execute(&self, code: &str) -> super::python_sandbox::ExecutionResult {
        // 1. Verificar seguridad
        if let Some(error) = self.security_check(code) {
            return super::python_sandbox::ExecutionResult {
                success: false,
                stdout: String::new(),
                stderr: format!("SEGURIDAD: {}", error),
                execution_time_ms: 0,
                memory_used_kb: 0,
                exit_code: -1,
                parsed_errors: vec![super::python_sandbox::ParsedError {
                    error_type: "SecurityViolation".into(),
                    message: error,
                    line: None,
                    column: None,
                    suggestion: "No uses std::fs, std::net, std::process en el sandbox".into(),
                }],
                test_results: vec![],
                quality_metrics: super::python_sandbox::CodeQualityMetrics {
                    lines_of_code: code.lines().count(),
                    complexity_score: 0.0,
                    has_docstrings: code.contains("///"),
                    has_type_hints: true,
                    num_functions: code.matches("fn ").count(),
                    num_classes: code.matches("struct ").count(),
                },
            };
        }

        // 2. Escribir código
        let main_path = format!("{}/src/main.rs", self.workspace_dir);
        if let Err(e) = std::fs::write(&main_path, code) {
            return self.error_result(&format!("No se pudo escribir: {}", e));
        }

        // 3. Compilar (cargo check primero, más rápido)
        let start = Instant::now();

        let check_output = Command::new("cargo")
            .arg("check")
            .current_dir(&self.workspace_dir)
            .output();

        match check_output {
            Ok(output) if !output.status.success() => {
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                let parsed_errors = self.parse_rust_errors(&stderr);

                return super::python_sandbox::ExecutionResult {
                    success: false,
                    stdout: String::new(),
                    stderr: stderr.clone(),
                    execution_time_ms: start.elapsed().as_millis() as u64,
                    memory_used_kb: 0,
                    exit_code: 1,
                    parsed_errors,
                    test_results: vec![],
                    quality_metrics: self.analyze_rust_quality(code),
                };
            }
            Err(e) => {
                return self.error_result(&format!("cargo check falló: {}", e));
            }
            _ => {}
        }

        // 4. Compilar y ejecutar
        let run_output = Command::new("cargo")
            .arg("run")
            .arg("--release")
            .current_dir(&self.workspace_dir)
            .output();

        let execution_time = start.elapsed();

        match run_output {
            Ok(output) => {
                let stdout = String::from_utf8_lossy(&output.stdout).to_string();
                let stderr = String::from_utf8_lossy(&output.stderr).to_string();
                let success = output.status.success();

                let parsed_errors = if !success {
                    self.parse_rust_errors(&stderr)
                } else {
                    vec![]
                };

                super::python_sandbox::ExecutionResult {
                    success,
                    stdout,
                    stderr,
                    execution_time_ms: execution_time.as_millis() as u64,
                    memory_used_kb: 0,
                    exit_code: output.status.code().unwrap_or(-1),
                    parsed_errors,
                    test_results: vec![],
                    quality_metrics: self.analyze_rust_quality(code),
                }
            }
            Err(e) => {
                self.error_result(&format!("Ejecución falló: {}", e))
            }
        }
    }

    fn security_check(&self, code: &str) -> Option<String> {
        let dangerous = [
            ("std::fs", "Acceso a filesystem no permitido"),
            ("std::net", "Acceso a red no permitido"),
            ("std::process::Command", "Ejecución de procesos no permitida"),
            ("unsafe {", "Bloques unsafe no permitidos en sandbox"),
        ];

        for (pattern, message) in &dangerous {
            if code.contains(pattern) {
                return Some(message.to_string());
            }
        }

        None
    }

    fn parse_rust_errors(&self, stderr: &str) -> Vec<super::python_sandbox::ParsedError> {
        let mut errors = Vec::new();

        for line in stderr.lines() {
            if line.contains("error:") || line.contains("error[") {
                let message = line.to_string();
                let line_num = self.extract_rust_line(stderr);

                let suggestion = if message.contains("expected") {
                    "Verifica los tipos y la sintaxis".into()
                } else if message.contains("not found") {
                    "Variable o función no definida. ¿La importaste?".into()
                } else if message.contains("borrow") {
                    "Error de borrowing. Considera usar .clone() o referencias".into()
                } else if message.contains("lifetime") {
                    "Error de lifetime. Asegúrate de que las referencias vivan suficiente".into()
                } else {
                    "Revisa el mensaje del compilador".into()
                };

                errors.push(super::python_sandbox::ParsedError {
                    error_type: "CompileError".into(),
                    message,
                    line: line_num,
                    column: None,
                    suggestion,
                });
            }
        }

        errors
    }

    fn extract_rust_line(&self, stderr: &str) -> Option<usize> {
        for line in stderr.lines() {
            if line.contains("-->") && line.contains("main.rs:") {
                let parts: Vec<&str> = line.split("main.rs:").collect();
                if let Some(num_str) = parts.get(1) {
                    if let Ok(num) = num_str.split(':').next().unwrap_or("0").parse::<usize>() {
                        return Some(num);
                    }
                }
            }
        }
        None
    }

    fn analyze_rust_quality(&self, code: &str) -> super::python_sandbox::CodeQualityMetrics {
        super::python_sandbox::CodeQualityMetrics {
            lines_of_code: code.lines().filter(|l| !l.trim().is_empty() && !l.trim().starts_with("//")).count(),
            complexity_score: (code.matches("fn ").count() as f64 * 0.3) + (code.matches("match ").count() as f64 * 0.2),
            has_docstrings: code.contains("///"),
            has_type_hints: true,
            num_functions: code.matches("fn ").count(),
            num_classes: code.matches("struct ").count() + code.matches("enum ").count(),
        }
    }

    fn error_result(&self, message: &str) -> super::python_sandbox::ExecutionResult {
        super::python_sandbox::ExecutionResult {
            success: false,
            stdout: String::new(),
            stderr: message.to_string(),
            execution_time_ms: 0,
            memory_used_kb: 0,
            exit_code: -1,
            parsed_errors: vec![],
            test_results: vec![],
            quality_metrics: super::python_sandbox::CodeQualityMetrics {
                lines_of_code: 0, complexity_score: 0.0,
                has_docstrings: false, has_type_hints: false,
                num_functions: 0, num_classes: 0,
            },
        }
    }
}
