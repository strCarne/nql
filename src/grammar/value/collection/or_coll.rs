use crate::{grammar::value::OrdinaryValue, primitives, Parser, ParsingResult};

use super::collection_body;

pub fn or_coll(input: &str) -> ParsingResult<Vec<OrdinaryValue>> {
    collection_body
        .whitespace_wrap()
        .wrap_before(primitives::character('['))
        .wrap_after(primitives::character(']'))
        .parse(input)
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore = "not implemented yet"]
    fn or_coll_test() {
        todo!("Make unit test")
    }
}
