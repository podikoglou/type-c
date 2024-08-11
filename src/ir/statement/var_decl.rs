use crate::ir::{expression::Expression, types::Type};

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub name: String,
    pub var_type: Type,
    pub initializer: Option<Expression>,
}
