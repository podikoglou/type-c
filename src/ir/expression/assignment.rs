use std::rc::Rc;

use super::Expression;

#[derive(Debug, Clone)]
pub struct Assignment {
    pub left: Rc<Expression>,
    pub right: Rc<Expression>,
}
