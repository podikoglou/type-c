use std::rc::Rc;

use super::Expression;

#[derive(Debug, Clone)]
pub enum BinaryOperation {
    And(Rc<Expression>, Rc<Expression>),
    Or(Rc<Expression>, Rc<Expression>),
    Eq(Rc<Expression>, Rc<Expression>),
    NotEq(Rc<Expression>, Rc<Expression>),
    Gt(Rc<Expression>, Rc<Expression>),
    Lt(Rc<Expression>, Rc<Expression>),
    GtEq(Rc<Expression>, Rc<Expression>),
    LtEq(Rc<Expression>, Rc<Expression>),
}
