use super::{expression::Expression, types::Type};

#[derive(Debug, Clone)]
pub enum Statement {
    VariableDeclaration(VariableDeclaration),
    Assignment(Assignment),

    // If(IfStatement),
    // WhileLoop(WhileLoop),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
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

#[derive(Debug, Clone)]
pub struct ReturnStatement(pub Expression);

#[derive(Debug, Clone)]
pub struct ExpressionStatement(pub Expression);
