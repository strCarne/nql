use crate::Parser;

pub fn zero_or_one<'a, P, Output>(parser: P) -> impl Parser<'a, Option<Output>>
where
    P: Parser<'a, Output>,
{
    move |input| match parser.parse(input) {
        Ok((next_input, output)) => Ok((next_input, Some(output))),
        Err(err) => Ok((err, None)),
    }
}

#[cfg(test)]
mod tests {

    use crate::primitives::literal;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn zero_or_one_combinator() {
        let input_data = vec!["space", " "].into_iter();

        let parser = zero_or_one(literal(" "));

        let expected_results: Vec<Result<(&str, Option<()>), &str>> =
            vec![Ok(("space", None)), Ok(("", Some(())))];
        let expected_results = expected_results.into_iter();

        for (input, expected) in input_data.zip(expected_results) {
            assert_eq!(parser.parse(input), expected);
        }
    }
}
