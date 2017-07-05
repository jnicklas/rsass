use css;
use output_style::OutputStyle;
use std::io;

pub fn write_property(out: &mut io::Write,
                      style: OutputStyle,
                      name: &str,
                      value: &css::Value,
                      important: bool)
                      -> io::Result<()> {
    write!(out, "{: <1$}", "", style.indentation())?;
    write!(out, "{}:{}", name, style.property_separator())?;
    if style.is_compressed() {
        write!(out, "{:#}", value)?;
    } else {
        write!(out, "{}", value)?;
    }
    if important {
        write!(out, "{}!important", style.important_separator())?;
    }
    Ok(())
}
