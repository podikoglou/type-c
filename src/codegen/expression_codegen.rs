use super::ToC;
use crate::{ir::expression::Expression, writer::CodeWriter};
use anyhow::{bail, Result};

impl ToC for Expression {
    fn to_c(&self) -> Result<CodeWriter> {
        let mut writer = CodeWriter::default();

        match &self {
            Expression::Literal(literal) => return literal.to_c(),
            Expression::Variable(variable) => writer.write(variable),

            other => bail!("non-supported expression kind: {:?}", other),
        }

        Ok(writer)
    }
}
