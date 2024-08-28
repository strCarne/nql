use crate::grammar::{KeyValue, NQLang, NQToken};

pub fn extract_extensions(mut tokens: NQLang) -> (NQLang, Vec<KeyValue>) {
    let mut extensions = Vec::new();
    while let Some(NQToken::Extension(_)) = tokens.last() {
        if let NQToken::Extension(ext) = tokens.pop().unwrap() {
            extensions.push(ext);
        }
    }
    (tokens, extensions)
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::grammar::{KeyValue, NQToken, Unit};
    use pretty_assertions::assert_eq;

    #[test]
    fn extract_extensions_test() {
        let input_data = vec![
            vec![
                NQToken::Unit(Unit::Stmt(KeyValue::default())),
                NQToken::Extension(KeyValue::default()),
                NQToken::Extension(KeyValue::default()),
            ],
            vec![
                NQToken::Extension(KeyValue::default()),
                NQToken::Extension(KeyValue::default()),
            ],
            vec![
                NQToken::Unit(Unit::Stmt(KeyValue::default())),
                NQToken::Unit(Unit::Stmt(KeyValue::default())),
            ],
            vec![],
        ]
        .into_iter();

        let expected_results = vec![(1, 2), (0, 2), (2, 0), (0, 0)].into_iter();

        assert_eq!(
            input_data.len(),
            expected_results.len(),
            "BAD TEST: number of inputs is not equal to number of results [correct the source data]"
        );

        for (input, expected) in input_data.zip(expected_results) {
            let result = {
                let tmp = extract_extensions(input);
                (tmp.0.len(), tmp.1.len())
            };
            assert_eq!(result, expected);
        }
    }
}
