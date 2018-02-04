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
    parse_function(tokens).map(|func| {
        AST::Program(Box::new(func))
    })
}

fn parse_function(tokens: &mut slice::Iter<Token>) -> Option<AST> {
    consume(tokens, Token::IntType);

    if let Some(&Token::Identifier(ref name)) = tokens.next() {
        let func_name = name.clone();

        consume(tokens, Token::OpenParens);
        consume(tokens, Token::CloseParens);
        consume(tokens, Token::OpenBrace);
        consume(tokens, Token::Return);

        if let Some(&Token::NumLiteral(retval)) = tokens.next() {
            let body = AST::Return(Box::new(AST::IntConstant(retval)));

            consume(tokens, Token::Semicolon);
            consume(tokens, Token::CloseBrace);

            return Some(
                AST::Function(func_name, Box::new(body))
            );
        }
    }

    None
}

fn consume(tokens: &mut slice::Iter<Token>, token: Token) -> bool {
    tokens
        .next()
        .map(|next| *next == token)
        .unwrap_or(false)
}
