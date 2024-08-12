pub mod basic_types;
pub mod primitives;

pub use basic_types::*;

type NextSeq<'a> = &'a str;
type InputSeq<'a> = &'a str;
type ParsingResult<'a, Output> = Result<(NextSeq<'a>, Output), InputSeq<'a>>;

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
