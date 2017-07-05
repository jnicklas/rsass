use output_style::OutputStyle;
use std::io;

pub fn write_comment(out: &mut io::Write,
                     style: OutputStyle,
                     comment: &str)
                     -> io::Result<()> {
    write!(out, "{:<1$}", "", style.indentation())?;
    write!(out, "/*{}*/", comment)?;
    Ok(())
}
