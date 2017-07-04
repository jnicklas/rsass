use css::Item;
use selectors::Selectors;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Rule {
    pub selectors: Selectors,
    pub body: Vec<Item>,
}

impl Rule {
    pub fn new(selectors: Selectors, body: Vec<Item>) -> Self {
        Rule { selectors: selectors, body: body }
    }

    pub fn is_ascii(&self) -> bool {
        self.selectors.is_ascii() && self.body.iter().all(|v| v.is_ascii())
    }
}
