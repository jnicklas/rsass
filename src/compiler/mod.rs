use css;
use error::Error;
use file_context::FileContext;
use sass;
use variablescope::GlobalScope;

mod item;

pub fn compile(file_context: &FileContext,
               sass_items: &[sass::Item])
               -> Result<Vec<css::Item>, Error> {
    let mut scope = GlobalScope::new();

    item::compile_root_items(file_context, &mut scope, sass_items)
}
