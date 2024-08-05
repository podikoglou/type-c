use swc_ecma_ast::{Module, ModuleDecl, ModuleItem, Param};

use crate::ir::{Method, MethodParameter, Type};

use super::types::parse_type;

/// Walks a [[Module]] and tries to find all the exported functions, then converts
/// them to IR [[Method]]s so that they can eventually be converted to C methods.
pub fn parse_functions(ast: &Module) -> anyhow::Result<Vec<Method>> {
    Ok(ast
        .body
        .iter()
        .filter(|item| matches!(item, ModuleItem::ModuleDecl(ModuleDecl::ExportDecl(_))))
        .map(|item| {
            let module_decl = item.as_module_decl().unwrap();
            let export_decl = module_decl.as_export_decl().unwrap();
            let function_decl = export_decl.decl.as_fn_decl().unwrap();
            let function = &function_decl.function;

            // get name of the function
            let name = function_decl.ident.sym.to_string();

            // parse params
            let params: Vec<MethodParameter> = function
                .params
                .iter()
                .map(parse_param)
                .map(Result::unwrap)
                .collect();

            // parse return type
            let return_type = function.return_type.clone().unwrap().type_ann;
            let parsed_return_type = parse_type(&return_type).unwrap();

            Method {
                name,
                parameters: params,
                return_type: parsed_return_type,
            }
        })
        .collect())
}

/// Convert an AST [[Param]] into an IR [[MetohdParameter]]
pub fn parse_param(param: &Param) -> anyhow::Result<MethodParameter> {
    let ident = param.pat.as_ident().unwrap();

    // get name of the function
    let name = &ident.id.sym.to_string();

    // get type of the parameter
    let ts_type = ident.type_ann.clone().unwrap().type_ann;

    let parsed_type: Type = parse_type(&ts_type)?;

    Ok(MethodParameter {
        name: name.clone(),
        _type: parsed_type,
    })
}
