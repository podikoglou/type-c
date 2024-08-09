use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Variable(String),
    MethodCall(MethodCall),
    MemberAccess(MemberAccess),
    // BinaryOperation(Box<BinaryOperation>),
    // UnaryOperation(Box<UnaryOperation>),
}

#[derive(Debug, Clone)]
pub struct MethodCall {
    pub name: String,
    pub arguments: Vec<Expression>,
}

#[derive(Debug, Clone)]
pub struct MemberAccess {
    pub object: Rc<Expression>,

    // this doesn't have to be a number index (such as args[0]),
    // it can also be something like foo['bar'] in a JSON object
    // (which btw are not supported (yet))
    pub index: Rc<Expression>,
}

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<Expression>),
    Void,
}
