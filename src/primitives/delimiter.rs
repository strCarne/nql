use crate::ParsingResult;

pub fn delimiter(input: &str) -> ParsingResult<char> {
    match input.chars().next() {
        Some(c) if c.is_whitespace() || c.is_ascii_punctuation() => Ok((&input[c.len_utf8()..], c)),
        _ => Err(input),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delimiter_parsing() {
        let input = vec![
            "zho-zho",
            ".to_string()",
            " foo-foo",
            ",comma",
            "$ dollar sign",
            "Q letter",
        ];

        let expected = vec![
            Err("zho-zho"),
            Ok(("to_string()", '.')),
            Ok(("foo-foo", ' ')),
            Ok(("comma", ',')),
            Ok((" dollar sign", '$')),
            Err("Q letter"),
        ];

        assert_eq!(
            input.len(),
            expected.len(),
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        let source_data = input.iter().zip(expected.iter());
        for (input, expected) in source_data {
            let got = delimiter(&input);
            assert_eq!(*expected, got);
        }
    }
}
