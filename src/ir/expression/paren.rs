use super::Expression;

#[derive(Debug, Clone)]
pub struct Paren(pub Box<Expression>);
