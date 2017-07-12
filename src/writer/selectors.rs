use output_style::OutputStyle;
use selectors::{Selector, SelectorPart, Selectors};
use std::io;

pub fn write_selectors(out: &mut io::Write,
                       style: OutputStyle,
                       selectors: &Selectors)
                       -> io::Result<()> {
    if let Some((first, rest)) = selectors.0.split_first() {
        write_selector(out, style, first)?;
        for item in rest {
            write!(out, ",{}", style.selector_separator())?;
            write_selector(out, style, item)?;
        }
    }
    Ok(())
}

pub fn write_selector(out: &mut io::Write,
                      style: OutputStyle,
                      selector: &Selector)
                      -> io::Result<()> {
    // Note: There should be smarter whitespace-handling here, avoiding
    // the need to clean up afterwards.
    let mut buf = vec![];
    for p in &selector.0 {
        write_selector_part(&mut buf, style, p)?;
    }
    while buf.last() == Some(&b' ') {
        buf.pop();
    }
    while buf.first() == Some(&b' ') {
        buf.remove(0);
    }
    let buf = String::from_utf8(buf).unwrap();
    write!(out, "{}", &buf.replace("  ", " "))?;

    Ok(())
}

pub fn write_selector_part(out: &mut io::Write,
                           style: OutputStyle,
                           part: &SelectorPart)
                           -> io::Result<()> {
    match *part {
        SelectorPart::Simple(ref s) => {
            write!(out, "{}", s)?;
        }
        SelectorPart::Descendant => {
            write!(out, " ")?;
        }
        SelectorPart::RelOp(ref c) => {
            if *c == b'~' {
                write!(out, " {} ", *c as char)?;
            } else {
                let sep = style.selector_separator();
                write!(out, "{}{}{}", sep, *c as char, sep)?;
            }
        }
        SelectorPart::Attribute { ref name, ref op, ref val } => {
            write!(out, "[{}{}{}]", name, op, val)?;
        }
        SelectorPart::PseudoElement(ref name) => {
            write!(out, "::{}", name)?;
        }
        SelectorPart::Pseudo { ref name, ref arg } => {
            if let Some(ref arg) = *arg {
                write!(out, ":{}(", name)?;
                // It seems some pseudo-classes should always have
                // their arg in compact form.  Maybe we need more
                // hard-coded names here, or maybe the condition
                // should be on the argument rather than the name?
                if name == "nth-child" || name == "nth-of-type" {
                    write_selectors(out, OutputStyle::Compressed, arg)?;
                } else {
                    write_selectors(out, style, arg)?;
                }
                write!(out, ")")?;
            } else {
                write!(out, ":{}", name)?;
            }
        }
        SelectorPart::BackRef => {
            write!(out, "&")?;
        }
    }
    Ok(())
}
