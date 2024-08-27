use crate::{combinators, Parser, ParsingResult};

use super::OrdinaryValue;

mod and_coll;
pub use and_coll::and_coll;

mod or_coll;
pub use or_coll::or_coll;

mod coll_primitive;
pub use coll_primitive::coll_primitive;

#[derive(Debug)]
pub enum Collection {
    AndColl(Vec<OrdinaryValue>),
    OrColl(Vec<OrdinaryValue>),
}

pub fn collection(input: &str) -> ParsingResult<Collection> {
    combinators::single_of(vec![
        and_coll
            .map(|output| Collection::AndColl(output)),
        or_coll
            .map(|output| Collection::OrColl(output)),
    ])
    .parse(input)
}
