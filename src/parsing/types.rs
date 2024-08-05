use std::rc::Rc;

use anyhow::anyhow;
use swc_ecma_ast::{TsKeywordTypeKind, TsType};

use crate::ir::Type;

/// Parses a [[TsType] into an IR [[Type]].
pub fn parse_type(type_ann: &TsType) -> anyhow::Result<Type> {
    match type_ann {
        TsType::TsKeywordType(inner) => match &inner.kind {
            TsKeywordTypeKind::TsNumberKeyword => Ok(Type::Number),
            TsKeywordTypeKind::TsBooleanKeyword => Ok(Type::Boolean),
            TsKeywordTypeKind::TsStringKeyword => Ok(Type::Pointer(Rc::new(Type::Char))),
            TsKeywordTypeKind::TsVoidKeyword => Ok(Type::Void),

            _ => {
                return Err(anyhow!("non-supported type"));
            }
        },

        TsType::TsArrayType(inner) => {
            // yet another TsType, so we can just recursively parse this
            let inner = inner.elem_type.clone();
            let parsed_type = parse_type(&inner)?;

            Ok(Type::Pointer(Rc::new(parsed_type)))
        }

        _ => {
            return Err(anyhow!("non-supported type"));
        }
    }
}
