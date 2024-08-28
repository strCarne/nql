use crate::{combinators, Parser, ParsingResult};

use super::{link, unit, NQLang, NQToken};

pub fn units_sequance(mut input: &str) -> ParsingResult<NQLang> {
    let mut sequance = Vec::new();

    let (next_input, unit_token) = unit(input)?;
    sequance.push(NQToken::Unit(unit_token));
    input = next_input;

    let parser = combinators::pair(link, unit);
    loop {
        match parser.parse(input) {
            Ok((next_input, nq_tokens)) => {
                sequance.push(NQToken::Link(nq_tokens.0));
                sequance.push(NQToken::Unit(nq_tokens.1));
                input = next_input;
            }
            Err(_) => {
                return Ok((input, sequance));
            }
        }
    }
}

#[cfg(test)]
mod tests {

    #[test]
    #[ignore = "not implemented"]
    fn units_sequance_test() {
        todo!("Implement test");
    }
}
