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
