use crate::Parser;

pub fn one_or_more<'a, P, Output>(parser: P) -> impl Parser<'a, Vec<Output>>
where
    P: Parser<'a, Output>,
{
    move |mut input| {
        let mut result = Vec::new();

        if let Ok((next_input, first_output)) = parser.parse(input) {
            input = next_input;
            result.push(first_output);
        } else {
            return Err(input);
        }

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
    fn one_or_more_combinator() {
        todo!("Make unit test for combinators::one_or_more")
    }
}
