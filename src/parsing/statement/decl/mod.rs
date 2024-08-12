pub mod var;

use crate::{
    def_parser,
    ir::statement::{block::BlockStatement, Statement},
};
use anyhow::bail;
use swc_ecma_ast::Decl;

def_parser!(Decl, Statement, |decl| {
    match decl {
        Decl::Var(decl) => decl.to_ir(),
        Decl::Fn(_) => {
            eprintln!("warning: you have non exported functions. They will not be transpiled.");

            // this is not written since it's an empty block
            Ok(Statement::Block(BlockStatement(vec![])))
        }

        other => bail!("non-supported declaration kind: {:?}", other),
    }
});
