use crate::parser::{Token, Tokens};
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


