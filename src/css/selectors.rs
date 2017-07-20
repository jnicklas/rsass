use std::ascii::AsciiExt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Selectors(pub Vec<Selector>);

impl Selectors {
    pub fn is_ascii(&self) -> bool {
        self.0.iter().all(|s| s.is_ascii())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Selector(pub Vec<SelectorPart>);

impl Selector {
    pub fn is_ascii(&self) -> bool {
        self.0.iter().all(|s| s.is_ascii())
    }
}

/// A selector consist of a sequence of these parts.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SelectorPart {
    Simple(String),
    Descendant,
    RelOp(u8), // >, +, ~
    Attribute { name: String, op: String, val: String },
    /// A css3 pseudo-element
    PseudoElement(String),
    /// A pseudo-class or a css2 pseudo-element
    Pseudo { name: String, arg: Option<Selectors> },
    BackRef,
}

impl SelectorPart {
    pub fn is_ascii(&self) -> bool {
        match *self {
            SelectorPart::Descendant |
            SelectorPart::BackRef |
            SelectorPart::RelOp(_) => true,
            SelectorPart::Simple(ref string) => string.is_ascii(),
            SelectorPart::Attribute { ref name, ref op, ref val } => {
                name.is_ascii() && op.is_ascii() && val.is_ascii()
            }
            SelectorPart::PseudoElement(ref name) => name.is_ascii(),
            SelectorPart::Pseudo { ref name, arg: Some(ref arg) } => {
                name.is_ascii() && arg.is_ascii()
            }
            SelectorPart::Pseudo { ref name, arg: None } => name.is_ascii(),
        }
    }
}
