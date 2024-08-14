use crate::{combinators, primitives, Parser, ParsingResult};

pub fn regular_string(mut input: &str) -> ParsingResult<String> {
    // 1. Check if it is a quoted string
    if let Ok(_) = combinators::single_of(vec![
        Box::new(primitives::literal("'")),
        Box::new(primitives::literal("\"")),
    ])
    .parse(input)
    {
        return Err(input);
    }

    let mut matched = String::new();

    let non_whitespace_symbols = combinators::pred(primitives::any, |c| !c.is_whitespace());

    while let Ok((next_input, output)) = non_whitespace_symbols.parse(input) {
        matched.push(output);
        input = next_input;
    }

    Ok((input, matched))
}

#[cfg(test)]
mod tests {

    #[test]
    fn regular_string_test() {
        todo!("Make unit test for basic_types::string::regular_string")
    }
}