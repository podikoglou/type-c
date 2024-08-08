use super::{expression::Expression, types::Type};

#[derive(Debug, Clone)]
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    Assignment(Assignment),

    // IfStatement(IfStatement),
    // WhileLoop(WhileLoop),
    ReturnStatement(Expression),

    ExpressionStatement(Expression),
}

#[derive(Debug, Clone)]
pub struct VariableDeclaration {
    pub name: String,
    pub var_type: Type,
    pub initializer: Option<Expression>,
}

#[derive(Debug, Clone)]
pub struct Assignment {
    pub target: Expression,
    pub value: Expression,
}
