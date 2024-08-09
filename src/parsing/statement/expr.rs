use crate::{
    def_parser,
    ir::statement::{ExpressionStatement, Statement},
};
use swc_ecma_ast::ExprStmt;

def_parser!(ExprStmt, Statement, |statement| {
    let parsed = statement.expr.to_ir()?;

    Ok(Statement::Expression(ExpressionStatement(parsed)))
});
