use crate::{
    def_parser,
    ir::expression::{binary::BinaryOperation, Expression},
};
use anyhow::bail;
use swc_ecma_ast::{BinExpr, BinaryOp};

def_parser!(BinExpr, Expression, |expr| {
    match expr.op {
        BinaryOp::LogicalOr => Ok(Expression::BinaryOperation(BinaryOperation::Or(
            Box::new(expr.left.clone().to_ir()?),
            Box::new(expr.right.clone().to_ir()?),
        ))),

        BinaryOp::LogicalAnd => Ok(Expression::BinaryOperation(BinaryOperation::And(
            Box::new(expr.left.clone().to_ir()?),
            Box::new(expr.right.clone().to_ir()?),
        ))),

        BinaryOp::EqEq => Ok(Expression::BinaryOperation(BinaryOperation::Eq(
            Box::new(expr.left.clone().to_ir()?),
            Box::new(expr.right.clone().to_ir()?),
        ))),

        BinaryOp::NotEq => Ok(Expression::BinaryOperation(BinaryOperation::NotEq(
            Box::new(expr.left.clone().to_ir()?),
            Box::new(expr.right.clone().to_ir()?),
        ))),

        BinaryOp::Gt => Ok(Expression::BinaryOperation(BinaryOperation::Gt(
            Box::new(expr.left.clone().to_ir()?),
            Box::new(expr.right.clone().to_ir()?),
        ))),

        BinaryOp::Lt => Ok(Expression::BinaryOperation(BinaryOperation::Lt(
            Box::new(expr.left.clone().to_ir()?),
            Box::new(expr.right.clone().to_ir()?),
        ))),

        BinaryOp::GtEq => Ok(Expression::BinaryOperation(BinaryOperation::GtEq(
            Box::new(expr.left.clone().to_ir()?),
            Box::new(expr.right.clone().to_ir()?),
        ))),

        BinaryOp::LtEq => Ok(Expression::BinaryOperation(BinaryOperation::LtEq(
            Box::new(expr.left.clone().to_ir()?),
            Box::new(expr.right.clone().to_ir()?),
        ))),

        // TODO: Make this actually have the same effect as in JS/TS
        BinaryOp::EqEqEq => Ok(Expression::BinaryOperation(BinaryOperation::Eq(
            Box::new(expr.left.clone().to_ir()?),
            Box::new(expr.right.clone().to_ir()?),
        ))),

        // TODO: Make this actually have the same effect as in JS/TS
        BinaryOp::NotEqEq => Ok(Expression::BinaryOperation(BinaryOperation::NotEq(
            Box::new(expr.left.clone().to_ir()?),
            Box::new(expr.right.clone().to_ir()?),
        ))),

        BinaryOp::Add => Ok(Expression::BinaryOperation(BinaryOperation::Add(
            Box::new(expr.left.clone().to_ir()?),
            Box::new(expr.right.clone().to_ir()?),
        ))),

        BinaryOp::Sub => Ok(Expression::BinaryOperation(BinaryOperation::Subtract(
            Box::new(expr.left.clone().to_ir()?),
            Box::new(expr.right.clone().to_ir()?),
        ))),

        BinaryOp::Mul => Ok(Expression::BinaryOperation(BinaryOperation::Multiply(
            Box::new(expr.left.clone().to_ir()?),
            Box::new(expr.right.clone().to_ir()?),
        ))),

        BinaryOp::Div => Ok(Expression::BinaryOperation(BinaryOperation::Divide(
            Box::new(expr.left.clone().to_ir()?),
            Box::new(expr.right.clone().to_ir()?),
        ))),

        BinaryOp::Mod => Ok(Expression::BinaryOperation(BinaryOperation::Modulus(
            Box::new(expr.left.clone().to_ir()?),
            Box::new(expr.right.clone().to_ir()?),
        ))),

        other => bail!("operation not supported: {:?}", other),
    }
});
