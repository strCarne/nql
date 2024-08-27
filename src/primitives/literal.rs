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

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn literal_test() {
        let input_data = vec![" dada", "dada", "match", "fAil"].into_iter();

        let parsers = vec![
            literal("dada"),
            literal("dada"),
            literal("match"),
            literal("fail"),
        ]
        .into_iter();

        let expected_results = vec![Err(" dada"), Ok(("", ())), Ok(("", ())), Err("fAil")];

        assert!(
            input_data.len() == parsers.len() && parsers.len() == expected_results.len(),
            "BAD TEST: number of input is not equal to number of results [correct the source data]"
        );

        let dataset = input_data
            .zip(parsers)
            .zip(expected_results)
            .map(|((input, parser), expected)| (input, parser, expected));

        for (input, parser, expected) in dataset {
            assert_eq!(parser.parse(input), expected);
        }
    }

    #[test]
    fn iliteral_test() {
        let input_data = vec![" dada", "dada", "match", "fAil"].into_iter();

        let parsers = vec![
            iliteral("dada"),
            iliteral("dada"),
            iliteral("match"),
            iliteral("fail"),
        ]
        .into_iter();

        let expected_results = vec![Err(" dada"), Ok(("", ())), Ok(("", ())), Ok(("", ()))];

        assert!(
            input_data.len() == parsers.len() && parsers.len() == expected_results.len(),
            "BAD TEST: number of input is not equal to number of results [correct the source data]"
        );

        let dataset = input_data
            .zip(parsers)
            .zip(expected_results)
            .map(|((input, parser), expected)| (input, parser, expected));

        for (input, parser, expected) in dataset {
            assert_eq!(parser.parse(input), expected);
        }
    }
}
