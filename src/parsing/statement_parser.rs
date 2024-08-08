use super::expression_parser::parse_expr;
use crate::{
    ir::{
        expression::{Expression, Literal},
        statement::{Statement, VariableDeclaration},
    },
    parsing::type_parser::parse_type,
};
use anyhow::{bail, Result};
use swc_ecma_ast::{Decl, ExprStmt, ReturnStmt, Stmt};

/// Parses a [[Stmt] into an IR [[Statement]].
pub fn parse_statement(statement: &Stmt) -> Result<Statement> {
    match statement {
        Stmt::Expr(statement) => parse_expr_statement(statement),
        Stmt::Return(statement) => parse_return_statement(statement),
        Stmt::Decl(statement) => parse_decl_statement(statement),

        other => bail!("non-supported statement kind: {:?}", other),
    }
}

fn parse_expr_statement(statement: &ExprStmt) -> Result<Statement> {
    let parsed = parse_expr(&statement.expr)?;

    Ok(Statement::ExpressionStatement(parsed))
}

fn parse_return_statement(statement: &ReturnStmt) -> Result<Statement> {
    match &statement.arg {
        // parse expression & return it
        Some(expr) => {
            let parsed = parse_expr(expr)?;

            Ok(Statement::ReturnStatement(parsed))
        }

        // return void
        None => Ok(Statement::ReturnStatement(Expression::Literal(
            Literal::Void,
        ))),
    }
}

fn parse_decl_statement(decl: &Decl) -> Result<Statement> {
    match decl {
        Decl::Var(decl) => {
            // a Decl can have multiple decls stored in a Vec. I assume this is
            // becuase of stuff like multiple assignments in a single statement?
            //
            // anyway, we don't support this right now.
            if decl.decls.len() != 1 {
                bail!("there must be one declaration");
            }

            let decl = decl.decls.first().unwrap();

            // decl.name for whatever reason also contains the type annotation if present,
            // so we'll try to unpack the type annotation (and fail if it isn't there)
            let ident = decl.name.clone().ident().unwrap();

            // get the name of the variable
            let name = ident.id.sym.to_string();

            // get the type (annotation) of the variable
            let type_ann = *ident.type_ann.expect("expected type annotation").type_ann;
            let var_type = parse_type(&type_ann)?;

            // now, check if we also have an initializer
            let initializer = match &decl.init {
                Some(initializer) => Some(parse_expr(initializer)?),
                None => None,
            };

            Ok(Statement::VariableDeclaration(VariableDeclaration {
                name,
                var_type,
                initializer,
            }))
        }

        other => bail!("non-supported declaration kind: {:?}", other),
    }
}
