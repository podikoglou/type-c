use std::rc::Rc;

use crate::ir::expression::{Expression, Literal, MemberAccess, MethodCall};
use anyhow::{bail, Result};
use swc_ecma_ast::{CallExpr, Expr, Ident, Lit, MemberExpr, MemberProp};

pub fn parse_expr(expr: &Expr) -> Result<Expression> {
    match expr {
        Expr::Call(expr) => parse_call_expr(expr),
        Expr::Lit(expr) => parse_literal(expr),
        Expr::Member(expr) => parse_member_expr(expr),
        Expr::Ident(expr) => parse_ident_expr(expr),

        other => bail!("non-supported expression kind: {:?}", other),
    }
}

fn parse_call_expr(expr: &CallExpr) -> Result<Expression> {
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
        .map(|expr| parse_expr(&expr.expr))
        .map(Result::unwrap)
        .collect();

    Ok(Expression::MethodCall(MethodCall { name, arguments }))
}

fn parse_literal(expr: &Lit) -> Result<Expression> {
    Ok(Expression::Literal(match expr {
        Lit::Str(val) => Literal::String(val.value.to_string()),
        Lit::Bool(val) => Literal::Boolean(val.value),
        Lit::Num(val) => Literal::Number(val.value),

        other => bail!("non-supported literal: {:?}", other),
    }))
}

fn parse_member_expr(expr: &MemberExpr) -> Result<Expression> {
    // get the object of which we're accessing the member
    if !expr.obj.is_ident() {
        bail!("can't index such complex expression yet")
    }

    let obj = *expr.obj.clone();
    let obj = obj.ident().unwrap().sym.to_string();

    match &expr.prop {
        MemberProp::Computed(prop) => {
            let parsed_expr = parse_expr(&prop.expr)?;

            Ok(Expression::MemberAccess(MemberAccess {
                object: Rc::new(Expression::Variable(obj)),
                index: Rc::new(parsed_expr),
            }))
        }

        other => bail!("non-supported prop type: {:?}", other),
    }
}

fn parse_ident_expr(expr: &Ident) -> Result<Expression> {
    Ok(Expression::Variable(expr.sym.to_string()))
}
