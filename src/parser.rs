use lexer::Token;
use std::slice;

#[derive(Debug, PartialEq)]
pub enum AST {
    Program(Box<AST>),
    Function(Box<str>, Box<AST>),
    Return(Box<AST>),
    IntConstant(u64),
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

    if let Some(&Token::Identifier(ref name)) = tokens.next() {
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

        if !consume(tokens, Token::Return) {
            return None;
        }

        if let Some(&Token::NumLiteral(retval)) = tokens.next() {
            let body = AST::Return(Box::new(AST::IntConstant(retval)));

            if !consume(tokens, Token::Semicolon) {
                return None;
            }

            if !consume(tokens, Token::CloseBrace) {
                return None;
            }

            return Some(AST::Function(func_name, Box::new(body)));
        }
    }

    None
}

fn consume(tokens: &mut slice::Iter<Token>, token: Token) -> bool {
    tokens.next().map(|next| *next == token).unwrap_or(false)
}
