use anyhow::{bail, Result};
use std::rc::Rc;
use swc_ecma_ast::{TsArrayType, TsKeywordTypeKind, TsType, TsTypeRef};

use crate::ir::types::Type;

/// Parses a [[TsType] into an IR [[Type]].
pub fn parse_type(type_ann: &TsType) -> Result<Type> {
    match type_ann {
        TsType::TsKeywordType(inner) => parse_primitive_type(&inner.kind),
        TsType::TsArrayType(inner) => parse_array_type(&inner),
        TsType::TsTypeRef(inner) => parse_type_ref(&inner),

        other => bail!("non-supported type: {:?}", other),
    }
}

fn parse_array_type(input: &TsArrayType) -> Result<Type> {
    // yet another TsType, so we can just recursively parse this
    let inner = input.elem_type.clone();
    let parsed_type = parse_type(&inner)?;

    Ok(Type::Pointer(Rc::new(parsed_type)))
}

fn parse_primitive_type(input: &TsKeywordTypeKind) -> Result<Type> {
    match input {
        TsKeywordTypeKind::TsNumberKeyword => Ok(Type::Number),
        TsKeywordTypeKind::TsBooleanKeyword => Ok(Type::Boolean),
        TsKeywordTypeKind::TsStringKeyword => Ok(Type::Pointer(Rc::new(Type::Char))),
        TsKeywordTypeKind::TsVoidKeyword => Ok(Type::Void),

        other => bail!("non-supported type: {:?}", other),
    }
}

fn parse_type_ref(input: &TsTypeRef) -> Result<Type> {
    let type_name = &input.type_name.as_ident().unwrap().sym.to_string();

    match type_name.as_str() {
        "Pointer" => parse_pointer(&input),

        other => bail!("non-supported type: {:?}", other),
    }
}

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
    let inner_type = parse_type(&inner)?;

    Ok(Type::Pointer(Rc::new(inner_type)))
}
