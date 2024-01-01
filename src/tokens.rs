#[derive(Debug, PartialEq, Clone)]
pub enum Token {
    Operator(char, u32, u32),
    WholeNumber(i64),
    DecimalNumber(f64),
    FuncCall(String),
    Comma,
    LParen,
    RParen,
    Whitespace,
}

pub const L_ASSOC: u32 = 1;
pub const R_ASSOC: u32 = 2;
