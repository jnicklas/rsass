use css::{AtRule, Rule, Value};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Item {
    Import(Value),
    AtRule(AtRule),
    Rule(Rule),
    Property(String, Value, bool),
    Comment(String),
}
