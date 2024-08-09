pub mod var;

use crate::{def_parser, ir::statement::Statement};
use anyhow::bail;
use swc_ecma_ast::Decl;

def_parser!(Decl, Statement, |decl: &Decl| {
    match decl {
        Decl::Var(decl) => decl.to_ir(),

        other => bail!("non-supported declaration kind: {:?}", other),
    }
});
