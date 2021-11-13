mod token;
pub struct Lexer {
    input: String,
    position: i32,
    read_position: i32,
    ch: u8,
}

impl Lexer {
   fn read_char(&mut self) {
       if (self.read_position as usize) >= self.input.chars().count() {
            self.ch = 0;
       } else {
           // Sketch
            self.ch = self.input.as_bytes()[self.read_position as usize];
       }

       self.position = self.read_position;
       self.read_position += 1;
   }

   fn new_token(token_type: token::TokenType, ch: char) -> token::Token {
        return token::Token {
            Type: token_type,
            Literal: ch,
        }
    }

   pub fn next_token(&mut self) -> token::Token {
       let mut tok = token::Token {
           Literal: '\0',
           Type: "".to_owned(),
       };

       if self.ch == 0 {
        tok.Literal = '\0';
        tok.Type = token::EOF.to_owned();
        return tok;
       };

       match (self.ch as char) {
        '=' => tok = Lexer::new_token(token::ASSIGN.to_owned(), self.ch as char),
        ';' => tok = Lexer::new_token(token::SEMICOLON.to_owned(), self.ch as char),
        '(' => tok = Lexer::new_token(token::LPAREN.to_owned(), self.ch as char),
        ')' => tok = Lexer::new_token(token::RPAREN.to_owned(), self.ch as char),
        ',' => tok = Lexer::new_token(token::COMMA.to_owned(), self.ch as char),
        '+' => tok = Lexer::new_token(token::PLUS.to_owned(), self.ch as char),
        '{' => tok = Lexer::new_token(token::LBRACE.to_owned(), self.ch as char),
        '}' => tok = Lexer::new_token(token::RBRACE.to_owned(), self.ch as char),
        _ => unreachable!()
 
        }

        return tok;
    }
    
}

pub fn new(input: &str) -> Lexer {
    let mut l = Lexer {
        input: input.to_owned(),
        position: 0,
        read_position: 0,
        ch: 0,
    };
    l.read_char();
    return l;
}


