use crate::{
    def_parser,
    ir::{
        expression::{Expression, Literal},
        statement::Statement,
    },
};
use swc_ecma_ast::ReturnStmt;

def_parser!(ReturnStmt, Statement, |statement: &ReturnStmt| {
    match &statement.arg {
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
});
