use std::rc::Rc;

use super::Statement;
use crate::ir::expression::Expression;

#[derive(Debug, Clone)]
pub struct WhileStatement {
    pub test: Expression,
    pub body: Rc<Statement>,
}
