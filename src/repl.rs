use std::io;

use crate::lexer;
use crate::token;

const PROMPT: &str = ">> ";

pub fn start() {
    while true {
        //  print!(">> ");

        let mut input = String::new();
        io::stdin().read_line(&mut input);

        let mut l = lexer::new(&input);

        let mut t = token::Token {
            Type: "".to_owned(),
            Literal: "".to_owned(),
        };

        while t.Type != token::EOF {
            t = l.next_token();
            println!("{}, {}\n", t.Literal, t.Type);
        }
    }
}
