use css;
use output_style::OutputStyle;
use selectors::Selectors;
use std::io;
use writer::*;

pub fn write_root_at_rule(out: &mut io::Write,
                          style: OutputStyle,
                          at_rule: &css::AtRule)
                          -> io::Result<()> {
    write_at_rule_with_body_fun(out, style, at_rule, write_items)
}

pub fn write_at_rule(out: &mut io::Write,
                     style: OutputStyle,
                     parent_selectors: &Selectors,
                     at_rule: &css::AtRule)
                     -> io::Result<()> {
    write_at_rule_with_body_fun(out, style, at_rule, |out, style, body| {
        let body_rule = css::Rule::new(parent_selectors.clone(), body.to_vec());
        write_rule(out, style, &body_rule)
    })
}

fn write_at_rule_with_body_fun<F>(out: &mut io::Write,
                                  style: OutputStyle,
                                  at_rule: &css::AtRule,
                                  fun: F)
                                  -> io::Result<()>
    where F: Fn(&mut io::Write, OutputStyle, &[css::Item]) -> io::Result<()>
{
    write!(out, "{: <1$}", "", style.indentation())?;
    write!(out, "@{}", at_rule.name)?;
    if !at_rule.args.is_null() {
        write!(out, " {}", at_rule.args)?;
    }

    if let &Some(ref body) = &at_rule.body {

        if style.is_compressed() {
            write!(out, "{{")?;
        } else {
            write!(out, " {{")?;
        }
        write!(out, "{}", style.rule_opening_separator())?;

        fun(out, style.indent(), body)?;

        write!(out, "{: <1$}", "", style.indentation())?;
        write!(out, "}}")?;
    } else {
        write!(out, ";")?;
    }
    write!(out, "{}", style.item_separator())?;

    Ok(())
}
