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
    And,
    Or,
    Equal,
    NotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
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
            match self.next() {
                Some(c) => {
                    let maybe_token = Token::from_char(c).or_else(|| {
                        if c.is_digit(10) {
                            self.lex_number(c)
                        } else if c.is_alphabetic() || c == '_' {
                            self.lex_word(c)
                        } else {
                            self.lex_multichar_operator(c)
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
            match self.peek() {
                Some(next) => {
                    if next.is_digit(10) {
                        digits.push(*next);
                    } else {
                        break;
                    }
                }
                None => break,
            }

            self.next();
        }

        digits.parse::<u64>().map(Token::NumLiteral).ok()
    }

    fn lex_word(&mut self, ch: char) -> Option<Token> {
        let mut word = ch.to_string();

        loop {
            match self.peek() {
                Some(next) => {
                    if next.is_alphabetic() || next.is_digit(10) || *next == '_' {
                        word.push(*next);
                    } else {
                        break;
                    }
                }
                None => break,
            }

            self.next();
        }

        match word.as_ref() {
            "int" => Some(Token::IntType),
            "return" => Some(Token::Return),
            _ => Some(Token::Identifier(word.into_boxed_str())),
        }
    }

    fn lex_multichar_operator(&mut self, ch: char) -> Option<Token> {
        match ch {
            '&' => self.next()
                .and_then(|next| char_to_token(next, '&', Token::And)),
            '|' => self.next()
                .and_then(|next| char_to_token(next, '|', Token::Or)),
            '=' => self.next()
                .and_then(|next| char_to_token(next, '=', Token::Equal)),
            '!' => self.peek()
                .and_then(|next| char_to_token(*next, '=', Token::NotEqual))
                .map(|token| self.advance_token(token))
                .or_else(|| Some(Token::Bang)),
            '<' => self.peek()
                .and_then(|next| char_to_token(*next, '=', Token::LessThanOrEqual))
                .map(|token| self.advance_token(token))
                .or_else(|| Some(Token::LessThan)),
            '>' => self.peek()
                .and_then(|next| char_to_token(*next, '=', Token::GreaterThanOrEqual))
                .map(|token| self.advance_token(token))
                .or_else(|| Some(Token::GreaterThan)),
            _ => None,
        }
    }

    // Consume the next character and return the given token.
    fn advance_token(&mut self, token: Token) -> Token {
        self.next();
        token
    }

    fn next(&mut self) -> Option<char> {
        self.chars.next()
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }
}

fn char_to_token(ch: char, want: char, token: Token) -> Option<Token> {
    if ch == want {
        Some(token)
    } else {
        None
    }
}
