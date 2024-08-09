use super::ToC;
use crate::{def_codegen, ir::method::Method};
use anyhow::Result;

def_codegen!(Method, |method| {
    let mut buffer = CodeBuffer::default();

    let name = &method.name;

    let return_type = method.return_type.to_c()?;

    let params: Vec<String> = method
        .parameters
        .iter()
        .map(ToC::to_c)
        .map(Result::unwrap)
        .map(CodeBuffer::into)
        .collect();

    // type signature
    buffer.write(format!(
        "{} {}({})",
        return_type.to_string(),
        name,
        params.join(", ")
    ));

    // body
    buffer.write_line("{");

    for statement in &method.body {
        buffer.concat(&statement.to_c()?);
    }

    buffer.write_line("}");

    Ok(buffer)
});
