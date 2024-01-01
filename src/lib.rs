extern crate num;

pub mod token;
pub mod parser;
pub mod eval;

use eval::eval;
use num::Num;
use parser::parse;
use std::str::FromStr;

pub fn evaluate<T>(code: &str) -> Result<T, String> where T: Num + FromStr + Clone + Copy + Into<f64>
{
    match parse::<T>(code) {
        Ok(tokens) => Ok(eval(&tokens)),
        Err(e) => Err(e),
    }
}