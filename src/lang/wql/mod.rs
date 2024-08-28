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

    #[test]
    #[ignore = "not implemented"]
    fn convert_test() {
        todo!("Implement test");
    }
}
