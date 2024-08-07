use super::ToC;
use crate::{ir::Import, writer::CodeWriter};
use anyhow::Result;

impl ToC for Import {
    /// Converts an [[Import]] into a C include statement.
    ///
    /// For now:
    /// 1. All imports are <> imports (opposed to "", so something like "stdio.h")
    /// 2. The imported items ([[Import::items]]) are not used; we only
    ///    use the [[Import::module]] field
    fn to_c(&self) -> Result<CodeWriter> {
        let mut writer = CodeWriter::default();

        writer.write("#include <");
        writer.write(self.module.as_str());
        writer.write(">");

        Ok(writer)
    }
}
