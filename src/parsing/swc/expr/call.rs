use crate::{
    def_parser,
    ir::expression::{method_call::MethodCall, Expression},
};
use anyhow::{bail, Result};
use swc_ecma_ast::{CallExpr, Expr};

def_parser!(CallExpr, Expression, |expr| {
    let callee = *expr.callee.as_expr().unwrap().clone();

    // NOTE: This does *not* support member functions (i.e. calling functions
    // on objects) for example: `foo.bar()`. This is for the most part because
    // this is just not a common paradigm in C. The equivalent in C would be
    // bar(foo) or bar(&foo).

    // attempt to get name of the funciton being called
    let name = match callee {
        Expr::Ident(ident) => ident.sym.to_string(),

        other => bail!("non-supported callee kind: {:?}", other),
    };

    let arguments: Vec<Expression> = expr
        .args
        .iter()
        .map(|expr| expr.expr.to_ir())
        .map(Result::unwrap)
        .collect();

    Ok(Expression::MethodCall(MethodCall { name, arguments }))
});
