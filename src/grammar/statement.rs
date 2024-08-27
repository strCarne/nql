use crate::{Parser, ParsingResult};

use super::{key_value, KeyValue};

pub fn statement(input: &str) -> ParsingResult<KeyValue> {
    key_value.parse(input)
}

#[cfg(test)]
mod tests {

    use crate::{
        basic_types::Number,
        grammar::{
            value::{OrdinaryValue, Value},
            ComparasionOperator,
        },
    };

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn statement_test() {
        let input_data = vec![
            "key =  value",
            " key=value",
            "$limit <> 10",
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
            Err("$limit <> 10"),
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
            assert_eq!(statement(input), expected);
        }
    }
}
