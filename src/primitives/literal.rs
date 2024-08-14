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
