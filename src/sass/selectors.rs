use sass::InterpolationString;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Selectors(pub Vec<Selector>);

impl Selectors {
    pub fn root() -> Self {
        Selectors(vec![Selector::root()])
    }

    pub fn is_last(&self, selector: &Selector) -> bool {
        self.0.iter().last() == Some(selector)
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
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Selector(pub Vec<SelectorPart>);

impl Selector {
    pub fn root() -> Self {
        Selector(vec![])
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
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum SelectorPart {
    Simple(InterpolationString),
    Descendant,
    RelOp(u8), // >, +, ~
    /// A css3 pseudo-element
    PseudoElement(InterpolationString),
    Attribute { name: InterpolationString, op: String, val: String },
    Pseudo { name: InterpolationString, arg: Option<Selectors> },
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
