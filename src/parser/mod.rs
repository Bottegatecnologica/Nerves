use chumsky::prelude::*;
use chumsky::Parser;
use crate::lexer::Token;
use crate::ast::nodes::*;

fn expression_parser() -> impl Parser<Token, Expression, Error = Simple<Token>> {
    select! { 
        Token::Number(num) => Expression::Literal(
            if num.contains('.') {
                Literal::Float(num.parse().unwrap())
            } else {
                Literal::Integer(num.parse().unwrap())
            }
        ),
        Token::String(s) => Expression::Literal(Literal::String(s)),
        Token::Identifier(name) => Expression::Variable(name)
    }.boxed()
}

fn statement_parser() -> impl Parser<Token, Statement, Error = Simple<Token>> {
    choice([
        // Return statement with optional expression
        just(Token::Identifier(String::from("return")))
            .ignore_then(expression_parser().or_not())
            .then_ignore(just(Token::Semicolon))
            .map(Statement::Return)
            .boxed(),
        
        // Empty statement (semicolon)
        just(Token::Semicolon)
            .map(|_| Statement::Return(None))
            .boxed()
    ])
}

// Rest of the parser implementation remains the same...

// Function for parsing statements
fn statement_parser() -> impl Parser<Token, Statement, Error = Simple<Token>> {
    choice([
        // Return statement with optional expression
        just(Token::Identifier(String::from("return")))
            .ignore_then(expression_parser().or_not())
            .then_ignore(just(Token::Semicolon))
            .map(Statement::Return),
        
        // Empty statement (semicolon)
        just(Token::Semicolon)
            .map(|_| Statement::Return(None))
    ])
}

// Add the parse function
pub fn parse(tokens: Vec<Token>) -> Result<Program, Vec<Simple<Token>>> {
    let program_parser = program_parser();
    
    program_parser.parse(tokens)
}

fn program_parser() -> impl Parser<Token, Program, Error = Simple<Token>> {
    realm_parser()
        .repeated()
        .map(|realms| Program { realms })
        .then_ignore(end())
}

fn realm_parser() -> impl Parser<Token, Realm, Error = Simple<Token>> {
    just(Token::Realm)
        .ignore_then(select! { Token::Identifier(name) => name })
        .then_ignore(just(Token::LBrace))
        .then(being_parser().repeated())
        .then_ignore(just(Token::RBrace))
        .map(|(name, beings)| Realm { name, beings })
}

fn being_parser() -> impl Parser<Token, Being, Error = Simple<Token>> {
    just(Token::Being)
        .ignore_then(select! { Token::Identifier(name) => name })
        .then_ignore(just(Token::LBrace))
        .then(variable_parser().repeated().or(empty().to(vec![])))
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
        .ignore_then(select! { Token::Identifier(name) => name })
        .then_ignore(just(Token::LParen))
        .then(parameter_parser().repeated().or(empty().to(vec![])))
        .then_ignore(just(Token::RParen))
        .then(type_parser().or(just(Token::Identifier(String::from("void"))).to(Type::Void)))
        .then_ignore(just(Token::LBrace))
        .then(statement_parser().repeated().or(empty().to(vec![])))
        .then_ignore(just(Token::RBrace))
        .map(|(((name, parameters), return_type), body)| Ritual {
            name, 
            parameters, 
            return_type, 
            body 
        })
}

// Add variable parser
fn variable_parser() -> impl Parser<Token, Variable, Error = Simple<Token>> {
    select! { Token::Identifier(name) => name }
        .then_ignore(just(Token::Colon))
        .then(type_parser())
        .map(|(name, var_type)| Variable { name, var_type })
}

fn parameter_parser() -> impl Parser<Token, Variable, Error = Simple<Token>> {
    select! { Token::Identifier(name) => name }
        .then_ignore(just(Token::Colon))
        .then(type_parser())
        .map(|(name, var_type)| Variable { name, var_type })
}

fn type_parser() -> impl Parser<Token, Type, Error = Simple<Token>> {
    select! {
        Token::Identifier(name) => match name.as_str() {
            "int" => Type::Integer,
            "float" => Type::Float,
            "string" => Type::String,
            "bool" => Type::Boolean,
            "void" => Type::Void,
            _ => Type::Custom(name)
        }
    }
}