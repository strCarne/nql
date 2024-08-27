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

    panic!("Error: unknown SQL table name. Please add $table_name='your_table_name' to your query");
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
