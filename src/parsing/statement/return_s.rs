use crate::{
    ir::{
        expression::{Expression, Literal},
        statement::Statement,
    },
    parsing::AstToIR,
};
use anyhow::Result;
use swc_ecma_ast::ReturnStmt;

impl AstToIR<Statement> for ReturnStmt {
    fn to_ir(&self) -> Result<Statement> {
        match &self.arg {
            // parse expression & return it
            Some(expr) => {
                let parsed = expr.to_ir()?;

                Ok(Statement::ReturnStatement(parsed))
            }

            // return void
            None => Ok(Statement::ReturnStatement(Expression::Literal(
                Literal::Void,
            ))),
        }
    }
}
