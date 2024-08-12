use crate::{
    def_parser,
    ir::expression::{binary::BinaryOperation, Expression},
};
use anyhow::bail;
use std::rc::Rc;
use swc_ecma_ast::{BinExpr, BinaryOp};

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

        BinaryOp::EqEq => Ok(Expression::BinaryOperation(BinaryOperation::Eq(
            Rc::new(expr.left.clone().to_ir()?),
            Rc::new(expr.right.clone().to_ir()?),
        ))),

        BinaryOp::NotEq => Ok(Expression::BinaryOperation(BinaryOperation::NotEq(
            Rc::new(expr.left.clone().to_ir()?),
            Rc::new(expr.right.clone().to_ir()?),
        ))),

        BinaryOp::Gt => Ok(Expression::BinaryOperation(BinaryOperation::Gt(
            Rc::new(expr.left.clone().to_ir()?),
            Rc::new(expr.right.clone().to_ir()?),
        ))),

        BinaryOp::Lt => Ok(Expression::BinaryOperation(BinaryOperation::Lt(
            Rc::new(expr.left.clone().to_ir()?),
            Rc::new(expr.right.clone().to_ir()?),
        ))),

        BinaryOp::GtEq => Ok(Expression::BinaryOperation(BinaryOperation::GtEq(
            Rc::new(expr.left.clone().to_ir()?),
            Rc::new(expr.right.clone().to_ir()?),
        ))),

        BinaryOp::LtEq => Ok(Expression::BinaryOperation(BinaryOperation::LtEq(
            Rc::new(expr.left.clone().to_ir()?),
            Rc::new(expr.right.clone().to_ir()?),
        ))),

        // TODO: Make this actually have the same effect as in JS/TS
        BinaryOp::EqEqEq => Ok(Expression::BinaryOperation(BinaryOperation::Eq(
            Rc::new(expr.left.clone().to_ir()?),
            Rc::new(expr.right.clone().to_ir()?),
        ))),

        // TODO: Make this actually have the same effect as in JS/TS
        BinaryOp::NotEqEq => Ok(Expression::BinaryOperation(BinaryOperation::NotEq(
            Rc::new(expr.left.clone().to_ir()?),
            Rc::new(expr.right.clone().to_ir()?),
        ))),

        BinaryOp::Add => Ok(Expression::BinaryOperation(BinaryOperation::Add(
            Rc::new(expr.left.clone().to_ir()?),
            Rc::new(expr.right.clone().to_ir()?),
        ))),

        BinaryOp::Sub => Ok(Expression::BinaryOperation(BinaryOperation::Subtract(
            Rc::new(expr.left.clone().to_ir()?),
            Rc::new(expr.right.clone().to_ir()?),
        ))),

        BinaryOp::Mul => Ok(Expression::BinaryOperation(BinaryOperation::Multiply(
            Rc::new(expr.left.clone().to_ir()?),
            Rc::new(expr.right.clone().to_ir()?),
        ))),

        BinaryOp::Div => Ok(Expression::BinaryOperation(BinaryOperation::Divide(
            Rc::new(expr.left.clone().to_ir()?),
            Rc::new(expr.right.clone().to_ir()?),
        ))),

        other => bail!("operation not supported: {:?}", other),
    }
});
