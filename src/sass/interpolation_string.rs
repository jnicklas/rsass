use sass::Value;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InterpolationString(pub Vec<InterpolationStringPart>);

impl InterpolationString {
    pub fn to_string(&self) -> String {
        let parts: Vec<String> = self.0.iter().map(|p| p.to_string()).collect();
        parts.join("")
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum InterpolationStringPart {
    Simple(String),
    Value(Value),
}

impl InterpolationStringPart {
    pub fn to_string(&self) -> String {
        match self {
            &InterpolationStringPart::Simple(ref s) => s.clone(),
            &InterpolationStringPart::Value(ref _v) => {
                panic!("InterpolationString string with interpolations should \
                        be interpolated before being converted to string")
            }
        }
    }
}

impl<'a> From<&'a str> for InterpolationString {
    fn from(value: &'a str) -> Self {
        InterpolationString(vec![value.into()])
    }
}

impl<'a> From<&'a str> for InterpolationStringPart {
    fn from(value: &'a str) -> Self {
        InterpolationStringPart::Simple(value.into())
    }
}
