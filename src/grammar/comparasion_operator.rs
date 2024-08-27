use crate::{combinators::single_of, primitives::literal, Parser, ParsingResult};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ComparasionOperator {
    Eq,
    NotEq,
    Less,
    Greater,
    LessEq,
    GreaterEq,
}

// COMPARSION_OPERATOR ::= '=' | '!=' | '<' | '>' | '<=' | '>='
pub fn comparasion_operator(input: &str) -> ParsingResult<ComparasionOperator> {
    single_of(vec![
        literal("!=").map(|_| ComparasionOperator::NotEq),
        literal("=").map(|_| ComparasionOperator::Eq),
        literal("<=").map(|_| ComparasionOperator::LessEq),
        literal(">=").map(|_| ComparasionOperator::GreaterEq),
        literal("<").map(|_| ComparasionOperator::Less),
        literal(">").map(|_| ComparasionOperator::Greater),
    ])
    .whitespace_wrap()
    .parse(input)
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn comparasion_operator_test() {
        use ComparasionOperator::*;

        let input_data = vec![
            " = ",
            "<>",
            "  aboba > boba ",
            "=!",
            "!=",
            "f <=",
            "<=",
            "=<",
            "=>",
            ">=",
            "",
        ]
        .into_iter();

        let expected_results = vec![
            Ok(("", Eq)),
            Ok((">", Less)),
            Err("aboba > boba "),
            Ok(("!", Eq)),
            Ok(("", NotEq)),
            Err("f <="),
            Ok(("", LessEq)),
            Ok(("<", Eq)),
            Ok((">", Eq)),
            Ok(("", GreaterEq)),
            Err(""),
        ]
        .into_iter();

        assert_eq!(
            input_data.len(),
            expected_results.len(),
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        for (input, expected) in input_data.zip(expected_results) {
            assert_eq!(comparasion_operator(input), expected);
        }
    }
}
