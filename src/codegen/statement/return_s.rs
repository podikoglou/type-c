use crate::{
    def_codegen,
    ir::{
        expression::{Expression, Literal},
        statement::ReturnStatement,
    },
};

def_codegen!(ReturnStatement, |statement| {
    let mut buffer = CodeBuffer::default();

    // in the case that we're returning void, just write "return;"
    // and return early.
    match &statement.0 {
        Expression::Literal(lit) => match lit {
            Literal::Void => {
                buffer.write("return;");

                return Ok(buffer);
            }
            _ => {}
        },
        _ => {}
    }

    let expression: String = statement.0.to_c()?.into();

    buffer.write(format!("return {};", expression));

    Ok(buffer)
});
