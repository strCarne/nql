use crate::{combinators, primitives, Parser, ParsingResult};

use super::{
    comparasion_operator, identifier,
    value::{value, Value},
    ComparasionOperator,
};

#[derive(Debug)]
pub struct KeyValue {
    pub k: String,
    pub op: ComparasionOperator,
    pub v: Value,
}

pub fn key_value(input: &str) -> ParsingResult<KeyValue> {
    let whitespaces =
        |input| combinators::zero_or_more(primitives::any.pred(|c| c.is_whitespace())).parse(input);

    let (input, k) = identifier.wrap(whitespaces.clone()).parse(input)?;

    let (input, op) = comparasion_operator
        .wrap(whitespaces.clone())
        .parse(input)?;

    let (input, v) = value.wrap(whitespaces).parse(input)?;

    Ok((input, KeyValue { k, op, v }))
}
