use crate::{
    grammar::{nqlang, NQLang},
    ParsingResult,
};

pub fn tokenize(input: &str) -> ParsingResult<NQLang> {
    let result = nqlang(input)?;
    if result.0.is_empty() {
        Ok(result)
    } else {
        Err(input)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore = "Not implemented yet"]
    fn tokenize_test() {
        todo!("Make unit test")
    }

    #[test]
    #[ignore = "Not implemented yet"]
    fn token_test() {
        todo!("Make unit test")
    }
}
