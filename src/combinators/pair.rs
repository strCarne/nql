use crate::Parser;

pub fn pair<'a, P1, P2, R1, R2>(parser_1: P1, parser_2: P2) -> impl Parser<'a, (R1, R2)>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    move |input| match parser_1.parse(input) {
        Ok((next_input, result_1)) => match parser_2.parse(next_input) {
            Ok((next_input, result_2)) => Ok((next_input, (result_1, result_2))),
            Err(err) => Err(err),
        },

        Err(err) => Err(err),
    }
}

#[cfg(test)]
mod tests {

    use crate::{basic_types::{self, Number}, combinators, primitives};

    use super::*;

    #[test]
    fn pair_combinator_number_literal() {
        let input_data = vec![
            "124 skittle",
            "69.point",
            " 123 hehe, whitespace at the start",
            "2452.145, buga-buga-doo",
        ];

        let parsers = vec![
            (basic_types::number, primitives::literal("skittle")),
            (basic_types::number, primitives::literal(".")),
            (basic_types::number, primitives::literal(" ")),
            (basic_types::number, primitives::literal(", ")),
        ];

        let expected_results = vec![
            Err(" skittle"),
            Ok(("point", (Number::Integer(69), ()))),
            Err(" 123 hehe, whitespace at the start"),
            Ok(("buga-buga-doo", (Number::Float(2452.145), ()))),
        ];

        assert!(
            input_data.len() == parsers.len() && parsers.len() == expected_results.len(),
            "BAD TEST: number of input is not equal to number of results [correct the source data]"
        );

        let dataset = input_data
            .into_iter()
            .zip(parsers.into_iter())
            .zip(expected_results.into_iter());

        for ((input, parser), expected) in dataset {
            let pair_value_parser = pair(parser.0, parser.1);

            let result = pair_value_parser.parse(input);

            assert_eq!(expected, result);
        }
    }

}
