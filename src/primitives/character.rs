use crate::Parser;

pub fn character<'a>(expected: char) -> impl Parser<'a, ()> {
    move |input: &'a str| match input.chars().next() {
        Some(c) if c == expected => Ok((&input[expected.len_utf8()..], ())),
        _ => Err(input),
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn character_test() { 
        let input_data = vec!["cringe", "character", "", "\nLF"].into_iter();

        let parsers = vec![
            character('n'),
            character('c'),
            character('\n'),
            character('\n'),
        ]
        .into_iter();

        let expected_results =
            vec![Err("cringe"), Ok(("haracter", ())), Err(""), Ok(("LF", ()))].into_iter();

        assert!(
            input_data.len() == parsers.len() && parsers.len() == expected_results.len(),
            "BAD TEST: number of input is not equal to number of results [correct the source data]"
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
