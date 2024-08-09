pub mod buffer;
pub mod codegen;
pub mod ir;
pub mod parsing;

use anyhow::Result;
use buffer::CodeBuffer;
use codegen::ToC;
use parsing::visitor::Visitor;
use std::{env, path::Path};
use swc_common::{
    errors::{ColorConfig, Handler},
    sync::Lrc,
    SourceMap,
};
use swc_ecma_parser::{lexer::Lexer, Capturing, Parser, StringInput, Syntax};
use swc_ecma_visit::VisitAll;

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

    // initialize the two main components of the transpiler:
    //
    // * the visitor: takes care of crawling the AST and
    //   generating the IR representation of the program on the fly
    //
    // * the buffer: the buffer where the C code is written into
    //   by the `ToC` implementations of the IR objects.
    let mut visitor = Visitor::default();
    let mut buffer = CodeBuffer::default();

    // Step 1. Visit / walk the entire AST using SWC
    visitor.visit_module(&module);

    // this our IR AST (which is basically a dumbed down and more universal
    // version of swc's AST)
    let program = visitor.program;

    // Step 2. Convert the imports to C includes
    program.imports.iter().for_each(|import| {
        let w = (*import).to_c().unwrap();

        buffer.concat(&w);
    });

    buffer.write_line("");

    // Step 3. Convert the methods to C methods
    program.methods.iter().for_each(|method| {
        let w = (*method).to_c().unwrap();

        buffer.concat(&w);
        buffer.write_line("");
    });

    // output out C code
    println!("{}", Into::<String>::into(buffer));

    Ok(())
}
