use crate::{
    combinators,
    primitives::{self, any},
    Parser, ParsingResult,
};

// KEY ::= IDENT(\.IDENT)*
// IDENT ::= [a-zA-Z_][a-zA-Z0-9_]*
pub fn key(mut input: &str) -> ParsingResult<String> {
    let (next_input, mut matched) = ident(input)?;
    input = next_input;

    let parser = combinators::right(primitives::character('.'), ident);
    while let Ok((next_input, next_ident)) = parser.parse(input) {
        matched.push('.');
        matched += &next_ident;
        input = next_input;
    }

    Ok((input, matched))
}

fn ident(input: &str) -> ParsingResult<String> {
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
    fn key_test() {
        let input_data = vec![
            "valid",
            "_also_valid",
            "_",
            "-",
            "high_five",
            "cringe2004",
            "9_times",
            "a.b.c",
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
            Ok(("", String::from("a.b.c"))),
        ]
        .into_iter();

        assert_eq!(
            input_data.len(),
            expected_results.len(),
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        for (input, expected) in input_data.zip(expected_results) {
            assert_eq!(key(input), expected);
        }
    }
}
