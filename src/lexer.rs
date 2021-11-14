pub mod token;
pub struct Lexer {
    input: String,
    position: i32,
    read_position: i32,
    pub ch: u8,
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
            Literal: ch.to_string(),
        };
    }

    fn is_letter(ch: u8) -> bool {
        return b'a' <= ch && ch <= b'z' || b'A' <= ch && ch <= b'Z' || ch == b'_';
    }

    fn read_identifier(&mut self) -> &str {
        let position = self.position;
        while Lexer::is_letter(self.ch) {
            self.read_char()
        }

        return &self.input[(position as usize)..(self.position as usize)];
    }

    fn skip_whitespace(&mut self) {
        while self.ch == b' ' || self.ch == b'\t' || self.ch == b'\n' || self.ch == b'\r' {
            self.read_char();
        }
    }

    fn is_digit(ch: u8) -> bool {
        return b'0' <= ch && ch <= b'9';
    }

    fn read_number(&mut self) -> &str {
        let position = self.position;
        while Lexer::is_digit(self.ch) {
            self.read_char()
        }
        return &self.input[(position as usize)..(self.position as usize)];
    }

    pub fn next_token(&mut self) -> token::Token {
        let mut tok = token::Token {
            Literal: '\0'.to_string(),
            Type: "".to_owned(),
        };

        self.skip_whitespace();

        if self.ch == 0 {
            tok.Literal = '\0'.to_string();
            tok.Type = token::EOF.to_owned();
            return tok;
        };

        // println!("{}\n", self.ch as char);

        match self.ch as char {
            '=' => {
                tok = Lexer::new_token(token::ASSIGN.to_owned(), self.ch as char);
                self.read_char();
            }
            ';' => {
                tok = Lexer::new_token(token::SEMICOLON.to_owned(), self.ch as char);
                self.read_char();
            }
            '(' => {
                tok = Lexer::new_token(token::LPAREN.to_owned(), self.ch as char);
                self.read_char();
            }
            ')' => {
                tok = Lexer::new_token(token::RPAREN.to_owned(), self.ch as char);
                self.read_char();
            }
            ',' => {
                tok = Lexer::new_token(token::COMMA.to_owned(), self.ch as char);
                self.read_char();
            }
            '+' => {
                tok = Lexer::new_token(token::PLUS.to_owned(), self.ch as char);
                self.read_char();
            }
            '{' => {
                tok = Lexer::new_token(token::LBRACE.to_owned(), self.ch as char);
                self.read_char();
            }
            '}' => {
                tok = Lexer::new_token(token::RBRACE.to_owned(), self.ch as char);
                self.read_char();
            }
            _ => {
                if Lexer::is_letter(self.ch) {
                    tok.Literal = self.read_identifier().to_owned();
                    tok.Type = token::lookup_identifier(&tok.Literal).to_owned();
                    return tok;
                } else if Lexer::is_digit(self.ch) {
                    tok.Type = token::INT.to_owned();
                    tok.Literal = Lexer::read_number(self).to_owned();
                    return tok;
                } else {
                    tok = Lexer::new_token(token::ILLEGAL.to_owned(), self.ch as char)
                }
            }
        }

        self.read_char();
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
