mod parser;


fn main() {
    let src = "1+x=2";
    println!("{:?}", parser::parse(src));
}
