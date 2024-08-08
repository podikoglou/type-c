use method::Method;

pub mod expression;
pub mod method;
pub mod statement;
pub mod types;

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
