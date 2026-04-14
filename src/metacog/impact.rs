use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum RiskLevel {
    Low,      // Acción rutinaria, no requiere reporte.
    Medium,   // Riesgo moderado, advertencia de una línea.
    High,     // Riesgo alto, requiere Informe de Impacto Helix 2.0.
    Critical, // Riesgo sistémico, requiere reporte y doble confirmación.
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImpactReport {
    pub action: String,
    pub risks: Vec<String>,
    pub proposed_alternative: String,
    pub severity: f64,
    pub risk_level: RiskLevel,
    pub backup_suggested: bool,
}

impl ImpactReport {
    pub fn format_report(&self) -> String {
        match self.risk_level {
            RiskLevel::Low => String::new(),
            RiskLevel::Medium => format!("⚠️ **AVISO RÁPIDO:** {}, considera que {}.", self.action, self.risks[0]),
            RiskLevel::High | RiskLevel::Critical => {
                let mut report = String::from("\n🚨 **INFORME DE IMPACTO CRÍTICO (Helix 2.0)**:\n\n");
                report.push_str(&format!("• **Acción:** {}\n", self.action));
                report.push_str("• **Riesgos:**\n");
                for risk in &self.risks { report.push_str(&format!("  - {}\n", risk)); }
                report.push_str(&format!("• **Propuesta:** {}\n", self.proposed_alternative));
                if self.backup_suggested {
                    report.push_str("💡 **Nota:** He preparado un backup temporal (git stash) por si deseas revertir.\n");
                }
                report.push_str("\n¿Procedo o seguimos mi propuesta?");
                report
            }
        }
    }
}

pub struct ImpactAnalyzer;

impl ImpactAnalyzer {
    pub fn analyze(input: &str, _context_info: &str) -> Option<ImpactReport> {
        let input_low = input.to_lowercase();
        
        // --- NIVEL: CRITICAL (Borrado de archivos de sistema, BD, auth) ---
        if (input_low.contains("borra") || input_low.contains("elimina")) && 
           (input_low.contains("auth") || input_low.contains("main") || input_low.contains("cargo") || input_low.contains("database")) {
            return Some(ImpactReport {
                action: "Borrado de archivo de núcleo central".into(),
                risks: vec!["Ruptura inmediata del proyecto.".into(), "Pérdida de configuración vital.".into()],
                proposed_alternative: "Mover a .bak y documentar la razón.".into(),
                severity: 0.95,
                risk_level: RiskLevel::Critical,
                backup_suggested: true,
            });
        }

        // --- NIVEL: HIGH (Eliminación de módulos normales) ---
        if input_low.contains("borra") || input_low.contains("quitar") {
            return Some(ImpactReport {
                action: format!("Eliminación de: '{}'", input),
                risks: vec!["Posibles dependencias huérfanas.".into()],
                proposed_alternative: "Mover a la carpeta 'deprecated'.".into(),
                severity: 0.7,
                risk_level: RiskLevel::High,
                backup_suggested: true,
            });
        }

        // --- NIVEL: MEDIUM (Cambios estéticos agresivos, cambios leves de arquitectura) ---
        if input_low.contains("rosa neón") || input_low.contains("refactorizar") {
            return Some(ImpactReport {
                action: "Modificación estética/lógica agresiva".into(),
                risks: vec!["Fatiga visual o desajuste de UI.".into()],
                proposed_alternative: "Usar una rama de prueba.".into(),
                severity: 0.4,
                risk_level: RiskLevel::Medium,
                backup_suggested: false,
            });
        }

        None
    }
}
