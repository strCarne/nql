use crate::{Parser, ParsingResult};

use super::{
    comparasion_operator, key,
    value::{value, Value},
    ComparasionOperator,
};

#[derive(Debug, PartialEq)]
pub struct KeyValue {
    pub k: String,
    pub op: ComparasionOperator,
    pub v: Value,
}

pub fn key_value(input: &str) -> ParsingResult<KeyValue> {
    let (input, k) = key.parse(input)?;

    let (input, op) = comparasion_operator.parse(input)?;

    let (input, v) = value.parse(input)?;

    Ok((input, KeyValue { k, op, v }))
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore = "not implemented yet"]
    fn key_value_test() {
        todo!("Make unit test")
    }
}
