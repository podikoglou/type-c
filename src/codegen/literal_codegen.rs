use super::ToC;
use crate::{ir::expression::Literal, writer::CodeWriter};
use anyhow::Result;

impl ToC for Literal {
    fn to_c(&self) -> Result<CodeWriter> {
        let mut writer = CodeWriter::default();

        match self {
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
    }
}
