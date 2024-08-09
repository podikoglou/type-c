use crate::{
    def_codegen,
    ir::{expression::Expression, statement::Statement},
};
use anyhow::bail;

def_codegen!(Statement, |statement| {
    let mut writer = CodeWriter::default();

    match statement {
        // write variable declaration (not assignment)
        Statement::VariableDeclaration(declaration) => {
            let type_name: String = declaration.var_type.to_c()?.into();

            writer.write(type_name.as_str());
            writer.write(" ");
            writer.write(declaration.name.as_str());

            if let Some(initializer) = &declaration.initializer {
                let initializer: String = initializer.to_c()?.into();

                writer.write(" = ");
                writer.write(initializer.as_str());
            }

            writer.write(";");
        }

        // value assignment
        Statement::Assignment(assignment) => {
            let target: String = assignment.target.to_c()?.into();
            let value: String = assignment.value.to_c()?.into();

            writer.write(target.as_str());
            writer.write(" = ");
            writer.write(value.as_str());
            writer.write(";");
        }

        // return statements
        Statement::ReturnStatement(expression) => {
            let expression: String = expression.to_c()?.into();

            writer.write("return ");
            writer.write(expression.as_str());
            writer.write(";");
        }

        // expression statements (just method calls -- anything else errors)
        Statement::ExpressionStatement(statement) => match statement {
            Expression::MethodCall(call) => {
                writer.write(call.name.as_str());
                writer.write("(");

                // arguments
                writer.write(
                    call.arguments
                        .iter()
                        .map(|arg| arg.to_c().unwrap().into())
                        .collect::<Vec<String>>()
                        .join(", ")
                        .as_str(),
                );

                writer.write(")");

                writer.write(";");
            }

            other => bail!("expression not supported: {:?}", other),
        },
    }

    Ok(writer)
});
