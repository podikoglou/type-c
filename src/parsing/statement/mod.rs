pub mod decl;
pub mod expr;
pub mod return_s;

use super::AstToIR;
use crate::ir::statement::Statement;
use anyhow::{bail, Result};
use swc_ecma_ast::Stmt;

impl AstToIR<Statement> for Stmt {
    fn to_ir(&self) -> Result<Statement> {
        match self {
            Stmt::Expr(statement) => statement.to_ir(),
            Stmt::Return(statement) => statement.to_ir(),
            Stmt::Decl(statement) => statement.to_ir(),

            other => bail!("non-supported statement kind: {:?}", other),
        }
    }
}
