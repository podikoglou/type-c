use crate::{
    def_parser,
    ir::expression::{Expression, Literal},
};
use anyhow::bail;
use swc_ecma_ast::Lit;

def_parser!(Lit, Expression, |lit| {
    Ok(Expression::Literal(match lit {
        Lit::Str(val) => Literal::String(val.value.to_string()),
        Lit::Bool(val) => Literal::Boolean(val.value),
        Lit::Num(val) => Literal::Number(val.value),

        other => bail!("non-supported literal: {:?}", other),
    }))
});
