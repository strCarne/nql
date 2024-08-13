use std::fmt::Display;

pub mod float_number;
pub mod int_number;
pub mod number;

#[derive(Debug, PartialEq)]
pub enum Number {
    Integer(i64),
    Float(f64),
}

impl Display for Number {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Number::Integer(int) => write!(f, "{}", int),
            Number::Float(float) => write!(f, "{}", float),
        }
    }
}
