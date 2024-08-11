use super::Expression;

#[derive(Debug, Clone)]
pub struct MethodCall {
    pub name: String,
    pub arguments: Vec<Expression>,
}
