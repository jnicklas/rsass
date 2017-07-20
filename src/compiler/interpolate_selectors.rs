use error::Error;
use parser::parse_selector_data;
use sass;
use std::io::Write;
use variablescope::Scope;

pub fn interpolate_selectors(scope: &mut Scope,
                             selectors: &sass::Selectors)
                             -> Result<sass::Selectors, Error> {
    let mut buffer = Vec::new();
    write_selectors(&mut buffer, scope, selectors)?;
    write!(&mut buffer, ";")?;

    let selectors = parse_selector_data(&buffer)?;
    Ok(selectors)
}

fn write_selectors(buffer: &mut Vec<u8>,
                   scope: &mut Scope,
                   selectors: &sass::Selectors)
                   -> Result<(), Error> {
    for selector in &selectors.0 {
        write_selector(buffer, scope, selector)?;
        if !selectors.is_last(selector) {
            write!(buffer, ",")?;
        }
    }
    Ok(())
}

fn write_selector(buffer: &mut Vec<u8>,
                  scope: &mut Scope,
                  selector: &sass::Selector)
                  -> Result<(), Error> {
    for part in &selector.0 {
        write_selector_part(buffer, scope, part)?
    }
    Ok(())
}

fn write_selector_part(buffer: &mut Vec<u8>,
                       scope: &mut Scope,
                       part: &sass::SelectorPart)
                       -> Result<(), Error> {
    match part {
        &sass::SelectorPart::Simple(ref string) => {
            write_interpolation_string(buffer, scope, string)?;
        }
        &sass::SelectorPart::Attribute { ref name, ref op, ref val } => {
            write!(buffer, "[")?;
            write_interpolation_string(buffer, scope, name)?;
            write!(buffer, "{}{}]", op, val)?;
        }
        &sass::SelectorPart::PseudoElement(ref name) => {
            write!(buffer, "::")?;
            write_interpolation_string(buffer, scope, name)?;
        }
        &sass::SelectorPart::Pseudo { ref name, ref arg } => {
            write!(buffer, ":")?;
            write_interpolation_string(buffer, scope, name)?;
            if let Some(ref arg) = *arg {
                write!(buffer, "(")?;
                write_selectors(buffer, scope, arg)?;
                write!(buffer, ")")?;
            }
        }
        &sass::SelectorPart::Descendant => {
            write!(buffer, " ")?;
        }
        &sass::SelectorPart::RelOp(c) => {
            write!(buffer, " {} ", c as char)?;
        }
        &sass::SelectorPart::BackRef => {
            write!(buffer, "&")?;
        }
    }

    Ok(())
}

pub fn write_interpolation_string(buffer: &mut Vec<u8>,
                                  scope: &mut Scope,
                                  string: &sass::InterpolationString)
                                  -> Result<(), Error> {

    for part in &string.0 {
        match part {
            &sass::InterpolationStringPart::Simple(ref string) => {
                write!(buffer, "{}", string)?;
            }
            &sass::InterpolationStringPart::Value(ref sass_value) => {
                let css_value = sass_value.do_evaluate(scope, true);
                write!(buffer, "{}", css_value)?;
            }
        }
    }

    Ok(())
}
