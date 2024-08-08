use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub enum Type {
    #[default]
    Void,
    Char,
    Number,
    Boolean,
    Pointer(Rc<Type>),
}
