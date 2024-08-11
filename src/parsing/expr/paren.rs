use crate::{def_parser, ir::expression::Expression};
use swc_ecma_ast::ParenExpr;

def_parser!(ParenExpr, Expression, |expr| {
    let inner = *(expr.clone().expr);

    inner.to_ir()
});
