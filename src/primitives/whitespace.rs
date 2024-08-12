use crate::{Parser, ParsingResult};

pub struct Whitespace;
impl Parser for Whitespace {
    fn parse<'a>(input: &'a str) -> ParsingResult<'a> {
        match input.chars().next() {
            Some(c) if c.is_whitespace() => Ok((&input[c.len_utf8()..], c.to_string())),
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
            "\tnice",
            "\n\tLF TAB",
            "  ",
        ];

        let expected = vec![
            Err("zho-zho"),
            Err(".to_string()"),
            Ok(("foo-foo", String::from(" "))),
            Ok(("nice", String::from("\t"))),
            Ok(("\tLF TAB", String::from("\n"))),
            Ok((" ", String::from(" "))),
        ];

        assert_eq!(
            input.len(),
            expected.len(),
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        let source_data = input.iter().zip(expected.iter());
        for (input, expected) in source_data {
            let got = Whitespace::parse(&input);
            assert_eq!(*expected, got);
        }
    }
}
