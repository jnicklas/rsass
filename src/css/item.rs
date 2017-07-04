use css::{AtRule, Rule, Value};
use std::cmp::{Ordering, PartialOrd};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Item {
    Import(Value),
    AtRule(AtRule),
    Rule(Rule),
    Property(String, Value, bool),
    Comment(String),
}

impl Item {
    pub fn priority(&self) -> u8 {
        match self {
            &Item::Import(_) => 0,
            _ => 1,
        }
    }
}
impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        self.priority().cmp(&other.priority())
    }
}
