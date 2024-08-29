use crate::grammar::{KeyValue, NQLang, NQToken, Unit};

mod conv;

pub fn convert(tokens: &NQLang, _extensions: Option<&Vec<KeyValue>>) -> String {
    let mut buf = String::new();

    for token in tokens {
        match token {
            NQToken::Unit(u) => match u {
                Unit::Stmt(stmt) => {
                    buf += &conv::statement(stmt);
                }
                Unit::Grp(grp) => {
                    buf += &conv::group(grp);
                }
            },
            NQToken::Link(l) => {
                buf += &conv::link(l);
            }
            _ => panic!("tokens must contain only NQToken::Unit and NQToken::Link"),
        }
    }

    buf
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::{
        basic_types::Number,
        grammar::{
            value::{Collection, OrdinaryValue, Range, RangeBounds, RangeOp, Value},
            ComparasionOperator, Link,
        },
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn convert_test() {
        let input_data: std::vec::IntoIter<(Vec<NQToken>, Option<&Vec<KeyValue>>)> = vec![
            (
                vec![
                    NQToken::Unit(Unit::Stmt(KeyValue {
                        k: String::from("key_1"),
                        op: ComparasionOperator::Eq,
                        v: Value::Range(Range {
                            bounds: RangeBounds::NumberRange(
                                Number::Integer(-1),
                                Number::Integer(1),
                            ),
                            op: RangeOp::IE,
                        }),
                    })),
                    NQToken::Link(Link::And),
                    NQToken::Link(Link::Or),
                    NQToken::Link(Link::And),
                ],
                None,
            ),
            (
                vec![
                    NQToken::Unit(Unit::Grp(vec![
                        NQToken::Unit(Unit::Grp(vec![
                            NQToken::Unit(Unit::Stmt(KeyValue {
                                k: String::from("name"),
                                op: ComparasionOperator::Eq,
                                v: Value::OrdinaryValue(OrdinaryValue::String(String::from(
                                    "Semion",
                                ))),
                            })),
                            NQToken::Link(Link::And),
                            NQToken::Unit(Unit::Stmt(KeyValue {
                                k: String::from("surname"),
                                op: ComparasionOperator::Eq,
                                v: Value::OrdinaryValue(OrdinaryValue::String(String::from(
                                    "Voevoda",
                                ))),
                            })),
                        ])),
                        NQToken::Link(Link::Or),
                        NQToken::Unit(Unit::Grp(vec![
                            NQToken::Unit(Unit::Stmt(KeyValue {
                                k: String::from("name"),
                                op: ComparasionOperator::Eq,
                                v: Value::OrdinaryValue(OrdinaryValue::String(String::from(
                                    "Simon",
                                ))),
                            })),
                            NQToken::Link(Link::And),
                            NQToken::Unit(Unit::Stmt(KeyValue {
                                k: String::from("surname"),
                                op: ComparasionOperator::Eq,
                                v: Value::OrdinaryValue(OrdinaryValue::String(String::from(
                                    "Vogue",
                                ))),
                            })),
                        ])),
                    ])),
                    NQToken::Link(Link::And),
                    NQToken::Unit(Unit::Stmt(KeyValue {
                        k: String::from("favourite_colors"),
                        op: ComparasionOperator::Eq,
                        v: Value::Collection(Collection::AndColl(vec![
                            OrdinaryValue::String(String::from("Black")),
                            OrdinaryValue::String(String::from("Green")),
                            OrdinaryValue::String(String::from("Blue")),
                            OrdinaryValue::String(String::from("Purple")),
                            OrdinaryValue::String(String::from("Red")),
                            OrdinaryValue::String(String::from("White")),
                        ])),
                    })),
                ],
                None,
            ),
        ]
        .into_iter();

        let expected_results = vec![
            "key_1:[-1 TO 1} AND  OR  AND ",
            "((name:'Semion' AND surname:'Voevoda') OR (name:'Simon' AND surname:'Vogue')) AND (favourite_colors:'Black' AND favourite_colors:'Green' AND favourite_colors:'Blue' AND favourite_colors:'Purple' AND favourite_colors:'Red' AND favourite_colors:'White')",
        ].into_iter();

        assert_eq!(
        input_data.len(),
        expected_results.len(),
        "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
    );

        for (input, expected) in input_data.zip(expected_results) {
            assert_eq!(convert(&input.0, input.1), expected);
        }
    }
}
