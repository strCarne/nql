use crate::{
    grammar::{nqlang, NQLang},
    ParsingResult,
};

pub fn tokenize(input: &str) -> ParsingResult<NQLang> {
    let result = nqlang(input)?;
    if result.0.is_empty() {
        Ok(result)
    } else {
        Err(result.0)
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        basic_types::{Date, Number},
        grammar::{
            value::{Collection, OrdinaryValue, Range, RangeBounds, RangeOp, Value},
            ComparasionOperator, KeyValue, Link, NQToken, Unit,
        },
    };

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn tokenize_test() {
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

                incorrect_token_at_the_end
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
            Err(r#"incorrect_token_at_the_end
            "#),
            Err(""),
            Err("& incorrect_token"),
        ]
        .into_iter();

        assert_eq!(
            input_data.len(),
            expected_results.len(),
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        for (input, expected) in input_data.zip(expected_results) {
            assert_eq!(tokenize(input), expected);
        }
    }
}
