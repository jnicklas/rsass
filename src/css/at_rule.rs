use css::{Item, Value};
use std::ascii::AsciiExt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct AtRule {
    pub name: String,
    pub args: Value,
    pub body: Option<Vec<Item>>,
}

impl AtRule {
    pub fn new(name: String, args: Value, body: Option<Vec<Item>>) -> Self {
        AtRule { name: name, args: args, body: body }
    }

    pub fn is_ascii(&self) -> bool {
        if let Some(ref body) = self.body {
            if !body.iter().all(|v| v.is_ascii()) {
                return false;
            }
        }
        self.name.is_ascii() && self.args.is_ascii()
    }
}
