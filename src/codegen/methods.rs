use anyhow::Result;

use crate::{
    ir::{Method, MethodParameter},
    writer::CodeWriter,
};

use super::types::type_to_c;

/// Converts a [[Method]] into a C method.
pub fn method_to_c(method: &Method) -> Result<CodeWriter> {
    let mut writer = CodeWriter::default();

    let return_type = type_to_c(&method.return_type);

    let params: Vec<String> = method.parameters.iter().map(parameter_to_c).collect();

    writer.write(format!(
        "{} {}({})",
        return_type,
        method.name,
        params.join(", ")
    ));

    writer.write("{".to_string());

    // TODO: Statements!

    writer.write("}".to_string());

    Ok(writer)
}

/// Converts a [[MethodParameter]] into a C method parameter.
pub fn parameter_to_c(param: &MethodParameter) -> String {
    format!("{} {}", type_to_c(&param._type), param.name)
}
