//! Sass reimplemented in rust with nom.
//!
//! The "r" in the name might stand for the Rust programming language,
//! or for my name Rasmus.
//!
//! # Example
//!
//! ```
//! use std::io::Write;
//! use rsass::{OutputStyle, compile_scss_file};
//!
//! let mut buffer = Vec::new();
//! let path = "tests/basic/14_imports/a.scss";
//! let style = OutputStyle::Compressed;
//! compile_scss_file(path.as_ref(), &mut buffer, style).unwrap();
//! assert_eq!(&buffer, b"div span{moo:goo}\n");
//! ```
//!
//! # Sass language and implemetation status
//!
//! The sass language [is defined in its reference
//! doc](http://sass-lang.com/documentation/file.SASS_REFERENCE.html).
//! This implementation is incomplete but getting there, if slowly.
//!
//! Progress: ![1146](http://progressed.io/bar/114?scale=331&suffix=6)
//! of 3310 tests passed
//! (or 1215 of 6065 when claiming to be libsass).
//!
//! If you want a working rust library for sass right now, you will
//! probably be better of with [sass-rs](https://crates.io/crates/sass-rs)
//! which is a rust wrapper around libsass.
//! Another alternative is [sassers](https://crates.io/crates/sassers)
//! which is another early stage pure rust implementation.
//! That said, this implementation has reached a version where I find it
//! usable for my personal projects, and the number of working tests are
//! improving.
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate nom;
extern crate num_rational;
extern crate num_traits;
extern crate rand;

mod error;
mod functions;
mod value;
mod variablescope;
mod output_style;
mod parser;
mod file_context;
pub mod compiler;
pub mod writer;

pub mod sass;
pub mod css;

pub use error::Error;

pub use file_context::FileContext;
pub use functions::SassFunction;
pub use num_rational::Rational;
pub use output_style::OutputStyle;
pub use parser::{parse_scss_data, parse_scss_file, parse_value_data};
pub use sass::Item;
use std::io;
use std::path::Path;

pub use value::{ListSeparator, Quotes, Unit};
pub use variablescope::{GlobalScope, Scope};

/// Parse scss data from a buffer and write css in the given style.
///
/// # Example
///
/// ```
/// use rsass::compile_value;
///
/// assert_eq!(compile_value(b"10px + 4px").unwrap(), b"14px");
/// ```
pub fn compile_value(input: &[u8]) -> Result<Vec<u8>, Error> {
    let scope = GlobalScope::new();
    let value = parse_value_data(input)?;
    let buffer = format!("{}", value.evaluate(&scope)).into_bytes();
    Ok(buffer)
}

/// Parse scss data from a buffer and write css in the given style.
///
/// # Example
///
/// ```
/// ```
/// use std::io::Write;
/// use rsass::{OutputStyle, compile_scss_file};
///
/// let mut buffer = Vec::new();
/// let path = "tests/basic/14_imports/a.scss";
/// let style = OutputStyle::Compressed;
/// let input = b"foo {\n\
///                 bar {\n\
///                   baz:value;\n\
///                 }\n\
///               }"
/// compile_scss_file(input, &mut buffer, style).unwrap();
/// assert_eq!(&buffer, b"foo bar{baz:value}\n");
/// ```
pub fn compile_scss(input: &[u8],
                    output: &mut io::Write,
                    style: OutputStyle)
                    -> Result<(), Error> {
    let file_context = FileContext::new();
    let sass_items = parse_scss_data(input)?;
    let css_items = compiler::compile(&file_context, &sass_items)?;

    writer::write(output, style, &css_items)?;
    Ok(())
}

/// Parse a file of scss data and write css in the given style.
///
/// Any `@import` directives will be handled relative to the directory
/// part of `file`.
///
/// # Example
///
/// ```
/// use std::io::Write;
/// use rsass::{OutputStyle, compile_scss_file};
///
/// let mut buffer = Vec::new();
/// let path = "tests/basic/14_imports/a.scss";
/// let style = OutputStyle::Compressed;
/// compile_scss_file(path.as_ref(), &mut buffer, style).unwrap();
/// assert_eq!(&buffer, b"div span{moo:goo}\n");
/// ```
pub fn compile_scss_file(file: &Path,
                         output: &mut io::Write,
                         style: OutputStyle)
                         -> Result<(), Error> {
    let file_context = FileContext::new();
    let (sub_context, file) = file_context.file(file);
    let sass_items = parse_scss_file(&file)?;
    let css_items = compiler::compile(&sub_context, &sass_items)?;

    writer::write(output, style, &css_items)?;
    Ok(())
}
