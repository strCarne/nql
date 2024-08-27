use crate::grammar::NQToken;

pub fn to_sql(_tokens: &Vec<NQToken>) -> String {
    todo!("not implemented yet");
    // let table_name = 'a: {
    //     for token in tokens {
    //         match token {
    //             Token::Stmt(Statement::Extension(ext))
    //                 if ext.k.to_ascii_lowercase() == "table_name" =>
    //             {
    //                 if let Value::OrdinaryValue(OrdinaryValue::String(name)) = &ext.v {
    //                     break 'a name.clone();
    //                 }
    //             }
    //             _ => continue,
    //         }
    //     }

    //     panic!(
    //         "Error: unknown SQL table name. Please add $table_name='your_table_name' to your query"
    //     );
    // };

    // let limit = 'a: {
    //     for token in tokens {
    //         match token {
    //             Token::Stmt(Statement::Extension(ext)) if ext.k.to_ascii_lowercase() == "limit" => {
    //                 if let Value::OrdinaryValue(OrdinaryValue::Number(Number::Integer(limit))) =
    //                     ext.v
    //                 {
    //                     break 'a Some(limit);
    //                 }
    //             }
    //             _ => continue,
    //         }
    //     }

    //     None
    // };

    // let offset = 'a: {
    //     for token in tokens {
    //         match token {
    //             Token::Stmt(Statement::Extension(ext))
    //                 if ext.k.to_ascii_lowercase() == "offset" =>
    //             {
    //                 if let Value::OrdinaryValue(OrdinaryValue::Number(Number::Integer(offset))) =
    //                     ext.v
    //                 {
    //                     break 'a Some(offset);
    //                 }
    //             }
    //             _ => continue,
    //         }
    //     }
    //     None
    // };

    // let where_close = {
    //     let mut buf = String::new();
    //     let mut prev_is_stmt = false;
    //     let mut prev_is_link = false;

    //     for token in tokens {
    //         match token {
    //             Token::Stmt(Statement::Field(field)) => {
    //                 if prev_is_stmt && !prev_is_link {
    //                     buf.push_str(" AND ");
    //                 }
    //                 prev_is_stmt = true;
    //                 prev_is_link = false;

    //                 let op = match field.op {
    //                     ComparasionOperator::Eq => "=",
    //                     ComparasionOperator::Greater => ">",
    //                     ComparasionOperator::Less => "<",
    //                     ComparasionOperator::GreaterOrEq => ">=",
    //                     ComparasionOperator::LessOrEq => "<=",
    //                     ComparasionOperator::NotEq => "!=",
    //                 };

    //                 match &field.v {
    //                     Value::OrdinaryValue(val) => {
    //                         buf.push_str(&format!("{} {} {}", field.k, op, val.to_string()));
    //                         buf.push(' ');
    //                     }
    //                     Value::Range(Range { bounds, op }) => {
    //                         let bounds = bounds.to_strings();
    //                         let result = match op {
    //                             RangeOp::EE => format!(
    //                                 "{} < {} AND {} < {}",
    //                                 bounds.0, &field.k, &field.k, bounds.1
    //                             ),
    //                             RangeOp::EI => format!(
    //                                 "{} < {} AND {} <= {}",
    //                                 bounds.0, &field.k, &field.k, bounds.1
    //                             ),
    //                             RangeOp::IE => format!(
    //                                 "{} <= {} AND {} < {}",
    //                                 bounds.0, &field.k, &field.k, bounds.1
    //                             ),
    //                             RangeOp::II => format!(
    //                                 "{} <= {} AND {} <= {}",
    //                                 bounds.0, &field.k, &field.k, bounds.1
    //                             ),
    //                         };
    //                         buf.push_str(&result);
    //                         buf.push(' ');
    //                     }
    //                     Value::Collection(Collection::AndColl(coll)) => {
    //                         buf.push('(');

    //                         let mut iter = coll.iter();
    //                         if let Some(val) = iter.next() {
    //                             buf.push_str(&format!("{} {} {}", field.k, op, val.to_string()));
    //                         }

    //                         for val in iter {
    //                             buf.push_str(" AND ");
    //                             buf.push_str(&format!("{} {} {}", field.k, op, val.to_string()));
    //                         }
    //                         buf.push(')');
    //                         buf.push(' ');
    //                         prev_is_stmt = true;
    //                         prev_is_link = false;
    //                     }
    //                     Value::Collection(Collection::OrColl(coll)) => {
    //                         buf.push('(');
    //                         let mut iter = coll.iter();
    //                         if let Some(val) = iter.next() {
    //                             buf.push_str(&format!("{} {} {}", field.k, op, val.to_string()));
    //                         }

    //                         for val in iter {
    //                             buf.push_str(" OR ");
    //                             buf.push_str(&format!("{} {} {}", field.k, op, val.to_string()));
    //                         }
    //                         buf.push(')');
    //                         buf.push(' ');
    //                         prev_is_stmt = true;
    //                         prev_is_link = false;
    //                     }
    //                 }
    //             }
    //             Token::OpenBrace => {
    //                 if prev_is_stmt && !prev_is_link {
    //                     buf.push_str("AND ");
    //                 }
    //                 prev_is_stmt = false;
    //                 prev_is_link = false;
    //                 buf.push('(');
    //             }
    //             Token::CloseBrace => {
    //                 prev_is_stmt = true;
    //                 prev_is_link = false;
    //                 buf.push(')');
    //             }
    //             Token::Link(link) => {
    //                 prev_is_stmt = false;
    //                 prev_is_link = true;
    //                 buf.push_str(match link {
    //                     Link::And => " AND ",
    //                     Link::Or => " OR ",
    //                 })
    //             }
    //             _ => continue,
    //         }
    //     }

    //     buf
    // };

    // format!(
    //     "SELECT * FROM {}{}{}{};",
    //     table_name,
    //     if !where_close.is_empty() {
    //         format!(" WHERE {}", where_close)
    //     } else {
    //         "".to_string()
    //     },
    //     if let Some(limit) = limit {
    //         format!(" LIMIT {}", limit)
    //     } else {
    //         "".to_string()
    //     },
    //     if let Some(offset) = offset {
    //         format!(" OFFSET {}", offset)
    //     } else {
    //         "".to_string()
    //     }
    // )
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore = "not implemented yet"]
    fn to_sql_test() {
        todo!("Make unit test")
    }
}
