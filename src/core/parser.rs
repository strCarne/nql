use crate::combinators;
use crate::BoxedParser;

type NextInput<'a> = &'a str;
type Input<'a> = &'a str;
pub type ParsingResult<'a, Output> = Result<(NextInput<'a>, Output), Input<'a>>;

pub trait Parser<'a, Output> {
    fn parse(&self, input: &'a str) -> ParsingResult<'a, Output>;

    fn map<F, NewOutput>(self, map_fn: F) -> impl Parser<'a, NewOutput>
    where
        Self: Sized + 'a,
        Output: 'a,
        NewOutput: 'a,
        F: Fn(Output) -> NewOutput + 'a,
    {
        combinators::map(self, map_fn)
    }

    fn pred<F>(self, pred_fn: F) -> impl Parser<'a, Output>
    where
        Self: Sized + 'a,
        Output: 'a,
        F: Fn(&Output) -> bool + 'a,
    {
        combinators::pred(self, pred_fn)
    }

    fn into_box(self) -> BoxedParser<'a, Output>
    where
        Self: Sized + 'a,
    {
        BoxedParser::new(self)
    }

    fn and_then<F, NextParser, NewOutput>(self, f: F) -> impl Parser<'a, NewOutput>
    where
        Self: Sized + 'a,
        Output: 'a,
        NewOutput: 'a,
        NextParser: Parser<'a, NewOutput> + 'a,
        F: Fn(Output) -> NextParser + 'a,
    {
        combinators::and_then(self, f)
    }

    fn wrap<Wrapper, WrapperOutput>(self, wrapper: Wrapper) -> impl Parser<'a, Output>
    where
        Self: Sized + 'a,
        Output: 'a,
        WrapperOutput: 'a,
        Wrapper: Parser<'a, WrapperOutput> + Clone + 'a,
    {
        combinators::wrap(self, wrapper)
    }

    fn wrap_before<Wrapper, WrapperOutput>(self, wrapper: Wrapper) -> impl Parser<'a, Output>
    where
        Self: Sized + 'a,
        Output: 'a,
        WrapperOutput: 'a,
        Wrapper: Parser<'a, WrapperOutput> + 'a,
    {
        combinators::wrap_before(self, wrapper)
    }

    fn wrap_after<Wrapper, WrapperOutput>(self, wrapper: Wrapper) -> impl Parser<'a, Output>
    where
        Self: Sized + 'a,
        Output: 'a,
        WrapperOutput: 'a,
        Wrapper: Parser<'a, WrapperOutput> + 'a,
    {
        combinators::wrap_after(self, wrapper)
    }
}
