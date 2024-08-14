pub mod basic_types;
pub mod combinators;
pub mod primitives;

type NextInput<'a> = &'a str;
type Input<'a> = &'a str;
type ParsingResult<'a, Output> = Result<(NextInput<'a>, Output), Input<'a>>;

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
}

impl<'a, Output, F> Parser<'a, Output> for F
where
    F: Fn(&'a str) -> ParsingResult<'a, Output>,
{
    fn parse(&self, input: &'a str) -> ParsingResult<'a, Output> {
        self(input)
    }
}

pub struct BoxedParser<'a, Output> {
    inner_parser: Box<dyn Parser<'a, Output> + 'a>,
}

impl<'a, Output> BoxedParser<'a, Output> {
    pub fn new<P>(parser: P) -> Self
    where
        P: Parser<'a, Output> + 'a,
    {
        BoxedParser {
            inner_parser: Box::new(parser),
        }
    }
}

impl<'a, Output> Parser<'a, Output> for BoxedParser<'a, Output> {
    fn parse(&self, input: &'a str) -> ParsingResult<'a, Output> {
        self.inner_parser.parse(input)
    }
}
