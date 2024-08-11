use std::rc::Rc;

use super::Statement;
use crate::ir::expression::Expression;

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub test: Expression,

    pub cons: Rc<Statement>,
    pub alt: Rc<Option<Statement>>,
}
