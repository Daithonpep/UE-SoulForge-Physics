use crate::forge::ast::logic_tree::*;

/// Sistema que lee errores del compilador y corrige el AST
pub struct AutoDebugger;

impl AutoDebugger {
    /// Analizar error y sugerir corrección al AST
    pub fn diagnose_and_fix(
        _ast: &LogicNode,
        error: &crate::code_lab::sandbox::python_sandbox::ParsedError,
    ) -> Option<ASTFix> {
        match error.error_type.as_str() {
            t if t.contains("NameError") => Self::fix_name_error(_ast, &error.message),
            t if t.contains("TypeError") => Self::fix_type_error(_ast, &error.message),
            t if t.contains("IndexError") => Self::fix_index_error(_ast, &error.message),
            t if t.contains("SyntaxError") => Self::fix_syntax_error(_ast, &error.message),
            t if t.contains("ZeroDivision") => Self::fix_zero_division(_ast),
            _ => None,
        }
    }

    fn fix_name_error(_ast: &LogicNode, message: &str) -> Option<ASTFix> {
        // Extraer nombre de variable no definida
        if let Some(var_name) = message.split('\'').nth(1) {
            Some(ASTFix {
                description: format!("Variable '{}' no definida. Añadir declaración.", var_name),
                fix_type: FixType::AddDeclaration {
                    var_name: var_name.to_string(),
                    suggested_type: DataType::Any,
                },
            })
        } else {
            None
        }
    }

    fn fix_type_error(_ast: &LogicNode, message: &str) -> Option<ASTFix> {
        if message.contains("unsupported operand") {
            Some(ASTFix {
                description: "Tipos incompatibles en operación. Añadir conversión.".into(),
                fix_type: FixType::AddTypeConversion,
            })
        } else {
            None
        }
    }

    fn fix_index_error(_ast: &LogicNode, _message: &str) -> Option<ASTFix> {
        Some(ASTFix {
            description: "Índice fuera de rango. Añadir verificación de límites.".into(),
            fix_type: FixType::AddBoundsCheck,
        })
    }

    fn fix_syntax_error(_ast: &LogicNode, _message: &str) -> Option<ASTFix> {
        Some(ASTFix {
            description: "Error de sintaxis. Regenerar desde AST.".into(),
            fix_type: FixType::RegenerateFromAST,
        })
    }

    fn fix_zero_division(_ast: &LogicNode) -> Option<ASTFix> {
        Some(ASTFix {
            description: "División por cero. Añadir guard.".into(),
            fix_type: FixType::AddGuard {
                condition: "divisor != 0".into(),
            },
        })
    }
}

#[derive(Debug, Clone)]
pub struct ASTFix {
    pub description: String,
    pub fix_type: FixType,
}

#[derive(Debug, Clone)]
pub enum FixType {
    AddDeclaration { var_name: String, suggested_type: DataType },
    AddTypeConversion,
    AddBoundsCheck,
    AddGuard { condition: String },
    RegenerateFromAST,
    #[allow(dead_code)]
    WrapInTryCatch,
}
