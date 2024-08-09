use crate::{
    def_codegen,
    ir::{expression::Expression, statement::Statement},
};
use anyhow::bail;

def_codegen!(Statement, |statement| {
    let mut buffer = CodeBuffer::default();

    match statement {
        // write variable declaration (not assignment)
        Statement::VariableDeclaration(declaration) => {
            let type_name: String = declaration.var_type.to_c()?.into();

            buffer.write(type_name.as_str());
            buffer.write(" ");
            buffer.write(declaration.name.as_str());

            if let Some(initializer) = &declaration.initializer {
                let initializer: String = initializer.to_c()?.into();

                buffer.write(" = ");
                buffer.write(initializer.as_str());
            }

            buffer.write(";");
        }

        // value assignment
        Statement::Assignment(assignment) => {
            let target: String = assignment.target.to_c()?.into();
            let value: String = assignment.value.to_c()?.into();

            buffer.write(target.as_str());
            buffer.write(" = ");
            buffer.write(value.as_str());
            buffer.write(";");
        }

        // return statements
        Statement::ReturnStatement(expression) => {
            let expression: String = expression.to_c()?.into();

            buffer.write("return ");
            buffer.write(expression.as_str());
            buffer.write(";");
        }

        // expression statements (just method calls -- anything else errors)
        Statement::ExpressionStatement(statement) => match statement {
            Expression::MethodCall(call) => {
                buffer.write(call.name.as_str());
                buffer.write("(");

                // arguments
                buffer.write(
                    call.arguments
                        .iter()
                        .map(|arg| arg.to_c().unwrap().into())
                        .collect::<Vec<String>>()
                        .join(", ")
                        .as_str(),
                );

                buffer.write(")");

                buffer.write(";");
            }

            other => bail!("expression not supported: {:?}", other),
        },
    }

    Ok(buffer)
});
