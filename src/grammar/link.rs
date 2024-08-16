use crate::{combinators, primitives, Parser, ParsingResult};

#[derive(Debug, PartialEq, Eq)]
pub enum Link {
    And,
    Or,
    Xor,
}

pub fn link(input: &str) -> ParsingResult<Link> {
    combinators::single_of(vec![
        primitives::character('&').map(|_| Link::And).into_box(),
        primitives::character('|').map(|_| Link::Or).into_box(),
        primitives::character('^').map(|_| Link::Xor).into_box(),
    ]).parse(input)
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn link_test() {
        let input_data = vec![" &", "&", "val | val", "| val", "#", "^ xorik"].into_iter();

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
            assert_eq!(link(input), expected);
        }
    }
}
