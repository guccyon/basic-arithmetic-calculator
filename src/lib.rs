pub mod lexer;

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
