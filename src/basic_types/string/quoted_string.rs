use crate::{combinators, primitives, Parser};

pub enum QuoteType {
    Single,
    Double,
}

// WARNING: character escaping is not supported yet
pub fn quoted_string<'a>(quote_type: QuoteType) -> impl Parser<'a, String> {
    let quote = match quote_type {
        QuoteType::Single => '\'',
        QuoteType::Double => '"',
    };

    combinators::right(
        primitives::character(quote),
        combinators::left(
            combinators::zero_or_more(primitives::any.pred(move |c| *c != quote)),
            primitives::character(quote),
        ),
    )
    .map(|chars| chars.into_iter().collect())
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn single_quoted_string_test() {
        let input_data = vec![
            " 'space, so parser didn't even reach string'",
            "'nice'",
            "'very nice'",
            "'a   lot   of   spaces   here' next tokens",
            r#"'"yea, this is possible too"'"#,
            r#""this is double quoted string, not acceptable""#,
            "'did not close the quote",
        ]
        .into_iter();

        let parser = quoted_string(QuoteType::Single);

        let expected_results = vec![
            Err(" 'space, so parser didn't even reach string'"),
            Ok(("", String::from("nice"))),
            Ok(("", String::from("very nice"))),
            Ok((" next tokens", String::from("a   lot   of   spaces   here"))),
            Ok(("", String::from(r#""yea, this is possible too""#))),
            Err(r#""this is double quoted string, not acceptable""#),
            Err(""),
        ]
        .into_iter();

        assert_eq!(
            input_data.len(),
            expected_results.len(),
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        for (input, expected) in input_data.zip(expected_results) {
            assert_eq!(parser.parse(input), expected);
        }
    }

    #[test]
    fn double_quoted_string_test() {
        let input_data = vec![
            r#" "space, so parser didn't even reach string""#,
            r#""nice""#,
            r#""very nice""#,
            r#""a   lot   of   spaces   here" next tokens"#,
            r#""'yea, this is possible too'""#,
            r#"'this is single quoted string, not acceptable'"#,
            r#""did not close the quote"#,

            "",
        ]
        .into_iter();

        let parser = quoted_string(QuoteType::Double);

        let expected_results = vec![
            Err(r#" "space, so parser didn't even reach string""#),
            Ok(("", String::from("nice"))),
            Ok(("", String::from("very nice"))),
            Ok((" next tokens", String::from("a   lot   of   spaces   here"))),
            Ok(("", String::from("'yea, this is possible too'"))),
            Err("'this is single quoted string, not acceptable'"),
            Err(""),

            Err(""),
        ]
        .into_iter();

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
