use super::Expression;

#[derive(Debug, Clone)]
pub enum Literal {
    String(String),
    Number(f64),
    Boolean(bool),
    Array(Vec<Expression>),
    Void,
}
