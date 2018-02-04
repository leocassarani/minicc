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

    let tokens = lex(&source);
    println!("{:?}", tokens);
}

#[derive(Debug, PartialEq)]
enum Token {
    OpenBrace,
    CloseBrace,
    OpenParens,
    CloseParens,
    Semicolon,
    Return,
    IntType,
    NumLiteral(u64),
    Identifier(Box<String>),
}

fn lex(source: &str) -> Option<Vec<Token>> {
    let mut tokens = Vec::new();
    let mut chars = source.chars().peekable();

    loop {
        match chars.next() {
            Some(c) => {
                let maybe_token = if c == '{' {
                    Some(Token::OpenBrace)
                } else if c == '}' {
                    Some(Token::CloseBrace)
                } else if c == '(' {
                    Some(Token::OpenParens)
                } else if c == ')' {
                    Some(Token::CloseParens)
                } else if c == ';' {
                    Some(Token::Semicolon)
                } else if c.is_digit(10) {
                    lex_number(&mut chars, c)
                } else if c.is_alphabetic() || c == '_' {
                    lex_word(&mut chars, c)
                } else {
                    None
                };

                if let Some(token) = maybe_token {
                    tokens.push(token);
                }
            },
            None => break,
        }
    }

    Some(tokens)
}

fn lex_number(chars: &mut std::iter::Peekable<std::str::Chars>, digit: char) -> Option<Token> {
    let mut digits = digit.to_string();

    loop {
        match chars.peek() {
            Some(next) => {
                if next.is_digit(10) {
                    digits.push(*next);
                } else {
                    break;
                }
            },
            None => break,
        }

        chars.next();
    }

    digits
        .parse::<u64>()
        .map(Token::NumLiteral)
        .ok()
}

fn lex_word(chars: &mut std::iter::Peekable<std::str::Chars>, ch: char) -> Option<Token> {
    let mut word = ch.to_string();

    loop {
        match chars.peek() {
            Some(next) => {
                if next.is_alphabetic() || next.is_digit(10) || *next == '_' {
                    word.push(*next);
                } else {
                    break;
                }
            },
            None => break,
        }

        chars.next();
    }

    match word.as_ref() {
        "int"    => Some(Token::IntType),
        "return" => Some(Token::Return),
        _        => Some(Token::Identifier(Box::new(word))),
    }
}
