use css;
use output_style::OutputStyle;
use selectors::Selectors;
use std::io;
use writer::*;

pub fn write_at_rule(out: &mut io::Write,
                     style: OutputStyle,
                     parent_selectors: &Selectors,
                     at_rule: &css::AtRule)
                     -> io::Result<()> {
    write!(out, "{: <1$}", "", style.indentation())?;
    write!(out, "@{} {}", at_rule.name, at_rule.args)?;

    if let &Some(ref body) = &at_rule.body {

        if style.is_compressed() {
            write!(out, "{{")?;
        } else {
            write!(out, " {{")?;
        }
        write!(out, "{}", style.rule_opening_separator())?;

        let body_rule = css::Rule::new(parent_selectors.clone(), body.clone());

        write_rule(out, style.indent(), &body_rule)?;

        write!(out, "{: <1$}", "", style.indentation())?;
        write!(out, "}}")?;
        write!(out, "{}", style.item_separator())?;
    } else {
        write!(out, ";")?;
    }

    Ok(())
}
