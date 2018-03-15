use lexer::Token;
use std::{iter, slice};

#[derive(Debug, PartialEq)]
pub enum AST {
    Program(Box<AST>),
    Function(Box<str>, Box<AST>),
    Return(Box<AST>),
    UnaryOp(UnaryOperator, Box<AST>),
    IntConstant(u64),
}

#[derive(Debug, PartialEq)]
pub enum UnaryOperator {
    Minus,
    Tilde,
    Bang,
}

impl UnaryOperator {
    fn from_token(token: &Token) -> Option<Self> {
        match *token {
            Token::Minus => Some(UnaryOperator::Minus),
            Token::Tilde => Some(UnaryOperator::Tilde),
            Token::Bang => Some(UnaryOperator::Bang),
            _ => None,
        }
    }
}

pub struct Parser<'a> {
    tokens: iter::Peekable<slice::Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Parser {
        Parser {
            tokens: tokens.iter().peekable(),
        }
    }

    pub fn parse(&mut self) -> Option<AST> {
        self.parse_program()
    }

    fn parse_program(&mut self) -> Option<AST> {
        self.parse_function()
            .map(|func| AST::Program(Box::new(func)))
    }

    fn parse_function(&mut self) -> Option<AST> {
        if !self.consume(Token::IntType) {
            return None;
        }

        match self.tokens.next() {
            Some(&Token::Identifier(ref name)) => {
                let func_name = name.clone();

                if !self.consume(Token::OpenParens) {
                    return None;
                }

                if !self.consume(Token::CloseParens) {
                    return None;
                }

                if !self.consume(Token::OpenBrace) {
                    return None;
                }

                let func = self.parse_statement()
                    .map(|body| AST::Function(func_name, Box::new(body)));

                if self.consume(Token::CloseBrace) {
                    func
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn parse_statement(&mut self) -> Option<AST> {
        // The only type of statement that we support is a return statement.
        if !self.consume(Token::Return) {
            return None;
        }

        let statement = self.parse_expression()
            .map(|expr| AST::Return(Box::new(expr)));

        if self.consume(Token::Semicolon) {
            statement
        } else {
            None
        }
    }

    fn parse_expression(&mut self) -> Option<AST> {
        match self.tokens.next() {
            Some(&Token::NumLiteral(num)) => Some(AST::IntConstant(num)),
            Some(ref token) => {
                if let Some(operator) = UnaryOperator::from_token(token) {
                    self.parse_expression()
                        .map(|expr| AST::UnaryOp(operator, Box::new(expr)))
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    fn consume(&mut self, token: Token) -> bool {
        self.tokens
            .next()
            .map(|next| *next == token)
            .unwrap_or(false)
    }
}
