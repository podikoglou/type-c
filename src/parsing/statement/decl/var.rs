use crate::{
    ir::statement::{Statement, VariableDeclaration},
    parsing::AstToIR,
};
use anyhow::{bail, Result};
use swc_ecma_ast::VarDecl;

impl AstToIR<Statement> for VarDecl {
    fn to_ir(&self) -> Result<Statement> {
        // a Decl can have multiple decls stored in a Vec. I assume this is
        // becuase of stuff like multiple assignments in a single statement?
        //
        // anyway, we don't support this right now.
        if self.decls.len() != 1 {
            bail!("there must be one declaration");
        }

        let decl = self.decls.first().unwrap();

        // decl.name for whatever reason also contains the type annotation if present,
        // so we'll try to unpack the type annotation (and fail if it isn't there)
        let ident = decl.name.clone().ident().unwrap();

        // get the name of the variable
        let name = ident.id.sym.to_string();

        // get the type (annotation) of the variable
        let type_ann = *ident.type_ann.expect("expected type annotation").type_ann;
        let var_type = type_ann.to_ir()?;

        // now, check if we also have an initializer
        let initializer = match &decl.init {
            Some(initializer) => Some(initializer.to_ir()?),
            None => None,
        };

        Ok(Statement::VariableDeclaration(VariableDeclaration {
            name,
            var_type,
            initializer,
        }))
    }
}
