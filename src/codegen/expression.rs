use crate::{def_codegen, ir::expression::Expression};

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
    }

    Ok(buffer)
});
