use crate::{
    grammar::value::{ordinary_value, OrdinaryValue},
    primitives, Parser, ParsingResult,
};

pub fn coll_primitive(mut input: &str) -> ParsingResult<Vec<OrdinaryValue>> {
    let (next_input, head) = ordinary_value.whitespace_wrap().parse(input)?;

    let mut elems = Vec::new();
    elems.push(head);
    input = next_input;

    let parser = primitives::character(',')
        .whitespace_wrap()
        .and_then(|_| ordinary_value.whitespace_wrap());

    while let Ok((next_input, elem)) = parser.parse(input) {
        elems.push(elem);
        input = next_input;
    }

    Ok((input, elems))
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore = "Not implemented yet"]
    fn coll_primitive_test() {
        todo!("Make unit test")
    }
}