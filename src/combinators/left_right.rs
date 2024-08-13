use crate::Parser;

pub fn left<'a, P1, P2, R1, R2>(parser_1: P1, parser_2: P2) -> impl Parser<'a, R1>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    super::map(super::pair(parser_1, parser_2), |(left, _right)| left)
}

pub fn right<'a, P1, P2, R1, R2>(parser_1: P1, parser_2: P2) -> impl Parser<'a, R2>
where
    P1: Parser<'a, R1>,
    P2: Parser<'a, R2>,
{
    super::map(super::pair(parser_1, parser_2), |(_left, right)| right)
}

#[cfg(test)]
mod tests {

    use pretty_assertions::assert_eq;

    use crate::{
        basic_types::{self, Number},
        primitives::literal,
    };

    use super::*;

    #[test]
    fn left_combinator_literals() {
        let input_data = vec![
            " mama",
            "foobar",
            "sussy_baka",
            "doha",
            "zho-zho",
            "Joe, what's up?",
        ];

        let parsers = vec![
            (literal(" "), literal("mama")),
            (literal("bar"), literal("foo")),
            (literal("sussy"), literal("_")),
            (literal("do"), literal("ho")),
            (literal("zh"), literal("o-")),
            (literal("Joe"), literal(", ")),
        ];

        let expected_results = vec![
            Ok(("", ())),
            Err("foobar"),
            Ok(("baka", ())),
            Err("ha"),
            Ok(("zho", ())),
            Ok(("what's up?", ())),
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
            let left_value_parser = left(parser.0, parser.1);

            let result = left_value_parser.parse(input);

            assert_eq!(expected, result);
        }
    }

    #[test]
    fn left_combinator_literal_number() {
        let input_data = vec![" 123.41", "literal6969.", "aboba1.1gang", "fail123"];

        let parsers = vec![
            (literal(" "), basic_types::number),
            (literal("literal"), basic_types::number),
            (literal("aboba"), basic_types::number),
            (literal("success"), basic_types::number),
        ];

        let expected_results = vec![
            Ok(("", ())),
            Ok((".", ())),
            Ok(("gang", ())),
            Err("fail123"),
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
            let left_value_parser = left(parser.0, parser.1);

            let result = left_value_parser.parse(input);

            assert_eq!(expected, result);
        }
    }

    #[test]
    fn left_combinator_number_literal() {
        let input_data = vec![
            "124 skittle",
            "69.point",
            " 123 hehe, whitespace at the start",
            "2452.145, buga-buga-doo",
        ];

        let parsers = vec![
            (basic_types::number, literal("skittle")),
            (basic_types::number, literal(".")),
            (basic_types::number, literal(" ")),
            (basic_types::number, literal(", ")),
        ];

        let expected_results = vec![
            Err(" skittle"),
            Ok(("point", Number::Integer(69))),
            Err(" 123 hehe, whitespace at the start"),
            Ok(("buga-buga-doo", Number::Float(2452.145))),
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
            let left_value_parser = left(parser.0, parser.1);

            let result = left_value_parser.parse(input);

            assert_eq!(expected, result);
        }
    }

    #[test]
    fn right_combinator_literals() {
        let input_data = vec![
            " mama",
            "foobar",
            "sussy_baka",
            "doha",
            "zho-zho",
            "Joe, what's up?",
        ];

        let parsers = vec![
            (literal(" "), literal("mama")),
            (literal("bar"), literal("foo")),
            (literal("sussy"), literal("_")),
            (literal("do"), literal("ho")),
            (literal("zh"), literal("o-")),
            (literal("Joe"), literal(", ")),
        ];

        let expected_results = vec![
            Ok(("", ())),
            Err("foobar"),
            Ok(("baka", ())),
            Err("ha"),
            Ok(("zho", ())),
            Ok(("what's up?", ())),
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
            let right_value_parser = right(parser.0, parser.1);

            let result = right_value_parser.parse(input);

            assert_eq!(expected, result);
        }
    }

    #[test]
    fn right_combinator_literal_number() {
        let input_data = vec![" 123.41", "literal6969.", "aboba1.1gang", "fail123"];

        let parsers = vec![
            (literal(" "), basic_types::number),
            (literal("literal"), basic_types::number),
            (literal("aboba"), basic_types::number),
            (literal("success"), basic_types::number),
        ];

        let expected_results = vec![
            Ok(("", Number::Float(123.41))),
            Ok((".", Number::Integer(6969))),
            Ok(("gang", Number::Float(1.1))),
            Err("fail123"),
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
            let right_value_parser = right(parser.0, parser.1);

            let result = right_value_parser.parse(input);

            assert_eq!(expected, result);
        }
    }

    #[test]
    fn right_combinator_number_literal() {
        let input_data = vec![
            "124 skittle",
            "69.point",
            " 123 hehe, whitespace at the start",
            "2452.145, buga-buga-doo",
        ];

        let parsers = vec![
            (basic_types::number, literal("skittle")),
            (basic_types::number, literal(".")),
            (basic_types::number, literal(" ")),
            (basic_types::number, literal(", ")),
        ];

        let expected_results = vec![
            Err(" skittle"),
            Ok(("point", ())),
            Err(" 123 hehe, whitespace at the start"),
            Ok(("buga-buga-doo", ())),
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
            let right_value_parser = right(parser.0, parser.1);

            let result = right_value_parser.parse(input);

            assert_eq!(expected, result);
        }
    }
}
