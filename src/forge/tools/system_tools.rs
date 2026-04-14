/// Herramientas que Daithon puede usar desde su código
pub struct SystemTools {
    /// Directorio permitido para archivos
    allowed_dir: String,
    /// Dominios permitidos para HTTP
    allowed_domains: Vec<String>,
    /// Log de uso de herramientas
    #[allow(dead_code)]
    usage_log: Vec<ToolUsageRecord>,
}

#[derive(Debug, Clone)]
pub struct ToolUsageRecord {
    pub tool: String,
    pub args: Vec<String>,
    pub result: ToolResult,
    pub timestamp: u64,
}

#[derive(Debug, Clone)]
pub enum ToolResult {
    Success(String),
    Error(String),
    Denied(String),
}

impl SystemTools {
    pub fn new() -> Self {
        Self {
            allowed_dir: "code_lab/workspace".to_string(),
            allowed_domains: vec![
                "docs.rs".into(),
                "doc.rust-lang.org".into(),
                "docs.python.org".into(),
                "en.wikipedia.org".into(),
            ],
            usage_log: Vec::new(),
        }
    }

    pub fn read_file(&mut self, path: &str) -> ToolResult {
        // Verificar que está en directorio permitido
        if !path.starts_with(&self.allowed_dir) {
            let result = ToolResult::Denied(format!("Solo puedes leer archivos en {}", self.allowed_dir));
            self.log("read_file", &[path], &result);
            return result;
        }

        match std::fs::read_to_string(path) {
            Ok(content) => {
                let result = ToolResult::Success(content);
                self.log("read_file", &[path], &result);
                result
            }
            Err(e) => {
                let result = ToolResult::Error(format!("No se pudo leer: {}", e));
                self.log("read_file", &[path], &result);
                result
            }
        }
    }

    pub fn write_file(&mut self, path: &str, content: &str) -> ToolResult {
        if !path.starts_with(&self.allowed_dir) {
            let result = ToolResult::Denied(format!("Solo puedes escribir en {}", self.allowed_dir));
            self.log("write_file", &[path], &result);
            return result;
        }

        if let Some(parent) = std::path::Path::new(path).parent() {
            std::fs::create_dir_all(parent).ok();
        }

        match std::fs::write(path, content) {
            Ok(_) => {
                let result = ToolResult::Success(format!("Escrito: {} bytes", content.len()));
                self.log("write_file", &[path], &result);
                result
            }
            Err(e) => {
                let result = ToolResult::Error(format!("No se pudo escribir: {}", e));
                self.log("write_file", &[path], &result);
                result
            }
        }
    }

    pub async fn http_get(&mut self, url: &str) -> ToolResult {
        // Verificar dominio
        let allowed = self.allowed_domains.iter()
            .any(|domain| url.contains(domain));

        if !allowed {
            let result = ToolResult::Denied(format!(
                "Dominio no permitido. Permitidos: {:?}", self.allowed_domains
            ));
            self.log("http_get", &[url], &result);
            return result;
        }

        match reqwest::get(url).await {
            Ok(response) => {
                match response.text().await {
                    Ok(body) => {
                        // Limitar tamaño
                        let truncated = if body.len() > 50000 {
                            format!("{}... [truncado]", &body[..50000])
                        } else {
                            body
                        };
                        let result = ToolResult::Success(truncated);
                        self.log("http_get", &[url], &result);
                        result
                    }
                    Err(e) => {
                        let result = ToolResult::Error(format!("Error leyendo respuesta: {}", e));
                        self.log("http_get", &[url], &result);
                        result
                    }
                }
            }
            Err(e) => {
                let result = ToolResult::Error(format!("Error HTTP: {}", e));
                self.log("http_get", &[url], &result);
                result
            }
        }
    }

    fn log(&mut self, tool: &str, args: &[&str], result: &ToolResult) {
        self.usage_log.push(ToolUsageRecord {
            tool: tool.to_string(),
            args: args.iter().map(|a| a.to_string()).collect(),
            result: result.clone(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        });
    }
}
