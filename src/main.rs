mod lexer;
mod token;

fn main() {
    let input = "let test = 1 == 3;
    let five = 5;
    let ten = 10;
    let add = fn(x, y) {
    x + y;
    };
    let result = add(five, ten);
    !-/*5;
    5 < 10 > 5;
    if (5 < 10) {
    return true;
    } else {
    return false;
    }";

    let mut l = lexer::new(input);

    let mut t = lexer::token::Token {
        Type: "".to_owned(),
        Literal: "".to_owned(),
    };

    while t.Type != token::EOF {
        println!("{}, {}\n", t.Literal, t.Type);
        t = l.next_token();
    }
}
