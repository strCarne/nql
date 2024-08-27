use crate::Parser;

pub fn and_then<'a, P, F, A, B, NextP>(parser: P, f: F) -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    NextP: Parser<'a, B>,
    F: Fn(A) -> NextP,
{
    move |input| {
        parser
            .parse(input)
            .and_then(|(next_input, output)| f(output).parse(next_input))
    }
}

#[cfg(test)]
mod tests {

    use crate::primitives;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_and_then() {
        let input_data = vec!["mamapapa", "cammapopa"].into_iter();

        let parsers = vec![
            primitives::literal("mama").and_then(|_| primitives::literal("papa")),
            primitives::literal("cammma").and_then(|_| primitives::literal("papa")),
        ]
        .into_iter();

        let expected_results = vec![Ok(("", ())), Err("cammapopa")];

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
