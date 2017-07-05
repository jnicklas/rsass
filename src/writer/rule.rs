use css;
use output_style::OutputStyle;
use std::io;
use writer::*;

pub fn write_rule(out: &mut io::Write,
                  style: OutputStyle,
                  rule: &css::Rule)
                  -> io::Result<()> {

    let body_items: Vec<&css::Item> = rule.body
        .iter()
        .filter(|item| item.is_body_item(style.include_comments()))
        .collect();

    if !body_items.is_empty() {
        write!(out, "{: <1$}", "", style.indentation())?;

        if style.is_compressed() {
            write!(out, "{:#}{{", rule.selectors)?;
        } else {
            write!(out, "{} {{", rule.selectors)?;
        }

        write!(out, "{}", style.rule_opening_separator())?;
        write_rule_body_items(out, style.indent(), &body_items)?;

        write!(out, "{: <1$}", "", style.indentation())?;
        write!(out, "}}{}", style.item_separator())?;
    }

    for item in &rule.body {
        match item {
            &css::Item::Rule(ref rule) => {
                write_rule(out, style, rule)?;
            }
            &css::Item::AtRule(ref at_rule) => {
                write_at_rule(out, style, &rule.selectors, at_rule)?;
            }
            _ => {} // do nothing
        }
    }

    Ok(())
}

fn write_rule_body_items(out: &mut io::Write,
                         style: OutputStyle,
                         body_items: &[&css::Item])
                         -> io::Result<()> {
    for (index, item) in body_items.iter().enumerate() {
        match *item {
            &css::Item::Property(ref name, ref value, important) => {
                write_property(out, style, name, value, important)?;
                if style.include_trailing_semicolon() ||
                   index != (body_items.len() - 1) {
                    write!(out, ";")?;
                }
            }
            &css::Item::Comment(ref c) => {
                write_comment(out, style, c)?;
            }
            _ => panic!("not a body item: {:?}", item),
        }
        write!(out, "{}", style.item_separator())?;
    }
    Ok(())
}
