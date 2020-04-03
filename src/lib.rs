pub mod lexer;
pub mod parser;

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
    Digit(u32),
    Minus {
        val: Box<Ast>
    },
    BinNode {
        op:Operator,
        lhs: Box<Ast>,
        rhs: Box<Ast>
    }
}
