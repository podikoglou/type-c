pub mod block;
pub mod expr;
pub mod if_s;
pub mod return_s;
pub mod var_decl;

use block::BlockStatement;
use expr::ExpressionStatement;
use if_s::IfStatement;
use return_s::ReturnStatement;
use var_decl::VariableDeclaration;

#[derive(Debug, Clone)]
pub enum Statement {
    VariableDeclaration(VariableDeclaration),

    If(IfStatement),
    // WhileLoop(WhileLoop),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
    Block(BlockStatement),
}
