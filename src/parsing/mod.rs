pub mod expr;
pub mod statement;
pub mod types;
pub mod visitor;

use anyhow::Result;

pub trait ToIR<T> {
    fn to_ir(&self) -> Result<T>;
}

#[macro_export]
macro_rules! def_parser {
    ($ast_type:ty, $ir_type:ty, $to_ir_fn:expr) => {
        use crate::parsing::ToIR;

        impl ToIR<$ir_type> for $ast_type {
            fn to_ir(&self) -> anyhow::Result<$ir_type> {
                ($to_ir_fn)(self)
            }
        }
    };
}
