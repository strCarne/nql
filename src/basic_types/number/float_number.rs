use crate::{combinators, primitives, Parser, ParsingResult};

use super::Number;

#[allow(unused)]
pub fn float_number(mut input: &str) -> ParsingResult<Number> {
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

    let digit = primitives::any.pred(|c| c.is_ascii_digit());

    // 2. Reading first digit.
    match digit.parse(&input) {
        Ok((next_seq, digit)) => {
            input = next_seq;
            matched.push(digit);
        }
        _ => return Err(input),
    }

    let point = primitives::any.pred(|c| *c == '.');

    // 3. Reading int part of a number.
    loop {
        match digit.parse(&input) {
            Ok((next_seq, digit)) => {
                input = next_seq;
                matched.push(digit);
            }
            _ => match point.parse(&input) {
                Ok((next_seq, point))
                    if !next_seq.is_empty()
                        && next_seq.chars().next().unwrap().is_ascii_digit() =>
                {
                    input = next_seq;
                    matched.push(point);
                    break;
                }
                _ => return Err(input),
            },
        }
    }

    // 4. Reading first digit in fractional part of a number.
    match digit.parse(&input) {
        Ok((next_seq, point)) => {
            input = next_seq;
            matched.push(point);
        }
        _ => return Err(input),
    }

    // 5. Reading fractional part of a number.
    loop {
        match digit.parse(&input) {
            Ok((next_seq, digit)) => {
                input = next_seq;
                matched.push(digit);
            }
            _ => {
                return Ok((
                    input,
                    Number::Float(
                        matched
                            .parse()
                            .expect("Parser 'float_number' parsed float number incorrectly"),
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
    fn float_number_parsing() {
        let input_data = vec![
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
        ].into_iter();

        let expected_results = vec![
            Err(""),
            Err(" "),
            Err(" 124"),
            Err(""),
            Err("."),
            Err("."),
            Ok(("", Number::Float(124.1))),
            Ok(("", Number::Float(214124.124124))),
            Err(""),
            Err("."),
            Ok(("", Number::Float(-124.14))),
        ].into_iter();

        assert_eq!(
            input_data.len(),
            expected_results.len(),
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        for (input, expected) in input_data.zip(expected_results) {
            assert_eq!(float_number(input), expected);
        }
    }
}
