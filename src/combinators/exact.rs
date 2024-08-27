use crate::Parser;

pub fn exact<'a, P, A>(parser: P, n: u64) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A>,
{
    move |input| {
        let mut outputs = Vec::with_capacity(n as usize);
        let mut next_input = input;
        for _ in 0..n {
            outputs.push({
                let res = parser.parse(next_input)?;
                next_input = res.0;
                res.1
            });
        }
        Ok((next_input, outputs))
    }
}

#[cfg(test)]
mod tests {

    use crate::primitives;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn exact_combinator() {
        let input_data = vec!["aaa", "aaaa", "aaaaa"].into_iter();

        let parser = exact(primitives::character('a'), 4);

        let expected_results = vec![
            Err(""),
            Ok(("", vec![(), (), (), ()])),
            Ok(("a", vec![(), (), (), ()])),
        ]
        .into_iter();

        assert_eq!(
            input_data.len(),
            expected_results.len(),
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        for (input, expected) in input_data.zip(expected_results) {
            assert_eq!(parser.parse(input), expected);
        }
    }
}
