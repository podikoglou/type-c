use crate::{def_codegen, ir::statement::VariableDeclaration};

def_codegen!(VariableDeclaration, |declaration| {
    let mut buffer = CodeBuffer::default();
    let type_name: String = declaration.var_type.to_c()?.into();

    buffer.write(type_name.as_str());
    buffer.write(" ");
    buffer.write(declaration.name.as_str());

    if let Some(initializer) = &declaration.initializer {
        let initializer: String = initializer.to_c()?.into();

        buffer.write(" = ");
        buffer.write(initializer.as_str());
    }

    buffer.write(";");

    Ok(buffer)
});
