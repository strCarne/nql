use crate::ParsingResult;

pub fn point(input: &str) -> ParsingResult<char> {
    match input.chars().next() {
        Some(c) if c == '.' => Ok((&input[c.len_utf8()..], c)),
        _ => Err(input),
    }
}
