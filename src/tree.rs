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
    Int(u32)    // -n = 0 - n
}

impl Expr {
    pub fn new(tokens: Tokens) -> Self {
        return Self::Int(1);
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
