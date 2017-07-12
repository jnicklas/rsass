use css;
use error::Error;
use sass;
use variablescope::{Scope, ScopeImpl};

pub fn compile_function_items(scope: &mut Scope,
                              sass_items: &[sass::Item])
                              -> Result<Option<css::Value>, Error> {
    for sass_item in sass_items {
        let result = match sass_item {
            &sass::Item::IfStatement(ref cond, ref do_if, ref do_else) => {
                if cond.evaluate(scope).is_true() {
                    compile_function_items(scope, do_if)
                } else {
                    compile_function_items(scope, do_else)
                }
            }
            &sass::Item::Each(ref name, ref values, ref body) => {
                let values = match values.evaluate(scope) {
                    css::Value::List(v, _) => v,
                    v => vec![v],
                };
                for value in values {
                    scope.define(name, &value);
                    if let Some(r) = compile_function_items(scope, body)? {
                        return Ok(Some(r));
                    }
                }
                Ok(None)
            }
            &sass::Item::For {
                ref name,
                ref from,
                ref to,
                inclusive,
                ref body,
            } => {
                let from = from.evaluate(scope).integer_value().unwrap();
                let to = to.evaluate(scope).integer_value().unwrap();
                let to = if inclusive { to + 1 } else { to };
                for value in from..to {
                    scope.define(name, &css::Value::scalar(value));
                    if let Some(r) = compile_function_items(scope, body)? {
                        return Ok(Some(r));
                    }
                }
                Ok(None)
            }
            &sass::Item::VariableDeclaration {
                ref name,
                ref val,
                default,
                global,
            } => {
                let val = val.evaluate(scope);
                if default {
                    scope.define_default(name, &val, global);
                } else if global {
                    scope.define_global(name, &val);
                } else {
                    scope.define(name, &val);
                }
                Ok(None)
            }
            &sass::Item::Return(ref v) => Ok(Some(v.evaluate(scope))),
            &sass::Item::While(ref cond, ref body) => {
                let mut scope = ScopeImpl::sub(scope);
                while cond.evaluate(&scope).is_true() {
                    if let Some(r) = compile_function_items(&mut scope, body)? {
                        return Ok(Some(r));
                    }
                }
                Ok(None)
            }
            &sass::Item::None => Ok(None),
            x => {
                panic!("Not implemented in function: {:?}", x);
            }
        };
        if let Some(value) = result? {
            return Ok(Some(value));
        }
    }
    Ok(None)
}
