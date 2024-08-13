use crate::Parser;

pub fn map<'a, P, F, A, B>(parser: P, map_fn: F) -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    F: Fn(A) -> B,
{
    move |input| {
        parser
            .parse(input)
            .map(|(next_seq, output)| (next_seq, map_fn(output)))
    }
}

#[cfg(test)]
mod tests {

    #[test]
    fn map_combinator() {
        todo!("Make unit test for combinators::map")
    }
}
