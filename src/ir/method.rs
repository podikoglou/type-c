use super::{statement::Statement, types::Type};

#[derive(Debug, Default, Clone)]
pub struct Method {
    pub name: String,
    pub return_type: Type,
    pub parameters: Vec<MethodParameter>,
    pub body: Vec<Statement>,
}

#[derive(Debug, Default, Clone)]
pub struct MethodParameter {
    pub name: String,
    pub _type: Type,
}
