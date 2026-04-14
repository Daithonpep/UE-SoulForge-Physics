use serde::{Deserialize, Serialize};

/// Nodo del Árbol de Sintaxis Abstracta de Daithon.
/// Daithon piensa en ESTOS nodos, no en texto.
/// Cada nodo representa un CONCEPTO lógico, no sintaxis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LogicNode {
    // ═══ PROGRAMA ═══
    Program {
        name: String,
        body: Vec<LogicNode>,
    },

    // ═══ FUNCIONES ═══
    FunctionDef {
        name: String,
        params: Vec<Parameter>,
        return_type: DataType,
        body: Vec<LogicNode>,
        doc: Option<String>,
    },

    FunctionCall {
        name: String,
        args: Vec<LogicNode>,
    },

    Return(Box<LogicNode>),

    // ═══ VARIABLES ═══
    DeclareVar {
        name: String,
        var_type: DataType,
        value: Box<LogicNode>,
        mutable: bool,
    },

    AssignVar {
        name: String,
        value: Box<LogicNode>,
    },

    Variable(String),

    // ═══ VALORES ═══
    IntLiteral(i64),
    FloatLiteral(f64),
    StringLiteral(String),
    BoolLiteral(bool),
    ListLiteral(Vec<LogicNode>),
    NoneLiteral,

    // ═══ OPERACIONES ═══
    BinaryOp {
        op: BinaryOperator,
        left: Box<LogicNode>,
        right: Box<LogicNode>,
    },

    UnaryOp {
        op: UnaryOperator,
        operand: Box<LogicNode>,
    },

    // ═══ CONTROL DE FLUJO ═══
    IfElse {
        condition: Box<LogicNode>,
        then_body: Vec<LogicNode>,
        else_body: Option<Vec<LogicNode>>,
    },

    #[allow(dead_code)]
    ForLoop {
        variable: String,
        iterable: Box<LogicNode>,
        body: Vec<LogicNode>,
    },

    ForRange {
        variable: String,
        start: Box<LogicNode>,
        end: Box<LogicNode>,
        body: Vec<LogicNode>,
    },

    #[allow(dead_code)]
    WhileLoop {
        condition: Box<LogicNode>,
        body: Vec<LogicNode>,
    },

    #[allow(dead_code)]
    Break,
    #[allow(dead_code)]
    Continue,

    // ═══ ESTRUCTURAS ═══
    #[allow(dead_code)]
    StructDef {
        name: String,
        fields: Vec<Parameter>,
    },

    #[allow(dead_code)]
    FieldAccess {
        object: Box<LogicNode>,
        field: String,
    },

    MethodCall {
        object: Box<LogicNode>,
        method: String,
        args: Vec<LogicNode>,
    },

    // ═══ COLECCIONES ═══
    IndexAccess {
        collection: Box<LogicNode>,
        index: Box<LogicNode>,
    },

    #[allow(dead_code)]
    ListAppend {
        list: Box<LogicNode>,
        value: Box<LogicNode>,
    },

    ListLength(Box<LogicNode>),

    #[allow(dead_code)]
    DictLiteral(Vec<(LogicNode, LogicNode)>),

    #[allow(dead_code)]
    DictAccess {
        dict: Box<LogicNode>,
        key: Box<LogicNode>,
    },

    // ═══ MANEJO DE ERRORES ═══
    #[allow(dead_code)]
    TryCatch {
        try_body: Vec<LogicNode>,
        catch_var: String,
        catch_body: Vec<LogicNode>,
    },

    // ═══ IMPRESIÓN / DEBUG ═══
    Print(Box<LogicNode>),

    #[allow(dead_code)]
    FormatString {
        template: String,
        values: Vec<LogicNode>,
    },

    // ═══ HERRAMIENTAS (Tool Use) ═══
    #[allow(dead_code)]
    ToolCall {
        tool: ToolType,
        args: Vec<LogicNode>,
    },

    // ═══ COMENTARIO (intención) ═══
    Comment(String),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Parameter {
    pub name: String,
    pub param_type: DataType,
    pub default: Option<Box<LogicNode>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DataType {
    Int,
    Float,
    String,
    Bool,
    List(Box<DataType>),
    #[allow(dead_code)]
    Dict(Box<DataType>, Box<DataType>),
    Optional(Box<DataType>),
    Struct(String),
    Custom(String),
    Any,
    Void,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BinaryOperator {
    Add, Subtract, Multiply, Divide, Modulo, Power,
    Equal, NotEqual, LessThan, GreaterThan, LessEqual, GreaterEqual,
    And, Or,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UnaryOperator {
    #[allow(dead_code)]
    Negate, 
    #[allow(dead_code)]
    Not,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ToolType {
    ReadFile,
    WriteFile,
    Execute,
    HttpGet,
    HttpPost,
}
