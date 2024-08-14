use crate::{
    basic_types::{self, Number},
    combinators, BoxedParser, Parser,
};

#[derive(Debug)]
pub enum OrdinaryValue {
    Boolean(bool),
    Number(Number),
    String(String),
}

pub fn ordinary_value<'a>() -> impl Parser<'a, OrdinaryValue> {
    combinators::single_of(vec![
        BoxedParser::new(basic_types::boolean.map(|output| OrdinaryValue::Boolean(output))),
        BoxedParser::new(basic_types::number.map(|output| OrdinaryValue::Number(output))),
        BoxedParser::new(basic_types::string.map(|output| OrdinaryValue::String(output))),
    ])
}

#[cfg(test)]
mod tests {

    #[test]
    fn ordinary_value_test() {
        todo!("Make unit test for grammar::value::ordinary_value")
    }
}
