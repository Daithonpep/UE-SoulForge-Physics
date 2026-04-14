use std::fmt;

/// Sistema de Álgebra Computacional (Computer Algebra System)
pub struct CASEngine {
    symbol_table: std::collections::HashMap<String, Expression>,
}

/// Expresión simbólica
#[derive(Debug, Clone, PartialEq)]
pub enum Expression {
    Number(f64),
    Variable(String),
    Add(Box<Expression>, Box<Expression>),
    Subtract(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Power(Box<Expression>, Box<Expression>),
    Function {
        name: String,
        args: Vec<Expression>,
    },
}

impl Expression {
    /// Simplificar expresión
    pub fn simplify(&self) -> Expression {
        match self {
            Expression::Add(a, b) => {
                let a_simp = a.simplify();
                let b_simp = b.simplify();

                match (&a_simp, &b_simp) {
                    (Expression::Number(x), Expression::Number(y)) => Expression::Number(x + y),
                    (Expression::Number(0.0), expr) | (expr, Expression::Number(0.0)) => expr.clone(),
                    _ => Expression::Add(Box::new(a_simp), Box::new(b_simp)),
                }
            }
            Expression::Multiply(a, b) => {
                let a_simp = a.simplify();
                let b_simp = b.simplify();

                match (&a_simp, &b_simp) {
                    (Expression::Number(x), Expression::Number(y)) => Expression::Number(x * y),
                    (Expression::Number(0.0), _) | (_, Expression::Number(0.0)) => Expression::Number(0.0),
                    (Expression::Number(1.0), expr) | (expr, Expression::Number(1.0)) => expr.clone(),
                    _ => Expression::Multiply(Box::new(a_simp), Box::new(b_simp)),
                }
            }
            Expression::Power(base, exp) => {
                let base_simp = base.simplify();
                let exp_simp = exp.simplify();

                match (&base_simp, &exp_simp) {
                    (Expression::Number(b), Expression::Number(e)) => Expression::Number(b.powf(*e)),
                    (_, Expression::Number(0.0)) => Expression::Number(1.0),
                    (expr, Expression::Number(1.0)) => expr.clone(),
                    _ => Expression::Power(Box::new(base_simp), Box::new(exp_simp)),
                }
            }
            _ => self.clone(),
        }
    }

    /// Derivar respecto a una variable
    pub fn derivative(&self, var: &str) -> Expression {
        match self {
            Expression::Number(_) => Expression::Number(0.0),
            Expression::Variable(v) if v == var => Expression::Number(1.0),
            Expression::Variable(_) => Expression::Number(0.0),
            
            Expression::Add(a, b) => {
                Expression::Add(
                    Box::new(a.derivative(var)),
                    Box::new(b.derivative(var)),
                ).simplify()
            }
            
            Expression::Multiply(a, b) => {
                // Regla del producto: (fg)' = f'g + fg'
                Expression::Add(
                    Box::new(Expression::Multiply(
                        Box::new(a.derivative(var)),
                        b.clone(),
                    )),
                    Box::new(Expression::Multiply(
                        a.clone(),
                        Box::new(b.derivative(var)),
                    )),
                ).simplify()
            }
            
            Expression::Power(base, exp) => {
                // Regla de potencias: d/dx(x^n) = n*x^(n-1)
                if let Expression::Variable(v) = base.as_ref() {
                    if v == var {
                        if let Expression::Number(n) = exp.as_ref() {
                            return Expression::Multiply(
                                exp.clone(),
                                Box::new(Expression::Power(
                                    base.clone(),
                                    Box::new(Expression::Number(n - 1.0)),
                                )),
                            ).simplify();
                        }
                    }
                }
                
                // Caso general: usa logaritmos
                self.clone() // Simplificado
            }
            
            Expression::Function { name, args } => {
                match name.as_str() {
                    "sin" if args.len() == 1 => {
                        // d/dx(sin(u)) = cos(u) * du/dx
                        Expression::Multiply(
                            Box::new(Expression::Function {
                                name: "cos".into(),
                                args: args.clone(),
                            }),
                            Box::new(args[0].derivative(var)),
                        ).simplify()
                    }
                    "cos" if args.len() == 1 => {
                        // d/dx(cos(u)) = -sin(u) * du/dx
                        Expression::Multiply(
                            Box::new(Expression::Number(-1.0)),
                            Box::new(Expression::Multiply(
                                Box::new(Expression::Function {
                                    name: "sin".into(),
                                    args: args.clone(),
                                }),
                                Box::new(args[0].derivative(var)),
                            )),
                        ).simplify()
                    }
                    _ => self.clone(), // Función no implementada
                }
            }
            
            _ => self.clone(),
        }
    }

