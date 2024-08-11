use crate::{def_codegen, ir::statement::if_s::IfStatement};

def_codegen!(IfStatement, |statement| {
    let mut buffer = CodeBuffer::default();

    buffer.write("if(");
    buffer.write(statement.test.to_c()?);
    buffer.write(")");

    buffer.concat(&statement.cons.to_c()?);

    Ok(buffer)
});
