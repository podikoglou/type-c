pub mod call;
pub mod ident;
pub mod literal;
pub mod member;

use super::AstToIR;
use crate::ir::expression::Expression;
use anyhow::{bail, Result};
use swc_ecma_ast::Expr;

impl AstToIR<Expression> for Expr {
    fn to_ir(&self) -> Result<Expression> {
        match self {
            Expr::Call(expr) => expr.to_ir(),
            Expr::Lit(expr) => expr.to_ir(),
            Expr::Member(expr) => expr.to_ir(),
            Expr::Ident(expr) => expr.to_ir(),

            other => bail!("non-supported expression kind: {:?}", other),
        }
    }
}
