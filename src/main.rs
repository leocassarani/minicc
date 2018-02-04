mod lexer;
mod parser;

use std::env;
use std::fs::File;
use std::io::Read;

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("usage: minicc [input.c]");
        std::process::exit(1);
    }

    let filename = &args[0];
    let mut source = String::new();
    let mut file = File::open(filename).expect("file not found");
    file.read_to_string(&mut source).expect("couldn't read from file");

    let ast = lexer::lex(&source)
        .and_then(|tokens| parser::parse(&tokens));

    println!("{:?}", ast);
}
