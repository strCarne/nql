use crate::{
    combinators::{self},
    primitives, Parser, ParsingResult,
};

use super::{collection, ordinary_value, range, Collection, OrdinaryValue, Range};

#[derive(Debug)]
pub enum Value {
    OrdinaryValue(OrdinaryValue),
    Range(Range),
    Collection(Collection),
}

pub fn value(input: &str) -> ParsingResult<Value> {
    let whitespaces =
        |input| combinators::zero_or_more(primitives::any.pred(|c| c.is_whitespace())).parse(input);

    combinators::single_of(vec![
        range
            .wrap(whitespaces.clone())
            .map(|range| Value::Range(range)),
        // collection
        //     .wrap(whitespaces)
        //     .map(|collection| Value::Collection(collection)),
        ordinary_value
            .wrap(whitespaces.clone())
            .map(|ordinary| Value::OrdinaryValue(ordinary)),
    ])
    .parse(input)
} 
