use crate::{combinators, primitives, Parser};

pub enum QuoteType {
    Single,
    Double,
}

pub fn quoted_string<'a>(quote_type: QuoteType) -> impl Parser<'a, String> {
    let quote = match quote_type {
        QuoteType::Single => "'",
        QuoteType::Double => "\"",
    };

    combinators::map(
        combinators::right(
            primitives::literal(quote),
            combinators::left(
                combinators::zero_or_more(combinators::pred(primitives::any, |c| *c != '"')),
                primitives::literal(quote),
            ),
        ),
        |chars| chars.into_iter().collect(),
    )
}

#[cfg(test)]
mod tests {

    #[test]
    fn quoted_string_test() {
        todo!("Make unit test for basic_types::string::quoted_string")
    }
}