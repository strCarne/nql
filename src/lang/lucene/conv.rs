use crate::grammar::{
    value::{Collection, OrdinaryValue, Range, RangeOp, Value},
    ComparasionOperator, KeyValue, Link, NQLang, NQToken, Unit,
};

pub fn statement(stmt: &KeyValue) -> String {
    let mut buf = String::new();

    let op = match stmt.op {
        ComparasionOperator::Eq => ":",
        ComparasionOperator::Greater => ":>",
        ComparasionOperator::Less => ":<",
        ComparasionOperator::GreaterEq => ":>=",
        ComparasionOperator::LessEq => ":<=",
        ComparasionOperator::NotEq => ":",
    };

    match &stmt.v {
        Value::OrdinaryValue(val) => {
            if let ComparasionOperator::NotEq = stmt.op {
                buf += "NOT ";
            }
            buf += &format!("{}{}{}", &stmt.k, op, val.to_string());
        }
        Value::Range(Range { bounds, op }) => {
            buf += &stmt.k;
            buf.push(':');
            let bounds = bounds.to_strings();
            let result = match op {
                RangeOp::EE => format!("{{{} TO {}}}", bounds.0, bounds.1),
                RangeOp::EI => format!("{{{} TO {}]", bounds.0, bounds.1),
                RangeOp::IE => format!("[{} TO {}}}", bounds.0, bounds.1),
                RangeOp::II => format!("[{} TO {}]", bounds.0, bounds.1),
            };
            buf.push_str(&result);
        }
        Value::Collection(Collection::AndColl(coll)) => {
            buf += &collection_body(&stmt.k, &op, coll, &Link::And);
        }
        Value::Collection(Collection::OrColl(coll)) => {
            buf += &collection_body(&stmt.k, &op, coll, &Link::Or);
        }
    }

    buf
}

fn collection_body(key: &str, op: &str, coll: &Vec<OrdinaryValue>, l: &Link) -> String {
    let mut buf = String::new();
    buf.push('(');

    let mut iter = coll.iter();
    if let Some(val) = iter.next() {
        buf.push_str(&format!("{}{}{}", key, op, val.to_string()));
    }

    for val in iter {
        buf.push_str(&link(l));
        buf.push_str(&format!("{}{}{}", key, op, val.to_string()));
    }

    buf.push(')');
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

pub fn link(l: &Link) -> &'static str {
    match l {
        Link::And => " AND ",
        Link::Or => " OR ",
    }
}
