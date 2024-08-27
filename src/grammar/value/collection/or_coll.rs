use crate::{grammar::value::OrdinaryValue, primitives, Parser, ParsingResult};

use super::collection_body;

pub fn or_coll(input: &str) -> ParsingResult<Vec<OrdinaryValue>> {
    collection_body
        .whitespace_wrap()
        .wrap_before(primitives::character('['))
        .wrap_after(primitives::character(']'))
        .parse(input)
}

#[cfg(test)]
mod tests {

    use crate::basic_types::Number;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn or_coll_test() {
        let input_data = vec![
            "[ 1, 2, 3, ]",
            "[ 1, 2, 3]",
            "{1, 2, 3}",
            "[mama,papa,ya,is,big,semya] next_input",
        ]
        .into_iter();

        let expected_results = vec![
            Err(", ]"),
            Ok((
                "",
                vec![
                    OrdinaryValue::Number(Number::Integer(1)),
                    OrdinaryValue::Number(Number::Integer(2)),
                    OrdinaryValue::Number(Number::Integer(3)),
                ],
            )),
            Err("{1, 2, 3}"),
            Ok((
                " next_input",
                vec![
                    OrdinaryValue::String(String::from("mama")),
                    OrdinaryValue::String(String::from("papa")),
                    OrdinaryValue::String(String::from("ya")),
                    OrdinaryValue::String(String::from("is")),
                    OrdinaryValue::String(String::from("big")),
                    OrdinaryValue::String(String::from("semya")),
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
            assert_eq!(or_coll(input), expected);
        }
    }
}
