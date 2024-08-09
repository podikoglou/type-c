use crate::{def_codegen, ir::method::Method};
use anyhow::Result;

def_codegen!(Method, |method: &Method| {
    let mut writer = CodeWriter::default();

    let return_type = &method.return_type.to_c()?;

    let params: Vec<String> = method
        .parameters
        .iter()
        .map(ToC::to_c)
        .map(Result::unwrap)
        .map(CodeWriter::into)
        .collect();

    // return type
    writer.concat(return_type);
    writer.write(" ");

    // method name
    writer.write(method.name.as_str());

    // parameters list
    writer.write("(");
    writer.write(params.join(", ").as_str());
    writer.write(")");

    // body
    writer.write_line("{");

    for statement in &method.body {
        writer.concat(&statement.to_c()?);
    }

    writer.write_line("}");

    Ok(writer)
});
