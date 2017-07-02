use css;
use error::Error;
use file_context::FileContext;
use sass;
use variablescope::GlobalScope;

pub fn compile(_file_context: &FileContext,
               _items: &[sass::Item])
               -> Result<Vec<css::Item>, Error> {
    Ok(vec![])
}
