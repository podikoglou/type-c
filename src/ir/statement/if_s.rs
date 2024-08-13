
use super::Statement;
use crate::ir::expression::Expression;

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub test: Expression,

    pub cons: Box<Statement>,
    pub alt: Option<Box<Statement>>,
}
