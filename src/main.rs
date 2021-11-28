mod ast;
mod lexer;
mod parser;
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

    // repl::start();

    let input = "let x = 5;
   let y = 10;
   let foobar = 838383;";

    let l = lexer::new(input);
    let mut p = parser::new(l);

    let program = p.parse_program();

    if program.Statements.len() != 3 {
        println!(
            "Error: program does not contain 3 elements, it contains {}.",
            program.Statements.len()
        );
        return;
    }

    let mut tests = [""; 3];
    tests[0] = "x";
    tests[1] = "y";
    tests[2] = "foobar";

    for n in 0..3 {
        if program.Statements[n].token_literal() != "let" {
            println!("Print");
            return;
        }

        println!("{}", program.Statements[n].name.Value);

        if program.Statements[n].name.Value != tests[n] {
            println!(
                "s.name.value not {}. got={}",
                tests[n], program.Statements[n].name.Value
            );
            return;
        }

        if program.Statements[n].name.token_literal() != tests[n] {
            println!(
                "s.name.token_literal not {}. got={}",
                tests[n],
                program.Statements[n].name.token_literal()
            );
            return;
        }

        return;
    }
}
