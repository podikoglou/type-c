use std::rc::Rc;

#[derive(Debug)]
pub struct Import {
    pub module: String,
    pub items: Vec<String>,
}

#[derive(Debug)]
pub struct Method {
    pub name: String,
    pub return_type: Type,
    pub parameters: Vec<MethodParameter>,
}

#[derive(Debug)]
pub struct MethodParameter {
    pub name: String,
    pub _type: Type,
}

#[derive(Debug)]
pub enum Type {
    Char,
    Number,
    Boolean,
    Void,
    Pointer(Rc<Type>),
    Array(Rc<Type>),
}

// #[derive(Debug)]
// pub enum Literal {
//     String(String),
//     Number(i64),
//     Boolean(bool),
//     Void,
// }
