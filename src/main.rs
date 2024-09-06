pub mod codegen;
pub mod ir;
pub mod parsing;

use anyhow::Result;
use codegen::buffer::CodeBuffer;
use codegen::ToC;
use std::env;

fn main() -> Result<()> {
    // try to read cli args
    let args = env::args().collect::<Vec<String>>();
    let file_path = args.last().unwrap().to_string();

    // initialize the two main components of the transpiler:
    //
    // * the visitor: takes care of crawling the AST and
    //   generating the IR representation of the program on the fly
    //
    // * the buffer: the buffer where the C code is written into
    //   by the `ToC` implementations of the IR objects.
    let mut buffer = CodeBuffer::default();

    // Step 1. Parse code into IR AST using the given parser
    //
    // (this is hardcoded for now until we add more parsers)
    let program = parsing::swc::load(file_path)?;

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
