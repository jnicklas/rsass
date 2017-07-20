use css::Value;
use std::ascii::AsciiExt;
use std::default::Default;
use std::fmt;

/// the actual arguments of a function or mixin call.
///
/// Each argument has a Value.  Arguments may be named.
/// If the optional name is None, the argument is positional.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CallArgs(pub Vec<(Option<String>, Value)>);

impl CallArgs {
    pub fn new(v: Vec<(Option<String>, Value)>) -> Self {
        CallArgs(v)
    }

    pub fn from_value(v: Value) -> Self {
        match v {
            Value::List(v, _) => {
                CallArgs(v.into_iter().map(|v| (None, v)).collect())
            }
            v => CallArgs(vec![(None, v)]),
        }
    }

    pub fn iter(&self) -> ::std::slice::Iter<(Option<String>, Value)> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn get(&self, index: usize) -> Option<&(Option<String>, Value)> {
        self.0.get(index)
    }

    pub fn is_ascii(&self) -> bool {
        self.0
            .iter()
            .all(|&(ref s, ref v)| match s {
                     &Some(ref s) => s.is_ascii() && v.is_ascii(),
                     &None => v.is_ascii(),
                 })
    }
}

impl Default for CallArgs {
    fn default() -> Self {
        CallArgs(vec![])
    }
}

impl fmt::Display for CallArgs {
    fn fmt(&self, out: &mut fmt::Formatter) -> fmt::Result {
        let t = self.0
            .iter()
            .map(|kv| match *kv {
                     (Some(ref k), ref v) => format!("${}: {}", k, v),
                     (None, ref v) => format!("{}", v),
                 })
            .collect::<Vec<_>>()
            .join(", ");
        write!(out, "{}", t)
    }
}
