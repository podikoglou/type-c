pub mod expression;
pub mod import;
pub mod literal;
pub mod method;
pub mod parameter;
pub mod statement;
pub mod types;

use crate::writer::CodeWriter;
use anyhow::Result;

pub trait ToC {
    fn to_c(&self) -> Result<CodeWriter>;
}

#[macro_export]
macro_rules! def_codegen {
    ($ir_type:ty, |$param:ident| $body:expr) => {
        use crate::codegen::{CodeWriter, ToC};

        impl ToC for $ir_type {
            fn to_c(&self) -> anyhow::Result<CodeWriter> {
                (|$param: &$ir_type| $body)(self)
            }
        }
    };
}
