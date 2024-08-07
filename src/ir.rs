use std::rc::Rc;

#[derive(Debug, Default, Clone)]
pub struct Program {
    pub imports: Vec<Import>,
    pub methods: Vec<Method>,
}

#[derive(Debug, Default, Clone)]
pub struct Import {
    pub module: String,
    pub items: Vec<String>,
}

#[derive(Debug, Default, Clone)]
pub struct Method {
    pub name: String,
    pub return_type: Type,
    pub parameters: Vec<MethodParameter>,
}

#[derive(Debug, Default, Clone)]
pub struct MethodParameter {
    pub name: String,
    pub _type: Type,
}

#[derive(Debug, Default, Clone)]
pub enum Type {
    #[default]
    Void,
    Char,
    Number,
    Boolean,
    Pointer(Rc<Type>),
}

// #[derive(Debug)]
// pub enum Literal {
//     String(String),
//     Number(i64),
//     Boolean(bool),
//     Void,
// }
