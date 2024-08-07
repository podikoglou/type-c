use anyhow::Result;

use crate::{ir::Import, writer::CodeWriter};

/// Converts [[Import]]s into C include statements.
///
/// For now:
/// 1. All imports are <> imports (opposed to "", so something like "stdio.h")
/// 2. The imported items ([[Import::items]]) are not used; we only
///    use the [[Import::module]] field
pub fn imports_to_includes(imports: &Vec<Import>) -> Result<CodeWriter> {
    let mut writer = CodeWriter::default();

    for import in imports {
        writer.write(format!("#include <{}>", import.module));
    }

    Ok(writer)
}
