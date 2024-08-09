use crate::{
    def_parser,
    ir::expression::{Expression, Literal},
};
use swc_ecma_ast::ArrayLit;

def_parser!(ArrayLit, Literal, |lit| {
    Ok(Literal::Array(
        lit.elems
            .iter()
            .map(Clone::clone)
            .map(Option::unwrap)
            .map(|element| element.expr.to_ir())
            .map(Result::unwrap)
            .collect::<Vec<Expression>>(),
    ))
});

def_parser!(ArrayLit, Expression, |lit| {
    Ok(Expression::Literal(lit.to_ir()?))
});
