pub mod var;

use crate::{ir::statement::Statement, parsing::AstToIR};
use anyhow::{bail, Result};
use swc_ecma_ast::Decl;

impl AstToIR<Statement> for Decl {
    fn to_ir(&self) -> Result<Statement> {
        match self {
            Decl::Var(decl) => decl.to_ir(),

            other => bail!("non-supported declaration kind: {:?}", other),
        }
    }
}
