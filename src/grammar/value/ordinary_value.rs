use crate::{
    basic_types::{self, Number},
    combinators, BoxedParser, Parser,
};

#[derive(Debug, PartialEq)]
pub enum OrdinaryValue {
    Boolean(bool),
    Number(Number),
    String(String),
}

pub fn ordinary_value<'a>() -> impl Parser<'a, OrdinaryValue> {
    combinators::single_of(vec![
        BoxedParser::new(basic_types::boolean.map(|output| OrdinaryValue::Boolean(output))),
        BoxedParser::new(basic_types::number.map(|output| OrdinaryValue::Number(output))),
        BoxedParser::new(basic_types::string.map(|output| OrdinaryValue::String(output))),
    ])
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn ordinary_value_test() {
        let input_data = vec![
            "truE, but not for everyone",
            "cringe string",
            " common, no one likes whitespaces",
            "69 is a number",
            "fAlse is not true",
            "1241.141 that's float number",
            "'1234.141', but this one is string",
            r#""  I HATE WRITING UNIT TESTS   ""#,
            "",
        ].into_iter();

        let parser = ordinary_value();

        let expected_results = vec![
            Ok((", but not for everyone", OrdinaryValue::Boolean(true))),
            Ok((" string", OrdinaryValue::String(String::from("cringe")))),
            Err(" common, no one likes whitespaces"),
            Ok((" is a number", OrdinaryValue::Number(Number::Integer(69)))),
            Ok((" is not true", OrdinaryValue::Boolean(false))),
            Ok((" that's float number", OrdinaryValue::Number(Number::Float(1241.141)))),
            Ok((", but this one is string", OrdinaryValue::String(String::from("1234.141")))),
            Ok(("", OrdinaryValue::String(String::from("  I HATE WRITING UNIT TESTS   ")))),
            Err(""),
        ];

        assert_eq!(
            input_data.len(), 
            expected_results.len(), 
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        for (input, expected) in input_data.zip(expected_results) {
            assert_eq!(parser.parse(input), expected);
        }
    }
}
