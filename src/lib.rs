pub mod lexer;
pub mod parser;
pub mod interpreter;

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

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ast {
    Digit(i32),
    BinOp {
        op:Operator,
        lhs: Box<Ast>,
        rhs: Box<Ast>
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ParseError {
    UnexpectedToken,
    UnclosedParenthesis,
    UnexpectedOp,
    UnexpectedEof,
    TooMuchTokens,
}