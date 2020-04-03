use super::*;
use super::Operator::*;
use std::iter::Peekable;

fn expr<T>(tokens: &mut Peekable<T>) -> Ast
where
    T: Iterator<Item = Token>
{
    let lhs = term(tokens);

    if let Some(token) = tokens.peek() {
        let operator = match token {
            &Token::Plus => Add,
            &Token::Minus => Sub,
            _ => return lhs
        };
        tokens.next();

        return Ast::BinNode {
            op: operator,
            lhs: Box::new(lhs),
            rhs: Box::new(term(tokens))
        }
    } else {
        lhs
    }
}

fn term<T>(tokens: &mut Peekable<T>) -> Ast
where
    T: Iterator<Item = Token>
{
    let lhs = factor(tokens);

    if let Some(token) = tokens.peek() {
        let operator = match token {
            &Token::Asterisk => Mul,
            &Token::Slash => Div,
            _ => return lhs
        };
        tokens.next();

        return Ast::BinNode {
            op: operator,
            lhs: Box::new(lhs),
            rhs: Box::new(term(tokens))
        }
    } else {
        lhs
    }
}

fn factor<T>(tokens: &mut Peekable<T>) -> Ast
where
    T: Iterator<Item = Token>
{
    if let Some(&Token::Lparen) = tokens.peek() {
        tokens.next();
        let node = expr(tokens);            
        if let Some(&Token::Rparen) = tokens.peek() {
            tokens.next();
        }
        node 
    } else {
        digit(tokens)
    }
}

fn digit<T>(tokens: &mut Peekable<T>) -> Ast
where
    T: Iterator<Item = Token>
{
    match tokens.next() {
        Some(Token::Minus) => {
            Ast::Minus {
                val: Box::new(digit(tokens))
            }
        },
        Some(Token::Num(ref n)) => Ast::Digit(*n),
        _ => panic!("unexpected token")
    }
}


pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, tokens: Vec<Token>) -> Option<Ast> {
        let mut peekable = tokens.into_iter().peekable();
        Some(expr(&mut peekable))
    }
}

#[test]
fn test_parse() {
    let tokens = vec! [
        Token::Num(1),
        Token::Plus,
        Token::Num(2),
        Token::Asterisk,
        Token::Num(3)
    ];
    let node = Parser::new().parse(tokens);
    let expected = Ast::BinNode {
        op: Add,
        lhs: Box::new(Ast::Digit(1)),
        rhs: Box::new(Ast::BinNode {
            op: Mul,
            lhs: Box::new(Ast::Digit(2)),
            rhs: Box::new(Ast::Digit(3))
        })
    };

    assert_eq!(node, Some(expected))
}

#[test]
fn test_expr() {
    let tokens = vec! [
        Token::Num(4),
        Token::Plus,
        Token::Num(5)
    ];
    let node = expr(&mut tokens.into_iter().peekable());
    let expected = Ast::BinNode {
        op: Add,
        lhs: Box::new(Ast::Digit(4)),
        rhs: Box::new(Ast::Digit(5)),
    };

    assert_eq!(node, expected)
}

#[test]
fn test_term() {
    let tokens = vec! [
        Token::Num(2),
        Token::Asterisk,
        Token::Num(3)
    ];
    let node = term(&mut tokens.into_iter().peekable());
    let expected = Ast::BinNode {
        op: Mul,
        lhs: Box::new(Ast::Digit(2)),
        rhs: Box::new(Ast::Digit(3)),
    };

    assert_eq!(node, expected)
}

#[test]
fn test_factor() {
    let tokens = vec! [Token::Num(2)];
    let node = factor(&mut tokens.into_iter().peekable());

    assert_eq!(node, Ast::Digit(2))
}

#[test]
fn test_digit() {
    let tokens = vec! [Token::Num(5)];
    let node = digit(&mut tokens.into_iter().peekable());

    assert_eq!(node, Ast::Digit(5))
}

#[test]
fn test_digit_minus() {
    let tokens = vec! [
        Token::Minus,
        Token::Num(6)
    ];
    let node = digit(&mut tokens.into_iter().peekable());
    let expected = Ast::Minus {
        val: Box::new(Ast::Digit(6))
    };

    assert_eq!(node, expected)
}