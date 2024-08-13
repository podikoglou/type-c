
use super::Expression;

#[derive(Debug, Clone)]
pub enum UnaryOperation {
    Minus(Box<Expression>),
    Plus(Box<Expression>),
    Bang(Box<Expression>),
}
