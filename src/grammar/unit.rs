use crate::{combinators, Parser, ParsingResult};

use super::{group, statement, NQLang, Statement};

#[derive(Debug)]
pub enum Unit {
    Stmt(Statement),
    Grp(NQLang),
}

pub fn unit(input: &str) -> ParsingResult<Unit> {
    combinators::single_of(vec![
        group.map(|grp| Unit::Grp(grp)),
        statement.map(|stmt| Unit::Stmt(stmt)),
    ])
    .parse(input)
}
