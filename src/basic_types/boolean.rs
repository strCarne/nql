use crate::{combinators, primitives, Parser, ParsingResult};

pub fn boolean(input: &str) -> ParsingResult<bool> {
    let true_parser = combinators::single_of(vec![
        Box::new(primitives::iliteral("true")),
        Box::new(primitives::literal("1")),
    ]);

    let false_parser = combinators::single_of(vec![
        Box::new(primitives::iliteral("false")),
        Box::new(primitives::literal("0")),
    ]);

    match true_parser.parse(input) {
        Ok((next_input, _)) => Ok((next_input, true)),
        _ => match false_parser.parse(input) {
            Ok((next_input, _)) => Ok((next_input, false)),
            _ => Err(input),
        },
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn boolean_parsing() {
        let input_data = vec![
            "true",
            "False",
            "fAlse",
            "0 ",
            " 1",
            "tru false",
            " falsE",
            "truefalse",
        ]
        .into_iter();

        let expected_results = vec![
            Ok(("", true)),
            Ok(("", false)),
            Ok(("", false)),
            Ok((" ", false)),
            Err(" 1"),
            Err("tru false"),
            Err(" falsE"),
            Ok(("false", true)),
        ]
        .into_iter();

        for (input, expected) in input_data.zip(expected_results) {
            let result = boolean(input);
            assert_eq!(expected, result);
        }
    }
}
