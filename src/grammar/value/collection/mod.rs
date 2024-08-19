use crate::{combinators, Parser, ParsingResult};

use super::Value;

mod and_coll;
pub use and_coll::and_coll;

mod or_coll;
pub use or_coll::or_coll;

#[derive(Debug)]
pub enum Collection {
    AndColl(Vec<Value>),
    OrColl(Vec<Value>),
}

pub fn collection(input: &str) -> ParsingResult<Collection> {
    combinators::single_of(vec![and_coll.into_box(), or_coll.into_box()]).parse(input)
}
