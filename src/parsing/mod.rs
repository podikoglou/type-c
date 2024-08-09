pub mod expr;
pub mod statement;
pub mod types;
pub mod visitor;

use anyhow::Result;

pub trait AstToIR<T> {
    fn to_ir(&self) -> Result<T>;
}

#[macro_export]
macro_rules! def_parser {
    ($ast_type:ty, $ir_type:ty, $to_ir_fn:expr) => {
        use crate::parsing::AstToIR;

        impl AstToIR<$ir_type> for $ast_type {
            fn to_ir(&self) -> anyhow::Result<$ir_type> {
                ($to_ir_fn)(self)
            }
        }
    };
}
