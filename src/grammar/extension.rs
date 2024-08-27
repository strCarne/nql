use crate::{combinators, primitives, Parser, ParsingResult};

use super::{key_value, KeyValue};

pub fn extension(input: &str) -> ParsingResult<KeyValue> {
    primitives::character('$')
        .and_then(|_| key_value)
        .wrap_after(|input| {
            combinators::zero_or_more(primitives::any.pred(|c| c.is_whitespace())).parse(input)
        })
        .parse(input)
}

#[cfg(test)]
mod tests {
    use crate::grammar::{
        extension,
        value::{OrdinaryValue, Value},
        ComparasionOperator, KeyValue,
    };

    #[test]
    fn extension_test() {
        let input_data = vec!["$unique=true", " $unique=true", "$unique=true "].into_iter();

        let expected_results = vec![
            Ok((
                "",
                KeyValue {
                    k: String::from("unique"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::Boolean(true)),
                },
            )),
            Err(" $unique=true"),
            Ok((
                "",
                KeyValue {
                    k: String::from("unique"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::Boolean(true)),
                },
            )),
        ]
        .into_iter();

        assert_eq!(
            input_data.len(),
            expected_results.len(),
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        for (input, expected) in input_data.zip(expected_results) {
            assert_eq!(extension(input), expected);
        }
    }
}
