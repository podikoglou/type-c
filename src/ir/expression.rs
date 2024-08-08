#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Variable(String),
    // BinaryOperation(Box<BinaryOperation>),
    // FunctionCall(FunctionCall),
    // UnaryOperation(Box<UnaryOperation>),
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(i64),
    Boolean(bool),
    Void,
}
