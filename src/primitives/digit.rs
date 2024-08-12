use crate::ParsingResult;

pub fn digit(input: &str) -> ParsingResult<char> {
    match input.chars().next() {
        Some(c) if c.is_ascii_digit() => Ok((&input[c.len_utf8()..], c)),
        _ => Err(input),
    }
}
