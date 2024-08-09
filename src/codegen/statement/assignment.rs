use crate::{def_codegen, ir::statement::Assignment};

def_codegen!(Assignment, |assignment| {
    let mut buffer = CodeBuffer::default();

    let target: String = assignment.target.to_c()?.into();
    let value: String = assignment.value.to_c()?.into();

    buffer.write(target.as_str());
    buffer.write(" = ");
    buffer.write(value.as_str());
    buffer.write(";");

    Ok(buffer)
});
