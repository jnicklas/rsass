use css::Value;
use selectors::Selectors;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Item {
    Import(Value),
    AtRule { name: String, args: Value, body: Option<Vec<Item>> },
    Rule(Selectors, Vec<Item>),
    Property(String, Value, bool),
    Comment(String),
}