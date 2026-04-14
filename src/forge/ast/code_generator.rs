use super::logic_tree::*;

/// Traduce AST a código real en cualquier lenguaje
pub struct CodeGenerator;

impl CodeGenerator {
    /// Generar código Python desde AST
    pub fn to_python(node: &LogicNode, indent: usize) -> String {
        let pad = "    ".repeat(indent);

        match node {
            LogicNode::Program { body, .. } => {
                body.iter()
                    .map(|n| Self::to_python(n, 0))
                    .collect::<Vec<_>>()
                    .join("\n")
            }

            LogicNode::FunctionDef { name, params, return_type, body, doc } => {
                let params_str = params.iter()
                    .map(|p| {
                        let type_hint = Self::python_type(&p.param_type);
                        if let Some(default) = &p.default {
                            format!("{}: {} = {}", p.name, type_hint, Self::to_python(default, 0))
                        } else {
                            format!("{}: {}", p.name, type_hint)
                        }
                    })
                    .collect::<Vec<_>>()
                    .join(", ");

                let ret_type = Self::python_type(return_type);
                let body_str = body.iter()
                    .map(|n| Self::to_python(n, indent + 1))
                    .collect::<Vec<_>>()
                    .join("\n");

                let doc_str = if let Some(d) = doc {
                    format!("\n{}    \"\"\"{}\"\"\"", pad, d)
                } else {
                    String::new()
                };

                format!("{}def {}({}) -> {}:{}\n{}", pad, name, params_str, ret_type, doc_str, body_str)
            }

            LogicNode::FunctionCall { name, args } => {
                let args_str = args.iter()
                    .map(|a| Self::to_python(a, 0))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}{}({})", pad, name, args_str)
            }

            LogicNode::Return(value) => {
                format!("{}return {}", pad, Self::to_python(value, 0))
            }

            LogicNode::DeclareVar { name, value, .. } => {
                format!("{}{} = {}", pad, name, Self::to_python(value, 0))
            }

            LogicNode::AssignVar { name, value } => {
                format!("{}{} = {}", pad, name, Self::to_python(value, 0))
            }

            LogicNode::Variable(name) => name.clone(),

            LogicNode::IntLiteral(n) => format!("{}", n),
            LogicNode::FloatLiteral(n) => format!("{}", n),
            LogicNode::StringLiteral(s) => format!("\"{}\"", s),
            LogicNode::BoolLiteral(b) => if *b { "True".into() } else { "False".into() },
            LogicNode::NoneLiteral => "None".into(),

            LogicNode::ListLiteral(items) => {
                let items_str = items.iter()
                    .map(|i| Self::to_python(i, 0))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("[{}]", items_str)
            }

            LogicNode::BinaryOp { op, left, right } => {
                let op_str = match op {
                    BinaryOperator::Add => "+",
                    BinaryOperator::Subtract => "-",
                    BinaryOperator::Multiply => "*",
                    BinaryOperator::Divide => "/",
                    BinaryOperator::Modulo => "%",
                    BinaryOperator::Power => "**",
                    BinaryOperator::Equal => "==",
                    BinaryOperator::NotEqual => "!=",
                    BinaryOperator::LessThan => "<",
                    BinaryOperator::GreaterThan => ">",
                    BinaryOperator::LessEqual => "<=",
                    BinaryOperator::GreaterEqual => ">=",
                    BinaryOperator::And => "and",
                    BinaryOperator::Or => "or",
                };
                format!("({} {} {})", Self::to_python(left, 0), op_str, Self::to_python(right, 0))
            }

            LogicNode::IfElse { condition, then_body, else_body } => {
                let mut result = format!("{}if {}:\n", pad, Self::to_python(condition, 0));
                for node in then_body {
                    result.push_str(&Self::to_python(node, indent + 1));
                    result.push('\n');
                }
                if let Some(else_nodes) = else_body {
                    result.push_str(&format!("{}else:\n", pad));
                    for node in else_nodes {
                        result.push_str(&Self::to_python(node, indent + 1));
                        result.push('\n');
                    }
                }
                result
            }

            LogicNode::ForLoop { variable, iterable, body } => {
                let mut result = format!("{}for {} in {}:\n", pad, variable, Self::to_python(iterable, 0));
                for node in body {
                    result.push_str(&Self::to_python(node, indent + 1));
                    result.push('\n');
                }
                result
            }

            LogicNode::ForRange { variable, start, end, body } => {
                let mut result = format!("{}for {} in range({}, {}):\n",
                    pad, variable, Self::to_python(start, 0), Self::to_python(end, 0));
                for node in body {
                    result.push_str(&Self::to_python(node, indent + 1));
                    result.push('\n');
                }
                result
            }

            LogicNode::WhileLoop { condition, body } => {
                let mut result = format!("{}while {}:\n", pad, Self::to_python(condition, 0));
                for node in body {
                    result.push_str(&Self::to_python(node, indent + 1));
                    result.push('\n');
                }
                result
            }

            LogicNode::Print(value) => {
                format!("{}print({})", pad, Self::to_python(value, 0))
            }

            LogicNode::IndexAccess { collection, index } => {
                format!("{}[{}]", Self::to_python(collection, 0), Self::to_python(index, 0))
            }

            LogicNode::ListLength(list) => {
                format!("len({})", Self::to_python(list, 0))
            }

            LogicNode::MethodCall { object, method, args } => {
                let args_str = args.iter()
                    .map(|a| Self::to_python(a, 0))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}.{}({})", Self::to_python(object, 0), method, args_str)
            }

