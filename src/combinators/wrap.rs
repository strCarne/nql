use crate::{combinators, Parser};

pub fn wrap<'a, P1, P2, A, B>(parser: P1, wrapper: P2) -> impl Parser<'a, A>
where
    P1: Parser<'a, A> + 'a,
    A: 'a,
    P2: Parser<'a, B> + Clone + 'a,
    B: 'a,
{
    combinators::right(wrapper.clone(), combinators::left(parser, wrapper))
}

pub fn wrap_before<'a, P1, P2, A, B>(parser: P1, wrapper: P2) -> impl Parser<'a, A>
where
    P1: Parser<'a, A> + 'a,
    A: 'a,
    P2: Parser<'a, B> + 'a,
    B: 'a,
{
    combinators::right(wrapper, parser)
}

pub fn wrap_after<'a, P1, P2, A, B>(parser: P1, wrapper: P2) -> impl Parser<'a, A>
where
    P1: Parser<'a, A> + 'a,
    A: 'a,
    P2: Parser<'a, B> + 'a,
    B: 'a,
{
    combinators::left(parser, wrapper)
}

#[cfg(test)]
mod tests {

    #[test]
    #[ignore = "not implemented yet"]
    fn wrap_combinator() {
        todo!("Make unit test")
    }

    #[test]
    #[ignore = "not implemented yet"]
    fn wrap_after_combinator() {
        todo!("Make unit test")
    }

    #[test]
    #[ignore = "not implemented yet"]
    fn wrap_before_combinator() {
        todo!("Make unit test")
    }
}
