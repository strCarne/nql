use crate::Parser;

pub fn map<'a, P, F, A, B>(parser: P, map_fn: F) -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    F: Fn(A) -> B,
{
    move |input| {
        parser
            .parse(input)
            .map(|(next_seq, output)| (next_seq, map_fn(output)))
    }
}

#[cfg(test)]
mod tests {

    use crate::basic_types::{self, Number};

    use super::*;

    #[test]
    fn map_combinator() {
        let input = vec![
            "123",
            "531 ",
            " 124",
            "",
            ".",
            "3214.",
            "124.1",
            "214124.124124",
            "-1124",
            "-12451.",
            "-124.14",
        ]
        .into_iter();

        let number_parser_as_string = map(basic_types::number, |num| match num {
            Number::Float(float) => float.to_string(),
            Number::Integer(int) => int.to_string(),
        });

        let expected = vec![
            Ok(("", String::from("123"))),
            Ok((" ", String::from("531"))),
            Err(" 124"),
            Err(""),
            Err("."),
            Ok((".", String::from("3214"))),
            Ok(("", String::from("124.1"))),
            Ok(("", String::from("214124.124124"))),
            Ok(("", String::from("-1124"))),
            Ok((".", String::from("-12451"))),
            Ok(("", String::from("-124.14"))),
        ]
        .into_iter();

        assert_eq!(
            input.len(),
            expected.len(),
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        for (input, expected) in input.zip(expected) {
            let got = number_parser_as_string.parse(input);
            assert_eq!(expected, got);
        }
    }
}
