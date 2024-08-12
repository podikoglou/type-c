use std::rc::Rc;

use super::Statement;
use crate::ir::expression::Expression;

#[derive(Debug, Clone)]
pub struct ForStatement {
    pub init: Rc<Statement>,
    pub test: Expression,
    pub update: Rc<Statement>,
    pub body: Rc<Statement>,
}
