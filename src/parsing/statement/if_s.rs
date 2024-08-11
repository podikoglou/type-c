use crate::{
    def_parser,
    ir::statement::{if_s::IfStatement, Statement},
};
use std::rc::Rc;
use swc_ecma_ast::IfStmt;

def_parser!(IfStmt, Statement, |statement| {
    let test = (*statement.test.clone()).to_ir()?;

    let cons = (*statement.cons.clone()).to_ir()?;

    let alt = {
        match &statement.alt {
            Some(inner) => Some((*inner).to_ir()?),
            None => None,
        }
    };

    Ok(Statement::If(IfStatement {
        test,
        cons: Rc::new(cons),
        alt: Rc::new(alt),
    }))
});
