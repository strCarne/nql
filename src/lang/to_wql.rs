use crate::grammar::{
    value::{Collection, Range, RangeOp, Value},
    ComparasionOperator, Link, Statement,
};

use super::Token;

pub fn to_wql(tokens: &Vec<Token>) -> String {
    let mut buf = String::new();
    let mut prev_is_stmt = false;
    let mut prev_is_link = false;

    for token in tokens {
        match token {
            Token::Stmt(Statement::Field(field)) => {
                if prev_is_stmt && !prev_is_link {
                    buf.push_str(" AND ");
                }
                prev_is_stmt = true;
                prev_is_link = false;

                let op = match field.op {
                    ComparasionOperator::Eq => "=",
                    ComparasionOperator::Greater => ">",
                    ComparasionOperator::Less => "<",
                    ComparasionOperator::GreaterOrEq => ">=",
                    ComparasionOperator::LessOrEq => "<=",
                    ComparasionOperator::NotEq => "!=",
                };

                let gen_comp_eq = |field: &String, comp: char, val: &String| {
                    format!(
                        "({} {} {} OR {} = {})",
                        field,
                        comp,
                        val.to_string(),
                        field,
                        val.to_string()
                    )
                };

                match &field.v {
                    Value::OrdinaryValue(val) => {
                        match field.op {
                            ComparasionOperator::GreaterOrEq => {
                                buf.push_str(&gen_comp_eq(&field.k, '>', &val.to_string()));
                            }
                            ComparasionOperator::LessOrEq => {
                                buf.push_str(&gen_comp_eq(&field.k, '<', &val.to_string()));
                            }
                            _ => {
                                buf.push_str(&format!("{} {} {}", field.k, op, val.to_string()));
                            }
                        }
                        buf.push(' ');
                    }
                    Value::Range(Range { bounds, op }) => {
                        let bounds = bounds.to_strings();
                        let result = match op {
                            RangeOp::EE => format!(
                                "({} < {} AND {} < {})",
                                bounds.0, &field.k, &field.k, bounds.1
                            ),
                            RangeOp::EI => format!(
                                "({} < {} AND {})",
                                bounds.0,
                                &field.k,
                                gen_comp_eq(&field.k, '<', &bounds.1)
                            ),
                            RangeOp::IE => format!(
                                "({} AND {} < {})",
                                gen_comp_eq(&bounds.0, '<', &field.k),
                                &field.k,
                                bounds.1
                            ),
                            RangeOp::II => format!(
                                "({} AND {})",
                                gen_comp_eq(&bounds.0, '<', &field.k),
                                gen_comp_eq(&field.k, '<', &bounds.1)
                            ),
                        };
                        buf.push_str(&result);
                        buf.push(' ');
                    }
                    Value::Collection(Collection::AndColl(coll)) => {
                        buf.push('(');

                        let mut iter = coll.iter();
                        if let Some(val) = iter.next() {
                            match field.op {
                                ComparasionOperator::GreaterOrEq => {
                                    buf.push_str(&gen_comp_eq(&field.k, '>', &val.to_string()));
                                }
                                ComparasionOperator::LessOrEq => {
                                    buf.push_str(&gen_comp_eq(&field.k, '<', &val.to_string()));
                                }
                                _ => {
                                    buf.push_str(&format!(
                                        "{} {} {}",
                                        field.k,
                                        op,
                                        val.to_string()
                                    ));
                                }
                            }
                        }

                        for val in iter {
                            buf.push_str(" AND ");
                            match field.op {
                                ComparasionOperator::GreaterOrEq => {
                                    buf.push_str(&gen_comp_eq(&field.k, '>', &val.to_string()));
                                }
                                ComparasionOperator::LessOrEq => {
                                    buf.push_str(&gen_comp_eq(&field.k, '<', &val.to_string()));
                                }
                                _ => {
                                    buf.push_str(&format!(
                                        "{} {} {}",
                                        field.k,
                                        op,
                                        val.to_string()
                                    ));
                                }
                            }
                        }
                        buf.push(')');
                        buf.push(' ');
                        prev_is_stmt = true;
                        prev_is_link = false;
                    }
                    Value::Collection(Collection::OrColl(coll)) => {
                        buf.push('(');
                        let mut iter = coll.iter();
                        if let Some(val) = iter.next() {
                            match field.op {
                                ComparasionOperator::GreaterOrEq => {
                                    buf.push_str(&gen_comp_eq(&field.k, '>', &val.to_string()));
                                }
                                ComparasionOperator::LessOrEq => {
                                    buf.push_str(&gen_comp_eq(&field.k, '<', &val.to_string()));
                                }
                                _ => {
                                    buf.push_str(&format!(
                                        "{} {} {}",
                                        field.k,
                                        op,
                                        val.to_string()
                                    ));
                                }
                            }
                        }

                        for val in iter {
                            buf.push_str(" OR ");
                            match field.op {
                                ComparasionOperator::GreaterOrEq => {
                                    buf.push_str(&gen_comp_eq(&field.k, '>', &val.to_string()));
                                }
                                ComparasionOperator::LessOrEq => {
                                    buf.push_str(&gen_comp_eq(&field.k, '<', &val.to_string()));
                                }
                                _ => {
                                    buf.push_str(&format!(
                                        "{} {} {}",
                                        field.k,
                                        op,
                                        val.to_string()
                                    ));
                                }
                            }
                        }
                        buf.push(')');
                        buf.push(' ');
                        prev_is_stmt = true;
                        prev_is_link = false;
                    }
                }
            }
            Token::OpenBrace => {
                if prev_is_stmt && !prev_is_link {
                    buf.push_str("AND ");
                }
                prev_is_stmt = false;
                prev_is_link = false;
                buf.push('(');
            }
            Token::CloseBrace => {
                prev_is_stmt = true;
                prev_is_link = false;
                buf.push(')');
            }
            Token::Link(link) => {
                prev_is_stmt = false;
                prev_is_link = true;
                buf.push_str(match link {
                    Link::And => " AND ",
                    Link::Or => " OR ",
                })
            }
            _ => continue,
        }
    }

    buf
}
