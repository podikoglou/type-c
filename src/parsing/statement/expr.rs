use crate::{def_parser, ir::statement::Statement};
use swc_ecma_ast::ExprStmt;

def_parser!(ExprStmt, Statement, |statement: &ExprStmt| {
    let parsed = statement.expr.to_ir()?;

    Ok(Statement::ExpressionStatement(parsed))
});
