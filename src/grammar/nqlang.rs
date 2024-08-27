use crate::{combinators, Parser, ParsingResult};

use super::{link, unit, Link, Unit};

pub type NQLang = Vec<NQToken>;

#[derive(Debug, PartialEq)]
pub enum NQToken {
    Unit(Unit),
    Link(Link),
}

// NQLANG ::= UNIT (LINK UNIT)*
pub fn nqlang(mut input: &str) -> ParsingResult<NQLang> {
    let mut nq_lang = Vec::new();

    let (next_input, unit_token) = unit.parse(input)?;
    nq_lang.push(NQToken::Unit(unit_token));
    input = next_input;

    let parser = combinators::pair(link, unit);
    loop {
        match parser.parse(input) {
            Ok((next_input, nq_tokens)) => {
                nq_lang.push(NQToken::Link(nq_tokens.0));
                nq_lang.push(NQToken::Unit(nq_tokens.1));
                input = next_input;
            }
            Err(_) => {
                break;
            }
        }
    }

    Ok((input, nq_lang))
}

#[cfg(test)]
mod tests {

    #[test]
    #[ignore = "not implemented yet"]
    fn nqlang_test() {
        todo!("Make unit test")
    }
}
