#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Token {
    Num(u32),
    Plus,
    Minus,
    Asterisk,
    Slash,
    Lparen,
    Rparen
}

struct Parser;

struct Lexer <'a> {
    input: &'a str,
}

impl <'a> Lexer <'a> {
    fn new(input: &'a str) -> Self {
        Self { input }
    }

    fn lex(&self) -> Vec<Token> {
        vec! []
    }
}

fn main() {
    println!("Hello, world!");
}


#[test]
fn test_lex() {
    let input = "1 +  20";
    let lexer = Lexer::new(input);
    let result = lexer.lex();
    
    assert_eq!(
        result,
        vec! [
            Token::Num(1),
            Token::Plus,
            Token::Num(20)
        ]
    )
}