use super::*;

pub struct Parser {
    tokens: Vec<Token>
}

impl Parser {
    pub fn new(tokens: Vec<Token>) -> Self {
        Self { tokens }
    }

    pub fn parse(&self) -> Node {
        unimplemented!()
    }
}

#[test]
fn test_parse() {
    let tokens = vec! [
        Token::Num(1),
        Token::Plus,
        Token::Num(2)
    ];
    let ast = Parser::new(tokens).parse();
    let expected = Node::BinaryOp {
        op: Operator::Add,
        lhs: Box::new(Node::Operand(1)),
        rhs: Box::new(Node::Operand(2))
    };
    assert_eq!(ast, expected);
}