#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Variable(String),
    MethodCall(MethodCall),
    // BinaryOperation(Box<BinaryOperation>),
    // UnaryOperation(Box<UnaryOperation>),
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    Void,
}

#[derive(Debug, Clone)]
pub struct MethodCall {
    pub name: String,
    pub arguments: Vec<Expression>,
}
