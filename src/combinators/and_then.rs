use crate::Parser;

pub fn and_then<'a, P, F, A, B, NextP>(parser: P, f: F) -> impl Parser<'a, B>
where
    P: Parser<'a, A>,
    NextP: Parser<'a, B>,
    F: Fn(A) -> NextP,
{
    move |input| {
        parser
            .parse(input)
            .and_then(|(next_input, output)| f(output).parse(next_input))
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_and_then() {
        todo!("Make unit test");
    }
}
