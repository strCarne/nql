use crate::Parser;

pub fn zero_or_one<'a, P, Output>(parser: P) -> impl Parser<'a, Option<Output>>
where
    P: Parser<'a, Output>,
{
    move |input| match parser.parse(input) {
        Ok((next_input, output)) => Ok((next_input, Some(output))),
        Err(err) => Ok((err, None)),
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn zero_or_one_combinator() {
        todo!("Make unit test for combinators::zero_or_one")
    }
}
