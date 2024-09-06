use crate::{def_parser, ir::types::Type, parsing::ToIR};
use anyhow::{bail, Result};
use swc_ecma_ast::TsTypeRef;

def_parser!(TsTypeRef, Type, |t| {
    let type_name = &t.type_name.as_ident().unwrap().sym.to_string();

    match type_name.as_str() {
        "Pointer" => parse_pointer(t),
        "char" => Ok(Type::Char),

        other => bail!("non-supported type: {:?}", other),
    }
});

fn parse_pointer(input: &TsTypeRef) -> Result<Type> {
    let type_params = input.type_params.clone();

    // in this case, we have just `Pointer` as a type, without a generic inside it
    if type_params.is_none() {
        bail!("invalid Pointer usage (no generic)")
    }

    let type_params = type_params.unwrap();

    // we must only have 1 generic type -- no more, no less
    if type_params.params.len() != 1 {
        bail!("invalid Pointer usage (only use 1 generic)")
    }

    let inner = type_params.params.first().unwrap();
    let inner_type = inner.to_ir()?;

    Ok(Type::Pointer(Box::new(inner_type)))
}
