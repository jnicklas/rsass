use css::{Item, Value};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AtRule {
    name: String,
    args: Value,
    body: Option<Vec<Item>>,
}

impl AtRule {
    pub fn new(name: String, args: Value, body: Option<Vec<Item>>) -> Self {
        AtRule { name: name, args: args, body: body }
    }
}
