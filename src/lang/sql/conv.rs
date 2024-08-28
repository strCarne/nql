use crate::grammar::{value::{Collection, Range, RangeOp, Value}, ComparasionOperator, KeyValue, Link, NQLang, NQToken, Unit};

pub fn statement(stmt: &KeyValue) -> String {
    let mut buf = String::new();

    let op = match stmt.op {
        ComparasionOperator::Eq => "=",
        ComparasionOperator::Greater => ">",
        ComparasionOperator::Less => "<",
        ComparasionOperator::GreaterEq => ">=",
        ComparasionOperator::LessEq => "<=",
        ComparasionOperator::NotEq => "!=",
    };

    match &stmt.v {
        Value::OrdinaryValue(val) => {
            buf += &format!("{} {} {}", &stmt.k, op, val.to_string());
        }
        Value::Range(Range { bounds, op }) => {
            let bounds = bounds.to_strings();
            let result = match op {
                RangeOp::EE => format!(
                    "({} < {} AND {} < {})",
                    bounds.0, &stmt.k, &stmt.k, bounds.1
                ),
                RangeOp::EI => format!(
                    "({} < {} AND {} <= {})",
                    bounds.0, &stmt.k, &stmt.k, bounds.1
                ),
                RangeOp::IE => format!(
                    "({} <= {} AND {} < {})",
                    bounds.0, &stmt.k, &stmt.k, bounds.1
                ),
                RangeOp::II => format!(
                    "({} <= {} AND {} <= {})",
                    bounds.0, &stmt.k, &stmt.k, bounds.1
                ),
            };
            buf += &result;
        }
        Value::Collection(Collection::AndColl(coll)) => {
            buf.push('(');

            let mut iter = coll.iter();
            if let Some(val) = iter.next() {
                buf.push_str(&format!("{} {} {}", stmt.k, op, val.to_string()));
            }

            for val in iter {
                buf.push_str(" AND ");
                buf.push_str(&format!("{} {} {}", stmt.k, op, val.to_string()));
            }
            buf.push(')');
        }
        Value::Collection(Collection::OrColl(coll)) => {
            buf.push('(');
            let mut iter = coll.iter();
            if let Some(val) = iter.next() {
                buf.push_str(&format!("{} {} {}", stmt.k, op, val.to_string()));
            }

            for val in iter {
                buf.push_str(" OR ");
                buf.push_str(&format!("{} {} {}", stmt.k, op, val.to_string()));
            }
            buf.push(')');
        }
    }

    buf
}

pub fn group(grp: &NQLang) -> String {
    let mut buf = String::new();
    buf.push('(');

    for token in grp {
        match token {
            NQToken::Unit(u) => match u {
                Unit::Stmt(stmt) => {
                    buf += &super::conv::statement(stmt);
                }
                Unit::Grp(grp) => {
                    buf += &super::conv::group(grp);
                }
            },
            NQToken::Link(l) => {
                buf += &super::conv::link(l);
            }
            _ => panic!("tokens must contain only NQToken::Unit and NQToken::Link"),
        }
    }

    buf.push(')');
    buf
}

pub fn link(l: &Link) -> String {
    match l {
        Link::And => String::from(" AND "),
        Link::Or => String::from(" OR "),
    }
}

#[cfg(test)]
mod tests {

    #[test]
    #[ignore = "not implemented"]
    fn statement_conv_test() {
        todo!("Implement test");
    }

    #[test]
    #[ignore = "not implemented"]
    fn group_conv_test() {
        todo!("Implement test");
    }

    #[test]
    #[ignore = "not implemented"]
    fn link_conv_test() {
        todo!("Implement test");
    }
}