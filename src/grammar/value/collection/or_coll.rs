use crate::{combinators, grammar::value::OrdinaryValue, primitives, Parser, ParsingResult};

use super::coll_primitive;

pub fn or_coll(input: &str) -> ParsingResult<Vec<OrdinaryValue>> {
    let open_brace = primitives::character('[');
    let close_brace = primitives::character(']');

    combinators::right(
        open_brace,
        combinators::left(coll_primitive.whitespace_wrap(), close_brace),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore = "Not implemented yet"]
    fn or_coll_test() {
        todo!("Make unit test")
    }
}
