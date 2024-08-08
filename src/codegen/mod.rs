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
