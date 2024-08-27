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

    use super::*;
    use crate::{basic_types::Number, grammar::value::OrdinaryValue};
    use pretty_assertions::assert_eq;

    #[test]
    fn key_value_test() {
        let input_data = vec![
            "key =  value",
            " key=value",
            "limit <> 10",
            "limit != 10",
            "$key = value",
        ]
        .into_iter();

        let expected_results = vec![
            Ok((
                "",
                KeyValue {
                    k: String::from("key"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::String(String::from("value"))),
                },
            )),
            Err(" key=value"),
            Ok((
                " 10",
                KeyValue {
                    k: String::from("limit"),
                    op: ComparasionOperator::Less,
                    v: Value::OrdinaryValue(OrdinaryValue::String(String::from(">"))),
                },
            )),
            Ok((
                "",
                KeyValue {
                    k: String::from("limit"),
                    op: ComparasionOperator::NotEq,
                    v: Value::OrdinaryValue(OrdinaryValue::Number(Number::Integer(10))),
                },
            )),
            Err("$key = value"),
        ]
        .into_iter();

        assert_eq!(
            input_data.len(),
            expected_results.len(),
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        for (input, expected) in input_data.zip(expected_results) {
            assert_eq!(key_value(input), expected);
        }
    }
}
