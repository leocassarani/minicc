use lexer::Token;
use std::{iter, slice};

#[derive(Debug, PartialEq)]
pub enum AST {
    Program(Box<AST>),
    Function(Box<str>, Box<AST>),
    Return(Box<AST>),
    UnaryOp(UnaryOperator, Box<AST>),
    BinaryOp(BinaryOperator, Box<AST>, Box<AST>),
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

#[derive(Debug, PartialEq)]
pub enum BinaryOperator {
    Plus,
    Minus,
    Times,
    Divide,
}

impl BinaryOperator {
    fn from_token(token: &Token) -> Option<Self> {
        match *token {
            Token::Plus => Some(BinaryOperator::Plus),
            Token::Minus => Some(BinaryOperator::Minus),
            Token::Times => Some(BinaryOperator::Times),
            Token::Divide => Some(BinaryOperator::Divide),
            _ => None,
        }
    }
}

pub struct Parser<'a> {
    tokens: iter::Peekable<slice::Iter<'a, Token>>,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
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
        self.parse_term().and_then(|mut term| {
            loop {
                match self.tokens.peek() {
                    Some(&&Token::Plus) | Some(&&Token::Minus) => {
                        let next = self.tokens.next().unwrap();
                        let op = BinaryOperator::from_token(next).unwrap();

                        if let Some(next_term) = self.parse_term() {
                            term = AST::BinaryOp(op, Box::new(term), Box::new(next_term));
                        } else {
                            return None;
                        }
                    }
                    _ => {
                        break;
                    }
                }
            }

            Some(term)
        })
    }

    fn parse_term(&mut self) -> Option<AST> {
        self.parse_factor().and_then(|mut factor| {
            loop {
                match self.tokens.peek() {
                    Some(&&Token::Times) | Some(&&Token::Divide) => {
                        let next = self.tokens.next().unwrap();
                        let op = BinaryOperator::from_token(next).unwrap();

                        if let Some(next_factor) = self.parse_factor() {
                            factor = AST::BinaryOp(op, Box::new(factor), Box::new(next_factor));
                        } else {
                            return None;
                        }
                    }
                    _ => {
                        break;
                    }
                }
            }

            Some(factor)
        })
    }

    fn parse_factor(&mut self) -> Option<AST> {
        match self.tokens.next() {
            Some(&Token::NumLiteral(num)) => Some(AST::IntConstant(num)),
            Some(&Token::OpenParens) => {
                let expr = self.parse_expression();
                if expr.is_some() && self.consume(Token::CloseParens) {
                    expr
                } else {
                    None
                }
            }
            Some(ref token) => {
                if let Some(op) = UnaryOperator::from_token(token) {
                    self.parse_factor()
                        .map(|factor| AST::UnaryOp(op, Box::new(factor)))
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
