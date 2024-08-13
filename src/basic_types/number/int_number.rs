use crate::{combinators, primitives, Parser, ParsingResult};

use super::Number;

pub fn int_number(mut input: &str) -> ParsingResult<Number> {
    let mut matched = String::new();

    // 1. Check for minus at the start
    match primitives::literal("-").parse(input) {
        Ok((next_input, _))
            if !next_input.is_empty() && next_input.chars().next().unwrap().is_ascii_digit() =>
        {
            input = next_input;
            matched.push('-');
        }
        _ => (),
    }

    let digit = combinators::pred(primitives::any, |c| c.is_ascii_digit());

    // 2. Reading first digit.
    match digit.parse(&input) {
        Ok((next_seq, digit)) => {
            input = next_seq;
            matched.push(digit);
        }
        _ => return Err(input),
    }

    // 3. Reading int part of a number.
    loop {
        match digit.parse(&input) {
            Ok((next_seq, digit)) => {
                input = next_seq;
                matched.push(digit);
            }
            _ => {
                return Ok((
                    input,
                    Number::Integer(
                        matched
                            .parse()
                            .expect("Parser 'int_number' parsed integer number incorrectly"),
                    ),
                ))
            }
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn int_number_parsing() {
        let input = vec![
            "123",
            "531 ",
            " 124",
            "",
            ".",
            "3214.",
            "124.1",
            "214124.124124",
            "-1124",
            "-12451.",
            "-124.14",
            "number",
        ];

        let expected = vec![
            Ok(("", Number::Integer(123))),
            Ok((" ", Number::Integer(531))),
            Err(" 124"),
            Err(""),
            Err("."),
            Ok((".", Number::Integer(3214))),
            Ok((".1", Number::Integer(124))),
            Ok((".124124", Number::Integer(214124))),
            Ok(("", Number::Integer(-1124))),
            Ok((".", Number::Integer(-12451))),
            Ok((".14", Number::Integer(-124))),
            Err("number"),
        ];

        assert_eq!(
            input.len(),
            expected.len(),
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        let source_data = input.iter().zip(expected.iter());
        for (input, expected) in source_data {
            let got = int_number(&input);
            assert_eq!(*expected, got);
        }
    }
}
