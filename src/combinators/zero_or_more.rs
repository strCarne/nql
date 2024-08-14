use crate::Parser;

pub fn zero_or_more<'a, P, Output>(parser: P) -> impl Parser<'a, Vec<Output>>
where
    P: Parser<'a, Output>,
{
    move |mut input| {
        let mut result = Vec::new();

        while let Ok((next_input, next_output)) = parser.parse(input) {
            input = next_input;
            result.push(next_output);
        }

        Ok((input, result))
    }
}

#[cfg(test)]
mod tests {

    use crate::{combinators, primitives};

    use super::*;
    use pretty_assertions::assert_eq;

    // Basically this test just can't fail, because of combinator nature.
    // If it couldn't parse even a single token, than it will return an empty
    // Vec<Output>.
    // But who knows who will work on this combinator in future.
    #[test]
    fn zero_or_more_combinator() {
        let input_data = vec!["a milli a milli a milli", " \n\t cringe", "space"].into_iter();

        let parsers: Vec<Box<dyn Parser<Vec<()>>>> = vec![
            Box::new(zero_or_more(primitives::literal("a milli "))),
            Box::new(zero_or_more(combinators::map(
                combinators::pred(primitives::any, |c| c.is_whitespace()),
                |_| (),
            ))),
            Box::new(zero_or_more(primitives::literal(" "))),
        ];
        let parsers = parsers.into_iter();

        let expected_results = vec![
            Ok(("a milli", vec![(), ()])),
            Ok(("cringe", vec![(), (), (), ()])),
            Ok(("space", vec![])),
        ]
        .into_iter();

        assert!(
            input_data.len() == parsers.len() && parsers.len() == expected_results.len(),
            "BAD TEST: number of inputs is not equal to number of results or parsers [correct the source data]"
        );

        let dataset = input_data
            .zip(parsers)
            .zip(expected_results)
            .map(|((input, parser), expected)| (input, parser, expected));

        for (input, parser, expected) in dataset {
            assert_eq!(expected, parser.parse(input));
        }
    }
}
