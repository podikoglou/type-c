use crate::{
    def_parser,
    ir::{
        expression::{Expression, Literal},
        statement::{ReturnStatement, Statement},
    },
};
use swc_ecma_ast::ReturnStmt;

def_parser!(ReturnStmt, Statement, |statement| {
    match &statement.arg {
        // parse expression & return it
        Some(expr) => {
            let parsed = expr.to_ir()?;

            Ok(Statement::Return(ReturnStatement(parsed)))
        }

        // return void

        // I tried... It won't shrink.
        None => Ok(Statement::Return(ReturnStatement(Expression::Literal(
            Literal::Void,
        )))),
    }
});
