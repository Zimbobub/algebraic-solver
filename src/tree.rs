use std::fmt::{Display, write};

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

    pub fn reduce_terms(&mut self) {
        self.left.reduce_terms();
        self.right.reduce_terms();
    }

    pub fn rearrange_towards(&mut self, variable: char) -> Option<(Operation, Expr)> {
        let variable_on_right = self.search_for_variable(variable)?;

        // if variable_on_right {
        //     match self.right {
        //         Expr::BinOp(b) => {
        //             let variable_on_left = b.left.search_var(variable);
        //             match (variable_on_left, b.operation) {

        //             }
        //         },
        //         _ => None
        //     }
        // } else {

        // }        
        None
    }

    /// - none if not on left or right
    /// - false if on left
    /// - true if on right
    fn search_for_variable(&self, variable: char) -> Option<bool> {
        let variable_on_left = self.left.search_var(variable);
        let variable_on_right = self.right.search_var(variable);

        return match (variable_on_left, variable_on_right) {
            (true, true) => panic!("variable both on left and right not yet implemented"),
            (true, false) => Some(false),
            (false, true) => Some(true),
            (false, false) => None
        };
    }
}

impl Display for Equation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{} = {}", self.left, self.right)
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

    pub fn reduce_terms(&mut self) {
        if let Expr::BinOp(b) = self {
            b.left.reduce_terms();
            b.right.reduce_terms();
            if let Expr::Int(left) = b.left.as_ref() && let Expr::Int(right) = b.right.as_ref() {
                let value = match b.operation {
                    Operation::Add => left + right,
                    Operation::Sub => left - right,
                    Operation::Mul => left * right,
                    Operation::Div => left / right,
                };

                *self = Expr::Int(value);
            }
        }
    }

    /// search for a variable within this expression and sub-expressions
    pub fn search_var(&self, search_term: char) -> bool {
        match self {
            Expr::BinOp(e,) => {
                return e.left.search_var(search_term) || e.right.search_var(search_term);
            },
            Expr::Int(_) => {
                return false
            },
            Expr::Var(x) => {
                return *x == search_term
            } 
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Expr::BinOp(b) => write!(f, "{} {} {}", b.left, b.operation, b.right),
            Expr::Int(n) => write!(f, "{}", n),
            Expr::Var(x) => write!(f, "{}", x)
        };
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

impl Operation {
    pub fn opposite(&self) -> Self {
        match self {
            Operation::Add => Operation::Sub,
            Operation::Sub => Operation::Add,
            Operation::Mul => Operation::Div,
            Operation::Div => Operation::Mul,
        }
    }
}

impl Display for Operation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return match self {
            Operation::Add => write!(f, "+"),
            Operation::Sub => write!(f, "-"),
            Operation::Mul => write!(f, "*"),
            Operation::Div => write!(f, "/"),
        };
    }
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