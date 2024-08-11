use crate::{def_codegen, ir::statement::expr::ExpressionStatement};

def_codegen!(ExpressionStatement, |statement| {
    let mut buffer = CodeBuffer::default();

    buffer.write(statement.0.to_c()?);
    buffer.write(";");

    Ok(buffer)
});
