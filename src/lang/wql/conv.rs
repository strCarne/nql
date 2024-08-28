use crate::grammar::{
    value::{Collection, OrdinaryValue, Range, RangeOp, Value},
    ComparasionOperator, KeyValue, Link, NQLang, NQToken, Unit,
};

pub fn statement(stmt: &KeyValue) -> String {
    let mut buf = String::new();

    let op = simple_operator(&stmt.op);

    match &stmt.v {
        Value::OrdinaryValue(val) => match stmt.op {
            ComparasionOperator::GreaterEq => {
                buf += &gen_comp_eq(&stmt.k, '>', &val.to_string());
            }
            ComparasionOperator::LessEq => {
                buf += &gen_comp_eq(&stmt.k, '<', &val.to_string());
            }
            _ => {
                buf += &format!("{}{}{}", stmt.k, op, val.to_string());
            }
        },
        Value::Range(Range { bounds, op }) => {
            let bounds = bounds.to_strings();
            buf += &match op {
                RangeOp::EE => format!("({}>{}%3B{}<{})", &stmt.k, bounds.0, &stmt.k, bounds.1),
                RangeOp::EI => format!(
                    "({}>{}%3B{})",
                    &stmt.k,
                    bounds.0,
                    gen_comp_eq(&stmt.k, '<', &bounds.1)
                ),
                RangeOp::IE => format!(
                    "({}%3B{}<{})",
                    gen_comp_eq(&stmt.k, '>', &bounds.0),
                    &stmt.k,
                    bounds.1
                ),
                RangeOp::II => format!(
                    "({}%3B{})",
                    gen_comp_eq(&stmt.k, '>', &bounds.0),
                    gen_comp_eq(&stmt.k, '<', &bounds.1)
                ),
            };
        }
        Value::Collection(coll) => match coll {
            Collection::AndColl(coll) => {
                buf += &collection_body(&stmt.k, &stmt.op, coll, &Link::And)
            }
            Collection::OrColl(coll) => buf += &collection_body(&stmt.k, &stmt.op, coll, &Link::Or),
        },
    }

    buf
}

fn collection_body(
    key: &String,
    op: &ComparasionOperator,
    coll: &Vec<OrdinaryValue>,
    l: &Link,
) -> String {
    let mut buf = String::new();
    buf.push('(');
    let mut iter = coll.iter();
    if let Some(val) = iter.next() {
        match op {
            ComparasionOperator::GreaterEq => {
                buf += &gen_comp_eq(key, '>', &val.to_string());
            }
            ComparasionOperator::LessEq => {
                buf += &gen_comp_eq(key, '<', &val.to_string());
            }
            _ => {
                buf += &format!("{}{}{}", key, simple_operator(op), val.to_string());
            }
        }
    }

    for val in iter {
        buf += link(l);
        match op {
            ComparasionOperator::GreaterEq => {
                buf += &gen_comp_eq(key, '>', &val.to_string());
            }
            ComparasionOperator::LessEq => {
                buf += &gen_comp_eq(key, '<', &val.to_string());
            }
            _ => {
                buf += &format!("{}{}{}", key, simple_operator(op), val.to_string());
            }
        }
    }

    buf.push(')');
    buf
}

fn gen_comp_eq(field: &String, comp: char, val: &String) -> String {
    format!(
        "({}{}{},{}={})",
        field,
        comp,
        val.to_string(),
        field,
        val.to_string()
    )
}

fn simple_operator(op: &ComparasionOperator) -> &'static str {
    match op {
        ComparasionOperator::Greater => ">",
        ComparasionOperator::Less => "<",
        ComparasionOperator::GreaterEq => ">=",
        ComparasionOperator::LessEq => "<=",
        ComparasionOperator::Eq => "=",
        ComparasionOperator::NotEq => "!=",
    }
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
        Link::And => "%3B",
        Link::Or => ",",
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn link_conv_test() {
        let input_data = vec![Link::And, Link::Or].into_iter();

        let expected_results = vec!["%3B", ","].into_iter();

        assert_eq!(
            input_data.len(),
            expected_results.len(),
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        for (input, expected) in input_data.zip(expected_results) {
            assert_eq!(link(&input), expected);
        }
    }
}
