use std::rc::Rc;

use super::Expression;

#[derive(Debug, Clone)]
pub enum BinaryOperation {
    And(Rc<Expression>, Rc<Expression>),
    Or(Rc<Expression>, Rc<Expression>),
}
