use css::{AtRule, Rule, Value};
use std::ascii::AsciiExt;
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

    pub fn is_ascii(&self) -> bool {
        match self {
            &Item::Import(ref value) => value.is_ascii(),
            &Item::AtRule(ref at_rule) => at_rule.is_ascii(),
            &Item::Rule(ref rule) => rule.is_ascii(),
            &Item::Property(ref name, ref value, _) => {
                name.is_ascii() && value.is_ascii()
            }
            &Item::Comment(ref c) => c.is_ascii(),
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
