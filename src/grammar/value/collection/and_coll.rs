use crate::{grammar::value::OrdinaryValue, primitives, Parser, ParsingResult};

use super::collection_body;

// AND_COLL ::= \{ ORDINARY_VALUE (\, ORDINARY_VALUE)* \}
pub fn and_coll(input: &str) -> ParsingResult<Vec<OrdinaryValue>> {
    collection_body
        .whitespace_wrap()
        .wrap_before(primitives::character('{'))
        .wrap_after(primitives::character('}'))
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
