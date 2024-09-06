pub mod array;
pub mod primitive;
pub mod type_ref;

use crate::{def_parser, ir::types::Type};
use anyhow::bail;
use swc_ecma_ast::TsType;

def_parser!(TsType, Type, |t| {
    match t {
        TsType::TsKeywordType(inner) => inner.kind.to_ir(),
        TsType::TsArrayType(inner) => inner.to_ir(),
        TsType::TsTypeRef(inner) => inner.to_ir(),

        other => bail!("non-supported type: {:?}", other),
    }
});
