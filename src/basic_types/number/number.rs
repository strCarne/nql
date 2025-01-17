use crate::{combinators, primitives, Parser, ParsingResult};

use super::Number;

// NUMBER ::= INT_NUMBER | FLOAT_NUMBER
//
// INT_NUMBER >> see grammar::basic_types::number::int_number.rs
// FLOAT_NUMBER >> see grammar::basic_types::number::float_number.rs
pub fn number(mut input: &str) -> ParsingResult<Number> {
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

    // 3. Reading int number.
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
                _ => {
                    return Ok((
                        input,
                        Number::Integer(
                            matched
                                .parse()
                                .expect("Parser 'number' parsed integer number incorrectly"),
                        ),
                    ))
                }
            },
        }
    }

    // 4. Reading first digit after a point in float number.
    match digit.parse(&input) {
        Ok((next_seq, point)) => {
            input = next_seq;
            matched.push(point);
        }
        _ => return Err(input),
    }

    // 5. Reading float number.
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
                            .expect("Parser 'number' parsed float number incorrectly"),
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
    fn number_parsing() {
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
            Ok(("", Number::Integer(123))),
            Ok((" ", Number::Integer(531))),
            Err(" 124"),
            Err(""),
            Err("."),
            Ok((".", Number::Integer(3214))),
            Ok(("", Number::Float(124.1))),
            Ok(("", Number::Float(214124.124124))),
            Ok(("", Number::Integer(-1124))),
            Ok((".", Number::Integer(-12451))),
            Ok(("", Number::Float(-124.14))),
        ].into_iter();

        assert_eq!(
            input_data.len(),
            expected_results.len(),
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        for (input, expected) in input_data.zip(expected_results) {
            assert_eq!(number(input), expected);
        }
    }
}
