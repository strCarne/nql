use crate::ParsingResult;

pub fn any(input: &str) -> ParsingResult<char> {
    match input.chars().next() {
        Some(c) => Ok((&input[c.len_utf8()..], c)),
        _ => Err(input),
    }
}
