use crate::{Parser, ParsingResult};

pub struct Number;
impl Parser for Number {
    fn parse<'a>(mut input: &'a str) -> ParsingResult<'a> {
        let mut matched = String::new();

        // 1. Reading first digit.
        match Digit::parse(&input) {
            Ok((next_seq, parsed_seq)) => {
                input = next_seq;
                matched += &parsed_seq;
            }
            _ => return Err(input),
        }

        // 2. Reading int number.
        loop {
            match Digit::parse(&input) {
                Ok((next_seq, parsed_seq)) => {
                    input = next_seq;
                    matched += &parsed_seq;
                }
                _ => match Point::parse(&input) {
                    Ok((next_seq, parsed_seq))
                        if !next_seq.is_empty()
                            && next_seq.chars().next().unwrap().is_ascii_digit() =>
                    {
                        input = next_seq;
                        matched += &parsed_seq;
                        break;
                    }
                    _ => return Ok((input, matched)),
                },
            }
        }

        // 3. Reading first digit after a point in float number.
        match Digit::parse(&input) {
            Ok((next_seq, parsed_seq)) => {
                input = next_seq;
                matched += &parsed_seq;
            }
            _ => return Err(input),
        }

        // 4. Reading float number.
        loop {
            match Digit::parse(&input) {
                Ok((next_seq, parsed_seq)) => {
                    input = next_seq;
                    matched += &parsed_seq;
                }
                _ => return Ok((input, matched)),
            }
        }
    }
}

struct Digit;
impl Parser for Digit {
    fn parse<'a>(input: &'a str) -> ParsingResult<'a> {
        let mut matched = String::with_capacity(1);

        match input.chars().next() {
            Some(c) if c.is_ascii_digit() => {
                matched.push(c);
                Ok((&input[c.len_utf8()..], matched))
            }
            _ => Err(input),
        }
    }
}

struct Point;
impl Parser for Point {
    fn parse<'a>(input: &'a str) -> ParsingResult<'a> {
        let mut matched = String::with_capacity(1);

        match input.chars().next() {
            Some(c) if c == '.' => {
                matched.push(c);
                Ok((&input[c.len_utf8()..], matched))
            }
            _ => Err(input),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn number_parsing() {
        let input = vec![
            "123",
            "531 ",
            " 124",
            "",
            ".",
            "3214.",
            "124.1",
            "214124.124124",
        ];

        let expected = vec![
            Ok(("", String::from("123"))),
            Ok((" ", String::from("531"))),
            Err(" 124"),
            Err(""),
            Err("."),
            Ok((".", String::from("3214"))),
            Ok(("", String::from("124.1"))),
            Ok(("", String::from("214124.124124"))),
        ];

        assert_eq!(
            input.len(),
            expected.len(),
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        let source_data = input.iter().zip(expected.iter());
        for (input, expected) in source_data {
            let got = Number::parse(&input);
            assert_eq!(*expected, got);
        }
    }
}
