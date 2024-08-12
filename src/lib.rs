pub mod basic_types;

#[allow(unused_imports)]
pub use basic_types::*;

type NextSeq<'a> = &'a str;
type ParsedSeq = String;
type InputSeq<'a> = &'a str;
type ParsingResult<'a> = Result<(NextSeq<'a>, ParsedSeq), InputSeq<'a>>;

pub trait Parser {
    fn parse<'a>(input: &'a str) -> ParsingResult<'a>;
}
