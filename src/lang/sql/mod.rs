use crate::grammar::{KeyValue, NQLang, NQToken, Unit};

mod ext;

mod conv;

pub fn convert(tokens: &NQLang, extensions: Option<&Vec<KeyValue>>) -> String {
    let extensions = extensions.expect("sql::convert has must-have 'table' extension");

    let table_name = ext::table(extensions);

    let limit = ext::limit(extensions);

    let offset = ext::offset(extensions);

    let mut where_close = String::new();

    for token in tokens {
        match token {
            NQToken::Unit(u) => match u {
                Unit::Stmt(stmt) => {
                    where_close += &conv::statement(stmt);
                }
                Unit::Grp(grp) => {
                    where_close += &conv::group(grp);
                }
            },
            NQToken::Link(l) => {
                where_close += &conv::link(l);
            }
            _ => panic!("tokens must contain only NQToken::Unit and NQToken::Link"),
        }
    }

    format!(
        "SELECT * FROM {}{}{}{};",
        table_name,
        if !where_close.is_empty() {
            format!(" WHERE {}", where_close)
        } else {
            "".to_string()
        },
        if let Some(limit) = limit {
            format!(" LIMIT {}", limit)
        } else {
            "".to_string()
        },
        if let Some(offset) = offset {
            format!(" OFFSET {}", offset)
        } else {
            "".to_string()
        }
    )
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
        let table_ext = vec![KeyValue {
            k: String::from("table"),
            op: ComparasionOperator::Eq,
            v: Value::OrdinaryValue(OrdinaryValue::String(String::from("tests"))),
        }];

        let input_data = vec![
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
                Some(&table_ext),
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
                Some(&table_ext),
            ),
        ]
        .into_iter();

        let expected_results = vec![
            "SELECT * FROM tests WHERE (key_1 >= -1 AND key_1 < 1) AND  OR  AND ;",
            "SELECT * FROM tests WHERE ((name = 'Semion' AND surname = 'Voevoda') OR (name = 'Simon' AND surname = 'Vogue')) AND (favourite_colors = 'Black' AND favourite_colors = 'Green' AND favourite_colors = 'Blue' AND favourite_colors = 'Purple' AND favourite_colors = 'Red' AND favourite_colors = 'White');",
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
