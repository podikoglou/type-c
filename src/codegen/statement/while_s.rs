use crate::{def_codegen, ir::statement::while_s::WhileStatement};

def_codegen!(WhileStatement, |statement| {
    let mut buffer = CodeBuffer::default();

    buffer.write("while(");
    buffer.write(statement.test.to_c()?);
    buffer.write(")");

    buffer.concat(&statement.body.to_c()?);

    Ok(buffer)
});
