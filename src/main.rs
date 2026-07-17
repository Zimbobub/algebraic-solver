mod parser;
mod postfix;
mod tree;

fn main() {
    let src = "1+x=2";
    let (left_src, right_src) = split(src).expect("No equals sign");

    let left_infix = parser::parse(left_src);
    let right_infix = parser::parse(right_src);

    println!("{:?} {:?}", left_infix, right_infix);

    let left_postfix = postfix::postfix(left_infix);
    let right_postfix = postfix::postfix(right_infix);
}



fn split(src: &str) -> Option<(&str, &str)> {
    return Some(src.split_at(src.find('=')?));
}