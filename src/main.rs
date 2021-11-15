mod lexer;
mod repl;
mod token;

fn main() {
    /*let mut l = lexer::new(input);

    let mut t = lexer::token::Token {
        Type: "".to_owned(),
        Literal: "".to_owned(),
    };

    while t.Type != token::EOF {
        println!("{}, {}\n", t.Literal, t.Type);
        t = l.next_token();
    }*/

    repl::start();
}
