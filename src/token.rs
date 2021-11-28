//use std::collections::HashMap;

//#[derive(Copy, Clone)]
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
pub const MINUS: &str = "-";
pub const BANG: &str = "!";
pub const ASTERISK: &str = "*";
pub const SLASH: &str = "/";

pub const LT: &str = "<";
pub const GT: &str = ">";

pub const EQ: &str = "==";
pub const NOT_EQ: &str = "!=";
// Delimiters
pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ";";
pub const LPAREN: &str = "(";
pub const RPAREN: &str = ")";
pub const LBRACE: &str = "{";
pub const RBRACE: &str = "}";
// Keywords
pub const FUNCTION: &str = "FUNCTION";
pub const LET: &str = "LET";
pub const TRUE: &str = "TRUE";
pub const FALSE: &str = "FALSE";
pub const IF: &str = "IF";
pub const ELSE: &str = "ELSE";
pub const RETURN: &str = "RETURN";

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
        "true" => TRUE,
        "false" => FALSE,
        "if" => IF,
        "else" => ELSE,
        "return" => RETURN,
        _ => IDENT,
    }
}
