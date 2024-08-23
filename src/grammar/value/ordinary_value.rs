use crate::{
    basic_types::{self, Date, Number},
    combinators, Parser, ParsingResult
};

#[derive(Debug, PartialEq)]
pub enum OrdinaryValue {
    Boolean(bool),
    Date(Date),
    Number(Number),
    String(String),
}

impl ToString for OrdinaryValue {
    fn to_string(&self) -> String {
        match self {
            OrdinaryValue::Boolean(boolean) => boolean.to_string(),
            OrdinaryValue::Date(date) => date.to_string(),
            OrdinaryValue::Number(number) => number.to_string(),
            OrdinaryValue::String(string) => format!("'{}'", string),
        }
    }
}

pub fn ordinary_value(input: &str) -> ParsingResult<OrdinaryValue> {
    combinators::single_of(vec![
        basic_types::boolean.map(|output| OrdinaryValue::Boolean(output)).into_box(),
        basic_types::date.map(|output| OrdinaryValue::Date(output)),
        basic_types::number.map(|output| OrdinaryValue::Number(output)).into_box(),
        basic_types::string.map(|output| OrdinaryValue::String(output)).into_box(),
    ]).parse(input)
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
            assert_eq!(ordinary_value(input), expected);
        }
    }
}
