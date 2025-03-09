use logos::Logos;

#[derive(Logos, Debug, PartialEq, Clone, Hash, Eq)]
pub enum Token {
    // Keywords
    #[token("realm")]
    Realm,
    
    #[token("essence")]
    Essence,
    
    #[token("being")]
    Being,
    
    #[token("ritual")]
    Ritual,
    
    #[token("cycle")]
    Cycle,
    
    #[token("seal")]
    Seal,
    
    #[token("perceptions")]
    Perceptions,
    
    #[token("extensions")]
    Extensions,
    
    #[token("memory")]
    Memory,
    
    #[token("hive")]
    Hive,
    
    // Identifiers and literals
    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*", priority = 1, callback = |lex| lex.slice().to_string())]
    Identifier(String),
    
    #[regex(r#""([^"\\]|\\.)*""#, callback = |lex| lex.slice()[1..lex.slice().len()-1].to_string())]
    String(String),
    
    #[regex(r"[0-9]+(\.[0-9]+)?", callback = |lex| lex.slice().to_string())]
    Number(String),
    
    // Symbols
    #[token("(")]
    LParen,
    
    #[token(")")]
    RParen,
    
    #[token("{")]
    LBrace,
    
    #[token("}")]
    RBrace,
    
    #[token("[")]
    LBracket,
    
    #[token("]")]
    RBracket,
    
    #[token(";")]
    Semicolon,
    
    #[token(":")]
    Colon,
    
    #[token(",")]
    Comma,
    
    #[token(".")]
    Dot,
    
    // Skip whitespace and comments
    #[regex(r"[ \t\n\r]+", logos::skip)]
    #[regex(r"//[^\n]*", logos::skip)]
    #[regex(r"/\*([^*]|\*[^/])*\*/", logos::skip)]
    Whitespace,
    
    // Error fallback

    Error,
}