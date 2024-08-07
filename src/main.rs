pub mod codegen;
pub mod ir;
pub mod parsing;
pub mod writer;

use std::{env, path::Path};

use anyhow::Result;
use codegen::{imports::imports_to_includes, methods::method_to_c};
use parsing::{functions::parse_functions, imports::parse_imports};
use swc_common::{
    errors::{ColorConfig, Handler},
    sync::Lrc,
    SourceMap,
};
use swc_ecma_parser::{lexer::Lexer, Capturing, Parser, StringInput, Syntax};
use writer::CodeWriter;

fn main() -> Result<()> {
    // try to read cli args
    let args = env::args().collect::<Vec<String>>();
    let file_path = args.last().unwrap();

    let cm: Lrc<SourceMap> = Default::default();
    let handler = Handler::with_tty_emitter(ColorConfig::Auto, true, false, Some(cm.clone()));

    // Real usage
    let fm = cm.load_file(Path::new(file_path))?;

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

    let mut writer = CodeWriter::default();

    // Step 1.1 Parse TS imports into IR ones
    let imports = parse_imports(&module)?;

    // Step 1.2 Convert IR imports to C includes
    let includes = imports_to_includes(&imports)?;
    writer.concat(&includes);

    // Step 2.1 Parse TS functions into IR methods
    let methods = parse_functions(&module)?;

    // Step 2.2 Convert IR methods to C methods
    methods
        .iter()
        .map(method_to_c)
        .map(Result::unwrap)
        .for_each(|w| {
            writer.concat(&w);
        });

    // output out C code
    println!("{}", writer.code());

    Ok(())
}
