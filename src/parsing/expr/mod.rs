pub mod array;
pub mod assign;
pub mod binary;
pub mod call;
pub mod ident;
pub mod literal;
pub mod member;
pub mod paren;
pub mod unary;
pub mod update;

use crate::{def_parser, ir::expression::Expression};
use anyhow::bail;
use swc_ecma_ast::Expr;

def_parser!(Expr, Expression, |expr| {
    match expr {
        Expr::Call(expr) => expr.to_ir(),
        Expr::Lit(expr) => expr.to_ir(),
        Expr::Member(expr) => expr.to_ir(),
        Expr::Ident(expr) => expr.to_ir(),
        Expr::Array(expr) => expr.to_ir(),
        Expr::Assign(expr) => expr.to_ir(),
        Expr::Bin(expr) => expr.to_ir(),
        Expr::Unary(expr) => expr.to_ir(),
        Expr::Paren(expr) => expr.to_ir(),
        Expr::Update(expr) => expr.to_ir(),

        other => bail!("non-supported expression kind: {:?}", other),
    }
});
