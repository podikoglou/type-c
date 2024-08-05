use swc_ecma_ast::{Module, ModuleDecl, ModuleItem};

use crate::ir::Import;

/// Walks a [[Module]] and tries to find all the imports, and converts them
/// to our custom [[Import]] type so that they can be eventually
/// converted to C `#include` statements.
pub fn parse_imports(ast: &Module) -> anyhow::Result<Vec<Import>> {
    Ok(ast
        .body
        .iter()
        .filter(|item| matches!(item, ModuleItem::ModuleDecl(ModuleDecl::Import(_))))
        .map(|item| {
            let module_decl = item.as_module_decl().unwrap();
            let import = module_decl.as_import().unwrap();

            // the package/module were importing from
            let module = import.src.value.to_string();

            // convert the named imported items into strings
            // (doesn't work for importing the default exported item, for example:
            // `import foo from "bar"`)
            let items: Vec<String> = import
                .specifiers
                .iter()
                .filter(|specifier| matches!(specifier, ImportNamedSpecifier))
                .map(|specifier| specifier.as_named())
                .map(Option::unwrap)
                .map(|specifier| specifier.local.sym.to_string())
                .collect();

            Import { module, items }
        })
        .collect())
}
