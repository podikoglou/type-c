use crate::{
    def_codegen,
    ir::{method::MethodParameter, types::Type},
};

def_codegen!(MethodParameter, |param| {
    let mut buffer = CodeBuffer::default();

    buffer.write(param._type.to_c().unwrap());
    buffer.write(" ");
    buffer.write(param.name.as_str());

    if matches!(&param._type, Type::Array(_)) {
        buffer.write("[]");
    }

    Ok(buffer)
});
