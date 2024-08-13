use crate::Parser;

pub fn single_of<'a, Output>(
    parsers: Vec<Box<dyn Parser<'a, Output> + 'a>>,
) -> impl Parser<'a, Output> {
    move |input| {
        for parser in &parsers {
            if let Ok(success) = parser.parse(input) {
                return Ok(success);
            }
        }
        Err(input)
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn single_of_combinator() {
        todo!("Make unit test for combinators::single_of")
    }
}
