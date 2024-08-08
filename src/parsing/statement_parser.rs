use super::expression_parser::parse_expr;
use crate::ir::{
    expression::{Expression, Literal},
    statement::Statement,
};
use anyhow::{bail, Result};
use swc_ecma_ast::Stmt;

/// Parses a [[Stmt] into an IR [[Statement]].
pub fn parse_statement(statement: &Stmt) -> Result<Statement> {
    match statement {
        Stmt::Expr(stmt) => {
            let parsed = parse_expr(&stmt.expr)?;

            Ok(Statement::ExpressionStatement(parsed))
        }

        Stmt::Return(stmt) => match &stmt.arg {
            // parse expression & return it
            Some(expr) => {
                let parsed = parse_expr(expr)?;

                Ok(Statement::ReturnStatement(parsed))
            }

            // return void
            None => Ok(Statement::ReturnStatement(Expression::Literal(
                Literal::Void,
            ))),
        },

        other => bail!("non-supported statement kind: {:?}", other),
    }
}
