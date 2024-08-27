use crate::ParsingResult;

pub fn any(input: &str) -> ParsingResult<char> {
    match input.chars().next() {
        Some(c) => Ok((&input[c.len_utf8()..], c)),
        _ => Err(input),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore = "not implemented yet"]
    fn any_test() {
        todo!("Make unit test")
    }
}
