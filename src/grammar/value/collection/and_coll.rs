use crate::{combinators, grammar::value::OrdinaryValue, primitives, Parser, ParsingResult};

use super::coll_primitive;

// AND_COLL ::= \{ ORDINARY_VALUE (\, ORDINARY_VALUE)* \}
pub fn and_coll(input: &str) -> ParsingResult<Vec<OrdinaryValue>> {
    let open_brace = primitives::character('{');
    let close_brace = primitives::character('}');

    combinators::right(
        open_brace,
        combinators::left(coll_primitive.whitespace_wrap(), close_brace),
    )
    .parse(input)
}

#[cfg(test)]
mod tests {

    #[test]
    #[ignore = "not implemented yet"]
    fn and_coll_test() {
        todo!("Make unit test")
    }
}
