use crate::{primitives, Parser, ParsingResult};

pub const RESERVED_CHARS: [char; 13] = [
    '(', ')', ',', '&', '|', '^', '{', '}', '[', ']', '"', '\'', '\\',
];

pub fn reserved_chars(input: &str) -> ParsingResult<char> {
    primitives::any
        .pred(|c| RESERVED_CHARS.contains(c))
        .parse(input)
}
