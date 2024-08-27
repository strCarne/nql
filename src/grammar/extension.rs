use crate::{combinators, primitives, Parser, ParsingResult};

use super::{key_value, KeyValue};

pub fn extension(input: &str) -> ParsingResult<KeyValue> {
    primitives::character('$')
        .and_then(|_| key_value)
        .wrap_after(|input| {
            combinators::zero_or_more(primitives::any.pred(|c| c.is_whitespace())).parse(input)
        })
        .parse(input)
}

#[cfg(test)]
mod tests {

    #[test]
    #[ignore = "not implemented"]
    fn extension_test() {
        
    }
}
