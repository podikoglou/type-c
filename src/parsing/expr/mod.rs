pub mod call;
pub mod ident;
pub mod literal;
pub mod member;

use crate::{def_parser, ir::expression::Expression};
use anyhow::bail;
use swc_ecma_ast::Expr;

def_parser!(Expr, Expression, |expr: &Expr| {
    match expr {
        Expr::Call(expr) => expr.to_ir(),
        Expr::Lit(expr) => expr.to_ir(),
        Expr::Member(expr) => expr.to_ir(),
        Expr::Ident(expr) => expr.to_ir(),

        other => bail!("non-supported expression kind: {:?}", other),
    }
});
