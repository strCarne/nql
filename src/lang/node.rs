use crate::grammar::{Link, Statement};

#[derive(Debug)]
pub struct Node {
    pub data: Statement,
    pub link_to_next: Option<Link>,
}

impl Node {
    pub fn new(data: Statement) -> Self {
        Self {
            data,
            link_to_next: None,
        }
    }
}
