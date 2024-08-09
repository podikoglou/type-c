use crate::{def_parser, ir::types::Type};
use anyhow::bail;
use std::rc::Rc;
use swc_ecma_ast::TsKeywordTypeKind;

def_parser!(TsKeywordTypeKind, Type, |t: &TsKeywordTypeKind| {
    match t {
        TsKeywordTypeKind::TsNumberKeyword => Ok(Type::Number),
        TsKeywordTypeKind::TsBooleanKeyword => Ok(Type::Boolean),
        TsKeywordTypeKind::TsStringKeyword => Ok(Type::Pointer(Rc::new(Type::Char))),
        TsKeywordTypeKind::TsVoidKeyword => Ok(Type::Void),

        other => bail!("non-supported type: {:?}", other),
    }
});
