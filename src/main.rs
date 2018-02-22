mod codegen;
mod lexer;
mod parser;

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::process::Command;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("usage: minicc [input.c]");
        std::process::exit(1);
    }

    let filename = &args[0];
    let mut source = String::new();
    let mut file = File::open(filename).expect("file not found");
    file.read_to_string(&mut source)
        .expect("couldn't read from file");

    let asm = lexer::lex(&source)
        .and_then(|tokens| parser::parse(&tokens))
        .map(|ast| codegen::generate(ast));

    if let Some(lines) = asm {
        let mut out = File::create("out.s").expect("couldn't open file for writing");

        out.write_all(lines.join("\n").as_bytes())
            .expect("couldn't write output");

        out.write(b"\n").unwrap();

        Command::new("gcc")
            .args(&["-o", "out", "out.s"])
            .output()
            .expect("failed to invoke gcc");
    }
}
