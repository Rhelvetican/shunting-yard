use crate::token::{Operator, RPNToken};
use num::Float;
use std::{fmt::Debug, str::FromStr};

pub fn parse<T: Float + Debug + FromStr + Clone + Copy>(
    code: &str,
) -> Result<Vec<RPNToken<T>>, String> {
    let tokens = code.chars().filter(|c| !c.is_whitespace());
    let mut output: Vec<RPNToken<T>> = Vec::new();
    let mut stack: Vec<Operator> = Vec::new();
    let mut num: String = String::new();
    let mut neg: bool = true;
    let mut var: char;

    for tok in tokens {
        if tok.is_numeric() || tok == '.' {
            num.push(tok);
            neg = false;
        } else {
            if tok == '-' && neg {
                num.push('-');
                neg = false;
                continue;
            }

            if !num.is_empty() {
                let n = match num.parse::<T>() {
                    Ok(n) => n,
                    Err(_) => return Err(String::from("Failed to parse number.")),
                };
                let rpnt: RPNToken<T> = RPNToken::Operand(n);
                output.push(rpnt);
                num.clear();
            };

            if tok.is_alphabetic() {
                var = tok.clone();
                let rpnt: RPNToken<T> = RPNToken::Var(var);
                output.push(rpnt);
                continue;
            };

            match Operator::try_from_char(tok) {
                Some(Operator::LPAREN) => {
                    stack.push(Operator::LPAREN);
                    neg = true;
                }
                Some(Operator::RPAREN) => {
                    while let Some(op) = stack.pop() {
                        if op == Operator::LPAREN {
                            break;
                        }
                        output.push(RPNToken::Operator(op));
                    }
                    neg = false;
                }
                Some(Operator::FACTORIAL) => {
                    output.push(RPNToken::Operator(Operator::FACTORIAL));
                    neg = false;
                }
                Some(tokop) => {
                    while {
                        if let Some(&qe) = stack.last() {
                            tokop.val() <= qe.val()
                        } else {
                            false
                        }
                    } {
                        output.push(RPNToken::Operator(stack.pop().unwrap()));
                    }
                    stack.push(tokop);
                    neg = true;
                }
                None => return Err(String::from("Invalid operator.")),
            }
        };
    }

    if !num.is_empty() {
        let n = match num.parse::<T>() {
            Ok(n) => n,
            Err(_) => return Err(String::from("Failed to parse number.")),
        };
        let rpnt = RPNToken::Operand(n);
        output.push(rpnt);
    }

    while let Some(v) = stack.pop() {
        output.push(RPNToken::Operator(v));
    }

    Ok(output)
}

pub fn parse_f64<T: Float + FromStr + Clone + Copy>(n: f64) -> T {
    match T::from_str(&n.to_string()) {
        Ok(n) => n,
        Err(_) => panic!("Failed to parse number."),
    }
}
