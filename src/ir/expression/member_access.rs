use std::rc::Rc;

use super::Expression;

#[derive(Debug, Clone)]
pub struct MemberAccess {
    pub object: Rc<Expression>,

    // this doesn't have to be a number index (such as args[0]),
    // it can also be something like foo['bar'] in a JSON object
    // (which btw are not supported (yet))
    pub index: Rc<Expression>,
}
