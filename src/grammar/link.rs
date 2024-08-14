use crate::{combinators, primitives, BoxedParser, Parser};

#[derive(Debug, PartialEq, Eq)]
pub enum Link {
    And,
    Or,
    Xor,
}

pub fn link<'a>() -> impl Parser<'a, Link> {
    combinators::single_of(vec![
        BoxedParser::new(primitives::character('&').map(|_| Link::And)),
        BoxedParser::new(primitives::character('|').map(|_| Link::Or)),
        BoxedParser::new(primitives::character('^').map(|_| Link::Xor)),
    ])
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn link_test() {
        let input_data = vec![" &", "&", "val | val", "| val", "#", "^ xorik"].into_iter();

        let parser = link();

        let expected_results = vec![
            Err(" &"),
            Ok(("", Link::And)),
            Err("val | val"),
            Ok((" val", Link::Or)),
            Err("#"),
            Ok((" xorik", Link::Xor)),
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
