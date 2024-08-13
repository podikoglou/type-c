
use super::Expression;

#[derive(Debug, Clone)]
pub struct Assignment {
    pub left: Box<Expression>,
    pub right: Box<Expression>,
}
