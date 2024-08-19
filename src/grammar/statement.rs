use crate::{combinators, primitives, Parser, ParsingResult};

use super::{key_value, KeyValue};

#[derive(Debug)]
pub enum Statement {
    Field(KeyValue),
    Extension(KeyValue),
}

pub fn statement(input: &str) -> ParsingResult<Statement> {
    combinators::single_of(vec![
        key_value.map(|output| Statement::Field(output)),
        primitives::character('$')
            .and_then(|_| key_value.map(|output| Statement::Extension(output))),
    ])
    .parse(input)
}
