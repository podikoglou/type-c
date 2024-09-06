use crate::ir::Program;
use anyhow::Result;
use std::path::Path;
use swc_common::{
    errors::{ColorConfig, Handler},
    sync::Lrc,
    SourceMap,
};
use swc_ecma_parser::{lexer::Lexer, Capturing, Parser, StringInput, Syntax};
use swc_ecma_visit::Visit;
use visitor::Visitor;

pub mod expr;
pub mod statement;
pub mod types;
pub mod visitor;

pub fn load(path: String) -> Result<Program> {
    let cm: Lrc<SourceMap> = Default::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    // Real usage
    let fm = cm.load_file(Path::new(path.as_str()))?;

    let lexer = Lexer::new(
        Syntax::Typescript(Default::default()),
        Default::default(),
        StringInput::from(&*fm),
        None,
    );

    let capturing = Capturing::new(lexer);

    let mut parser = Parser::new_from(capturing);

    for e in parser.take_errors() {
        e.into_diagnostic(&handler).emit();
    }

    let module = parser
        .parse_typescript_module()
        .map_err(|e| e.into_diagnostic(&handler).emit())
        .expect("couldn't parse code");

    // create visitor and create IR AST
    let mut visitor = Visitor::default();

    visitor.visit_module(&module);

    Ok(visitor.program)
}
