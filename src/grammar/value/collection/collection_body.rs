use crate::{
    grammar::value::{ordinary_value, OrdinaryValue},
    primitives, Parser, ParsingResult,
};

pub fn collection_body(mut input: &str) -> ParsingResult<Vec<OrdinaryValue>> {
    let (next_input, head) = ordinary_value.parse(input)?;

    let mut elems = Vec::new();
    elems.push(head);
    input = next_input;

    let parser = primitives::character(',')
        .whitespace_wrap()
        .and_then(|_| ordinary_value);

    while let Ok((next_input, elem)) = parser.parse(input) {
        elems.push(elem);
        input = next_input;
    }

    Ok((input, elems))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::basic_types::{Date, Number};
    use pretty_assertions::assert_eq;

    #[test]
    fn collection_body_test() {
        let input_data = vec![
            "1, 2 , 3",
            "nice, 1, true, 01:01:2000 next_input",
            " 1, 2, 3",
            "1, 2, 3, ",
        ]
        .into_iter();

        let expected_results = vec![
            Ok((
                "",
                vec![
                    OrdinaryValue::Number(Number::Integer(1)),
                    OrdinaryValue::Number(Number::Integer(2)),
                    OrdinaryValue::Number(Number::Integer(3)),
                ],
            )),
            Ok((
                " next_input",
                vec![
                    OrdinaryValue::String(String::from("nice")),
                    OrdinaryValue::Number(Number::Integer(1)),
                    OrdinaryValue::Boolean(true),
                    OrdinaryValue::Date(Date {
                        day: 1,
                        month: 1,
                        year: 2000,
                    }),
                ],
            )),
            Err(" 1, 2, 3"),
            Ok((
                ", ",
                vec![
                    OrdinaryValue::Number(Number::Integer(1)),
                    OrdinaryValue::Number(Number::Integer(2)),
                    OrdinaryValue::Number(Number::Integer(3)),
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
            assert_eq!(collection_body(input), expected);
        }
    }
}
