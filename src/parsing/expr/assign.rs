use std::rc::Rc;

use anyhow::bail;
use swc_ecma_ast::{AssignExpr, AssignTarget, SimpleAssignTarget};

use crate::{
    def_parser,
    ir::expression::{Assignment, Expression},
};

def_parser!(AssignExpr, Expression, |expr| {
    // the difference between right and left is that left is an expression
    // while left is an AssignTarget.
    let left = expr.left.clone();
    let right = *expr.right.clone();

    let left_expr: Expression = match left {
        AssignTarget::Simple(target) => match target {
            SimpleAssignTarget::Ident(ident) => ident.to_ir()?,

            // SimpleAssignTarget::Member(member) => todo!(),
            _ => bail!("this simple assign target type is not supported"),
        },

        _ => bail!("this assign target type is not supported"),
    };

    let right_expr = right.to_ir()?;

    Ok(Expression::Assignment(Assignment {
        left: Rc::new(left_expr),
        right: Rc::new(right_expr),
    }))
});
