use super::*;
use crate::{combinators, BoxedParser, Parser, ParsingResult};

pub fn string(input: &str) -> ParsingResult<String> {
    combinators::single_of(vec![
        BoxedParser::new(regular_string),
        BoxedParser::new(quoted_string(QuoteType::Single)),
        BoxedParser::new(quoted_string(QuoteType::Double)),
    ])
    .parse(input)
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn string_test() {
        let input_data = vec![
            "haha, ",
            "rizz\n",
            " oh, error here because of a whitespace",
            "'yep, parser will think, that is a single-quoted string",
            "\"yep, parser will think, that is a double-quoted string",
            "that's\"nice next input tokens",

            " 'space, so parser didn't even reach string'",
            "'nice'",
            "'very nice'",
            "'a   lot   of   spaces   here' next tokens",
            r#"'"yea, this is possible too"'"#,
            r#""this is double quoted string, acceptable""#,
            "'did not close the quote",

            r#" "space, so parser didn't even reach string""#,
            r#""nice""#,
            r#""very nice""#,
            r#""a   lot   of   spaces   here" next tokens"#,
            r#""'yea, this is possible too'""#,
            r#"'this is single quoted string, acceptable'"#,
            r#""did not close the quote"#,

            "",
        ]
        .into_iter();

        let expected_results = vec![
            Ok((" ", String::from("haha,"))),
            Ok(("\n", String::from("rizz"))),
            Err(" oh, error here because of a whitespace"),
            Err("'yep, parser will think, that is a single-quoted string"),
            Err("\"yep, parser will think, that is a double-quoted string"),
            Ok((" next input tokens", String::from("that's\"nice"))),

            Err(" 'space, so parser didn't even reach string'"),
            Ok(("", String::from("nice"))),
            Ok(("", String::from("very nice"))),
            Ok((" next tokens", String::from("a   lot   of   spaces   here"))),
            Ok(("", String::from(r#""yea, this is possible too""#))),
            Ok(("", String::from("this is double quoted string, acceptable"))),
            Err("'did not close the quote"),

            Err(r#" "space, so parser didn't even reach string""#),
            Ok(("", String::from("nice"))),
            Ok(("", String::from("very nice"))),
            Ok((" next tokens", String::from("a   lot   of   spaces   here"))),
            Ok(("", String::from("'yea, this is possible too'"))),
            Ok(("", String::from("this is single quoted string, acceptable"))),
            Err(r#""did not close the quote"#),

            Err(""),
        ]
        .into_iter();

        assert_eq!(
            input_data.len(), 
            expected_results.len(), 
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        for (input, expected) in input_data.zip(expected_results) {
            assert_eq!(string(input), expected);
        }
    }
}
