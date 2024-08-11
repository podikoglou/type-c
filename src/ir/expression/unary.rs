use std::rc::Rc;

use super::Expression;

#[derive(Debug, Clone)]
pub enum UnaryOperation {
    Minus(Rc<Expression>),
    Plus(Rc<Expression>),
    Bang(Rc<Expression>),
}
