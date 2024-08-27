use crate::ParsingResult;

pub fn any(input: &str) -> ParsingResult<char> {
    match input.chars().next() {
        Some(c) => Ok((&input[c.len_utf8()..], c)),
        _ => Err(input),
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn any_test() {
        let input_data = vec!["", "fukk", "token", " space"].into_iter();

        let expected_results = vec![
            Err(""),
            Ok(("ukk", 'f')),
            Ok(("oken", 't')),
            Ok(("space", ' ')),
        ]
        .into_iter();

        assert_eq!(
            input_data.len(),
            expected_results.len(),
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        for (input, expected) in input_data.zip(expected_results) {
            assert_eq!(any(input), expected);
        }
    }
}
