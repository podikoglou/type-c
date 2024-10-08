pub mod block;
pub mod expr;
pub mod for_s;
pub mod if_s;
pub mod return_s;
pub mod var_decl;
pub mod while_s;

use block::BlockStatement;
use expr::ExpressionStatement;
use for_s::ForStatement;
use if_s::IfStatement;
use return_s::ReturnStatement;
use var_decl::VariableDeclaration;
use while_s::WhileStatement;

#[derive(Debug, Clone)]
pub enum Statement {
    VariableDeclaration(VariableDeclaration),

    If(IfStatement),
    While(WhileStatement),
    For(ForStatement),
    Return(ReturnStatement),
    Expression(ExpressionStatement),
    Block(BlockStatement),
}
