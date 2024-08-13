use crate::{
    def_parser,
    ir::expression::{unary::UnaryOperation, Expression},
};
use anyhow::bail;
use swc_ecma_ast::{UnaryExpr, UnaryOp};

def_parser!(UnaryExpr, Expression, |expr| {
    let arg = (*expr.arg.clone()).to_ir()?;

    match expr.op {
        UnaryOp::Minus => Ok(Expression::UnaryOperation(UnaryOperation::Minus(Box::new(
            arg,
        )))),

        UnaryOp::Plus => Ok(Expression::UnaryOperation(UnaryOperation::Plus(Box::new(
            arg,
        )))),

        UnaryOp::Bang => Ok(Expression::UnaryOperation(UnaryOperation::Bang(Box::new(
            arg,
        )))),

        other => bail!("operation not supported: {:?}", other),
    }
});
