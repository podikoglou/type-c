pub mod expression_codegen;
pub mod import_codegen;
pub mod literal_codegen;
pub mod method_codegen;
pub mod parameter_codegen;
pub mod statement_codegen;
pub mod type_codegen;

use crate::writer::CodeWriter;
use anyhow::Result;

pub trait ToC {
    fn to_c(&self) -> Result<CodeWriter>;
}

#[macro_export]
macro_rules! def_codegen {
    ($ir_type:ty, $to_ir_fn:expr) => {
        use crate::codegen::{CodeWriter, ToC};

        impl ToC for $ir_type {
            fn to_c(&self) -> anyhow::Result<CodeWriter> {
                ($to_ir_fn)(self)
            }
        }
    };
}
