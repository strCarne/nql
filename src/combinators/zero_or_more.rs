use crate::Parser;

pub fn zero_or_more<'a, P, Output>(parser: P) -> impl Parser<'a, Vec<Output>>
where
    P: Parser<'a, Output>,
{
    move |mut input| {
        let mut result = Vec::new();

        while let Ok((next_input, next_output)) = parser.parse(input) {
            input = next_input;
            result.push(next_output);
        }

        Ok((input, result))
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn zero_or_more_combinator() {
        todo!("Make unit test for combinators::zero_or_more")
    }
}
