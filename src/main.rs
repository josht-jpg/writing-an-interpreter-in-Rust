mod ast;
mod lexer;
mod parser;
mod repl;
mod token;

fn check_parse_errors(p: parser::Parser) {
    let errors = p.errors();
    if errors.len() == 0 {
        return;
    }

    println!("Parser has {} errors", errors.len());
    for error in errors.iter() {
        println!("parser error: {}", error);
    }
    return;
}

fn main() {
    let input = "let x = 5;
   let y = 10;
   let 838383;";

    let l = lexer::new(input);
    let mut p = parser::new(l);

    let program = p.parse_program();
    check_parse_errors(p);

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
