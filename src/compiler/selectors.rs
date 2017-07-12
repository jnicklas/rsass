use css;
use error::Error;
use sass;
use variablescope::Scope;

pub fn compile_selectors(scope: &mut Scope,
                         selectors: &sass::Selectors)
                         -> Result<css::Selectors, Error> {

    let mut inner = Vec::with_capacity(selectors.0.len());
    for selector in &selectors.0 {
        inner.push(compile_selector(scope, selector)?)
    }
    Ok(css::Selectors(inner))
}

pub fn compile_selector(scope: &mut Scope,
                        selector: &sass::Selector)
                        -> Result<css::Selector, Error> {
    let mut parts = Vec::with_capacity(selector.0.len());
    for part in &selector.0 {
        parts.push(compile_selector_part(scope, part)?)
    }
    Ok(css::Selector(parts))
}

pub fn compile_selector_part(scope: &mut Scope,
                             part: &sass::SelectorPart)
                             -> Result<css::SelectorPart, Error> {
    match part {
        &sass::SelectorPart::Simple(ref string) => {
            Ok(css::SelectorPart::Simple(string.clone()))
        }
        &sass::SelectorPart::Descendant => Ok(css::SelectorPart::Descendant),
        &sass::SelectorPart::RelOp(num) => Ok(css::SelectorPart::RelOp(num)),
        &sass::SelectorPart::Attribute { ref name, ref op, ref val } => {
            Ok(css::SelectorPart::Attribute {
                   name: name.clone(),
                   op: op.clone(),
                   val: val.clone(),
               })
        }
        &sass::SelectorPart::PseudoElement(ref name) => {
            Ok(css::SelectorPart::PseudoElement(name.clone()))
        }
        &sass::SelectorPart::Pseudo { ref name, ref arg } => {
            let arg = match arg {
                &Some(ref s) => Some(compile_selectors(scope, s)?),
                &None => None,
            };
            Ok(css::SelectorPart::Pseudo { name: name.clone(), arg: arg })
        }
        &sass::SelectorPart::BackRef => Ok(css::SelectorPart::BackRef),
    }
}
