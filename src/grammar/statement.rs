use crate::{combinators, primitives, Parser, ParsingResult};

use super::{key_value, KeyValue};

#[derive(Debug, PartialEq)]
pub enum Statement {
    Field(KeyValue),
    Extension(KeyValue),
}

pub fn statement(input: &str) -> ParsingResult<Statement> {
    combinators::single_of(vec![
        key_value.map(|output| Statement::Field(output)),
        primitives::character('$')
            .and_then(|_| key_value.map(|output| Statement::Extension(output))),
    ])
    .parse(input)
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
                Statement::Field(KeyValue {
                    k: String::from("key"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::String(String::from("value"))),
                }),
            )),
            Err(" key=value"),
            Ok((
                " 10",
                Statement::Extension(KeyValue {
                    k: String::from("limit"),
                    op: ComparasionOperator::Less,
                    v: Value::OrdinaryValue(OrdinaryValue::String(String::from(">"))),
                }),
            )),
            Ok((
                "",
                Statement::Field(KeyValue {
                    k: String::from("limit"),
                    op: ComparasionOperator::NotEq,
                    v: Value::OrdinaryValue(OrdinaryValue::Number(Number::Integer(10))),
                }),
            )),
            Ok((
                "",
                Statement::Extension(KeyValue {
                    k: String::from("key"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::String(String::from("value"))),
                }),
            )),
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
