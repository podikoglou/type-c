pub mod array;
pub mod primitive;
pub mod type_ref;

use super::AstToIR;
use crate::ir::types::Type;
use anyhow::{bail, Result};
use swc_ecma_ast::TsType;

impl AstToIR<Type> for TsType {
    fn to_ir(&self) -> Result<Type> {
        match self {
            TsType::TsKeywordType(inner) => inner.kind.to_ir(),
            TsType::TsArrayType(inner) => inner.to_ir(),
            TsType::TsTypeRef(inner) => inner.to_ir(),

            other => bail!("non-supported type: {:?}", other),
        }
    }
}
