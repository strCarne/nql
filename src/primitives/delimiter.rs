use crate::{Parser, ParsingResult};

pub struct Delimiter;
impl Parser for Delimiter {
    fn parse<'a>(input: &'a str) -> ParsingResult<'a> {
        match input.chars().next() {
            Some(c) if c.is_whitespace() || c.is_ascii_punctuation() => {
                Ok((&input[c.len_utf8()..], c.to_string()))
            }
            _ => Err(input),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn whitespace_parsing() {
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
            Ok(("to_string()", String::from("."))),
            Ok(("foo-foo", String::from(" "))),
            Ok(("comma", String::from(","))),
            Ok((" dollar sign", String::from("$"))),
            Err("Q letter"),
        ];

        assert_eq!(
            input.len(),
            expected.len(),
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        let source_data = input.iter().zip(expected.iter());
        for (input, expected) in source_data {
            let got = Delimiter::parse(&input);
            assert_eq!(*expected, got);
        }
    }
}
