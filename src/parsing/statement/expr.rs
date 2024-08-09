use crate::{ir::statement::Statement, parsing::AstToIR};
use anyhow::Result;
use swc_ecma_ast::ExprStmt;

impl AstToIR<Statement> for ExprStmt {
    fn to_ir(&self) -> Result<Statement> {
        let parsed = self.expr.to_ir()?;

        Ok(Statement::ExpressionStatement(parsed))
    }
}
