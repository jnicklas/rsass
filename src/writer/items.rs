use css;
use output_style::OutputStyle;
use std::io;
use writer::*;

pub fn write_items(out: &mut io::Write,
                   style: OutputStyle,
                   items: &[css::Item])
                   -> io::Result<()> {

    let root_items: Vec<&css::Item> = items
        .iter()
        .filter(|item| item.is_root_item(style.include_comments()))
        .collect();

    if !root_items.iter().all(|i| i.is_ascii()) {
        write!(out, "{}", style.byte_order_mark())?;
    }

    for (index, item) in root_items.iter().enumerate() {
        match *item {
            &css::Item::Import(ref value) => {
                write!(out, "@import {}", value)?;
                if style.include_trailing_semicolon() ||
                   index != (root_items.len() - 1) {
                    write!(out, ";")?;
                }
                write!(out, "{}", style.item_separator())?;
            }
            &css::Item::Rule(ref rule) => {
                write_rule(out, style, rule)?;
                if index != (items.len() - 1) {
                    write!(out, "{}", style.item_separator())?;
                }
            }
            &css::Item::AtRule(ref at_rule) => {
                write_root_at_rule(out, style, &at_rule)?;
            }
            &css::Item::Comment(ref c) => {
                write_comment(out, style, c)?;
                write!(out, "{}", style.item_separator())?;
            }
            &css::Item::Property(ref name, ref value, important) => {
                // TODO: this should raise an error if we are not in an @ rule.
                write_property(out, style, name, value, important)?;
                if style.include_trailing_semicolon() ||
                   index != (root_items.len() - 1) {
                    write!(out, ";")?;
                }
                write!(out, "{}", style.item_separator())?;
            }
        }
    }

    write!(out, "{}", style.end_of_file_separator())?;

    Ok(())
}
