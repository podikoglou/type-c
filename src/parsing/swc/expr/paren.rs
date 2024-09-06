use crate::{
    def_parser,
    ir::expression::{paren::Paren, Expression},
};
use swc_ecma_ast::ParenExpr;

def_parser!(ParenExpr, Expression, |expr| {
    let inner = *(expr.clone().expr);

    Ok(Expression::Paren(Paren(Box::new(inner.to_ir()?))))
});
