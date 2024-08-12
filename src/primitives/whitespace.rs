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
