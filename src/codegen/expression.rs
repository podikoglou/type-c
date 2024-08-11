use crate::{
    def_codegen,
    ir::expression::{binary::BinaryOperation, unary::UnaryOperation, Expression},
};

def_codegen!(Expression, |expr| {
    let mut buffer = CodeBuffer::default();

    match &expr {
        Expression::Literal(literal) => return literal.to_c(),
        Expression::Variable(variable) => buffer.write(variable),
        Expression::MethodCall(expr) => {
            buffer.write(expr.name.as_str());
            buffer.write("(");
            buffer.write(
                expr.arguments
                    .iter()
                    .map(|arg| arg.to_c().unwrap().into())
                    .collect::<Vec<String>>()
                    .join(", ")
                    .as_str(),
            );
            buffer.write(")");
        }
        Self::MemberAccess(access) => {
            buffer.write(access.object.to_c()?);
            buffer.write("[");
            buffer.write(access.index.to_c()?);
            buffer.write("]");
        }

        Expression::Assignment(assignment) => {
            let target: String = assignment.left.to_c()?.into();
            let value: String = assignment.right.to_c()?.into();

            buffer.write(target.as_str());
            buffer.write(" = ");
            buffer.write(value.as_str());
        }

        Expression::BinaryOperation(operation) => match operation {
            BinaryOperation::And(a, b) => {
                let a = a.to_c()?;
                let b = b.to_c()?;

                buffer.write(a);
                buffer.write(" && ");
                buffer.write(b);
            }
            BinaryOperation::Or(a, b) => {
                let a = a.to_c()?;
                let b = b.to_c()?;

                buffer.write(a);
                buffer.write(" || ");
                buffer.write(b);
            }
        },

        Expression::UnaryOperation(operation) => match operation {
            UnaryOperation::Minus(val) => {
                buffer.write("-");
                buffer.write(val.to_c()?);
            }
            UnaryOperation::Plus(val) => {
                buffer.write("+");
                buffer.write(val.to_c()?);
            }
            UnaryOperation::Bang(val) => {
                buffer.write("!");
                buffer.write(val.to_c()?);
            }
        },
    }

    Ok(buffer)
});
