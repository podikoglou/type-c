use crate::{def_codegen, ir::method::MethodParameter};

def_codegen!(MethodParameter, |param| {
    let mut buffer = CodeBuffer::default();

    buffer.write(param._type.to_c().unwrap());
    buffer.write(" ");
    buffer.write(param.name.as_str());

    Ok(buffer)
});
