use super::*;
use super::Ast::*;
use super::Operator::*;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum InterpretError {
    /// Division by Zero
    ZeroDivision
}

pub struct Interpreter;

impl Interpreter {
    pub fn new() -> Self {
        Self
    }

    pub fn eval(&self, ast: &Ast) -> Result<i32, InterpretError> {
        match ast {
            Digit(n) => Ok(*n),
            BinOp { op, lhs, rhs } => self.eval_binop(op, lhs, rhs)
        }
    }

    pub fn eval_binop(&self, op: &Operator, lhs: &Ast, rhs: &Ast) -> Result<i32, InterpretError> {
        let lhs = self.eval(lhs)?;
        let rhs = self.eval(rhs)?;

        match *op {
            Add => Ok(lhs + rhs),
            Sub => Ok(lhs - rhs),
            Mul => Ok(lhs * rhs),
            Div if rhs == 0 => Err(InterpretError::ZeroDivision),
            Div => Ok(lhs / rhs)
        }
    }
}

#[test]
fn test_eval() {
    let interp = Interpreter::new();
    let ast = BinOp {
        op: Add,
        lhs: Box::new(Digit(1)),
        rhs: Box::new(BinOp {
            op: Mul,
            lhs: Box::new(Digit(2)),
            rhs: Box::new(Digit(3))
        })
    };

    assert_eq!(interp.eval(&ast), Ok(7))
}

#[test]
fn test_eval_fail() {
    let interp = Interpreter::new();
    let ast = BinOp {
        op: Mul,
        lhs: Box::new(Digit(3)),
        rhs: Box::new(BinOp {
            op: Div,
            lhs: Box::new(Digit(3)),
            rhs: Box::new(Digit(0)),
        })
    };

    assert_eq!(interp.eval(&ast), Err(InterpretError::ZeroDivision))
}