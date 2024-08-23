use crate::{combinators, grammar, primitives, Parser, ParsingResult};

// REGULAR_STRING ::= ALLOWED_SYMBOL+
// ALLOWED_SYMBOL ::= !WHITESPACE | !RESERVED_CHARS
// WHITESPACE ::= char::is_whitespace()
// Whitespace - non visible character
// RESERVED_CHARS see grammar::reserved_chars.rse
pub fn regular_string(mut input: &str) -> ParsingResult<String> {
    // 1. Check if it is a quoted string
    if let Ok(_) = combinators::single_of(vec![
        grammar::reserved_chars().map(|_| ()),
        primitives::any
            .pred(|c| c.is_whitespace())
            .map(|_| ()),
    ])
    .parse(input)
    {
        return Err(input);
    }

    let mut matched = String::new();

    let allowed_symbols = primitives::any
        .pred(|c| !c.is_whitespace() && !grammar::RESERVED_CHARS.contains(c) || *c == '\\');

    // 2. Parse all non whitespace symbols
    while let Ok((next_input, output)) = allowed_symbols.parse(input) {
        if output == '\\' {
            match next_input.chars().next() {
                Some(next_c) => {
                    matched.push(next_c);
                    input = &next_input[next_c.len_utf8()..];
                }
                None => return Err(input),
            }
        } else {
            matched.push(output);
            input = next_input;
        }
    }

    if matched.is_empty() {
        Err("")
    } else {
        Ok((input, matched))
    }
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
            "that\\'s\"nice next input tokens",
            "",
        ]
        .into_iter();

        let expected_results = vec![
            Ok((", ", String::from("haha"))),
            Ok(("\n", String::from("rizz"))),
            Err(" oh, error here because of a whitespace"),
            Err("'yep, parser will think, that is a single-quoted string"),
            Err("\"yep, parser will think, that is a double-quoted string"),
            Ok(("\"nice next input tokens", String::from("that's"))),
            Err(""),
        ]
        .into_iter();

        assert_eq!(
            input_data.len(), 
            expected_results.len(), 
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        for (input, expected) in input_data.zip(expected_results) {
            assert_eq!(regular_string(input), expected);
        }
    }
}
