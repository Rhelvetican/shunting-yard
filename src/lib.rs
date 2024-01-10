extern crate num;

pub mod eval;
pub mod parser;
pub mod token;

use eval::eval;
use num::Float;
use parser::parse;
use std::str::FromStr;

pub fn evaluate<T>(code: &str) -> Result<T, String>
where
    T: Float + FromStr + Clone + Copy + Into<f64>,
{
    match parse::<T>(code) {
        Ok(tokens) => Ok(eval(&tokens)),
        Err(e) => Err(e),
    }
}
