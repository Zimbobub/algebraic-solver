use crate::tree::{Equation, Expr};

mod token;
mod postfix;
mod tree;

fn main() {
    // let src = "a+b*c+d=2";
    let src = "1+x=2";

    let (left_src, right_src) = src.split_once('=').expect("No equals sign");
    if right_src.contains('=') { panic!("More than one equals sign"); }

    let left_infix = token::parse(left_src);
    let right_infix = token::parse(right_src);

    println!("{} {}", left_infix, right_infix);

    let left_postfix = postfix::postfix(left_infix);
    let right_postfix = postfix::postfix(right_infix);

    println!("{} {}", left_postfix, right_postfix);

    let left_expr = Expr::new(left_postfix);
    let right_expr = Expr::new(right_postfix);

    let equation = Equation::new(left_expr, right_expr);

    println!("{:#?}", equation);
}


