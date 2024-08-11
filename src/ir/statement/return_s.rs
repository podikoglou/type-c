use crate::ir::expression::Expression;

#[derive(Debug, Clone)]
pub struct ReturnStatement(pub Expression);
