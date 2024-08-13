use crate::Parser;

pub fn pred<'a, P, Output, F>(parser: P, predicate: F) -> impl Parser<'a, Output>
where
    P: Parser<'a, Output>,
    F: Fn(&Output) -> bool,
{
    move |input| {
        if let Ok((next_input, output)) = parser.parse(input) {
            if predicate(&output) {
                return Ok((next_input, output));
            }
        }
        Err(input)
    }
}
