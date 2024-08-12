use crate::{
    def_parser,
    ir::statement::{while_s::WhileStatement, Statement},
};
use std::rc::Rc;
use swc_ecma_ast::WhileStmt;

def_parser!(WhileStmt, Statement, |statement| {
    let test = (*statement.test.clone()).to_ir()?;
    let body = (*statement.body.clone()).to_ir()?;

    Ok(Statement::While(WhileStatement {
        test,
        body: Rc::new(body),
    }))
});
