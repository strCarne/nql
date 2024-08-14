use crate::Parser;

pub fn literal<'a>(expected: &'static str) -> impl Parser<'a, ()> {
    move |input: &'a str| match input.get(0..expected.len()) {
        Some(next) if next == expected => Ok((&input[expected.len()..], ())),
        _ => Err(input),
    }
}

// 'iliteral' is similar to 'literal', but it is case-insensitive
pub fn iliteral<'a>(expected: &'static str) -> impl Parser<'a, ()> {
    move |input: &'a str| match input.get(0..expected.len()) {
        Some(next) if next.to_lowercase() == expected.to_lowercase() => {
            Ok((&input[expected.len()..], ()))
        }
        _ => Err(input),
    }
}

pub fn char_literal<'a>(expected: char) -> impl Parser<'a, ()> {
    move |input: &'a str| match input.chars().next() {
        Some(c) if c == expected => Ok((&input[expected.len_utf8()..], ())),
        _ => Err(input),
    }
}
