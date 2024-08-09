use crate::{ir::expression::Expression, parsing::AstToIR};
use anyhow::Result;
use swc_ecma_ast::Ident;

impl AstToIR<Expression> for Ident {
    fn to_ir(&self) -> Result<Expression> {
        Ok(Expression::Variable(self.sym.to_string()))
    }
}
