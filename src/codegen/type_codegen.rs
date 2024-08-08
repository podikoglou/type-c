use super::ToC;
use crate::{ir::types::Type, writer::CodeWriter};
use anyhow::Result;

impl ToC for Type {
    /// Converts [[Type]]s into the C equivalent.
    fn to_c(&self) -> Result<CodeWriter> {
        Ok(CodeWriter::from(match self {
            Type::Char => "char".to_string(),

            // this is bad: `number` in js is 64 bits, `int` in C is 32. realistically this should be a long
            Type::Number => "int".to_string(),

            // TODO: make it so that if this type ever occurrs in the code, we #include <stdbool.h> at the start of the program
            Type::Boolean => "bool".to_string(),

            Type::Void => "void".to_string(),

            Type::Pointer(inner) => {
                let inner: String = inner.to_c().unwrap().into();

                format!("{}*", inner)
            }
        }))
    }
}
