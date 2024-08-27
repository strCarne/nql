use crate::{combinators, Parser, ParsingResult};

use super::{group, statement, NQLang, Statement};

#[derive(Debug, PartialEq)]
pub enum Unit {
    Stmt(Statement),
    Grp(NQLang),
}

pub fn unit(input: &str) -> ParsingResult<Unit> {
    combinators::single_of(vec![
        group.map(|grp| Unit::Grp(grp)),
        statement.map(|stmt| Unit::Stmt(stmt)),
    ])
    .whitespace_wrap()
    .parse(input)
}

#[cfg(test)]
mod tests {

    use crate::{
        basic_types::Number,
        grammar::{
            value::{OrdinaryValue, Value},
            ComparasionOperator, KeyValue, NQToken,
        },
    };

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn unit_test() {
        let input_data = vec![
            "( key = value )",
            " =value",
            "$limit <> 10",
            "limit != 10",
            "$key = value",
        ]
        .into_iter();

        let expected_results = vec![
            Ok((
                "",
                Unit::Grp(vec![NQToken::Unit(Unit::Stmt(Statement::Field(
                    KeyValue {
                        k: String::from("key"),
                        op: ComparasionOperator::Eq,
                        v: Value::OrdinaryValue(OrdinaryValue::String(String::from("value"))),
                    },
                )))]),
            )),
            Err("=value"),
            Ok((
                "10",
                Unit::Stmt(Statement::Extension(KeyValue {
                    k: String::from("limit"),
                    op: ComparasionOperator::Less,
                    v: Value::OrdinaryValue(OrdinaryValue::String(String::from(">"))),
                })),
            )),
            Ok((
                "",
                Unit::Stmt(Statement::Field(KeyValue {
                    k: String::from("limit"),
                    op: ComparasionOperator::NotEq,
                    v: Value::OrdinaryValue(OrdinaryValue::Number(Number::Integer(10))),
                })),
            )),
            Ok((
                "",
                Unit::Stmt(Statement::Extension(KeyValue {
                    k: String::from("key"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::String(String::from("value"))),
                })),
            )),
        ]
        .into_iter();

        assert_eq!(
            input_data.len(),
            expected_results.len(),
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        for (input, expected) in input_data.zip(expected_results) {
            assert_eq!(unit(input), expected);
        }
    }
}
