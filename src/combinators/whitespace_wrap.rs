use crate::{combinators, primitives, Parser};

pub fn whitespace_wrap<'a, P, A>(p: P) -> impl Parser<'a, A>
where
    P: Parser<'a, A> + 'a,
    A: 'a,
{
    let whitespaces = || combinators::zero_or_more(primitives::any.pred(|c| c.is_whitespace()));

    combinators::right(whitespaces(), combinators::left(p, whitespaces()))
}
