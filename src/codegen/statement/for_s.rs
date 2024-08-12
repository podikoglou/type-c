use crate::{def_codegen, ir::statement::for_s::ForStatement};

def_codegen!(ForStatement, |statement| {
    let mut buffer = CodeBuffer::default();

    buffer.write("for(");

    buffer.write(statement.init.to_c()?);
    // buffer.write(";");

    buffer.write(statement.test.to_c()?);
    buffer.write(";");

    // HACK: I hate this. We need to architect the way semicolons are handled better.
    buffer.write(statement.update.to_c()?.to_string().trim_end_matches(";"));

    buffer.write(")");

    buffer.concat(&statement.body.to_c()?);

    Ok(buffer)
});
