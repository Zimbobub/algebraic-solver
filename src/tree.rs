use std::fmt::Display;

use crate::token::{Token, Tokens};


#[derive(Debug)]
pub struct Equation {
    left: Box<Expr>,
    right: Box<Expr>,
}

impl Equation {
    pub fn new(left: Expr, right: Expr) -> Self {
        return Equation { left: Box::new(left), right: Box::new(right) };
    }
}

#[derive(Debug)]
pub enum Expr {
    BinOp(BinOp),
    Var(char),
    Int(i32)    // -n = 0 - n
}

impl Expr {
    pub fn new(tokens: Tokens) -> Self {
        let mut stack: Vec<Expr> = Vec::new();

        for token in tokens.0 {
            match token {
                Token::Number(n) => {
                    stack.push(Expr::Int(n));
                },
                Token::Variable(c) => {
                    stack.push(Expr::Var(c));
                },
                op => {
                    let right = stack.pop().unwrap();
                    let left = stack.pop().unwrap();
                    stack.push(Expr::BinOp(BinOp {
                        left: Box::new(left),
                        right: Box::new(right),
                        operation: Operation::try_from(op).unwrap()
                    }));
                }
            }
        }
        
        if stack.len() != 1 {
            panic!("0 or 2+ items left in stack");
        }

        return stack.pop().unwrap();
    }
}


#[derive(Debug)]
pub struct BinOp {
    left: Box<Expr>,
    right: Box<Expr>,
    operation: Operation
}

#[derive(Debug)]
pub enum Operation {
    Add,
    Sub,
    Mul,
    Div
}



#[derive(Debug)]
pub struct TryFromTokenError {}
impl std::error::Error for TryFromTokenError {}
impl Display for TryFromTokenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "tried to convert token that isn't an operation into an Operation");
    }
}



impl TryFrom<Token> for Operation {
    type Error = TryFromTokenError;

    fn try_from(value: Token) -> Result<Self, Self::Error> {
        return match value {
            Token::Add => Ok(Operation::Add),
            Token::Sub => Ok(Operation::Sub),
            Token::Mul => Ok(Operation::Mul),
            Token::Div => Ok(Operation::Div),
            _ => Err(TryFromTokenError {})
        };
    }
}