use super::Statement;

#[derive(Debug, Clone)]
pub struct BlockStatement(pub Vec<Statement>);
