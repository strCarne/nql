use crate::{primitives::any, Parser, ParsingResult};

// IDENTTIFIER ::= [a-zA-Z_][a-zA-Z0-9_]*
pub fn identifier(input: &str) -> ParsingResult<String> {
    any.pred(|c| c.is_ascii_alphabetic() || *c == '_')
        .and_then(|c| {
            move |mut input| {
                let mut ident = String::new();
                ident.push(c);
                while let Ok((next_input, c)) = any
                    .pred(|c| c.is_ascii_alphanumeric() || *c == '_')
                    .parse(input)
                {
                    ident.push(c);
                    input = next_input;
                }
                Ok((input, ident))
            }
        })
        .parse(input)
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn identifier_test() {
        let input_data = vec![
            "valid",
            "_also_valid",
            "_",
            "-",
            "high_five",
            "cringe2004",
            "9_times",
        ]
        .into_iter();

        let expected_results = vec![
            Ok(("", String::from("valid"))),
            Ok(("", String::from("_also_valid"))),
            Ok(("", String::from("_"))),
            Err("-"),
            Ok(("", String::from("high_five"))),
            Ok(("", String::from("cringe2004"))),
            Err("9_times"),
        ]
        .into_iter();

        assert_eq!(
            input_data.len(),
            expected_results.len(),
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        for (input, expected) in input_data.zip(expected_results) {
            assert_eq!(identifier(input), expected);
        }
    }
}
