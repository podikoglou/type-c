pub mod block;
pub mod expr;
pub mod for_s;
pub mod if_s;
pub mod return_s;
pub mod var_declaration;
pub mod while_s;

use crate::{def_codegen, ir::statement::Statement};

def_codegen!(Statement, |statement| {
    let mut buffer = match statement {
        Statement::VariableDeclaration(inner) => inner.to_c(),
        // Statement::Assignment(inner) => inner.to_c(),
        Statement::Return(inner) => inner.to_c(),
        Statement::Expression(inner) => inner.to_c(),
        Statement::Block(inner) => inner.to_c(),
        Statement::If(inner) => inner.to_c(),
        Statement::While(inner) => inner.to_c(),
        Statement::For(inner) => inner.to_c(),
    }?;

    match &statement {
        Statement::VariableDeclaration(_) | Statement::Return(_) | Statement::Expression(_) => {
            buffer.write(";");
        }
        _ => {}
    }

    Ok(buffer)
});
