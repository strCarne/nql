use crate::{combinators, primitives, Parser, ParsingResult};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Link {
    And,
    Or,
}

pub fn link(input: &str) -> ParsingResult<Link> {
    let res = combinators::single_of(vec![
        primitives::character('&').map(|_| Link::And),
        primitives::character('|').map(|_| Link::Or),
        primitives::any.pred(|c| c.is_whitespace()).map(|_| Link::And),
    ]).parse(input);

    res
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn link_test() {
        let input_data = vec![" ", "&", "val | val", "| val", "#"].into_iter();

        let expected_results = vec![
            Ok(("", Link::And)),
            Ok(("", Link::And)),
            Err("val | val"),
            Ok((" val", Link::Or)),
            Err("#"),
        ]
        .into_iter();

        assert_eq!(
            input_data.len(), 
            expected_results.len(), 
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        for (input, expected) in input_data.zip(expected_results) {
            assert_eq!(link(input), expected);
        }
    }
}
