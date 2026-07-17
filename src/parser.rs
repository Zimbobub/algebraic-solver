use std::fmt::Display;

#[derive(Debug, Clone)]
pub struct Tokens(pub Vec<Token>);

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


impl Token {
    pub fn precedence(&self) -> u8 {
        return match self {
            Token::Mul | Token::Div => 2,
            Token::Add | Token::Sub => 1,
            _ => 0
        };
    }
}



pub fn parse(src: &str) -> Tokens {
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

    return Tokens(output);
}

impl Display for Tokens {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        return write!(f, "{}", &self.0
            .iter().map(|token| token.to_string()).reduce(|acc, s| acc + &s).unwrap()
        );
    }
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Token::Variable(x) => x.to_string(),
            Token::Number(n) => n.to_string(),
            Token::Equals => "=".to_string(),
            Token::Add => "+".to_string(),
            Token::Sub => "-".to_string(),
            Token::Mul => "*".to_string(),
            Token::Div => "/".to_string(),
            Token::LBracket => "(".to_string(),
            Token::RBracket => ")".to_string(),
        };
        return write!(f,"{}", s);
    }
}
