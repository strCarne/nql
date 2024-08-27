use crate::{combinators, Parser, ParsingResult};

use super::{extension, link, unit, KeyValue, Link, Unit};

pub type NQLang = Vec<NQToken>;

#[derive(Debug, PartialEq)]
pub enum NQToken {
    Unit(Unit),
    Link(Link),
    Extension(KeyValue),
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

    loop {
        match extension(input) {
            Ok((next_input, nq_token)) => {
                nq_lang.push(NQToken::Extension(nq_token));
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
    use super::*;
    use crate::{
        basic_types::{Date, Number},
        grammar::{
            nqlang,
            value::{Collection, OrdinaryValue, Range, RangeBounds, RangeOp, Value},
            ComparasionOperator, KeyValue, Link, Unit,
        },
    };
    use pretty_assertions::assert_eq;

    #[test]
    fn nqlang_test() {
        let input_data = vec![
            r#"
                ((name = Semion & surname = Voevoda) | (name = Simon & surname = Vogue)) &
                acceptable_nicknames = [MEGATRON_2004, Simon, 'Повелитель Вселенной'] &

                born_in_range_of = 1:12:2004==31:12:2004 &
                age=19 &

                habit = 'growing money tree' &

                was_in = {
                    Belarus,
                    Poland,
                    Lithuania,
                    Ukraine,
                    'Czech Republic',
                    Germany,
                    Italia,
                    Austria
                } &

                is_cringe = False &

                number_of_friends > 10

                $example_created_at=21:08:2024
                $unique=true
                $iq=140

                $table_name=famous_persons
                $limit = 1
                $offset = 0
            "#,
            "",
            "((name = Semion & surname = Voevoda) | (name = Simon & surname = Vogue)) & incorrect_token"
        ]
        .into_iter();

        let expected_results = vec![
            Ok((
                "",
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
                        k: String::from("acceptable_nicknames"),
                        op: ComparasionOperator::Eq,
                        v: Value::Collection(Collection::OrColl(vec![
                            OrdinaryValue::String(String::from("MEGATRON_2004")),
                            OrdinaryValue::String(String::from("Simon")),
                            OrdinaryValue::String(String::from("Повелитель Вселенной")),
                        ])),
                    })),
                    NQToken::Link(Link::And),
                    NQToken::Unit(Unit::Stmt(KeyValue {
                        k: String::from("born_in_range_of"),
                        op: ComparasionOperator::Eq,
                        v: Value::Range(Range {
                            bounds: RangeBounds::DateRange(
                                Date {
                                    day: 1,
                                    month: 12,
                                    year: 2004,
                                },
                                Date {
                                    day: 31,
                                    month: 12,
                                    year: 2004,
                                },
                            ),
                            op: RangeOp::II,
                        }),
                    })),
                    NQToken::Link(Link::And),
                    NQToken::Unit(Unit::Stmt(KeyValue {
                        k: String::from("age"),
                        op: ComparasionOperator::Eq,
                        v: Value::OrdinaryValue(OrdinaryValue::Number(Number::Integer(19))),
                    })),
                    NQToken::Link(Link::And),
                    NQToken::Unit(Unit::Stmt(KeyValue {
                        k: String::from("habit"),
                        op: ComparasionOperator::Eq,
                        v: Value::OrdinaryValue(OrdinaryValue::String(String::from(
                            "growing money tree",
                        ))),
                    })),
                    NQToken::Link(Link::And),
                    NQToken::Unit(Unit::Stmt(KeyValue {
                        k: String::from("was_in"),
                        op: ComparasionOperator::Eq,
                        v: Value::Collection(Collection::AndColl(vec![
                            OrdinaryValue::String(String::from("Belarus")),
                            OrdinaryValue::String(String::from("Poland")),
                            OrdinaryValue::String(String::from("Lithuania")),
                            OrdinaryValue::String(String::from("Ukraine")),
                            OrdinaryValue::String(String::from("Czech Republic")),
                            OrdinaryValue::String(String::from("Germany")),
                            OrdinaryValue::String(String::from("Italia")),
                            OrdinaryValue::String(String::from("Austria")),
                        ])),
                    })),
                    NQToken::Link(Link::And),
                    NQToken::Unit(Unit::Stmt(KeyValue {
                        k: String::from("is_cringe"),
                        op: ComparasionOperator::Eq,
                        v: Value::OrdinaryValue(OrdinaryValue::Boolean(false)),
                    })),
                    NQToken::Link(Link::And),
                    NQToken::Unit(Unit::Stmt(KeyValue {
                        k: String::from("number_of_friends"),
                        op: ComparasionOperator::Greater,
                        v: Value::OrdinaryValue(OrdinaryValue::Number(Number::Integer(10))),
                    })),
                    NQToken::Extension(KeyValue {
                        k: String::from("example_created_at"),
                        op: ComparasionOperator::Eq,
                        v: Value::OrdinaryValue(OrdinaryValue::Date(Date {
                            day: 21,
                            month: 8,
                            year: 2024,
                        })),
                    }),
                    NQToken::Extension(KeyValue {
                        k: String::from("unique"),
                        op: ComparasionOperator::Eq,
                        v: Value::OrdinaryValue(OrdinaryValue::Boolean(true)),
                    }),
                    NQToken::Extension(KeyValue {
                        k: String::from("iq"),
                        op: ComparasionOperator::Eq,
                        v: Value::OrdinaryValue(OrdinaryValue::Number(Number::Integer(140))),
                    }),
                    NQToken::Extension(KeyValue {
                        k: String::from("table_name"),
                        op: ComparasionOperator::Eq,
                        v: Value::OrdinaryValue(OrdinaryValue::String(String::from(
                            "famous_persons",
                        ))),
                    }),
                    NQToken::Extension(KeyValue {
                        k: String::from("limit"),
                        op: ComparasionOperator::Eq,
                        v: Value::OrdinaryValue(OrdinaryValue::Number(Number::Integer(1))),
                    }),
                    NQToken::Extension(KeyValue {
                        k: String::from("offset"),
                        op: ComparasionOperator::Eq,
                        v: Value::OrdinaryValue(OrdinaryValue::Number(Number::Integer(0))),
                    }),
                ],
            )),
            Err(""),
            Ok((
                "& incorrect_token",
                vec![NQToken::Unit(Unit::Grp(vec![
                    NQToken::Unit(Unit::Grp(vec![
                        NQToken::Unit(Unit::Stmt(KeyValue {
                            k: String::from("name"),
                            op: ComparasionOperator::Eq,
                            v: Value::OrdinaryValue(OrdinaryValue::String(String::from("Semion"))),
                        })),
                        NQToken::Link(Link::And),
                        NQToken::Unit(Unit::Stmt(KeyValue {
                            k: String::from("surname"),
                            op: ComparasionOperator::Eq,
                            v: Value::OrdinaryValue(OrdinaryValue::String(String::from("Voevoda"))),
                        })),
                    ])),
                    NQToken::Link(Link::Or),
                    NQToken::Unit(Unit::Grp(vec![
                        NQToken::Unit(Unit::Stmt(KeyValue {
                            k: String::from("name"),
                            op: ComparasionOperator::Eq,
                            v: Value::OrdinaryValue(OrdinaryValue::String(String::from("Simon"))),
                        })),
                        NQToken::Link(Link::And),
                        NQToken::Unit(Unit::Stmt(KeyValue {
                            k: String::from("surname"),
                            op: ComparasionOperator::Eq,
                            v: Value::OrdinaryValue(OrdinaryValue::String(String::from("Vogue"))),
                        })),
                    ])),
                ]))],
            )),
        ]
        .into_iter();

        assert_eq!(
            input_data.len(),
            expected_results.len(),
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        for (input, expected) in input_data.zip(expected_results) {
            assert_eq!(nqlang(input), expected);
        }
    }
}
