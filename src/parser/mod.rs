use chumsky::prelude::*;
use crate::lexer::Token;
use crate::ast::nodes::*;
use crate::parser::error::ParseError;

pub fn parse(tokens: Vec<Token>) -> Result<Program, ParseError> {
    let realm_parser = realm_parser();
    
    realm_parser
        .parse(tokens)
        .map_err(|errors| ParseError::Syntax(
            errors.into_iter()
                .map(|e| e.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        ))
}

fn realm_parser() -> impl Parser<Token, Realm, Error = Simple<Token>> {
    // Implementazione base del parser per un Realm
    just(Token::Realm)
        .ignore_then(identifier())
        .then_ignore(just(Token::LBrace))
        .then(being_parser().repeated())
        .then_ignore(just(Token::RBrace))
        .map(|(name, beings)| Realm { name, beings })
}

fn being_parser() -> impl Parser<Token, Being, Error = Simple<Token>> {
    just(Token::Being)
        .ignore_then(identifier())
        .then_ignore(just(Token::LBrace))
        .then(variable_parser().repeated())
        .then(ritual_parser().repeated())
        .then_ignore(just(Token::RBrace))
        .map(|((name, variables), rituals)| Being { 
            name, 
            variables, 
            rituals 
        })
}

fn ritual_parser() -> impl Parser<Token, Ritual, Error = Simple<Token>> {
    just(Token::Ritual)
        .ignore_then(identifier())
        .then_ignore(just(Token::LParen))
        .then(parameter_parser().repeated().separated_by(just(Token::Comma)))
        .then_ignore(just(Token::RParen))
        .then(type_parser())
        .then_ignore(just(Token::LBrace))
        .then(statement_parser().repeated())
        .then_ignore(just(Token::RBrace))
        .map(|(((name, parameters), return_type), body)| Ritual {
            name, 
            parameters, 
            return_type, 
            body 
        })
}

fn identifier() -> impl Parser<Token, String, Error = Simple<Token>> {
    select! { Token::Identifier => todo!("Implementare estrazione identificatore") }
}

fn type_parser() -> impl Parser<Token, Type, Error = Simple<Token>> {
    todo!("Implementare parser per i tipi")
}

fn parameter_parser() -> impl Parser<Token, Variable, Error = Simple<Token>> {
    todo!("Implementare parser per parametri")
}

fn statement_parser() -> impl Parser<Token, Statement, Error = Simple<Token>> {
    todo!("Implementare parser per statement")
}

fn expression_parser() -> impl Parser<Token, Expression, Error = Simple<Token>> {
    todo!("Implementare parser per espressioni")
}