use crate::{ir::method::Method, writer::CodeWriter};

use super::ToC;
use anyhow::Result;

impl ToC for Method {
    /// Converts a [[Method]] into a C method.
    fn to_c(&self) -> Result<CodeWriter> {
        let mut writer = CodeWriter::default();

        let return_type = &self.return_type.to_c()?;

        let params: Vec<String> = self
            .parameters
            .iter()
            .map(ToC::to_c)
            .map(Result::unwrap)
            .map(CodeWriter::into)
            .collect();

        // return type
        writer.concat(return_type);
        writer.write(" ");

        // method name
        writer.write(self.name.as_str());

        // parameters list
        writer.write("(");
        writer.write(params.join(", ").as_str());
        writer.write(")");

        // body
        writer.write_line("{");

        for statement in &self.body {
            writer.concat(&statement.to_c()?);
        }

        writer.write_line("}");

        Ok(writer)
    }
}
