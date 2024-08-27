use crate::{
    grammar::value::{ordinary_value, OrdinaryValue},
    primitives, Parser, ParsingResult,
};

pub fn collection_body(mut input: &str) -> ParsingResult<Vec<OrdinaryValue>> {
    let (next_input, head) = ordinary_value.parse(input)?;

    let mut elems = Vec::new();
    elems.push(head);
    input = next_input;

    let parser = primitives::character(',')
        .whitespace_wrap()
        .and_then(|_| ordinary_value);

    while let Ok((next_input, elem)) = parser.parse(input) {
        elems.push(elem);
        input = next_input;
    }

    Ok((input, elems))
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore = "not implemented yet"]
    fn collection_body_test() {
        todo!("Make unit test")
    }
}
