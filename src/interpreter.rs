use super::*;

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Self
    }

    pub fn eval(&self, ast: &Ast) -> i32 {
        0
    }
}

#[test]
fn test_eval() {
    let interp = Interpreter::new();
    let ast = Ast::BinNode {
        op: Operator::Add,
        lhs: Box::new(Ast::Digit(3)),
        rhs: Box::new(Ast::Digit(4))
    };
    let n = interp.eval(&ast);

    assert_eq!(n, 7)
}