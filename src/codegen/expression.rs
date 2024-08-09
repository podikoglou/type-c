use crate::{def_codegen, ir::expression::Expression};
use anyhow::bail;

def_codegen!(Expression, |expr| {
    let mut buffer = CodeBuffer::default();

    match &expr {
        Expression::Literal(literal) => return literal.to_c(),
        Expression::Variable(variable) => buffer.write(variable),

        other => bail!("non-supported expression kind: {:?}", other),
    }

    Ok(buffer)
});
