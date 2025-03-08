#[derive(Debug, thiserror::Error)]
pub enum ParseError {
    #[error("Syntax error: {0}")]
    Syntax(String),
    
    #[error("Unexpected token")]
    UnexpectedToken,
    
    #[error("Generic error: {0}")]
    Generic(String),
}