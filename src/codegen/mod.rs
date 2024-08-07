pub mod import_codegen;
pub mod method_codegen;
pub mod parameter_codegen;
pub mod type_codegen;

use crate::writer::CodeWriter;
use anyhow::Result;

trait ToC {
    fn to_c(&self) -> Result<CodeWriter>;
}
