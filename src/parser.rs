use super::*;
use super::Operator::*;
use std::iter::Peekable;

fn expr<T>(tokens: &mut Peekable<T>) -> Result<Ast, ParseError>
where
    T: Iterator<Item = Token>
{
    let lhs = term(tokens)?;

    if let Some(token) = tokens.peek() {
        let operator = match token {
            &Token::Plus => Add,
            &Token::Minus => Sub,
            _ => return Ok(lhs)
        };
        tokens.next();

        return Ok(Ast::BinOp {
            op: operator,
            lhs: Box::new(lhs),
            rhs: Box::new(term(tokens)?)
        })
    } else {
        Ok(lhs)
    }
}

fn term<T>(tokens: &mut Peekable<T>) -> Result<Ast, ParseError>
where
    T: Iterator<Item = Token>
{
    let lhs = factor(tokens)?;

    if let Some(token) = tokens.peek() {
        let operator = match token {
            &Token::Asterisk => Mul,
            &Token::Slash => Div,
            _ => return Ok(lhs)
        };
        tokens.next();

        return Ok(Ast::BinOp {
            op: operator,
            lhs: Box::new(lhs),
            rhs: Box::new(factor(tokens)?)
        })
    } else {
        Ok(lhs)
    }
}

fn factor<T>(tokens: &mut Peekable<T>) -> Result<Ast, ParseError>
where
    T: Iterator<Item = Token>
{
    if let Some(&Token::Lparen) = tokens.peek() {
        tokens.next();
        let node = expr(tokens);            
        if let Some(&Token::Rparen) = tokens.peek() {
            tokens.next();
            node 
        } else {
            Err(ParseError::UnclosedParenthesis)
        }
    } else {
        digit(tokens)
    }
}

fn digit<T>(tokens: &mut Peekable<T>) -> Result<Ast, ParseError>
where
    T: Iterator<Item = Token>
{
    match tokens.next() {
        Some(Token::Minus) => {
            if let Some(Token::Num(ref n)) = tokens.next() {
                Ok(Ast::Digit((*n as i32) * -1))
            } else {
                Err(ParseError::UnexpectedToken)
            }
        },
        Some(Token::Num(ref n)) => Ok(Ast::Digit(*n as i32)),
        None => Err(ParseError::UnexpectedEof),
        _ => Err(ParseError::UnexpectedToken)
    }
}


pub struct Parser;

impl Parser {
    pub fn new() -> Self {
        Self
    }

    pub fn parse(&self, tokens: Vec<Token>) -> Result<Ast, ParseError> {
        let mut peekable = tokens.into_iter().peekable();
        let ast = expr(&mut peekable);
        if let Some(_) = peekable.peek() {
            Err(ParseError::TooMuchTokens)
        } else {
            ast
        }
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
    let expected = Ast::BinOp {
        op: Add,
        lhs: Box::new(Ast::Digit(1)),
        rhs: Box::new(Ast::BinOp {
            op: Mul,
            lhs: Box::new(Ast::Digit(2)),
            rhs: Box::new(Ast::Digit(3))
        })
    };

    assert_eq!(node, Ok(expected))
}

#[test]
fn test_parse_fail_too_much_tokens() {
    let tokens = vec! [
        Token::Num(1),
        Token::Num(2)
    ];
    let node = Parser::new().parse(tokens);
    let expected = ParseError::TooMuchTokens;

    assert_eq!(node, Err(expected))
}

#[test]
fn test_expr() {
    let tokens = vec! [
        Token::Num(4),
        Token::Plus,
        Token::Num(5)
    ];
    let node = expr(&mut tokens.into_iter().peekable());
    let expected = Ast::BinOp {
        op: Add,
        lhs: Box::new(Ast::Digit(4)),
        rhs: Box::new(Ast::Digit(5)),
    };

    assert_eq!(node, Ok(expected))
}

#[test]
fn test_term() {
    let tokens = vec! [
        Token::Num(2),
        Token::Asterisk,
        Token::Num(3)
    ];
    let node = term(&mut tokens.into_iter().peekable());
    let expected = Ast::BinOp {
        op: Mul,
        lhs: Box::new(Ast::Digit(2)),
        rhs: Box::new(Ast::Digit(3)),
    };

    assert_eq!(node, Ok(expected))
}

#[test]
fn test_factor() {
    let tokens = vec! [
        Token::Lparen,
        Token::Num(2),
        Token::Rparen
    ];
    let node = factor(&mut tokens.into_iter().peekable());
    let expected = Ast::Digit(2);

    assert_eq!(node, Ok(expected))
}

#[test]
fn test_factor_fail_unclosed_paren() {
    let tokens = vec! [
        Token::Lparen,
        Token::Num(3)
    ];
    let node = factor(&mut tokens.into_iter().peekable());
    assert_eq!(node, Err(ParseError::UnclosedParenthesis))
}

#[test]
fn test_digit() {
    let tokens = vec! [Token::Num(5)];
    let node = digit(&mut tokens.into_iter().peekable());
    let expected = Ast::Digit(5);

    assert_eq!(node, Ok(expected))
}

#[test]
fn test_digit_minus() {
    let tokens = vec! [
        Token::Minus,
        Token::Num(6)
    ];
    let node = digit(&mut tokens.into_iter().peekable());
    let expected = Ast::Digit(-6);

    assert_eq!(node, Ok(expected))
}