#![allow(unused_imports)]

use std::fmt::Display;

mod float_number;
pub use float_number::float_number;

mod int_number;
pub use int_number::int_number;

mod number;
pub use number::number;

#[derive(Debug, PartialEq, Clone, Copy)]
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
