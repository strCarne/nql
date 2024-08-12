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
