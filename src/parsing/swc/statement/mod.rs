pub mod block;
pub mod decl;
pub mod expr;
pub mod for_s;
pub mod if_s;
pub mod return_s;
pub mod while_s;

use crate::{def_parser, ir::statement::Statement};
use anyhow::bail;
use swc_ecma_ast::Stmt;

def_parser!(Stmt, Statement, |statement| {
    match statement {
        Stmt::Expr(statement) => statement.to_ir(),
        Stmt::Return(statement) => statement.to_ir(),
        Stmt::Decl(statement) => statement.to_ir(),
        Stmt::Block(statement) => statement.to_ir(),
        Stmt::If(statement) => statement.to_ir(),
        Stmt::While(statement) => statement.to_ir(),
        Stmt::For(statement) => statement.to_ir(),
        Stmt::ForIn(statement) => statement.to_ir(),
        Stmt::ForOf(statement) => statement.to_ir(),

        other => bail!("non-supported statement kind: {:?}", other),
    }
});
