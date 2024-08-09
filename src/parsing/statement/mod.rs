pub mod decl;
pub mod expr;
pub mod return_s;

use crate::{def_parser, ir::statement::Statement};
use anyhow::bail;
use swc_ecma_ast::Stmt;

def_parser!(Stmt, Statement, |statement| {
    match statement {
        Stmt::Expr(statement) => statement.to_ir(),
        Stmt::Return(statement) => statement.to_ir(),
        Stmt::Decl(statement) => statement.to_ir(),

        other => bail!("non-supported statement kind: {:?}", other),
    }
});
