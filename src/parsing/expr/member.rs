use crate::{
    def_parser,
    ir::expression::{Expression, MemberAccess},
};
use anyhow::bail;
use std::rc::Rc;
use swc_ecma_ast::{MemberExpr, MemberProp};

def_parser!(MemberExpr, Expression, |expr: &MemberExpr| {
    // get the object of which we're accessing the member
    if !expr.obj.is_ident() {
        bail!("can't index such complex expression yet")
    }

    let obj = *expr.obj.clone();
    let obj = obj.ident().unwrap().sym.to_string();

    match &expr.prop {
        MemberProp::Computed(prop) => {
            let parsed_expr = prop.expr.to_ir()?;

            Ok(Expression::MemberAccess(MemberAccess {
                object: Rc::new(Expression::Variable(obj)),
                index: Rc::new(parsed_expr),
            }))
        }

        other => bail!("non-supported prop type: {:?}", other),
    }
});
