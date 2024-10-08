use crate::{def_codegen, ir::Import};

// For now:
// 1. All imports are <> imports (opposed to "", so something like "stdio.h")
// 2. The imported items ([[Import::items]]) are not used; we only
//    use the [[Import::module]] field
def_codegen!(Import, |import| {
    let mut buffer = CodeBuffer::default();

    buffer.write("#include <");
    buffer.write(import.module.as_str());
    buffer.write(">");

    Ok(buffer)
});
