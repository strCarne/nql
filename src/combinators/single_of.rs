use crate::{BoxedParser, Parser};

pub fn single_of<'a, Output>(parsers: Vec<BoxedParser<'a, Output>>) -> impl Parser<'a, Output> {
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
                primitives::literal("a").into_box(),
                primitives::literal("b").into_box(),
                primitives::literal(" ").into_box(),
            ]),
            single_of(vec![
                primitives::literal("or").into_box(),
                primitives::literal("wo").into_box(),
                primitives::literal("rd").into_box(),
            ]),
            single_of(vec![
                primitives::literal("i").into_box(),
                primitives::literal("i").into_box(),
                primitives::literal("l").into_box(),
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
            assert_eq!(parser.parse(input), expected);
        }
    }
}
