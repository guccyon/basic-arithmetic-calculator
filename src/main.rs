use basic_arithmetic_calculator::lexer::*;

fn main() {
    let input = "1 + 20 * 4 / ( 2 - -2)";
    let result = Lexer::new(input).lex();
    println!("{:?}", result);
}
