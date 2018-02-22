use std::{iter, str};

#[derive(Debug, PartialEq)]
pub enum Token {
    OpenBrace,
    CloseBrace,
    OpenParens,
    CloseParens,
    Semicolon,
    Return,
    IntType,
    NumLiteral(u64),
    Identifier(Box<str>),
    Minus,
    Tilde,
    Bang,
}

pub fn lex(source: &str) -> Option<Vec<Token>> {
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
                } else if c == '-' {
                    Some(Token::Minus)
                } else if c == '~' {
                    Some(Token::Tilde)
                } else if c == '!' {
                    Some(Token::Bang)
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
            }
            None => break,
        }
    }

    Some(tokens)
}

fn lex_number(chars: &mut iter::Peekable<str::Chars>, digit: char) -> Option<Token> {
    let mut digits = digit.to_string();

    loop {
        match chars.peek() {
            Some(next) => {
                if next.is_digit(10) {
                    digits.push(*next);
                } else {
                    break;
                }
            }
            None => break,
        }

        chars.next();
    }

    digits.parse::<u64>().map(Token::NumLiteral).ok()
}

fn lex_word(chars: &mut iter::Peekable<str::Chars>, ch: char) -> Option<Token> {
    let mut word = ch.to_string();

    loop {
        match chars.peek() {
            Some(next) => {
                if next.is_alphabetic() || next.is_digit(10) || *next == '_' {
                    word.push(*next);
                } else {
                    break;
                }
            }
            None => break,
        }

        chars.next();
    }

    match word.as_ref() {
        "int" => Some(Token::IntType),
        "return" => Some(Token::Return),
        _ => Some(Token::Identifier(word.into_boxed_str())),
    }
}
