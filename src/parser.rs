use lexer::Token;
use std::slice;

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

pub fn parse(tokens: &[Token]) -> Option<AST> {
    parse_program(&mut tokens.iter())
}

fn parse_program(tokens: &mut slice::Iter<Token>) -> Option<AST> {
    parse_function(tokens).map(|func| AST::Program(Box::new(func)))
}

fn parse_function(tokens: &mut slice::Iter<Token>) -> Option<AST> {
    if !consume(tokens, Token::IntType) {
        return None;
    }

    match tokens.next() {
        Some(&Token::Identifier(ref name)) => {
            let func_name = name.clone();

            if !consume(tokens, Token::OpenParens) {
                return None;
            }

            if !consume(tokens, Token::CloseParens) {
                return None;
            }

            if !consume(tokens, Token::OpenBrace) {
                return None;
            }

            let func = parse_statement(tokens).map(|body| AST::Function(func_name, Box::new(body)));

            if consume(tokens, Token::CloseBrace) {
                func
            } else {
                None
            }
        }
        _ => None,
    }
}

fn parse_statement(tokens: &mut slice::Iter<Token>) -> Option<AST> {
    // The only type of statement that we support is a return statement.
    if !consume(tokens, Token::Return) {
        return None;
    }

    let statement = parse_expression(tokens).map(|expr| AST::Return(Box::new(expr)));

    if consume(tokens, Token::Semicolon) {
        statement
    } else {
        None
    }
}

fn parse_expression(tokens: &mut slice::Iter<Token>) -> Option<AST> {
    match tokens.next() {
        Some(&Token::NumLiteral(num)) => Some(AST::IntConstant(num)),
        Some(ref token) => {
            if let Some(operator) = UnaryOperator::from_token(token) {
                parse_expression(tokens).map(|expr| AST::UnaryOp(operator, Box::new(expr)))
            } else {
                None
            }
        }
        _ => None,
    }
}

fn consume(tokens: &mut slice::Iter<Token>, token: Token) -> bool {
    tokens.next().map(|next| *next == token).unwrap_or(false)
}
