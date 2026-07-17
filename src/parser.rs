
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Token {
    Variable(char),
    Number(i32),
    Equals,
    Add,
    Sub,
    Mul,
    Div,
    LBracket,
    RBracket
}

pub fn parse(src: &str) -> Vec<Token> {
    let mut output: Vec<Token> = Vec::new();

    let mut char_iter = src.chars().peekable();
    loop {
        match char_iter.next() {
            None => break,
            Some('=') => output.push(Token::Equals),
            Some('+') => output.push(Token::Add),
            Some('-') => output.push(Token::Sub),
            Some('*') => output.push(Token::Mul),
            Some('/') => output.push(Token::Div),
            Some('(') => output.push(Token::LBracket),
            Some(')') => output.push(Token::RBracket),
            Some(x) => {
                if x.is_ascii_digit() {
                    let mut number = x.to_string();
                    while let Some(c) = char_iter.peek() {
                        if !c.is_ascii_digit() { break; }
                        number.push(*c);
                    }
    
                    output.push(Token::Number(i32::from_str_radix(&number, 10).unwrap()));
                } else {
                    output.push(Token::Variable(x));
                }
            }
        }
    }

    return output;
}