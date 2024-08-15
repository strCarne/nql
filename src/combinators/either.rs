use crate::Parser;

pub fn either<'a, P1, P2, Output>(p_1: P1, p_2: P2) -> impl Parser<'a, Output>
where
    P1: Parser<'a, Output>,
    P2: Parser<'a, Output>,
{
    move |input| match p_1.parse(input) {
        ok @ Ok(_) => ok,
        _ => p_2.parse(input),
    }
}

#[cfg(test)]
mod tests {

    use crate::primitives;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn either_combinator() {
        let input_data = vec!["papa", "error"].into_iter();

        let parsers = vec![
            either(primitives::literal("mama"), primitives::literal("papa")),
            either(primitives::literal("succes"), primitives::literal("ok")),
        ]
        .into_iter();

        let expected_results = vec![Ok(("", ())), Err("error")].into_iter();

        assert!(
            input_data.len() == parsers.len() && parsers.len() == expected_results.len(),
            "BAD TEST: number of input is not equal to number of results [correct the source data]"
        );

        input_data
            .zip(parsers)
            .zip(expected_results)
            .map(|((input, parser), expected)| (input, parser, expected))
            .for_each(|(input, parser, expected)| assert_eq!(parser.parse(input), expected));
    }
}
