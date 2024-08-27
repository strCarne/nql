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

    use crate::primitives;

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn wrap_combinator() {
        let input_data = vec![",comma,", "comma,"].into_iter();

        let comma_wrapper = |input| primitives::literal(",").parse(input);
        let parser = wrap(primitives::literal("comma"), comma_wrapper);

        let expected_results = vec![Ok(("", ())), Err("comma,")].into_iter();

        assert_eq!(
            input_data.len(),
            expected_results.len(),
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        for (input, expected) in input_data.zip(expected_results) {
            assert_eq!(parser.parse(input), expected);
        }
    }

    #[test]
    fn wrap_after_combinator() {
        let input_data = vec![",comma,", "comma,"].into_iter();

        let comma_wrapper = |input| primitives::literal(",").parse(input);
        let parser = wrap_after(primitives::literal("comma"), comma_wrapper);

        let expected_results = vec![Err(",comma,"), Ok(("", ()))].into_iter();

        assert_eq!(
            input_data.len(),
            expected_results.len(),
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        for (input, expected) in input_data.zip(expected_results) {
            assert_eq!(parser.parse(input), expected);
        }
    }

    #[test]
    fn wrap_before_combinator() {
        let input_data = vec![",comma,", "comma,"].into_iter();

        let comma_wrapper = |input| primitives::literal(",").parse(input);
        let parser = wrap_before(primitives::literal("comma"), comma_wrapper);

        let expected_results = vec![Ok((",", ())), Err("comma,")].into_iter();

        assert_eq!(
            input_data.len(),
            expected_results.len(),
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        for (input, expected) in input_data.zip(expected_results) {
            assert_eq!(parser.parse(input), expected);
        }
    }
}
