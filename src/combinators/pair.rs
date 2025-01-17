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

    use crate::{
        basic_types::{self, Number},
        combinators, primitives,
    };

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn pair_combinator_number_literal() {
        let input_data = vec![
            "124 skittle",
            "69.point",
            " 123 hehe, whitespace at the start",
            "2452.145, buga-buga-doo",
        ]
        .into_iter();

        let parsers = vec![
            (basic_types::number, primitives::literal("skittle")),
            (basic_types::number, primitives::literal(".")),
            (basic_types::number, primitives::literal(" ")),
            (basic_types::number, primitives::literal(", ")),
        ]
        .into_iter();

        let expected_results = vec![
            Err(" skittle"),
            Ok(("point", (Number::Integer(69), ()))),
            Err(" 123 hehe, whitespace at the start"),
            Ok(("buga-buga-doo", (Number::Float(2452.145), ()))),
        ]
        .into_iter();

        assert!(
            input_data.len() == parsers.len() && parsers.len() == expected_results.len(),
            "BAD TEST: number of input is not equal to number of results [correct the source data]"
        );

        let dataset = input_data
            .zip(parsers)
            .zip(expected_results)
            .map(|((input, parser), expected)| (input, parser, expected));

        for (input, parser, expected) in dataset {
            let pair_value_parser = pair(parser.0, parser.1);

            let result = pair_value_parser.parse(input);

            assert_eq!(result, expected);
        }
    }

    #[test]
    fn pair_combinator_number_number() {
        let input_data = vec![
            "123 124",
            "69.69",
            " 123 124 hehe, whitespace at the start",
            "10.72.144.25",
        ]
        .into_iter();

        let parsers = vec![
            (
                basic_types::number,
                primitives::literal(" "),
                basic_types::number,
            ),
            (
                basic_types::number,
                primitives::literal("."),
                basic_types::number,
            ),
            (
                basic_types::number,
                primitives::literal(" "),
                basic_types::number,
            ),
            (
                basic_types::number,
                primitives::literal("."),
                basic_types::number,
            ),
        ]
        .into_iter();

        let expected_results = vec![
            Ok(("", (Number::Integer(123), Number::Integer(124)))),
            Err(""),
            Err(" 123 124 hehe, whitespace at the start"),
            Ok(("", (Number::Float(10.72), Number::Float(144.25)))),
        ]
        .into_iter();

        assert!(
            input_data.len() == parsers.len() && parsers.len() == expected_results.len(),
            "BAD TEST: number of input is not equal to number of results [correct the source data]"
        );

        let dataset = input_data
            .zip(parsers)
            .zip(expected_results)
            .map(|((input, parser), expected)| (input, parser, expected));

        for (input, parser, expected) in dataset {
            let pair_value_parser = pair(combinators::left(parser.0, parser.1), parser.2);

            let result = pair_value_parser.parse(input);

            assert_eq!(result, expected);
        }
    }
}
