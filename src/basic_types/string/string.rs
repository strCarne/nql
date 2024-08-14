use super::*;
use crate::{combinators, Parser, ParsingResult};

pub fn string(input: &str) -> ParsingResult<String> {
    combinators::single_of(vec![
        Box::new(regular_string),
        Box::new(quoted_string(QuoteType::Single)),
        Box::new(quoted_string(QuoteType::Double)),
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