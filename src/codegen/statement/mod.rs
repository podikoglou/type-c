pub mod block;
pub mod expr;
pub mod if_s;
pub mod return_s;
pub mod var_declaration;

use crate::{def_codegen, ir::statement::Statement};

def_codegen!(Statement, |statement| {
    match statement {
        Statement::VariableDeclaration(inner) => inner.to_c(),
        // Statement::Assignment(inner) => inner.to_c(),
        Statement::Return(inner) => inner.to_c(),
        Statement::Expression(inner) => inner.to_c(),
        Statement::Block(inner) => inner.to_c(),
        Statement::If(inner) => inner.to_c(),
    }
});
