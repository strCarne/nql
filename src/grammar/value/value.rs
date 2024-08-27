use crate::{
    combinators::{self},
    Parser, ParsingResult,
};

use super::{collection, ordinary_value, range, Collection, OrdinaryValue, Range};

#[derive(Debug)]
pub enum Value {
    OrdinaryValue(OrdinaryValue),
    Range(Range),
    Collection(Collection),
}

pub fn value(input: &str) -> ParsingResult<Value> {
    combinators::single_of(vec![
        range.map(|range| Value::Range(range)),
        collection.map(|collection| Value::Collection(collection)),
        ordinary_value.map(|ordinary| Value::OrdinaryValue(ordinary)),
    ])
    .parse(input)
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore = "not implemented yet"]
    fn value_test() {
        todo!("Make unit test")
    }
}
