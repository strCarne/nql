use crate::{primitives, Parser};

pub const RESERVED_CHARS: [char; 13] = [
    '(', ')', ',', '&', '|', '^', '{', '}', '[', ']', '"', '\'', '\\',
];

pub fn reserved_chars<'a>() -> impl Parser<'a, char> {
    primitives::any.pred(|c| RESERVED_CHARS.contains(c))
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore = "Not implemented yet"]
    fn reserved_chars_test() {
        todo!("Make unit test")
    }
}