use crate::parser::Token;



pub struct Equation {
    left: Box<Expr>,
    right: Box<Expr>,
}

pub enum Expr {
    BinOp(BinOp),
    Var(char),
    Int(u32)    // -n = 0 - n
}


pub struct BinOp {
    left: Box<Expr>,
    right: Box<Expr>,
    operation: Operation
}

pub enum Operation {
    Add,
    Sub,
    Mul,
    Div
}


pub fn generate_equation(tokens: Vec<Token>) -> Equation {
    let mut split_tokens = tokens.split(|&token| token == Token::Equals);
    let left_tokens = split_tokens.next().unwrap();
    let right_tokens = split_tokens.next().unwrap();

    return Equation { left: Box::new(Expr::Int(1)), right: Box::new(Expr::Int(1)) };
}

pub fn generate_expr(tokens: Vec<Token>) -> Expr {
    return Expr::Int(1);
}