use crate::{def_codegen, ir::expression::Literal};

def_codegen!(Literal, |literal| {
    let mut buffer = CodeBuffer::default();

    match literal {
        Literal::String(val) => {
            // always sanitize your input.
            let val = val.replace("\"", "\\\"");
            let val = val.replace("\n", "\\n");

            buffer.write(format!("\"{}\"", val).as_str())
        }

        Literal::Number(val) => buffer.write(format!("{}", val).as_str()),

        Literal::Boolean(val) => match val {
            true => buffer.write("true"),
            false => buffer.write("false"),
        },

        Literal::Void => {}
    }

    Ok(buffer)
});
