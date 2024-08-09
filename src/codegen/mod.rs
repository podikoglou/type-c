pub mod expression;
pub mod import;
pub mod literal;
pub mod method;
pub mod parameter;
pub mod statement;
pub mod types;

use crate::buffer::CodeBuffer;
use anyhow::Result;

pub trait ToC {
    fn to_c(&self) -> Result<CodeBuffer>;
}

#[macro_export]
macro_rules! def_codegen {
    ($ir_type:ty, |$param:ident| $body:expr) => {
        use crate::CodeBuffer;

        impl crate::codegen::ToC for $ir_type {
            fn to_c(&self) -> anyhow::Result<CodeBuffer> {
                (|$param: &$ir_type| $body)(self)
            }
        }
    };
}
