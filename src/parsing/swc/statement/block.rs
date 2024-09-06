use crate::{
    def_parser,
    ir::statement::{block::BlockStatement, Statement},
};
use swc_ecma_ast::BlockStmt;

def_parser!(BlockStmt, Statement, |block| {
    let statements: Vec<Statement> = block
        .stmts
        .iter()
        .map(|statement| statement.to_ir().unwrap())
        .collect();

    Ok(Statement::Block(BlockStatement(statements)))
});
