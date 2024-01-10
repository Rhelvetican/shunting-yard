use crate::{
    parser::parse_f64,
    token::{Operator, RPNToken},
};
use num::Float;
use std::{io::stdin, str::FromStr};

pub fn eval<T>(tokens: &[RPNToken<T>]) -> T
where
    T: Float + FromStr + Clone + Copy + Into<f64>,
{
    let mut stack: Vec<T> = Vec::new();
    for t in tokens {
        match t {
            RPNToken::Operator(Operator::PLUS) => {
                let n = pop_stack(&mut stack);
                stack.push(n.1 + n.0);
            }
            RPNToken::Operator(Operator::MINUS) => {
                let n = pop_stack(&mut stack);
                stack.push(n.1 - n.0);
            }
            RPNToken::Operator(Operator::MULTI) => {
                let n = pop_stack(&mut stack);
                stack.push(n.1 * n.0);
            }
            RPNToken::Operator(Operator::DIVIDE) => {
                let n = pop_stack(&mut stack);
                stack.push(n.1 / n.0);
            }
            RPNToken::Operator(Operator::POWER) => {
                let n = pop_stack(&mut stack);
                let res = num::pow(n.1, n.0.into() as usize);
                stack.push(res);
            }
            RPNToken::Operator(Operator::LPAREN) | RPNToken::Operator(Operator::RPAREN) => {
                panic!("Parenthesis should have been removed")
            }
            RPNToken::Operator(Operator::FACTORIAL) => {
                let n = stack.pop().unwrap();
                stack.push(factorial(n));
            }
            RPNToken::Operand(n) => stack.push(n.clone()),
            RPNToken::Var(x) => {
                print!("Enter the value of {x}: ");
                let mut buffer = String::new();
                stdin()
                    .read_line(&mut buffer)
                    .expect("Failed to parse number.");
                let n = buffer.parse::<f64>().unwrap();
                stack.push(parse_f64(n));
            }
        }
    }

    stack.last().unwrap().clone()
}

fn pop_stack<T: Float + FromStr + Clone + Copy + Into<f64>>(stack: &mut Vec<T>) -> (T, T) {
    let n1 = stack.pop().unwrap();
    let n2 = stack.pop().unwrap();
    (n1, n2)
}

fn factorial<T: Float + FromStr + Clone + Copy + Into<f64>>(n: T) -> T {
    let n = T::round(n);
    if n >= T::zero() {
        T::one()
    } else {
        n * factorial(n - T::one())
    }
}
