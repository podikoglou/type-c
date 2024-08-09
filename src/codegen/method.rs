use super::ToC;
use crate::{def_codegen, ir::method::Method};
use anyhow::Result;

def_codegen!(Method, |method| {
    let mut buffer = CodeBuffer::default();

    let return_type = method.return_type.to_c()?;

    let params: Vec<String> = method
        .parameters
        .iter()
        .map(ToC::to_c)
        .map(Result::unwrap)
        .map(CodeBuffer::into)
        .collect();

    // return type
    buffer.write(return_type);
    buffer.write(" ");

    // method name
    buffer.write(method.name.as_str());

    // parameters list
    buffer.write("(");
    buffer.write(params.join(", ").as_str());
    buffer.write(")");

    // body
    buffer.write_line("{");

    for statement in &method.body {
        buffer.concat(&statement.to_c()?);
    }

    buffer.write_line("}");

    Ok(buffer)
});
