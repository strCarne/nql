use crate::{
    combinators,
    grammar::{self, Link, Statement},
    primitives, Parser, ParsingResult,
};

#[derive(Debug)]
pub enum Token {
    Stmt(Statement),
    Link(Link),
    OpenBrace,
    CloseBrace,
}

pub fn tokenize(mut input: &str) -> ParsingResult<Vec<Token>> {
    let mut tokens = Vec::new();

    loop {
        match token(input) {
            Ok((next_input, token)) => {
                tokens.push(token);
                input = next_input;
            }
            Err(err) if err.is_empty() => return Ok((err, tokens)),
            Err(err) => return Err(err),
        }
    }
}

pub fn token(input: &str) -> ParsingResult<Token> {
    combinators::single_of(vec![
        grammar::statement.map(|stmt| Token::Stmt(stmt)),
        grammar::link.map(|link| Token::Link(link)),
        primitives::character('(').map(|_| Token::OpenBrace),
        primitives::character(')').map(|_| Token::CloseBrace),
    ])
    .whitespace_wrap()
    .parse(input)
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore = "Not implemented yet"]
    fn tokenize_test() {
        todo!("Make unit test")
    }

    #[test]
    #[ignore = "Not implemented yet"]
    fn token_test() {
        todo!("Make unit test")
    }
}
