use super::AstToIR;
use crate::ir::types::Type;
use anyhow::{bail, Result};
use std::rc::Rc;
use swc_ecma_ast::TsKeywordTypeKind;

impl AstToIR<Type> for TsKeywordTypeKind {
    fn to_ir(&self) -> Result<Type> {
        match self {
            TsKeywordTypeKind::TsNumberKeyword => Ok(Type::Number),
            TsKeywordTypeKind::TsBooleanKeyword => Ok(Type::Boolean),
            TsKeywordTypeKind::TsStringKeyword => Ok(Type::Pointer(Rc::new(Type::Char))),
            TsKeywordTypeKind::TsVoidKeyword => Ok(Type::Void),

            other => bail!("non-supported type: {:?}", other),
        }
    }
}
