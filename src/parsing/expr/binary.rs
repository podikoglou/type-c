use std::rc::Rc;

use anyhow::bail;
use swc_ecma_ast::{BinExpr, BinaryOp};

use crate::{
    def_parser,
    ir::expression::{binary::BinaryOperation, Expression},
};

def_parser!(BinExpr, Expression, |expr| {
    match expr.op {
        BinaryOp::LogicalOr => Ok(Expression::BinaryOperation(BinaryOperation::Or(
            Rc::new(expr.left.clone().to_ir()?),
            Rc::new(expr.right.clone().to_ir()?),
        ))),

        BinaryOp::LogicalAnd => Ok(Expression::BinaryOperation(BinaryOperation::And(
            Rc::new(expr.left.clone().to_ir()?),
            Rc::new(expr.right.clone().to_ir()?),
        ))),

        _ => bail!("operation not supported"),
    }
});
