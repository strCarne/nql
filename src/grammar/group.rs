use crate::{combinators, primitives, Parser, ParsingResult};

use super::{nqlang, NQLang};

pub fn group(input: &str) -> ParsingResult<NQLang> {
    combinators::right(
        primitives::character('('),
        combinators::left(nqlang, primitives::character(')')),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {

    #[test]
    #[ignore = "not implemented yet"]
    fn group_test() {
        todo!("Make unit test")
    }
}
