use crate::{def_codegen, ir::expression::Literal};

def_codegen!(Literal, |literal| {
    let mut writer = CodeWriter::default();

    match literal {
        Literal::String(val) => {
            // always sanitize your input.
            let val = val.replace("\"", "\\\"");
            let val = val.replace("\n", "\\n");

            writer.write(format!("\"{}\"", val).as_str())
        }

        Literal::Number(val) => writer.write(format!("{}", val).as_str()),

        Literal::Boolean(val) => match val {
            true => writer.write("true"),
            false => writer.write("false"),
        },

        Literal::Void => {}
    }

    Ok(writer)
});
