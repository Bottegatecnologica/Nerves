#[derive(Debug, Clone)]
pub enum Type {
    Integer,
    Float,
    String,
    Boolean,
    Void,
    Custom(String),
}

#[derive(Debug, Clone)]
pub struct Variable {
    pub name: String,
    pub var_type: Type,
}

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Variable(String),
    BinaryOperation {
        left: Box<Expression>,
        operator: BinaryOperator,
        right: Box<Expression>,
    },
    FunctionCall {
        name: String,
        arguments: Vec<Expression>,
    },
}

#[derive(Debug, Clone)]
pub enum Literal {
    Integer(i64),
    Float(f64),
    String(String),
    Boolean(bool),
}

#[derive(Debug, Clone)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    NotEqual,
    LessThan,
    GreaterThan,
}

#[derive(Debug, Clone)]
pub struct Ritual {
    pub name: String,
    pub parameters: Vec<Variable>,
    pub return_type: Type,
    pub body: Vec<Statement>,
}

#[derive(Debug, Clone)]
pub enum Statement {
    VariableDeclaration {
        variable: Variable,
        initializer: Option<Expression>,
    },
    Assignment {
        name: String,
        value: Expression,
    },
    RitualCall {
        name: String,
        arguments: Vec<Expression>,
    },
    Conditional {
        condition: Expression,
        true_branch: Vec<Statement>,
        false_branch: Option<Vec<Statement>>,
    },
    Cycle {
        condition: Option<Expression>,
        body: Vec<Statement>,
    },
    Return(Option<Expression>),
}

#[derive(Debug, Clone)]
pub struct Being {
    pub name: String,
    pub rituals: Vec<Ritual>,
    pub variables: Vec<Variable>,
}

#[derive(Debug, Clone)]
pub struct Realm {
    pub name: String,
    pub beings: Vec<Being>,
}

#[derive(Debug, Clone)]
pub struct Program {
    pub realms: Vec<Realm>,
}