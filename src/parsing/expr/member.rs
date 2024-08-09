use crate::{
    ir::expression::{Expression, MemberAccess},
    parsing::AstToIR,
};
use anyhow::{bail, Result};
use std::rc::Rc;
use swc_ecma_ast::{MemberExpr, MemberProp};

impl AstToIR<Expression> for MemberExpr {
    fn to_ir(&self) -> Result<Expression> {
        // get the object of which we're accessing the member
        if !self.obj.is_ident() {
            bail!("can't index such complex expression yet")
        }

        let obj = *self.obj.clone();
        let obj = obj.ident().unwrap().sym.to_string();

        match &self.prop {
            MemberProp::Computed(prop) => {
                let parsed_expr = prop.expr.to_ir()?;

                Ok(Expression::MemberAccess(MemberAccess {
                    object: Rc::new(Expression::Variable(obj)),
                    index: Rc::new(parsed_expr),
                }))
            }

            other => bail!("non-supported prop type: {:?}", other),
        }
    }
}
