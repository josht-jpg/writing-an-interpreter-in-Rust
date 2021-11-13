mod lexer;

fn main() {
    let input = "=+(){},;";
    let mut l = lexer::new(input);

    for c in input.chars() { 
        println!("{}\n", c);
        l.next_token();
    }

}
