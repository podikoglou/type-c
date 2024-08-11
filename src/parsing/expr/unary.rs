use crate::{
    def_parser,
    ir::expression::{unary::UnaryOperation, Expression},
};
use anyhow::bail;
use std::rc::Rc;
use swc_ecma_ast::{UnaryExpr, UnaryOp};

def_parser!(UnaryExpr, Expression, |expr| {
    let arg = (*expr.arg.clone()).to_ir()?;

    match expr.op {
        UnaryOp::Minus => Ok(Expression::UnaryOperation(UnaryOperation::Minus(Rc::new(
            arg,
        )))),

        UnaryOp::Plus => Ok(Expression::UnaryOperation(UnaryOperation::Plus(Rc::new(
            arg,
        )))),

        UnaryOp::Bang => Ok(Expression::UnaryOperation(UnaryOperation::Bang(Rc::new(
            arg,
        )))),

        other => bail!("operation not supported: {:?}", other),
    }
});
