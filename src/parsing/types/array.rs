use super::AstToIR;
use crate::ir::types::Type;
use anyhow::Result;
use std::rc::Rc;
use swc_ecma_ast::TsArrayType;

impl AstToIR<Type> for TsArrayType {
    fn to_ir(&self) -> Result<Type> {
        // yet another TsType, so we can just recursively parse this
        let inner = self.elem_type.clone();
        let parsed_type = inner.to_ir()?;

        Ok(Type::Pointer(Rc::new(parsed_type)))
    }
}
