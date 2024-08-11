use super::Expression;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub struct Paren(pub Rc<Expression>);
