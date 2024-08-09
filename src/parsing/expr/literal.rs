use crate::{
    ir::expression::{Expression, Literal},
    parsing::AstToIR,
};
use anyhow::{bail, Result};
use swc_ecma_ast::Lit;

impl AstToIR<Expression> for Lit {
    fn to_ir(&self) -> Result<Expression> {
        Ok(Expression::Literal(match self {
            Lit::Str(val) => Literal::String(val.value.to_string()),
            Lit::Bool(val) => Literal::Boolean(val.value),
            Lit::Num(val) => Literal::Number(val.value),

            other => bail!("non-supported literal: {:?}", other),
        }))
    }
}
