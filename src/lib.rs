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
pub enum UnaryOperator {
    Minus
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum BinaryOperator {
    Add,
    Sub,
    Mul,
    Div
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Ast {
    Leaf(u32),
    UniNode {
        op:UnaryOperator,
        val: Box<Ast>
    },
    BinNode {
        op:BinaryOperator,
        lhs: Box<Ast>,
        rhs: Box<Ast>
    }
}
