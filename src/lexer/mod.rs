mod token;

pub use token::Token;
use logos::Logos;

#[derive(Debug, thiserror::Error)]
pub enum LexerError {
    #[error("Lexer error: {0}")]
    Error(String),
}

/// Tokenizes the source code into a stream of tokens
pub fn tokenize(source: &str) -> Result<Vec<Token>, LexerError> {
    let lexer = Token::lexer(source);
    
    // Raccoglie i token, filtrando gli errori
    let mut tokens = Vec::new();
    for token_result in lexer {
        match token_result {
            Ok(token) => tokens.push(token),
            Err(_) => return Err(LexerError::Error("Invalid token".to_string())),
        }
    }
    
    Ok(tokens)
}