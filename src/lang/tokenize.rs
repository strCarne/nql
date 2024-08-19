use crate::{combinators, grammar, primitives, Parser, ParsingResult};

use super::node::Node;

pub fn tokenize(mut input: &str) -> ParsingResult<Vec<Node>> {
    let whitespaces = || {
        |input| combinators::zero_or_more(primitives::any.pred(|c| c.is_whitespace())).parse(input)
    };

    let mut nodes = Vec::new();

    if let Ok((next_input, Some(head))) =
        combinators::zero_or_one(grammar::statement.wrap(whitespaces())).parse(input)
    {
        nodes.push(Node::new(head));
        input = next_input;
    } else {
        return Ok((input, nodes));
    }

    let parser = grammar::link.and_then(|l| {
        grammar::statement
            .wrap(|input| {
                combinators::zero_or_more(primitives::any.pred(|c| c.is_whitespace())).parse(input)
            })
            .map(move |stmt| (l, stmt))
    });

    loop {
        match parser.parse(input) {
            Ok((next_input, (link, stmt))) => {
                nodes.last_mut().unwrap().link_to_next = Some(link);
                nodes.push(Node::new(stmt));
                input = next_input;
            }
            Err(left_input) if !left_input.is_empty() => return Err(left_input),
            _ => break,
        }
    }

    Ok((input, nodes))
}
