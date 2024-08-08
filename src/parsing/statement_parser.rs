use super::expression_parser::parse_expr;
use crate::ir::statement::Statement;
use anyhow::{bail, Result};
use swc_ecma_ast::Stmt;

/// Parses a [[Stmt] into an IR [[Statement]].
pub fn parse_statement(statement: &Stmt) -> Result<Statement> {
    match statement {
        Stmt::Expr(expr) => {
            let parsed = parse_expr(&expr.expr)?;

            Ok(Statement::ExpressionStatement(parsed))
        }

        other => bail!("non-supported statement kind: {:?}", other),
    }
}
