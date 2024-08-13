
use super::Statement;
use crate::ir::expression::Expression;

#[derive(Debug, Clone)]
pub struct ForStatement {
    pub init: Box<Statement>,
    pub test: Expression,
    pub update: Box<Statement>,
    pub body: Box<Statement>,
}