    /// Evaluar la expresión con valores asignados
    pub fn evaluate(&self, vars: &std::collections::HashMap<String, f64>) -> Result<f64, String> {
        match self {
            Expression::Number(n) => Ok(*n),
            Expression::Variable(v) => {
                vars.get(v)
                    .copied()
                    .ok_or_else(|| format!("Variable '{}' no definida", v))
            }
            Expression::Add(a, b) => Ok(a.evaluate(vars)? + b.evaluate(vars)?),
            Expression::Subtract(a, b) => Ok(a.evaluate(vars)? - b.evaluate(vars)?),
            Expression::Multiply(a, b) => Ok(a.evaluate(vars)? * b.evaluate(vars)?),
            Expression::Divide(a, b) => {
                let divisor = b.evaluate(vars)?;
                if divisor.abs() < 1e-10 {
                    Err("División por cero".into())
                } else {
                    Ok(a.evaluate(vars)? / divisor)
                }
            }
            Expression::Power(base, exp) => Ok(base.evaluate(vars)?.powf(exp.evaluate(vars)?)),
            Expression::Function { name, args } => {
                match name.as_str() {
                    "sin" if args.len() == 1 => Ok(args[0].evaluate(vars)?.sin()),
                    "cos" if args.len() == 1 => Ok(args[0].evaluate(vars)?.cos()),
                    "sqrt" if args.len() == 1 => Ok(args[0].evaluate(vars)?.sqrt()),
                    "ln" if args.len() == 1 => Ok(args[0].evaluate(vars)?.ln()),
                    "exp" if args.len() == 1 => Ok(args[0].evaluate(vars)?.exp()),
                    _ => Err(format!("Función '{}' no implementada", name)),
                }
            }
        }
    }
}

impl fmt::Display for Expression {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Expression::Number(n) => write!(f, "{}", n),
            Expression::Variable(v) => write!(f, "{}", v),
            Expression::Add(a, b) => write!(f, "({} + {})", a, b),
            Expression::Subtract(a, b) => write!(f, "({} - {})", a, b),
            Expression::Multiply(a, b) => write!(f, "{}·{}", a, b),
            Expression::Divide(a, b) => write!(f, "{}/{}", a, b),
            Expression::Power(base, exp) => write!(f, "{}^{}", base, exp),
            Expression::Function { name, args } => {
                write!(f, "{}(", name)?;
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{}", arg)?;
                }
                write!(f, ")")
            }
        }
    }
}

impl CASEngine {
    pub fn new() -> Self {
        Self {
            symbol_table: std::collections::HashMap::new(),
        }
    }

    pub fn define(&mut self, var: &str, expr: Expression) {
        self.symbol_table.insert(var.to_string(), expr);
    }

    pub fn get(&self, var: &str) -> Option<&Expression> {
        self.symbol_table.get(var)
    }

    /// Parser simple de expresiones
    pub fn parse(&self, input: &str) -> Result<Expression, String> {
        // Simplificado: solo maneja casos básicos
        // En producción, usar un parser completo (pest, nom, etc.)
        
        if let Ok(num) = input.parse::<f64>() {
            return Ok(Expression::Number(num));
        }

        if input.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Ok(Expression::Variable(input.to_string()));
        }

        // Detectar x^2
        if let Some((base, exp)) = input.split_once('^') {
            return Ok(Expression::Power(
                Box::new(self.parse(base.trim())?),
                Box::new(self.parse(exp.trim())?),
            ));
        }

        // Detectar 2*x
        if let Some((left, right)) = input.split_once('*') {
            return Ok(Expression::Multiply(
                Box::new(self.parse(left.trim())?),
                Box::new(self.parse(right.trim())?),
            ));
        }

        Err(format!("No puedo parsear: {}", input))
    }
}
