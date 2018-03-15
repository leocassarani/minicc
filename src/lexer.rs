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
    Plus,
    Times,
    Divide,
}

impl Token {
    fn from_char(c: char) -> Option<Token> {
        match c {
            '{' => Some(Token::OpenBrace),
            '}' => Some(Token::CloseBrace),
            '(' => Some(Token::OpenParens),
            ')' => Some(Token::CloseParens),
            ';' => Some(Token::Semicolon),
            '-' => Some(Token::Minus),
            '~' => Some(Token::Tilde),
            '!' => Some(Token::Bang),
            '+' => Some(Token::Plus),
            '*' => Some(Token::Times),
            '/' => Some(Token::Divide),
            _ => None,
        }
    }
}

pub struct Lexer<'a> {
    chars: iter::Peekable<str::Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a str) -> Self {
        Lexer {
            chars: source.chars().peekable(),
        }
    }

    pub fn lex(&mut self) -> Option<Vec<Token>> {
        let mut tokens = Vec::new();

        loop {
            match self.chars.next() {
                Some(c) => {
                    let maybe_token = Token::from_char(c).or_else(|| {
                        if c.is_digit(10) {
                            self.lex_number(c)
                        } else if c.is_alphabetic() || c == '_' {
                            self.lex_word(c)
                        } else {
                            None
                        }
                    });

                    if let Some(token) = maybe_token {
                        tokens.push(token);
                    }
                }
                None => break,
            }
        }

        Some(tokens)
    }

    fn lex_number(&mut self, digit: char) -> Option<Token> {
        let mut digits = digit.to_string();

        loop {
            match self.chars.peek() {
                Some(next) => {
                    if next.is_digit(10) {
                        digits.push(*next);
                    } else {
                        break;
                    }
                }
                None => break,
            }

            self.chars.next();
        }

        digits.parse::<u64>().map(Token::NumLiteral).ok()
    }

    fn lex_word(&mut self, ch: char) -> Option<Token> {
        let mut word = ch.to_string();

        loop {
            match self.chars.peek() {
                Some(next) => {
                    if next.is_alphabetic() || next.is_digit(10) || *next == '_' {
                        word.push(*next);
                    } else {
                        break;
                    }
                }
                None => break,
            }

            self.chars.next();
        }

        match word.as_ref() {
            "int" => Some(Token::IntType),
            "return" => Some(Token::Return),
            _ => Some(Token::Identifier(word.into_boxed_str())),
        }
    }
}
