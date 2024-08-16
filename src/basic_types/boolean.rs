use crate::{primitives, Parser, ParsingResult};

// BOOLEAN ::= true | false
pub fn boolean(input: &str) -> ParsingResult<bool> {
    let true_parser = primitives::iliteral("true");

    let false_parser = primitives::iliteral("false");

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
            "tru false",
            " falsE",
            "truefalse",
        ]
        .into_iter();

        let expected_results = vec![
            Ok(("", true)),
            Ok(("", false)),
            Ok(("", false)),
            Err("tru false"),
            Err(" falsE"),
            Ok(("false", true)),
        ]
        .into_iter();

        assert_eq!(
            input_data.len(), 
            expected_results.len(), 
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        for (input, expected) in input_data.zip(expected_results) {
            let result = boolean(input);
            assert_eq!(result, expected);
        }
    }
}
