use crate::{def_parser, ir::types::Type};
use swc_ecma_ast::TsArrayType;

def_parser!(TsArrayType, Type, |t| {
    // yet another TsType, so we can just recursively parse this
    let inner = t.elem_type.clone();
    let parsed_type = inner.to_ir()?;

    Ok(Type::Array(Box::new(parsed_type)))
});
