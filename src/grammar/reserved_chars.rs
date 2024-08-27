use crate::{primitives, Parser, ParsingResult};

pub const RESERVED_CHARS: [char; 13] = [
    '(', ')', ',', '&', '|', '^', '{', '}', '[', ']', '"', '\'', '\\',
];

pub fn reserved_chars(input: &str) -> ParsingResult<char> {
    primitives::any
        .pred(|c| RESERVED_CHARS.contains(c))
        .parse(input)
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn reserved_chars_test() {
        let input_data = vec![
            "", "(", ")", ",", "& john", "|", "{", "}", "[", "]", "\"", "'tailor'", "\\",
        ]
        .into_iter();
        let expected_results = vec![
            Err(""),
            Ok(("", '(')),
            Ok(("", ')')),
            Ok(("", ',')),
            Ok((" john", '&')),
            Ok(("", '|')),
            Ok(("", '{')),
            Ok(("", '}')),
            Ok(("", '[')),
            Ok(("", ']')),
            Ok(("", '"')),
            Ok(("tailor'", '\'')),
            Ok(("", '\\')),
        ]
        .into_iter();

        assert_eq!(
            input_data.len(),
            expected_results.len(),
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        for (input, expected) in input_data.zip(expected_results) {
            assert_eq!(reserved_chars(input), expected);
        }
    }
}
