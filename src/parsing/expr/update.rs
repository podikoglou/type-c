use crate::{
    def_parser,
    ir::expression::{
        assignment::Assignment, binary::BinaryOperation, literal::Literal, Expression,
    },
};
use std::rc::Rc;
use swc_ecma_ast::{UpdateExpr, UpdateOp};

def_parser!(UpdateExpr, Expression, |expr| {
    let arg = (*expr.arg.clone()).to_ir()?;

    // it's too much effort to add the concept of i++ to the compiler/transpiler
    // so we just transform it to i = i + 1
    let operation = Expression::BinaryOperation(match expr.op {
        UpdateOp::PlusPlus => BinaryOperation::Add(
            Rc::new(arg.clone()),
            Rc::new(Expression::Literal(Literal::Number(1.0))),
        ),

        UpdateOp::MinusMinus => BinaryOperation::Add(
            Rc::new(arg.clone()),
            Rc::new(Expression::Literal(Literal::Number(1.0))),
        ),
    });

    Ok(Expression::Assignment(Assignment {
        left: Rc::new(arg),
        right: Rc::new(operation),
    }))
});
