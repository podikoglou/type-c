use crate::{def_codegen, ir::statement::block::BlockStatement};

def_codegen!(BlockStatement, |block| {
    let mut buffer = CodeBuffer::new();

    buffer.write_line("{");

    for statement in &block.0 {
        buffer.concat(&statement.to_c()?);
    }

    buffer.write_line("}");

    Ok(buffer)
});
