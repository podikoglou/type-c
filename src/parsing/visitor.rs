use super::ToIR;
use crate::ir::{
    method::{Method, MethodParameter},
    statement::Statement,
    types::Type,
    Program,
};
use swc_ecma_visit::VisitAll;

#[derive(Default)]
pub struct Visitor {
    pub program: Program,

    pub current_function_name: Option<String>,
    pub current_function_params: Option<Vec<MethodParameter>>,
    pub current_function_return_type: Option<Type>,
    pub current_function_body: Option<Vec<Statement>>,
}

impl Visitor {
    fn insert_method(&mut self) {
        self.program.methods.push(Method {
            name: self
                .current_function_name
                .clone()
                .expect("expected function name"),

            parameters: self.current_function_params.clone().unwrap_or_default(),

            return_type: self
                .current_function_return_type
                .clone()
                .unwrap_or(Type::Void),

            body: self.current_function_body.clone().unwrap_or_default(),
        })
    }
}

impl VisitAll for Visitor {
    fn visit_import_decl(&mut self, node: &swc_ecma_ast::ImportDecl) {
        // get name of the module (i.e. package we're importing from)
        let module = node.src.value.to_string();

        // get the items we're importing as a vec of strings
        // this is actually not *that* useful, at least yet.
        let items = node
            .specifiers
            .iter()
            .map(|specifier| {
                let named = &specifier.clone().named().expect("expected named import");

                named.local.sym.to_string()
            })
            .collect::<Vec<String>>();

        // import import to the program
        self.program
            .imports
            .push(crate::ir::Import { module, items });

        <swc_ecma_ast::ImportDecl as swc_ecma_visit::VisitAllWith<Self>>::visit_children_with(
            node, self,
        )
    }

    fn visit_fn_decl(&mut self, node: &swc_ecma_ast::FnDecl) {
        self.current_function_name = Some(node.ident.sym.to_string());

        <swc_ecma_ast::FnDecl as swc_ecma_visit::VisitAllWith<Self>>::visit_children_with(
            node, self,
        );

        // after we're done with visiting the entirety of this function, we can
        // add it to our IR AST
        self.insert_method();

        // clear state
        self.current_function_name = None;
        self.current_function_params = None;
        self.current_function_return_type = None;
    }

    fn visit_function(&mut self, node: &swc_ecma_ast::Function) {
        // get name of the function
        // let name = node.ident.sym.to_string();

        let return_type: Type = {
            match node.return_type.clone() {
                Some(type_ann) => type_ann.type_ann.to_ir().unwrap(),

                None => Type::Void,
            }
        };

        self.current_function_return_type = Some(return_type);

        <swc_ecma_ast::Function as swc_ecma_visit::VisitAllWith<Self>>::visit_children_with(
            node, self,
        )
    }

    fn visit_param(&mut self, node: &swc_ecma_ast::Param) {
        let ident = node.pat.as_ident().unwrap();

        // get name of the function
        let name = &ident.id.sym.to_string();

        // get type of the parameter
        let ts_type = ident.type_ann.clone().expect("type expected").type_ann;

        let parsed_type: Type = ts_type.to_ir().unwrap();

        let parameter = MethodParameter {
            name: name.clone(),
            _type: parsed_type,
        };

        match &mut self.current_function_params {
            Some(params) => params.push(parameter),
            None => self.current_function_params = Some(vec![parameter]),
        }

        <swc_ecma_ast::Param as swc_ecma_visit::VisitAllWith<Self>>::visit_children_with(node, self)
    }

    fn visit_stmt(&mut self, node: &swc_ecma_ast::Stmt) {
        let statement = node.to_ir().unwrap();

        match &mut self.current_function_body {
            Some(body) => body.push(statement),
            None => self.current_function_body = Some(vec![statement]),
        }

        <swc_ecma_ast::Stmt as swc_ecma_visit::VisitAllWith<Self>>::visit_children_with(node, self)
    }
}
