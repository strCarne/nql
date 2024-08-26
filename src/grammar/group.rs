use crate::{combinators, primitives, Parser, ParsingResult};

use super::{nqlang, NQLang};

pub fn group(input: &str) -> ParsingResult<NQLang> {
    combinators::right(
        primitives::character('('),
        combinators::left(nqlang, primitives::character(')')),
    )
    .parse(input)
}
