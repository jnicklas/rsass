use std::ascii::AsciiExt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Selectors(pub Vec<Selector>);

impl Selectors {
    pub fn root() -> Self {
        Selectors(vec![Selector::root()])
    }
    pub fn inside(&self, parent: Option<&Self>) -> Self {
        if let Some(parent) = parent {
            let mut result = Vec::new();
            for ref p in &parent.0 {
                for ref s in &self.0 {
                    result.push(p.join(s));
                }
            }
            Selectors(result)
        } else {
            self.clone()
        }
    }

    pub fn is_root(&self) -> bool {
        self.0.len() == 1 && self.0[0].is_root()
    }


    pub fn is_ascii(&self) -> bool {
        self.0.iter().all(|s| s.is_ascii())
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Selector(pub Vec<SelectorPart>);

impl Selector {
    pub fn root() -> Self {
        Selector(vec![])
    }

    pub fn is_root(&self) -> bool {
        self.0.is_empty()
    }

    pub fn join(&self, other: &Selector) -> Selector {
        let mut split = other.0.splitn(2, |p| p == &SelectorPart::BackRef);
        let o1 = split.next().unwrap();
        if let Some(o2) = split.next() {
            let mut result = o1.to_vec();
            result.extend(self.0.iter().cloned());
            result.extend(o2.iter().cloned());
            Selector(result)
        } else {
            let mut result = self.0.clone();
            if !result.is_empty() &&
               !other.0.first().map(|p| p.is_operator()).unwrap_or(false) {
                result.push(SelectorPart::Descendant);
            }
            result.extend(other.0.iter().cloned());
            Selector(result)
        }
    }

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
    fn is_operator(&self) -> bool {
        match *self {
            SelectorPart::Descendant |
            SelectorPart::RelOp(_) => true,
            SelectorPart::Simple(_) |
            SelectorPart::Attribute { .. } |
            SelectorPart::PseudoElement(_) |
            SelectorPart::Pseudo { .. } |
            SelectorPart::BackRef => false,
        }
    }

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn root_join() {
        let s = Selector(vec![SelectorPart::Simple("foo".into())]);
        assert_eq!(Selector::root().join(&s), s)
    }
}
