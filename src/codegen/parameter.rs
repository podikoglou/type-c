use crate::{def_codegen, ir::method::MethodParameter};

def_codegen!(MethodParameter, |param: &MethodParameter| {
    let mut writer = CodeWriter::default();

    writer.concat(&param._type.to_c().unwrap());
    writer.write(" ");
    writer.write(param.name.as_str());

    Ok(writer)
});
