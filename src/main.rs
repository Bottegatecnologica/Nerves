mod lexer;
mod parser;
mod ast;
mod semantic;
mod codegen;
mod seal;
mod runtime;

use std::error::Error;
use std::fs;
use std::env;
use std::path::{Path, PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Nervs Compiler - Starting");
    
    // Controlla se è stato fornito un file
    let args: Vec<String> = env::args().collect();
    
    let source = if args.len() > 1 {
        // Ottieni il percorso del file
        let file_arg = &args[1];
        
        // Costruisci il percorso completo
        let file_path = if Path::new(file_arg).is_absolute() {
            PathBuf::from(file_arg)
        } else {
            // Se è un percorso relativo, prova prima nella directory corrente
            let current_dir_path = PathBuf::from(file_arg);
            if current_dir_path.exists() {
                current_dir_path
            } else {
                // Altrimenti, prova nella directory parent (NERVS)
                let parent_dir_path = Path::new("..").join(file_arg);
                if parent_dir_path.exists() {
                    parent_dir_path
                } else {
                    // Se non esiste neanche lì, usa il percorso originale e lascia
                    // che il sistema di lettura file gestisca l'errore
                    PathBuf::from(file_arg)
                }
            }
        };
        
        println!("Reading source from file: {}", file_path.display());
        fs::read_to_string(file_path)?
    } else {
        // Usa l'esempio incorporato
        println!("No input file specified, using built-in example");
        String::from(r#"
        realm TestRealm {
            being TestBeing {
                ritual hello() {
                    return "Hello, Nervs!";
                }
            }
        }
        "#)
    };
    
    // Test del lexer
    match lexer::tokenize(&source) {
        Ok(tokens) => {
            println!("Lexing successful! Found {} tokens", tokens.len());
            
            // Stampa i primi 20 token (o meno se ce ne sono meno)
            println!("\nToken preview (first 20):");
            for (i, token) in tokens.iter().enumerate().take(20) {
                println!("  {}: {:?}", i, token);
            }
            
            if tokens.len() > 20 {
                println!("  ... and {} more", tokens.len() - 20);
            }

            // Test del parser
            match parser::parse(tokens) {
                Ok(program) => {
                    println!("\nParsing successful!");
                    println!("Parsed {} realm(s)", program.realms.len());
                },
                Err(errors) => {
                    println!("Parsing errors: {:?}", errors);
                }
            }
        },
        Err(e) => println!("Lexing error: {}", e),
    }
    
    println!("\nCompilation process complete");
    Ok(())
}