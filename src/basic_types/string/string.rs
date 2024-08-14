use super::*;
use crate::{combinators, BoxedParser, Parser, ParsingResult};

pub fn string(input: &str) -> ParsingResult<String> {
    combinators::single_of(vec![
        BoxedParser::new(regular_string),
        BoxedParser::new(quoted_string(QuoteType::Single)),
        BoxedParser::new(quoted_string(QuoteType::Double)),
    ])
    .parse(input)
}

#[cfg(test)]
mod tests {

    #[test]
    fn string_test() {
        todo!("Make unit test for basic_types::string")
    }
}
