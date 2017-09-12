use std::ascii::AsciiExt;
use std::fmt;
use std::io::Write;

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
            SelectorPart::Pseudo { ref name, arg: Some(ref arg) } => {
                name.is_ascii() && arg.is_ascii()
            }
            SelectorPart::Pseudo { ref name, arg: None } => name.is_ascii(),
        }
    }
}

impl fmt::Display for Selectors {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        if let Some((first, rest)) = self.0.split_first() {
            first.fmt(out)?;
            let separator = if out.alternate() { "," } else { ", " };
            for item in rest {
                out.write_str(separator)?;
                item.fmt(out)?;
            }
        }
        Ok(())
    }
}

impl fmt::Display for Selector {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        // Note: There should be smarter whitespace-handling here, avoiding
        // the need to clean up afterwards.
        let mut buf = vec![];
        for p in &self.0 {
            if out.alternate() {
                write!(&mut buf, "{:#}", p).map_err(|_| fmt::Error)?;
            } else {
                write!(&mut buf, "{}", p).map_err(|_| fmt::Error)?;
            }
        }
        while buf.last() == Some(&b' ') {
            buf.pop();
        }
        while buf.first() == Some(&b' ') {
            buf.remove(0);
        }
        let buf = String::from_utf8(buf).map_err(|_| fmt::Error)?;
        out.write_str(&buf.replace("  ", " "))
    }
}

impl fmt::Display for SelectorPart {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SelectorPart::Simple(ref s) => write!(out, "{}", s),
            SelectorPart::Descendant => write!(out, " "),
            SelectorPart::RelOp(ref c) => {
                if out.alternate() && *c != b'~' {
                    write!(out, "{}", *c as char)
                } else {
                    write!(out, " {} ", *c as char)
                }
            }
            SelectorPart::Attribute { ref name, ref op, ref val } => {
                write!(out, "[{}{}{}]", name, op, val)
            }
            SelectorPart::PseudoElement(ref name) => write!(out, "::{}", name),
            SelectorPart::Pseudo { ref name, ref arg } => {
                if let Some(ref arg) = *arg {
                    // It seems some pseudo-classes should always have
                    // their arg in compact form.  Maybe we need more
                    // hard-coded names here, or maybe the condition
                    // should be on the argument rather than the name?
                    if out.alternate() || name == "nth-child" ||
                       name == "nth-of-type" {
                        write!(out, ":{}({:#})", name, arg)
                    } else {
                        write!(out, ":{}({})", name, arg)
                    }
                } else {
                    write!(out, ":{}", name)
                }
            }
            SelectorPart::BackRef => write!(out, "&"),
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
