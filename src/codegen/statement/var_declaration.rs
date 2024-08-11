use crate::{
    def_codegen,
    ir::{statement::var_decl::VariableDeclaration, types::Type},
};

def_codegen!(VariableDeclaration, |declaration| {
    let mut buffer = CodeBuffer::default();
    let type_name: String = declaration.var_type.to_c()?.into();

    buffer.write(type_name.as_str());
    buffer.write(" ");
    buffer.write(declaration.name.as_str());

    if matches!(&declaration.var_type, Type::Array(_)) {
        buffer.write("[]");
    }

    if let Some(initializer) = &declaration.initializer {
        let initializer: String = initializer.to_c()?.into();

        buffer.write(" = ");
        buffer.write(initializer.as_str());
    }

    buffer.write(";");

    Ok(buffer)
});
