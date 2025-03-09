pub mod analyzer;

use crate::ast::nodes::Program;

#[derive(Debug, thiserror::Error)]
pub enum SemanticError {
    #[error("Undefined variable: {0}")]
    UndefinedVariable(String),
    
    #[error("Type mismatch: expected {expected}, found {found}")]
    TypeMismatch {
        expected: String,
        found: String,
    },
    
    #[error("Ritual {0} not found")]
    UndefinedRitual(String),
    
    #[error("Semantic error: {0}")]
    Generic(String),
}

// Funzione pubblica per eseguire l'analisi semantica
pub fn analyze(program: &Program) -> Result<(), Box<dyn std::error::Error>> {
    analyzer::analyze(program)
}