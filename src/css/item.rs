use css::{Rule, Value};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Item {
    Import(Value),
    AtRule { name: String, args: Value, body: Option<Vec<Item>> },
    Rule(Rule),
    Property(String, Value, bool),
    Comment(String),
}