            LogicNode::TryCatch { try_body, catch_var, catch_body } => {
                let mut result = format!("{}try:\n", pad);
                for node in try_body {
                    result.push_str(&Self::to_python(node, indent + 1));
                    result.push('\n');
                }
                result.push_str(&format!("{}except Exception as {}:\n", pad, catch_var));
                for node in catch_body {
                    result.push_str(&Self::to_python(node, indent + 1));
                    result.push('\n');
                }
                result
            }

            LogicNode::ToolCall { tool, args } => {
                let tool_name = match tool {
                    ToolType::ReadFile => "tool_read_file",
                    ToolType::WriteFile => "tool_write_file",
                    ToolType::Execute => "tool_execute",
                    ToolType::HttpGet => "tool_http_get",
                    ToolType::HttpPost => "tool_http_post",
                };
                let args_str = args.iter()
                    .map(|a| Self::to_python(a, 0))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}{}({})", pad, tool_name, args_str)
            }

            LogicNode::Comment(text) => format!("{}# {}", pad, text),
            LogicNode::Break => format!("{}break", pad),
            LogicNode::Continue => format!("{}continue", pad),

            _ => format!("{}# TODO: nodo no implementado", pad),
        }
    }

    /// Generar código Rust desde AST
    pub fn to_rust(node: &LogicNode, indent: usize) -> String {
        let pad = "    ".repeat(indent);

        match node {
            LogicNode::Program { body, .. } => {
                let mut result = String::new();
                let has_main = body.iter().any(|n| matches!(n, LogicNode::FunctionDef { name, .. } if name == "main"));

                for n in body {
                    result.push_str(&Self::to_rust(n, 0));
                    result.push_str("\n\n");
                }

                if !has_main {
                    result.push_str("fn main() {\n    // Auto-generated\n}\n");
                }

                result
            }

            LogicNode::FunctionDef { name, params, return_type, body, doc } => {
                let params_str = params.iter()
                    .map(|p| format!("{}: {}", p.name, Self::rust_type(&p.param_type)))
                    .collect::<Vec<_>>()
                    .join(", ");

                let ret = match return_type {
                    DataType::Void => String::new(),
                    t => format!(" -> {}", Self::rust_type(t)),
                };

                let doc_str = if let Some(d) = doc {
                    format!("{}/// {}\n", pad, d)
                } else {
                    String::new()
                };

                let body_str = body.iter()
                    .map(|n| Self::to_rust(n, indent + 1))
                    .collect::<Vec<_>>()
                    .join("\n");

                format!("{}{}fn {}({}){} {{\n{}\n{}}}", pad, doc_str, name, params_str, ret, body_str, pad)
            }

            LogicNode::FunctionCall { name, args } => {
                let args_str = args.iter()
                    .map(|a| Self::to_rust(a, 0))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}{}({})", pad, name, args_str)
            }

            LogicNode::Return(value) => {
                format!("{}return {};", pad, Self::to_rust(value, 0))
            }

            LogicNode::DeclareVar { name, value, mutable, .. } => {
                let mut_keyword = if *mutable { "let mut" } else { "let" };
                format!("{}{} {} = {};", pad, mut_keyword, name, Self::to_rust(value, 0))
            }

            LogicNode::AssignVar { name, value } => {
                format!("{}{} = {};", pad, name, Self::to_rust(value, 0))
            }

            LogicNode::Variable(name) => name.clone(),
            LogicNode::IntLiteral(n) => format!("{}", n),
            LogicNode::FloatLiteral(n) => format!("{:.1}", n),
            LogicNode::StringLiteral(s) => format!("\"{}\"", s),
            LogicNode::BoolLiteral(b) => format!("{}", b),
            LogicNode::NoneLiteral => "None".into(),

            LogicNode::ListLiteral(items) => {
                let items_str = items.iter()
                    .map(|i| Self::to_rust(i, 0))
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("vec![{}]", items_str)
            }

            LogicNode::BinaryOp { op, left, right } => {
                let op_str = match op {
                    BinaryOperator::Add => "+",
                    BinaryOperator::Subtract => "-",
                    BinaryOperator::Multiply => "*",
                    BinaryOperator::Divide => "/",
                    BinaryOperator::Modulo => "%",
                    BinaryOperator::Equal => "==",
                    BinaryOperator::NotEqual => "!=",
                    BinaryOperator::LessThan => "<",
                    BinaryOperator::GreaterThan => ">",
                    BinaryOperator::LessEqual => "<=",
                    BinaryOperator::GreaterEqual => ">=",
                    BinaryOperator::And => "&&",
                    BinaryOperator::Or => "||",
                    BinaryOperator::Power => return format!("{}.pow({})",
                        Self::to_rust(left, 0), Self::to_rust(right, 0)),
                };
                format!("({} {} {})", Self::to_rust(left, 0), op_str, Self::to_rust(right, 0))
            }

            LogicNode::IfElse { condition, then_body, else_body } => {
                let mut result = format!("{}if {} {{\n", pad, Self::to_rust(condition, 0));
                for node in then_body {
                    result.push_str(&Self::to_rust(node, indent + 1));
                    result.push('\n');
                }
                result.push_str(&format!("{}}}", pad));
                if let Some(else_nodes) = else_body {
                    result.push_str(" else {\n");
                    for node in else_nodes {
                        result.push_str(&Self::to_rust(node, indent + 1));
                        result.push('\n');
                    }
                    result.push_str(&format!("{}}}", pad));
                }
                result
            }

            LogicNode::ForRange { variable, start, end, body } => {
                let mut result = format!("{}for {} in {}..{} {{\n",
                    pad, variable, Self::to_rust(start, 0), Self::to_rust(end, 0));
                for node in body {
                    result.push_str(&Self::to_rust(node, indent + 1));
                    result.push('\n');
                }
                result.push_str(&format!("{}}}", pad));
                result
            }

            LogicNode::ForLoop { variable, iterable, body } => {
                let mut result = format!("{}for {} in {} {{\n",
                    pad, variable, Self::to_rust(iterable, 0));
                for node in body {
                    result.push_str(&Self::to_rust(node, indent + 1));
                    result.push('\n');
                }
                result.push_str(&format!("{}}}", pad));
                result
            }

            LogicNode::Print(value) => {
                format!("{}println!(\"{{}}\", {});", pad, Self::to_rust(value, 0))
            }

            LogicNode::StructDef { name, fields } => {
                let fields_str = fields.iter()
                    .map(|f| format!("    pub {}: {},", f.name, Self::rust_type(&f.param_type)))
                    .collect::<Vec<_>>()
                    .join("\n");
                format!("{}pub struct {} {{\n{}\n{}}}", pad, name, fields_str, pad)
            }

            LogicNode::Comment(text) => format!("{}// {}", pad, text),
            LogicNode::Break => format!("{}break;", pad),
            LogicNode::Continue => format!("{}continue;", pad),

            _ => format!("{}// TODO: nodo no implementado", pad),
        }
    }

    fn python_type(dt: &DataType) -> String {
        match dt {
            DataType::Int => "int".into(),
            DataType::Float => "float".into(),
            DataType::String => "str".into(),
            DataType::Bool => "bool".into(),
            DataType::List(inner) => format!("list[{}]", Self::python_type(inner)),
            DataType::Dict(k, v) => format!("dict[{}, {}]", Self::python_type(k), Self::python_type(v)),
            DataType::Optional(inner) => format!("Optional[{}]", Self::python_type(inner)),
            DataType::Void => "None".into(),
            DataType::Any => "Any".into(),
            DataType::Struct(name) => name.clone(),
            DataType::Custom(name) => name.clone(),
        }
    }

    fn rust_type(dt: &DataType) -> String {
        match dt {
            DataType::Int => "i64".into(),
            DataType::Float => "f64".into(),
            DataType::String => "String".into(),
            DataType::Bool => "bool".into(),
            DataType::List(inner) => format!("Vec<{}>", Self::rust_type(inner)),
            DataType::Dict(k, v) => format!("HashMap<{}, {}>", Self::rust_type(k), Self::rust_type(v)),
            DataType::Optional(inner) => format!("Option<{}>", Self::rust_type(inner)),
            DataType::Void => "()".into(),
            DataType::Any => "Box<dyn std::any::Any>".into(),
            DataType::Struct(name) => name.clone(),
            DataType::Custom(name) => name.clone(),
        }
    }
}
