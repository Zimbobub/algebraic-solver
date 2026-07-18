use crate::token::{Token, Tokens};
use std::fmt::Write;



pub fn postfix(infix: Tokens) -> Tokens {
    let mut stack: Vec<Token> = Vec::new();
    let mut postfix: Vec<Token> = Vec::new();

    for token in infix.0 {
        println!("[{}] [{}]", Tokens(stack.clone()), Tokens(postfix.clone()));
        match token {
            Token::Variable(_) | Token::Number(_) => postfix.push(token),
            Token::LBracket => stack.push(token),
            Token::RBracket => {
                // pop until '('
                while let Some(top) = stack.pop_if(|tok| *tok != Token::LBracket) {
                    postfix.push(top);
                }
                // ensure final pop is '('
                if stack.pop().unwrap() != Token::LBracket { panic!("") }
            },
            // operator
            _ => {
                while let Some(top) = stack.last() && *top != Token::LBracket &&
                    (
                        top.precedence() > token.precedence() ||
                        (top.precedence() == token.precedence() && !token.is_right_associative())
                    ) {
                    postfix.push(stack.pop().unwrap());
                }
                stack.push(token);
            }
        };

    }

    for remaining_operator in stack {
        postfix.push(remaining_operator);
    }

    return Tokens(postfix);
}




#[cfg(test)]
mod postfix_test {
    use super::*;
    use crate::token;

    fn convert(src: &str) -> String {
        let infix = token::parse(src);
        let postfix = postfix(infix);

        let mut postfix_string = String::new();
        write!(postfix_string, "{}", postfix).unwrap();

        return postfix_string;
    }

    #[test]
    fn postfix_simple() {
        assert_eq!(convert("1+1"), "11+");
    }

    #[test]
    fn postfix_precedence() {
        assert_eq!(convert("a+b*c+d"), "abc*+d+");
    }

    #[test]
    fn postfix_parentheses() {
        assert_eq!(convert("a*(b+c)/d"), "abc+*d/");
    }
}