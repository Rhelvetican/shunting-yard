use num::Num;

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum RPNToken<T: Num + Copy> {
    Operand(T),
    Operator(Operator),
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Operator {
    PLUS,
    MINUS,
    MULTI,
    DIVIDE,
    POWER,
    FACTORIAL,
    LPAREN,
    RPAREN,
}

impl Operator {
    pub fn val(&self) -> u32 {
        match *self {
            Operator::LPAREN | Operator::RPAREN => 0,
            Operator::PLUS | Operator::MINUS => 1,
            Operator::MULTI | Operator::DIVIDE => 2,
            Operator::POWER => 3,
            Operator::FACTORIAL => 4,
        }
    }

    pub fn try_from_char(c: char) -> Option<Operator> {
        Some(match c {
            '+' => Operator::PLUS,
            '-' => Operator::MINUS,
            '*' => Operator::MULTI,
            '/' => Operator::DIVIDE,
            '^' => Operator::POWER,
            '(' => Operator::LPAREN,
            ')' => Operator::RPAREN,
            _ => return None,
        })
    }
}
