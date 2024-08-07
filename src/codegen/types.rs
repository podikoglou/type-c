use anyhow::Result;

use crate::ir::Type;

/// Converts [[Type]]s into the C equivalent.
pub fn type_to_c(_type: &Type) -> Result<String> {
    match _type {
        Type::Char => Ok("char".to_string()),

        // this is bad: `number` in js is 64 bits, `int` in C is 32. realistically this should be a long
        Type::Number => Ok("int".to_string()),

        // TODO: make it so that if this type ever occurrs in the code, we #include <stdbool.h> at the start of the program
        Type::Boolean => Ok("bool".to_string()),

        Type::Void => Ok("void".to_string()),

        Type::Pointer(inner) => {
            let inner = type_to_c(inner)?;

            Ok(format!("{}*", inner))
        }
    }
}
