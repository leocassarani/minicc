mod codegen;
mod lexer;
mod parser;

use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use std::process::Command;

use parser::Parser;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("usage: minicc [input.c]");
        std::process::exit(1);
    }

    let filepath = Path::new(&args[0]);
    let source = read_source(filepath);

    let asm = lexer::lex(&source)
        .and_then(|tokens| Parser::new(&tokens).parse())
        .map(|ast| codegen::generate(ast));

    match asm {
        Some(lines) => {
            let asm_filename = filepath.with_extension("s");
            write_assembly(&asm_filename, lines.join("\n").as_bytes());

            let binary_filename = filepath.with_extension("");
            assemble(&asm_filename, &binary_filename);
        }
        None => {
            eprintln!("an error has occurred");
            std::process::exit(1);
        }
    }
}

fn read_source(filepath: &Path) -> String {
    let mut source = String::new();

    File::open(filepath)
        .expect("file not found")
        .read_to_string(&mut source)
        .expect("couldn't read from file");

    source
}

fn write_assembly(path: &Path, bytes: &[u8]) {
    File::create(path)
        .expect("couldn't open file for writing")
        .write_all(bytes)
        .expect("couldn't write output");
}

fn assemble(asm_filename: &Path, binary_filename: &Path) {
    Command::new("gcc")
        .args(&[
            "-o",
            binary_filename.to_str().unwrap(),
            asm_filename.to_str().unwrap(),
        ])
        .output()
        .expect("failed to invoke gcc");
}
