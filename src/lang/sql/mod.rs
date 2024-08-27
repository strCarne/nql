use crate::grammar::{KeyValue, NQLang, NQToken, Unit};

mod ext;

mod conv;

pub fn convert(tokens: &NQLang, extensions: &Vec<KeyValue>) -> String {
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

    #[test]
    #[ignore = "not implemented"]
    fn convert_test() {
        todo!("Implement test");
    }
}
