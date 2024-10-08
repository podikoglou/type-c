use crate::{def_parser, ir::expression::Expression};
use swc_ecma_ast::Ident;

def_parser!(Ident, Expression, |ident| {
    Ok(Expression::Variable(ident.sym.to_string()))
});
