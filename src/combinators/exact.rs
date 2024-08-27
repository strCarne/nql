use crate::Parser;

pub fn exact<'a, P, A>(parser: P, n: u64) -> impl Parser<'a, Vec<A>>
where
    P: Parser<'a, A>,
{
    move |input| {
        let mut outputs = Vec::with_capacity(n as usize);
        let mut next_input = input;
        for _ in 0..n {
            outputs.push({
                let res = parser.parse(next_input)?;
                next_input = res.0;
                res.1
            });
        }
        Ok((next_input, outputs))
    }
}

#[cfg(test)]
mod tests {

    #[test]
    #[ignore = "not implemented yet"]
    fn exact_combinator() {
        todo!("Make unit test")
    }
}
