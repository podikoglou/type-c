pub mod assignment;
pub mod binary;
pub mod literal;
pub mod member_access;
pub mod method_call;
pub mod unary;

use assignment::Assignment;
use binary::BinaryOperation;
use literal::Literal;
use member_access::MemberAccess;
use method_call::MethodCall;
use unary::UnaryOperation;

#[derive(Debug, Clone)]
pub enum Expression {
    Literal(Literal),
    Variable(String),
    MethodCall(MethodCall),

    // NOTE:
    // 1) Keep in mind that structs are not supported at this moment
    // 2) This is translated to as foo[bar] in C -- **not** foo.bar
    MemberAccess(MemberAccess),
    BinaryOperation(BinaryOperation),
    UnaryOperation(UnaryOperation),
    Assignment(Assignment),
}
