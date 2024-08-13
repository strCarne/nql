pub mod basic_types;
pub mod primitives;
pub mod combinators;

type NextInput<'a> = &'a str;
type Input<'a> = &'a str;
type ParsingResult<'a, Output> = Result<(NextInput<'a>, Output), Input<'a>>;

pub trait Parser<'a, Output> {
    fn parse(&self, input: &'a str) -> ParsingResult<'a, Output>;
}

impl<'a, Output, F> Parser<'a, Output> for F
where
    F: Fn(&'a str) -> ParsingResult<'a, Output>,
{
    fn parse(&self, input: &'a str) -> ParsingResult<'a, Output> {
        self(input)
    }
}
