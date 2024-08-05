use crate::ir::Type;

/// Converts [[Type]]s into the C equivalent.
pub fn type_to_c(_type: &Type) -> anyhow::Result<String> {
    match _type {
        Type::Char => Ok("char".to_string()),
        Type::Number => Ok("long".to_string()),
        Type::Boolean => Ok("bool".to_string()),
        Type::Void => Ok("void".to_string()),

        Type::Pointer(inner) => {
            let inner = type_to_c(inner)?;

            Ok(format!("{}*", inner))
        }
    }
}
