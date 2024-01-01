use num::Num;
use std::str::FromStr;
use crate::token::{RPNToken, Operator};

pub fn eval<T>(tokens: &[RPNToken<T>]) -> T where T: Num + FromStr + Clone + Copy + Into<f64>
{
    let mut stack: Vec<T> = Vec::new();
    for t in tokens {
        match t {
            RPNToken::Operator(Operator::PLUS) => {
                let n = pop_stack(&mut stack);
                stack.push(n.1 + n.0);
            },
            RPNToken::Operator(Operator::MINUS) => {
                let n = pop_stack(&mut stack);
                stack.push(n.1 - n.0);
            },
            RPNToken::Operator(Operator::MULTI) => {
                let n = pop_stack(&mut stack);
                stack.push(n.1 * n.0);
            },
            RPNToken::Operator(Operator::DIVIDE) => {
                let n = pop_stack(&mut stack);
                stack.push(n.1 / n.0);
            },
            RPNToken::Operator(Operator::POWER) => {
                let n = pop_stack(&mut stack);
                let res = num::pow(n.1, n.0.into() as usize);
                stack.push(res);
            },
            RPNToken::Operator(Operator::LPAREN) | RPNToken::Operator(Operator::RPAREN) => panic!("Parenthesis should have been removed"),
            RPNToken::Operand(n) => stack.push(n.clone()),
        }
    };

    stack.last().unwrap().clone()
}

fn pop_stack<T: Num + FromStr + Clone + Copy + Into<f64>>(stack: &mut Vec<T>) -> (T, T) {
    let n1 = stack.pop().unwrap();
    let n2 = stack.pop().unwrap();
    (n1, n2)
}