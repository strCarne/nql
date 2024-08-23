use crate::Parser;

pub fn character<'a>(expected: char) -> impl Parser<'a, ()> {
    move |input: &'a str| match input.chars().next() {
        Some(c) if c == expected => Ok((&input[expected.len_utf8()..], ())),
        _ => Err(input),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore = "Not implemented yet"]
    fn character_test() {
        todo!("Make unit test")
    }
}