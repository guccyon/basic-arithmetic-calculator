#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    Num(u32),
    Plus,
    Minus,
    Asterisk,
    Slash,
    Lparen,
    Rparen
}

pub struct Parser;

pub struct Lexer <'a> {
    input: &'a str,
}

impl <'a> Lexer <'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }

    pub fn lex(&self) -> Vec<Token> {
        let mut tokens = Vec::new();

        let mut pos = 0;
        let chars:Vec<char> = self.input.chars().collect();
        while let Some(value) = chars.get(pos) {
            let result = match value {
                '+' => (Token::Plus, pos),
                '-' => (Token::Minus, pos),
                '*' => (Token::Asterisk, pos),
                '/' => (Token::Slash, pos),
                '(' => (Token::Lparen, pos),
                ')' => (Token::Rparen, pos),
                n if "1234567890".contains(*n) => self.lex_numbers(&chars, pos),
                _ => {
                    pos += 1;
                    continue
                }
            };
            pos = result.1 + 1;
            tokens.push(result.0);
        }

        tokens
    }

    fn lex_numbers(&self, chars: &Vec<char>, start:usize) -> (Token, usize) {
        let mut pos = start;
        while chars.len() > pos && "1234567890".contains(chars[pos]) {
            pos += 1;
        }

        let value = chars[start..pos]
            .into_iter()
            .collect::<String>()
            .parse::<u32>()
            .unwrap();
        (Token::Num(value), pos - 1)
    }
}

fn main() {
    let input = "1 + 20 * 4 / ( 2 - -2)";
    let result = Lexer::new(input).lex();
    println!("{:?}", result);
}


#[test]
fn test_lex() {
    let input = "1 + 20 * 4 / ( 2 - -2)";
    let lexer = Lexer::new(input);
    let result = lexer.lex();

    assert_eq!(
        result,
        vec! [
            Token::Num(1),
            Token::Plus,
            Token::Num(20),
            Token::Asterisk,
            Token::Num(4),
            Token::Slash,
            Token::Lparen,
            Token::Num(2),
            Token::Minus,
            Token::Minus,
            Token::Num(2),
            Token::Rparen,
        ]
    )
}