use crate::{
    def_parser,
    ir::statement::{expr::ExpressionStatement, for_s::ForStatement, Statement},
};
use swc_ecma_ast::{ForInStmt, ForOfStmt, ForStmt, VarDeclOrExpr};

def_parser!(ForStmt, Statement, |statement| {
    let init = match &statement.init.clone().expect("expected init variable") {
        VarDeclOrExpr::VarDecl(decl) => (*decl.clone()).to_ir()?,

        VarDeclOrExpr::Expr(expr) => {
            Statement::Expression(ExpressionStatement((*expr.clone()).to_ir()?))
        }
    };

    let test = (*statement.test.clone().expect("expected test")).to_ir()?;

    let update = Statement::Expression(ExpressionStatement(
        (*statement.update.clone().expect("expected update")).to_ir()?,
    ));

    let body = (*statement.body.clone()).to_ir()?;

    Ok(Statement::For(ForStatement {
        init: Box::new(init),
        test,
        update: Box::new(update),
        body: Box::new(body),
    }))
});

def_parser!(ForInStmt, Statement, |_statement| {
    todo!("for..in loops are not supported yet")
});

def_parser!(ForOfStmt, Statement, |_statement| {
    todo!("for..of loops are not supported yet")
});
