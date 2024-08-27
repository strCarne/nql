use crate::{
    combinators::{self},
    Parser, ParsingResult,
};

use super::{collection, ordinary_value, range, Collection, OrdinaryValue, Range};

#[derive(Debug, PartialEq)]
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

    use crate::basic_types::{Date, Number};

    use super::*;
    use pretty_assertions::assert_eq;
    use range::{RangeBounds, RangeOp};

    #[test]
    fn value_test() {
        let input_data = vec![
            "",
            "string",
            "69",
            "1=.2",
            "1:1:2000==2:1:2000",
            "[1, 2, 3]",
            "{1, 2, 3}",
            " 28:12:2004",
        ]
        .into_iter();

        let expected_results = vec![
            Err(""),
            Ok((
                "",
                Value::OrdinaryValue(OrdinaryValue::String(String::from("string"))),
            )),
            Ok((
                "",
                Value::OrdinaryValue(OrdinaryValue::Number(Number::Integer(69))),
            )),
            Ok((
                "",
                Value::Range(Range {
                    bounds: RangeBounds::NumberRange(Number::Integer(1), Number::Integer(2)),
                    op: RangeOp::IE,
                }),
            )),
            Ok((
                "",
                Value::Range(Range {
                    bounds: RangeBounds::DateRange(
                        Date {
                            day: 1,
                            month: 1,
                            year: 2000,
                        },
                        Date {
                            day: 2,
                            month: 1,
                            year: 2000,
                        },
                    ),
                    op: RangeOp::II,
                }),
            )),
            Ok((
                "",
                Value::Collection(Collection::OrColl(vec![
                    OrdinaryValue::Number(Number::Integer(1)),
                    OrdinaryValue::Number(Number::Integer(2)),
                    OrdinaryValue::Number(Number::Integer(3)),
                ])),
            )),
            Ok((
                "",
                Value::Collection(Collection::AndColl(vec![
                    OrdinaryValue::Number(Number::Integer(1)),
                    OrdinaryValue::Number(Number::Integer(2)),
                    OrdinaryValue::Number(Number::Integer(3)),
                ])),
            )),
            Err(" 28:12:2004"),
        ]
        .into_iter();

        assert_eq!(
            input_data.len(),
            expected_results.len(),
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        for (input, expected) in input_data.zip(expected_results) {
            assert_eq!(value(input), expected);
        }
    }
}
