pub mod expr;
pub mod statement;
pub mod types;
pub mod visitor;

use anyhow::Result;

pub trait AstToIR<T> {
    fn to_ir(&self) -> Result<T>;
}
