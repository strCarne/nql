use crate::{Parser, ParsingResult};

impl<'a, Output, F> Parser<'a, Output> for F
where
    F: Fn(&'a str) -> ParsingResult<'a, Output>,
{
    fn parse(&self, input: &'a str) -> ParsingResult<'a, Output> {
        self(input)
    }
}
