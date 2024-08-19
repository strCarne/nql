use crate::{combinators::single_of, primitives::literal, Parser, ParsingResult};

#[derive(Debug)]
pub enum ComparasionOperator {
    Eq,
    NotEq,
    Less,
    Greater,
    LessOrEq,
    GreaterOrEq,
}

// COMPARSION_OPERATOR ::= '=' | '!=' | '<' | '>' | '<=' | '>='
pub fn comparasion_operator(input: &str) -> ParsingResult<ComparasionOperator> {
    single_of(vec![
        literal("!=").map(|_| ComparasionOperator::NotEq),
        literal("=").map(|_| ComparasionOperator::Eq),
        literal("<=").map(|_| ComparasionOperator::LessOrEq),
        literal(">=").map(|_| ComparasionOperator::GreaterOrEq),
        literal("<").map(|_| ComparasionOperator::Less),
        literal(">").map(|_| ComparasionOperator::Greater),
    ])
    .parse(input)
}
