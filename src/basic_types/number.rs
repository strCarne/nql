use crate::{primitives, ParsingResult};

pub fn number(mut input: &str) -> ParsingResult<String> {
    let mut matched = String::new();

    // 1. Reading first digit.
    match primitives::digit(&input) {
        Ok((next_seq, digit)) => {
            input = next_seq;
            matched.push(digit);
        }
        _ => return Err(input),
    }

    // 2. Reading int number.
    loop {
        match primitives::digit(&input) {
            Ok((next_seq, digit)) => {
                input = next_seq;
                matched.push(digit);
            }
            _ => match primitives::point(&input) {
                Ok((next_seq, point))
                    if !next_seq.is_empty()
                        && next_seq.chars().next().unwrap().is_ascii_digit() =>
                {
                    input = next_seq;
                    matched.push(point);
                    break;
                }
                _ => return Ok((input, matched)),
            },
        }
    }

    // 3. Reading first digit after a point in float number.
    match primitives::digit(&input) {
        Ok((next_seq, point)) => {
            input = next_seq;
            matched.push(point);
        }
        _ => return Err(input),
    }

    // 4. Reading float number.
    loop {
        match primitives::digit(&input) {
            Ok((next_seq, digit)) => {
                input = next_seq;
                matched.push(digit);
            }
            _ => return Ok((input, matched)),
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
            let got = number(&input);
            assert_eq!(*expected, got);
        }
    }
}
