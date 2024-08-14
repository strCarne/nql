use crate::{combinators, primitives, BoxedParser, Parser};

pub enum Link {
    And,
    Or,
    Xor,
}

pub fn link<'a>() -> impl Parser<'a, Link> {
    combinators::single_of(vec![
        BoxedParser::new(primitives::character('&').map(|_| Link::And)),
        BoxedParser::new(primitives::character('|').map(|_| Link::Or)),
        BoxedParser::new(primitives::character('^').map(|_| Link::Xor)),
    ])
}
