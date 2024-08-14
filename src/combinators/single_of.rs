use crate::Parser;

pub fn single_of<'a, Output>(
    parsers: Vec<Box<dyn Parser<'a, Output> + 'a>>,
) -> impl Parser<'a, Output> {
    move |input| {
        for parser in &parsers {
            if let Ok(success) = parser.parse(input) {
                return Ok(success);
            }
        }
        Err(input)
    }
}

#[cfg(test)]
mod tests {

    use crate::primitives;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn single_of_combinator() {
        let input_data = vec![" ", "word", "fail"].into_iter();

        let parsers = vec![
            single_of(vec![
                Box::new(primitives::literal("a")),
                Box::new(primitives::literal("b")),
                Box::new(primitives::literal(" ")),
            ]),
            single_of(vec![
                Box::new(primitives::literal("or")),
                Box::new(primitives::literal("wo")),
                Box::new(primitives::literal("rd")),
            ]),
            single_of(vec![
                Box::new(primitives::literal("i")),
                Box::new(primitives::literal("i")),
                Box::new(primitives::literal("l")),
            ]),
        ]
        .into_iter();

        let expected_results = vec![Ok(("", ())), Ok(("rd", ())), Err("fail")];

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
