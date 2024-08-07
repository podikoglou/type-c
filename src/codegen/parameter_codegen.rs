use super::ToC;
use crate::{ir::MethodParameter, writer::CodeWriter};
use anyhow::Result;

impl ToC for MethodParameter {
    /// Converts a [[MethodParameter]] into a C method parameter.
    fn to_c(&self) -> Result<CodeWriter> {
        let mut writer = CodeWriter::default();

        writer.concat(&self._type.to_c().unwrap());
        writer.write(" ");
        writer.write(self.name.as_str());

        Ok(writer)
    }
}
