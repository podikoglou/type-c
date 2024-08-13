use crate::{
    def_parser,
    ir::expression::{member_access::MemberAccess, Expression},
};
use anyhow::bail;
use swc_ecma_ast::{MemberExpr, MemberProp};

def_parser!(MemberExpr, Expression, |expr| {
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
                object: Box::new(Expression::Variable(obj)),
                index: Box::new(parsed_expr),
            }))
        }

        other => bail!("non-supported prop type: {:?}", other),
    }
});
