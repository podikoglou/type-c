use crate::{def_codegen, ir::expression::Expression};
use anyhow::bail;

def_codegen!(Expression, |expr| {
    let mut buffer = CodeBuffer::default();

    match &expr {
        Expression::Literal(literal) => return literal.to_c(),
        Expression::Variable(variable) => buffer.write(variable),

        Expression::MethodCall(expr) => {
            buffer.write(expr.name.as_str());
            buffer.write("(");

            // arguments
            buffer.write(
                expr.arguments
                    .iter()
                    .map(|arg| arg.to_c().unwrap().into())
                    .collect::<Vec<String>>()
                    .join(", ")
                    .as_str(),
            );

            buffer.write(")");

            buffer.write(";");
        }
        other => bail!("non-supported expression kind: {:?}", other),
    }

    Ok(buffer)
});
