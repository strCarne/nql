use crate::{combinators, primitives, BoxedParser, Parser, ParsingResult};

pub fn regular_string(mut input: &str) -> ParsingResult<String> {
    // 1. Check if it is a quoted string
    if let Ok(_) = combinators::single_of(vec![
        BoxedParser::new(primitives::literal("'")),
        BoxedParser::new(primitives::literal("\"")),
        BoxedParser::new(combinators::map(
            combinators::pred(primitives::any, |c| c.is_whitespace()),
            |_| (),
        )),
    ])
    .parse(input)
    {
        return Err(input);
    }

    let mut matched = String::new();

    let non_whitespace_symbols = combinators::pred(primitives::any, |c| !c.is_whitespace());

    // 2. Parse all non whitespace symbols
    while let Ok((next_input, output)) = non_whitespace_symbols.parse(input) {
        matched.push(output);
        input = next_input;
    }

    Ok((input, matched))
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn regular_string_test() {
        let input_data = vec![
            "haha, ",
            "rizz\n",
            " oh, error here because of a whitespace",
            "'yep, parser will think, that is a single-quoted string",
            "\"yep, parser will think, that is a double-quoted string",
            "that's\"nice next input tokens",
        ]
        .into_iter();

        let expected_results = vec![
            Ok((" ", String::from("haha,"))),
            Ok(("\n", String::from("rizz"))),
            Err(" oh, error here because of a whitespace"),
            Err("'yep, parser will think, that is a single-quoted string"),
            Err("\"yep, parser will think, that is a double-quoted string"),
            Ok((" next input tokens", String::from("that's\"nice"))),
        ]
        .into_iter();

        assert_eq!(
            input_data.len(), 
            expected_results.len(), 
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        for (input, expected) in input_data.zip(expected_results) {
            assert_eq!(expected, regular_string(input));
        }
    }
}