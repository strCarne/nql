use crate::{
    basic_types::Number,
    grammar::{
        value::{OrdinaryValue, Value},
        KeyValue,
    },
};

pub fn table(extensions: &Vec<KeyValue>) -> String {
    for ext in extensions {
        if ext.k.to_ascii_lowercase() == "table" {
            if let Value::OrdinaryValue(OrdinaryValue::String(name)) = &ext.v {
                return name.clone();
            }
        }
    }

    panic!("Error: unknown SQL table name. Please add $table='your_table_name' to your query");
}

pub fn limit(extensions: &Vec<KeyValue>) -> Option<i64> {
    for ext in extensions {
        if ext.k.to_ascii_lowercase() == "limit" {
            if let Value::OrdinaryValue(OrdinaryValue::Number(Number::Integer(limit))) = ext.v {
                return Some(limit);
            }
        }
    }

    None
}

pub fn offset(extensions: &Vec<KeyValue>) -> Option<i64> {
    for ext in extensions {
        if ext.k.to_ascii_lowercase() == "offset" {
            if let Value::OrdinaryValue(OrdinaryValue::Number(Number::Integer(offset))) = ext.v {
                return Some(offset);
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {

    use std::panic;

    use super::*;
    use crate::grammar::ComparasionOperator;
    use pretty_assertions::assert_eq;

    #[test]
    fn table_test() {
        let input_data = vec![
            vec![
                KeyValue {
                    k: String::from("boost"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::Number(Number::Float(1.5))),
                },
                KeyValue {
                    k: String::from("table"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::String(String::from("famous_persons"))),
                },
                KeyValue {
                    k: String::from("limit"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::Number(Number::Integer(10))),
                },
            ],
            vec![
                KeyValue {
                    k: String::from("table"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::String(String::from("famous_persons"))),
                },
                KeyValue {
                    k: String::from("boost"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::Number(Number::Float(1.5))),
                },
                KeyValue {
                    k: String::from("limit"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::Number(Number::Integer(10))),
                },
            ],
            vec![
                KeyValue {
                    k: String::from("boost"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::Number(Number::Float(1.5))),
                },
                KeyValue {
                    k: String::from("limit"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::Number(Number::Integer(10))),
                },
                KeyValue {
                    k: String::from("table"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::String(String::from("famous_persons"))),
                },
            ],
            vec![
                KeyValue {
                    k: String::from("boost"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::Number(Number::Float(1.5))),
                },
                KeyValue {
                    k: String::from("limit"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::Number(Number::Integer(10))),
                },
                KeyValue {
                    k: String::from("table_name"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::String(String::from("famous_persons"))),
                },
            ],
        ]
        .into_iter();

        let expected_results = vec![
            (false, "famous_persons".to_string()),
            (false, "famous_persons".to_string()),
            (false, "famous_persons".to_string()),
            (true, "".to_string()),
        ]
        .into_iter();

        assert_eq!(
            input_data.len(),
            expected_results.len(),
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        for (input, expected) in input_data.zip(expected_results) {
            let result = match panic::catch_unwind(move || table(&input)) {
                Ok(result) => (false, result),
                Err(_) => (true, String::new()),
            };
            assert_eq!(result, expected);
        }
    }

    #[test]
    fn limit_test() {
        let input_data = vec![
            vec![
                KeyValue {
                    k: String::from("boost"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::Number(Number::Float(1.5))),
                },
                KeyValue {
                    k: String::from("table"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::String(String::from("famous_persons"))),
                },
                KeyValue {
                    k: String::from("limit"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::Number(Number::Integer(10))),
                },
            ],
            vec![
                KeyValue {
                    k: String::from("limit"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::Number(Number::Integer(10))),
                },
                KeyValue {
                    k: String::from("table"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::String(String::from("famous_persons"))),
                },
                KeyValue {
                    k: String::from("boost"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::Number(Number::Float(1.5))),
                },
            ],
            vec![
                KeyValue {
                    k: String::from("boost"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::Number(Number::Float(1.5))),
                },
                KeyValue {
                    k: String::from("limit"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::Number(Number::Integer(10))),
                },
                KeyValue {
                    k: String::from("table"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::String(String::from("famous_persons"))),
                },
            ],
            vec![
                KeyValue {
                    k: String::from("boost"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::Number(Number::Float(1.5))),
                },
                KeyValue {
                    k: String::from("limits"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::Number(Number::Integer(10))),
                },
                KeyValue {
                    k: String::from("table_name"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::String(String::from("famous_persons"))),
                },
            ],
        ]
        .into_iter();

        let expected_results = vec![Some(10), Some(10), Some(10), None].into_iter();

        assert_eq!(
            input_data.len(),
            expected_results.len(),
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        for (input, expected) in input_data.zip(expected_results) {
            assert_eq!(limit(&input), expected);
        }
    }

    #[test]
    fn offset_test() {
        let input_data = vec![
            vec![
                KeyValue {
                    k: String::from("boost"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::Number(Number::Float(1.5))),
                },
                KeyValue {
                    k: String::from("table"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::String(String::from("famous_persons"))),
                },
                KeyValue {
                    k: String::from("offset"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::Number(Number::Integer(10))),
                },
            ],
            vec![
                KeyValue {
                    k: String::from("offset"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::Number(Number::Integer(10))),
                },
                KeyValue {
                    k: String::from("table"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::String(String::from("famous_persons"))),
                },
                KeyValue {
                    k: String::from("boost"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::Number(Number::Float(1.5))),
                },
            ],
            vec![
                KeyValue {
                    k: String::from("boost"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::Number(Number::Float(1.5))),
                },
                KeyValue {
                    k: String::from("offset"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::Number(Number::Integer(10))),
                },
                KeyValue {
                    k: String::from("table"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::String(String::from("famous_persons"))),
                },
            ],
            vec![
                KeyValue {
                    k: String::from("boost"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::Number(Number::Float(1.5))),
                },
                KeyValue {
                    k: String::from("offsets"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::Number(Number::Integer(10))),
                },
                KeyValue {
                    k: String::from("table_name"),
                    op: ComparasionOperator::Eq,
                    v: Value::OrdinaryValue(OrdinaryValue::String(String::from("famous_persons"))),
                },
            ],
        ]
        .into_iter();

        let expected_results = vec![Some(10), Some(10), Some(10), None].into_iter();

        assert_eq!(
            input_data.len(),
            expected_results.len(),
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        for (input, expected) in input_data.zip(expected_results) {
            assert_eq!(offset(&input), expected);
        }
    }
}
