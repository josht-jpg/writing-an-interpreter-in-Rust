//use std::collections::HashMap;

pub struct Token {
    pub Type: String,
    pub Literal: String,
}

pub type TokenType = String;

pub const ILLEGAL: &str = "ILLEGAL";
pub const EOF: &str = "EOF";

pub const IDENT: &str = "IDENT";
pub const INT: &str = "INT";

// Operators
pub const ASSIGN: &str = "=";
pub const PLUS: &str = "+";
// Delimiters
pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ";";
pub const LPAREN: &str = "(";
pub const RPAREN: &str = ")";
pub const LBRACE: &str = "{";
pub const RBRACE: &str = "}";
// Keywords
pub const FUNCTION: &str = "FUNCTION";
pub const LET: &str= "LET";


/*fn key_words() -> HashMap<&'static str, &'static str> { 
    HashMap::from([
        ("fn", FUNCTION),
        ("let", LET),
    ])
}

pub fn lookup_identifier(identifier: &str) -> Result<&&str, &str>  {
   // if key_words().contains_key(identifier) {
        return key_words().get(identifier).ok_or(IDENT);
   // }
   // return IDENT;
}*/

pub fn lookup_identifier(identifier: &str) -> &str {
    match identifier {
        "fn" => FUNCTION,
        "let" => LET,
        _ => IDENT
    }
}

