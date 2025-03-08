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

/// Performs semantic analysis on the AST
pub fn analyze(program: &Program) -> Result<(), SemanticError> {
    // Implementazione base dell'analisi semantica
    
    // Controllo per realm duplicati
    let realm_names: std::collections::HashSet<_> = 
        program.realms.iter()
            .map(|realm| &realm.name)
            .collect();
    
    if realm_names.len() != program.realms.len() {
        return Err(SemanticError::Generic("Duplicate realm names found".to_string()));
    }
    
    // Analisi di base per ogni realm
    for realm in &program.realms {
        // Controllo beings duplicati nel realm
        let being_names: std::collections::HashSet<_> = 
            realm.beings.iter()
                .map(|being| &being.name)
                .collect();
        
        if being_names.len() != realm.beings.len() {
            return Err(SemanticError::Generic(
                format!("Duplicate being names in realm {}", realm.name)
            ));
        }
        
        // Analisi dei beings
        for being in &realm.beings {
            // Controllo rituali duplicati
            let ritual_names: std::collections::HashSet<_> = 
                being.rituals.iter()
                    .map(|ritual| &ritual.name)
                    .collect();
            
            if ritual_names.len() != being.rituals.len() {
                return Err(SemanticError::Generic(
                    format!("Duplicate ritual names in being {}", being.name)
                ));
            }
        }
    }
    
    Ok(())
}