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
pub enum Node {
    Operand(u32),
    UnaryOp {
        op:Operator, node: Box<Node>
    },
    BinaryOp {
        op:Operator,
        lhs: Box<Node>,
        rhs: Box<Node>
    }
}