use crate::{primitives, Parser, ParsingResult};

use super::{units_sequance, NQLang};

pub fn group(input: &str) -> ParsingResult<NQLang> {
    units_sequance
        .wrap_before(primitives::character('('))
        .wrap_after(primitives::character(')'))
        .parse(input)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::grammar::{
        value::{OrdinaryValue, Value},
        ComparasionOperator, KeyValue, Link, NQToken, Unit,
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn group_test() {
        let input_data = vec![
            "(key=value)",
            " ( key = value )",
            "( key = value )",
            "(key_1 = value_1 | key_2 = value_2 ) & key_3 < value_3",
        ]
        .into_iter();

        let expected_results = vec![
            Ok((
                "",
                vec![NQToken::Unit(Unit::Stmt(KeyValue {
                    k: String::from("key"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::String(String::from("value"))),
                }))],
            )),
            Err(" ( key = value )"),
            Ok((
                "",
                vec![NQToken::Unit(Unit::Stmt(KeyValue {
                    k: String::from("key"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::String(String::from("value"))),
                }))],
            )),
            Ok((
                " & key_3 < value_3",
                vec![
                    NQToken::Unit(Unit::Stmt(KeyValue {
                        k: String::from("key_1"),
                        op: ComparasionOperator::Eq,
                        v: Value::OrdinaryValue(OrdinaryValue::String(String::from("value_1"))),
                    })),
                    NQToken::Link(Link::Or),
                    NQToken::Unit(Unit::Stmt(KeyValue {
                        k: String::from("key_2"),
                        op: ComparasionOperator::Eq,
                        v: Value::OrdinaryValue(OrdinaryValue::String(String::from("value_2"))),
                    })),
                ],
            )),
        ]
        .into_iter();

        assert_eq!(
            input_data.len(),
            expected_results.len(),
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        for (input, expected) in input_data.zip(expected_results) {
            assert_eq!(group(input), expected);
        }
    }
}
