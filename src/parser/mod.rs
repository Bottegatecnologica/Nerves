pub mod error;

pub fn parse(_tokens: Vec<crate::lexer::Token>) -> Result<(), error::ParseError> {
    // Da implementare
    Err(error::ParseError::Generic("Parser not yet implemented".to_string()))
}