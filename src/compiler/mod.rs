use css;
use error::Error;
use file_context::FileContext;
use sass;
use variablescope::{GlobalScope, Scope};

mod root_items;
mod body_items;
mod function_items;
mod selectors;
mod interpolate_selectors;

pub fn compile(file_context: &FileContext,
               sass_items: &[sass::Item])
               -> Result<Vec<css::Item>, Error> {
    compile_in_scope(file_context, &mut GlobalScope::new(), sass_items)
}

pub fn compile_in_scope(file_context: &FileContext,
                        scope: &mut Scope,
                        sass_items: &[sass::Item])
                        -> Result<Vec<css::Item>, Error> {
    root_items::compile_root_items(file_context, scope, sass_items)
}

pub fn compile_value(scope: &mut Scope,
                     sass_items: &[sass::Item])
                     -> Result<css::Value, Error> {
    let result = function_items::compile_function_items(scope, sass_items)?;
    Ok(result.unwrap_or(css::Value::Null))
}
