extern crate num;

pub mod eval;
pub mod parser;
pub mod token;

use anyhow::Result;
use eval::eval;
use num::Float;
use parser::parse;
use std::{fmt::Debug, str::FromStr};

pub fn evaluate<T>(code: &str) -> Result<T>
where
    T: Float + FromStr + Debug + Clone + Copy + Into<f64>,
{
    match parse::<T>(code) {
        Ok(tokens) => Ok(eval(&tokens)?),
        Err(e) => Err(e),
    }
}
